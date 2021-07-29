#!/bin/sh

set -e

export CONFIGURATION=debug

xcodegen generate

./build_rust_deps.sh
