#!/bin/sh

FILE_NAME=$(readlink -f $0)
WORKDIR=$(dirname ${FILE_NAME})
SMARTBOAT_WORKDIR=${WORKDIR}/../../

docker run --rm --name container-smart-boat-bd-debian-bullseye --tty -v ${SMARTBOAT_WORKDIR}:/root/smart-boat smart-boat-stub_builder_debian:bullseye