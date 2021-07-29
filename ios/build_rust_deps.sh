#!/bin/sh

set -e

PATH=$PATH:$HOME/.cargo/bin
cd ..

cargo lipo
