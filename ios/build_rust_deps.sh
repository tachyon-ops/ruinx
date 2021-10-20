#!/bin/sh
echo "Total $# arguments passed to me are: $*"

set -e

PATH=$PATH:$HOME/.cargo/bin
cd ..

if [ -z "$1" ]; then
  RELEASE="no"
else
  if [ $1 = "yes" ]; then
    RELEASE="yes"
  else
    RELEASE="no"
  fi
fi

echo "Release: $RELEASE"

if [ $RELEASE = "yes" ]; then
  echo "Producing iOS release"
  cargo lipo --release
else
  echo "Producing iOS development"
  cargo lipo
fi
