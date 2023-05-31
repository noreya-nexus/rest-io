# IO Module RESTful API driver
[![Build](https://github.com/noreya-nexus/rest-io/actions/workflows/build.yml/badge.svg)](https://github.com/noreya-nexus/rest-io/actions/workflows/build.yml)

This driver provides an HTTP RESTful API and is written in [Rust](https://www.rust-lang.org/) using [Rocket](https://rocket.rs/).  
It uses the [IO Module driver](https://github.com/noreya-nexus/drv-io.git) and makes all [IO Module](https://noreya-nexus.tech/en/modules/io/)
functions available.

The API is stable and should be used for remote as well as local applications.  
The entire documentation can be found [here](https://doc.noreya-nexus.tech/en/module-restful-api/io-module/).  
The driver currently does not support authentication/session handling. The authentication is done via proxy settings.

## Building
To build this project for the target platform the "aarch64-unknown-linux-gnu" target must be installed via *rustup*.    
The "aarch64-linux-gnu-gcc" linker must also be configured (check the Dockerfile).
```bash
cargo build --target=aarch64-unknown-linux-gnu
```
The project can be build directly on the Nexus if Rust is installed, but it will take some time:
```bash
cargo build
```
### Docker
There is a Dockerfile in the project which allows you to build the project for armv8/arm64:
```bash
docker buildx build --platform linux/arm64 -t rust-cross-build .
docker run --platform linux/arm64 -t --rm -w "$PWD" -v "$PWD:$PWD":rw,z rust-cross-build cargo build --target=aarch64-unknown-linux-gnu --release
docker run --platform linux/arm64 -t --rm -w "$PWD" -v "$PWD:$PWD":rw,z rust-cross-build ./makedeb_github.sh
```

## Executing
The application must be executed on the Nexus and the *nexus-drv-io* service must be running.   
Please ensure the *nexus-rest-io* service is stopped.
```bash
RUST_APP_LOG="info" ROCKET_PROFILE=production PORT=8003 ./nexus-rest-bmc
```

## Packaging
We do not build Debian packages on Github because the aarch64 architecture is not supported.  
Please check the [packaging guide](https://doc.noreya-nexus.tech/en/technical-details/packaging/guide/) for details.

## Testing
The testsuite is currently not published because it relies on hardware, physical test tools and a specific wiring.

## License
This driver is licensed under [GPLv3](LICENSE).
