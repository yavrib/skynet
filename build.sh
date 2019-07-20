#!/usr/bin/env bash

if [ $1 == "ubuntu" ]; then
  apt install openssl
  export OPENSSL_INCLUDE_DIR=`which $(openssl)`/include
  export OPENSSL_LIB_DIR=`which $(openssl)`/lib
elif [ $1 == "redhat" ]; then
  yum install openssl
  export OPENSSL_INCLUDE_DIR=`which $(openssl)`/include
  export OPENSSL_LIB_DIR=`which $(openssl)`/lib
else
  export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
  export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
fi

cargo run
