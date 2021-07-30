# rust-jsx-ui

A cool experiment to create a Rust UI using JSX

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

#### Rust android

Assuming rustup's rust installation:

```bash
rustup target add armv7-linux-androideabi
rustup target add aarch64-linux-android
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

##### Android SDK

###### create a folder for all the android pre-requisites stuff

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

Note: the above will not work with `android.sh` utility as it was fine-tuned to use android 30 with build-tools 30.0.3.

##### Android NDK

- go to the same dir created for android-sdk

```bash
cd /path/from/previous/step/android

wget -q http://dl.google.com/android/repository/android-ndk-r20-linux-x86_64.zip
unzip -q android-ndk-r20-linux-x86_64.zip
rm android-ndk-r20-linux-x86_64.zip
```

##### Cargo APK

`cargo-quad-apk` is a cargo extension, allowing

`cargo install cargo-quad-apk`

###### Building an APK, a manual way

```bash
export ANDROID_HOME=/path/from/previous/step/android
export NDK_HOME=/path/from/previous/step/android/android-ndk-r20
```

Here are my personal environment variables (Android SDK was installed from Android Studio, google how to download precise NDKs and SDKs for android studio):

```bash
export ANDROID_SDK_HOME=/Users/$USER/Library/Android/sdk
export NDK_VERSION=21.4.7075529
export NDK_HOME=$ANDROID_SDK_ROOT/ndk/$NDK_VERSION
```

#### android.sh

Once you have my environmental variables, a bash script `android.sh` was made for your convenience.

For debug or build you can do `./android.sh -b` (-b or --build) or `./android.sh -r` (-r or --release).

#### for a debug build

`cargo quad-apk build`

#### for a release build

`cargo quad-apk build --release`

An apk will be in `target/android-artifacts/debug/apk` or `target/android-artifacts/release/apk`.

## Resources

### Scripting languages

Have a look at this cool exploration of rust scrtipting lang ecosystem as of 2021 [here](https://www.boringcactus.com/2020/09/16/survey-of-rust-embeddable-scripting-languages.html#duckscript).
We went for Dyon
