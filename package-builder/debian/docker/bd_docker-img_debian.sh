#!/bin/sh

FILE_NAME=$(readlink -f $0)
WORKDIR=$(dirname ${FILE_NAME})

docker build ${WORKDIR} -t smart-boat-stub_builder_debian:bullseye