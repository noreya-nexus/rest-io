#!/bin/bash
set -e

APP_VERSION="$(/usr/bin/python3 getversion.py)"

VERSION="$APP_VERSION"
DEB_VERSION="9999"
PACKAGE="nexus-rest-power"
PACKAGE_NAME="$PACKAGE"_"$VERSION"
YEAR=$(date +"%Y")
export DEBFULLNAME="Unofficial"
export DEBEMAIL="nobody@example.com"

# Clean and create build dir
rm -Rf builddeb/ || true
mkdir builddeb/
cd builddeb/


mkdir -p "$PACKAGE_NAME/lib/systemd/system/"
mkdir -p "$PACKAGE_NAME/usr/bin/"
mkdir -p "$PACKAGE_NAME/etc/nginx/sites-available/nexus/upstream"
mkdir -p "$PACKAGE_NAME/etc/nginx/sites-enabled-subdir/nexus/upstream"

cp ../systemd/$PACKAGE.service "$PACKAGE_NAME/lib/systemd/system/"
if [ -f "../target/release/$PACKAGE" ]; then
  cp ../target/release/$PACKAGE "$PACKAGE_NAME/usr/bin/"
else
  cp ../target/"$(arch)"-unknown-linux-gnu/release/$PACKAGE "$PACKAGE_NAME/usr/bin/"
fi
cp ../nginx/$PACKAGE.conf "$PACKAGE_NAME/etc/nginx/sites-available/nexus"
cp ../nginx/$PACKAGE-upstream.conf "$PACKAGE_NAME/etc/nginx/sites-available/nexus/upstream"

cd "$PACKAGE_NAME"

# Create manifest
mkdir debian
dch --create -v "$VERSION"-"$DEB_VERSION" --package "$PACKAGE" --distribution stable "Please visit https://github.com/noreya-nexus/rest-io for details!"
echo "10" > ./debian/compat

echo "Source: $PACKAGE" > ./debian/control
echo "Maintainer: $DEBFULLNAME <$DEBEMAIL>" >> ./debian/control
echo "Section: misc" >> ./debian/control
echo "Priority: optional" >> ./debian/control
echo "Standards-Version: 3.9.2" >> ./debian/control
echo "Build-Depends: debhelper (>= 10)" >> ./debian/control
echo "" >> ./debian/control
echo "Package: $PACKAGE" >> ./debian/control
echo "Architecture: any" >> ./debian/control
echo "Depends: \${misc:Depends}, \${shlibs:Depends}, bash (>= 2.1.0), systemd, nginx, nexus-nginx-config, nexus-drv-io" >> ./debian/control
echo "Description: IO Module REST API" >> ./debian/control
echo "	Webserver providing a REST API" >> ./debian/control

# Copyright
echo "Copyright $YEAR $DEBFULLNAME $DEBEMAIL" > ./debian/copyright
echo "Licenced under /usr/share/common-licenses/GPL-3" >> ./debian/copyright

# Rules
echo "#!/usr/bin/make -f" >>  ./debian/rules
echo "" >> ./debian/rules
echo "export DH_VERBOSE=1" >> ./debian/rules
echo "" >> ./debian/rules
echo "%:" >>  ./debian/rules
echo -e "\tdh \$@ --with-systemd --no-automatic-dbgsym" >>  ./debian/rules

# Deb format
mkdir ./debian/source
echo "3.0 (quilt)" > ./debian/source/format

# Install script
echo "lib/* lib/" >> ./debian/install
echo "usr/* usr/" >> ./debian/install
echo "etc/* etc/" >> ./debian/install

# Pre install script
echo "#!/bin/bash -e" >> ./debian/preinst
echo "#DEBHELPER#" >> ./debian/preinst

# Post install script
echo "#!/bin/bash -e" >> ./debian/postinst
echo "#DEBHELPER#" >> ./debian/postinst
echo "mkdir -p /etc/nginx/sites-enabled-subdir/"  # This is needed to bypass the default inlcude in nginx.conf
echo "ln -s /etc/nginx/sites-available/nexus/$PACKAGE.conf /etc/nginx/sites-enabled-subdir/nexus/$PACKAGE.conf" >> ./debian/postinst
echo "ln -s /etc/nginx/sites-available/nexus/upstream/$PACKAGE-upstream.conf /etc/nginx/sites-enabled-subdir/nexus/upstream/$PACKAGE-upstream.conf" >> ./debian/postinst
echo "if [ -d /run/systemd/system ]; then" >> ./debian/postinst
echo "deb-systemd-invoke reload nginx" >> ./debian/postinst
echo "fi" >> ./debian/postinst

# Pre removal script
echo "#!/bin/bash -e" >> ./debian/prerm
echo "#DEBHELPER#" >> ./debian/prerm
echo "rm -f /etc/nginx/sites-enabled-subdir/nexus/$PACKAGE.conf" >> ./debian/prerm
echo "rm -f /etc/nginx/sites-enabled-subdir/nexus/upstream/$PACKAGE-upstream.conf" >> ./debian/prerm
echo "if [ -d /run/systemd/system ]; then" >> ./debian/prerm
echo "deb-systemd-invoke reload nginx" >> ./debian/prerm
echo "fi" >> ./debian/prerm

# Post removal script
echo "#!/bin/bash -e" >> ./debian/postrm
echo "#DEBHELPER#" >> ./debian/postrm

#cd "$PACKAGE_NAME"
debuild -b -uc -us --no-sign
