#!/usr/bin/env bash

cd ios

echo -n $'Do you wish a release? (yes/no)\n'
read isRelease

./build.sh "$isRelease"
