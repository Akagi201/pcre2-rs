#!/bin/bash

if [ $# != 1 ]; then
  echo "USAGE: ./udplog.sh [port], default port: 8327"
  exit 1
fi

echo "UDP server listening on: $1";

nc -n -u -l -k 127.0.0.1 $1
