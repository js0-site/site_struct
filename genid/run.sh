#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR/../../conf/site_struct
set -a
. conf.env
. default.env
set +a
cd $DIR
set -x

cargo run --release
