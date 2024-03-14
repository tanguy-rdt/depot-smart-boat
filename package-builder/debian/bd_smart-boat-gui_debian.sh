#!/bin/sh

FILE_NAME=$(readlink -f $0)
DEBIAN_DOCKER_DIR=$(dirname ${FILE_NAME})
SMARTBOAT_DIR=${DEBIAN_DOCKER_DIR}/../..
DEB_DIR=${SMARTBOAT_DIR}/deb

cd $SMARTBOAT_DIR

$HOME/.cargo/bin/cargo build --release 
cp ${SMARTBOAT_DIR}/target/release/depot-smart-boat ${SMARTBOAT_DIR}/smart-boat-debian/smart-boat-debian-stub/usr/bin/smart-boat-stub

dpkg-deb --build smart-boat-debian/smart-boat-debian-stub
if [ $? -ne 0 ]; then
    echo "ERROR with cmd 'dpkg-deb --build smart-boat-debian/smart-boat-debian-stub': An error occurred during the build"
    exit 1
fi

if find "${SMARTBOAT_DIR}/smart-boat-debian" -maxdepth 1 -name "smart-boat-debian-stub.deb" | grep -q .; then
    if [ ! -d "${DEB_DIR}" ]; then
        mkdir -p ${DEB_DIR}
    fi

    mv ${SMARTBOAT_DIR}/smart-boat-debian/smart-boat-debian-stub.deb ${DEB_DIR}
    echo "SUCCESS -- You can find the .deb in depot-smart-boat/deb"
else
    echo "An error must have occurred because no .deb file was found"

    exit 3
fi





$HOME/.cargo/bin/cargo clean




$HOME/.cargo/bin/cargo build --release --features=on_target
cp ${SMARTBOAT_DIR}/target/release/depot-smart-boat ${SMARTBOAT_DIR}/smart-boat-debian/smart-boat-debian/usr/bin/smart-boat

dpkg-deb --build smart-boat-debian/smart-boat-debian
if [ $? -ne 0 ]; then
    echo "ERROR with cmd 'dpkg-deb --build smart-boat-debian/smart-boat-debian': An error occurred during the build"
    exit 1
fi

if find "${SMARTBOAT_DIR}/smart-boat-debian" -maxdepth 1 -name "smart-boat-debian.deb" | grep -q .; then
    if [ ! -d "${DEB_DIR}" ]; then
        mkdir -p ${DEB_DIR}
    fi

    mv ${SMARTBOAT_DIR}/smart-boat-debian/smart-boat-debian.deb ${DEB_DIR}
    echo "SUCCESS -- You can find the .deb in depot-smart-boat/deb"
else
    echo "An error must have occurred because no .deb file was found"

    exit 3
fi

exit 0