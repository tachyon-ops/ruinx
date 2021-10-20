# rust-jsx-ui

A cool experiment to create a Rust UI using JSX

## Clone

`git clone https://github.com/nmpribeiro/ruinx.git`

## Build and run

[cargo run] command also builds :)

1. `cargo build`
2. `cargo run`

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
./ios.sh
```

And answer the question "do you want a release? (yes/no)"

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

## TODO

- [ ] - android - Fix android lost of surface on home & switching apps and going back to the app

```log
I/rust-app: graphics::state: Resizing: 1080 x 2312
    wgpu_core::device: configuring surface with SurfaceConfiguration { usage: RENDER_ATTACHMENT, format: Rgba8UnormSrgb, width: 1080, height: 2312, present_mode: Fifo }
E/vulkan: NATIVE_WINDOW_MAX_BUFFER_COUNT query failed: No such device (-19)
E/rust-app: wgpu_hal::vulkan::adapter: get_physical_device_surface_capabilities: ERROR_SURFACE_LOST_KHR
I/RustStdoutStderr: thread '<unnamed>' panicked at 'Error in Surface::configure: surface does not support the adapter's queue family', /Users/nmpribeiro/.cargo/git/checkouts/wgpu-53e70f8674b08dd4/9bc5908/wgpu/src/backend/direct.rs:197:9
```

- [ ] - android - Fix android scalling very different from desktop and iOS
- [ ] - Touch screens - Create a functional Touch/Tap event
- [ ] - Touch screens - Figure how to connect the Touch/Tap event with Conrad widgets
- [ ] - iOS - fix `ld: in ../target/universal/release/libruinx.a(ruinx.ruinx.54ed59ac-cgu.9.rcgu.o), building for iOS Simulator, but linking in object file built for iOS, for architecture arm64` when trying to profile
- [ ] - UI - generate Conrod UI from internal UI structure
- [ ] - UI - generate internal UI structure (some DOM like) from RSX

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

[ ] - Latest version of `winit` (both master and `v0.25`) only works with `ndk_glue` `v0.3.0`. Other combinations have a bug where in android were we cannot aquire the surface properly. Issue [here](https://github.com/rust-windowing/winit/issues/1986).
