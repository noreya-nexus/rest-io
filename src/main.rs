#[macro_use] extern crate rocket;
#[macro_use] extern crate log;

use input::*;
use std::{env, thread};
use rocket::log::LogLevel;
use std::process::exit;
use noreya_sdbp::drv::api::Manager;
use noreya_sdbp::util::logging::init_systemd_logger;
use crate::settings::SOCKET_PATH;
use std::time::Duration;
use crate::api::notification::Notifications;
use std::sync::Mutex;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::collections::HashMap;
use sd_notify::{NotifyState};

mod api;
mod input;
mod settings;
mod response;
mod api_version;
mod error_catcher;

#[derive(Debug)]
pub struct SharedData {
    notifications: Mutex<HashMap<u16,Notifications>>,
    driver_session: Mutex<String>,
    power_mgmt_lock: Mutex<bool>
}

#[rocket::main]
async fn main() {
    let settings = settings::Settings::default();

    init_systemd_logger();
    let version = env!("CARGO_PKG_VERSION");
    info!("Version: {:?}",version);

    let mut cnt: u32 = 0;
    loop {
        match check_version() {
            Ok(_) => {break}
            Err(err) => {
                debug!("Could not check driver version ({})", err);
                if cnt > 10 {
                    error!("{}", err);
                    exit(1);
                }
                cnt += 1;
                thread::sleep(Duration::from_millis(500));
            }
        }
    }

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let shared = SharedData {
        notifications: Mutex::new( HashMap::new()),
        driver_session: Mutex::new(rand_string.to_ascii_uppercase()),
        power_mgmt_lock: Mutex::new(true)
    };

    info!("Start REST-API using: {}", settings.socket_path());
    let routes = routes![
        api::generic::index,
        api::generic::get_descriptor,
        api::powermgmt::powermgmt,
        api::input::set_input_mode,
        api::input::set_analog_threshold,
        api::input::set_digital_interrupt,
        api::input::set_digital_counter,
        api::input::get_values,
        api::input::get_current_values,
        api::output_state::set_output_state,
        api::output_pwm::set_output_pwm,
        api::notification::get_notification,
        api::reset::module_reset,
    ];
    let figment = rocket::Config::figment()
        .merge(("address", "127.0.0.1"))
        .merge(("log_level", parse_level()))
        .merge(("port", parse_port()));

    let result = rocket::custom(figment)
        .mount("/api", routes)
        .register("/",catchers![error_catcher::not_found, error_catcher::unprocessable_entity, error_catcher::internal_server_error, error_catcher::bad_gateway, error_catcher::bad_request])
        .manage(settings)
        .manage(shared)
        .launch();

    let _ = sd_notify::notify(false, &[NotifyState::Ready]);
    let _ = sd_notify::notify(false, &[NotifyState::Status("Waiting for requests...")]);
    if let Err(e) = result.await {
        error!("This rocket did not launch:");
        drop(e);
    };
    let _ = sd_notify::notify(false, &[NotifyState::Stopping]);
    let _ = sd_notify::notify(false, &[NotifyState::Status("Service stopped successfully")]);
}

fn parse_port() -> u16 {
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_e) => "none".to_string(),
    };

    let port = match port.parse::<u16>() {
        Ok(val) => val,
        Err(_e) => panic!("Invalid port number!"),
    };

    return port
}

fn parse_level() -> LogLevel {
    let log_level = match env::var("RUST_APP_LOG") {
        Ok(val) => val,
        Err(_e) => "none".to_string(),
    };

    let log_level = match log_level.as_str() {
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Normal,
        "critical" => LogLevel::Critical,
        _ => LogLevel::Off
    };

    return log_level
}

fn check_version() -> Result<(), String> {
    let mut com_manager = match Manager::new(SOCKET_PATH.to_string(), Some(Duration::from_secs(10))) {
        Ok(value) => value,
        Err(_err) => return Err("Could not connect to driver".to_string()),
    };

    let drv_info = match com_manager.get_info() {
        Ok(drv_info) => { drv_info }
        Err(_) => return Err("Failed getting device info".to_string()),
    };

    const COMPATIBLE_MAJOR: u16 = 1;
    const COMPATIBLE_MINOR: u16 = 0;

    let module_driver = drv_info.clone().get_version();
    if module_driver.major() != COMPATIBLE_MAJOR {
        return Err("Driver version incompatible (major)".to_string());
    }

    if module_driver.minor() < COMPATIBLE_MINOR {
        return Err("Driver version incompatible (minor)".to_string());
    }

    Ok(())
}
