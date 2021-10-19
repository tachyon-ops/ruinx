# rust-jsx-ui

A cool experiment to create a Rust UI using JSX

## Clone

`git clone https://github.com/nmpribeiro/ruinx.git`
`git submodule update --init --recursive`

or

`git clone --recursive https://github.com/nmpribeiro/ruinx.git`

## iOS BUILD

## Build & Run

### Install Rust targets

```bash
rustup target add aarch64-apple-ios x86_64-apple-ios
```

### Install [cargo-lipo](https://github.com/TimNN/cargo-lipo)

```bash
cargo install cargo-lipo
```

### Build [Xcode project](https://github.com/yonaskolb/XcodeGen)

```bash
cd ios
./build.sh
```

### Run example

1. Open generated xcodeproject
1. Set signing team
1. Select target device
1. Click Run button

Open xcode, open the project and build.

## Android BUILD

### Installing pre-requisites

Exact commands and pathes may depend on the host OS. Here linux commands are used, but on all the other OSes the idea should be very similar.

#### JRE or JDK

This step highly depends on the OS, for ubuntu:

`sudo apt-get install openjdk-8-jdk`

##### Rust android

Assuming rustup's rust installation:

```bash
rustup target add armv7-linux-androideabi
rustup target add aarch64-linux-android
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

##### Android SDK

Create a folder for all the android pre-requisites stuff

```bash
mkdir /this/may/be/any/path/android

cd android
wget -q https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip
unzip -q sdk-tools-linux-4333796.zip
rm sdk-tools-linux-4333796.zip
tools/bind/sdkmanager "platform-tools"
tools/bin/sdkmanager "platforms;android-29"
tools/bin/sdkmanager "build-tools;29.0.0"
tools/bin/sdkmanager --update
```

##### Android NDK

###### go to the same dir created for android-sdk

```bash
cd /path/from/previous/step/android

wget -q http://dl.google.com/android/repository/android-ndk-r20-linux-x86_64.zip
unzip -q android-ndk-r20-linux-x86_64.zip
rm android-ndk-r20-linux-x86_64.zip

For debug or build you can do `./android.sh -b` (-b or --build) or `./android.sh -r` (-r or --release).
```

###### Building an APK, a manual way

```bash
export ANDROID_HOME=/path/from/previous/step/android
export NDK_HOME=/path/from/previous/step/android/android-ndk-r20
```

Here are my personal environment variables:

```bash
export ANDROID_SDK_ROOT=/Users/$USER/android
export NDK_HOME=$ANDROID_SDK_ROOT/android/android-ndk-r20/
```

#### android.sh

Once you have my environmental variables, a bash script `android.sh` was made for your convenience.

#### for a debug build

`./android.sh -d`

#### for a release build

`./android.sh -r`

Head over to your android studio and run it. If you just want the apk, it will be in `target/android-artifacts/debug/apk` or `target/android-artifacts/release/apk`.

##### Log android

Use android project to check logs.

## Resources

### Scripting languages

Have a look at this cool exploration of rust scrtipting lang ecosystem as of 2021 [here](https://www.boringcactus.com/2020/09/16/survey-of-rust-embeddable-scripting-languages.html#duckscript).
We went for Dyon

### WebGPU

- A bunch of cool resources about WebGPU in rust: [Everything I know](https://wiki.nikitavoloboev.xyz/computer-graphics/webgpu)

### WebGPU 2D experiments

- [Nannou](https://github.com/nannou-org/nannou)
- [gfx-rs/wgpu 2D and much more](https://github.com/gfx-rs/wgpu/wiki/Users)

## Issues

[ ] - Currently `winit` iOS has a bug where if you touch the app, it will crash. It is solved by this [commit](https://github.com/nmpribeiro/winit/commit/665e8baab87f4b6c44952e5e0bf81848107c33b1).

[ ] - Latest version of `winit` (both master and `v0.25`) only works with `ndk_glue` `v0.3.0`. Other combinations have a bug where in android were we cannot aquire the surface properly. Issue [here](https://github.com/rust-windowing/winit/issues/1986).
