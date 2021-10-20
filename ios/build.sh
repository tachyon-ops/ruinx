# #!/usr/bin/env bash

set -e

export CONFIGURATION=debug

if [ ! -d "assets" ]; then
  echo "Linking 'assets' path to '../assets'"
  ln -s ../assets assets
fi

echo "Generating XCode project"
xcodegen generate

echo -n $'Do you wish a release? (yes/no)\n'
read isRelease
echo "Building rust"
./build_rust_deps.sh "$isRelease"
