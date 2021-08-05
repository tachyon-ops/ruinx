# #!/usr/bin/env bash

# # export ANDROID_SDK_ROOT=/Users/nmpribeiro/Library/Android/sdk
# # export NDK_HOME=/Users/nmpribeiro/Library/Android/sdk/ndk/22.1.7171670

# # cd android
# # wget -q https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip
# # unzip -q sdk-tools-linux-4333796.zip
# # rm sdk-tools-linux-4333796.zip
# # tools/bin/sdkmanager "platform-tools"
# # tools/bin/sdkmanager "platforms;android-29"
# # tools/bin/sdkmanager "build-tools;29.0.0"
# # tools/bin/sdkmanager --update

# # cd ..

# # export ANDROID_SDK_ROOT=/Users/$USER/Library/Android/sdk
# # export NDK_HOME=$ANDROID_SDK_ROOT/ndk/21.4.7075529
# # export ANDROID_NDK_HOME=$ANDROID_SDK_ROOT/ndk/21.4.7075529

export ANDROID_SDK_ROOT=/Users/$USER/Library/Android/sdk
# export ANDROID_SDK_HOME=/Users/$USER/Library/Android/sdk
NDK_VERSION=21.4.7075529
export NDK_HOME=$ANDROID_SDK_ROOT/ndk/$NDK_VERSION
echo $ANDROID_SDK_ROOT
# ls $ANDROID_SDK_ROOT
echo $NDK_HOME
# ls $NDK_HOME

# # export JAVA_OPTS='-XX:+IgnoreUnrecognizedVMOptions --add-modules java.se.ee'
# # export JAVA_OPTS='-XX:+IgnoreUnrecognizedVMOptions --add-modules java.xml.bind'

# # sudo $ANDROID_SDK_HOME/tools/bin/sdkmanager "platform-tools" "platforms;android-30" "build-tools;30.0.0"
# $ANDROID_SDK_HOME/tools/bin/sdkmanager "platform-tools"
# $ANDROID_SDK_HOME/tools/bin/sdkmanager "platforms;android-30"
# $ANDROID_SDK_HOME/tools/bin/sdkmanager "build-tools;30.0.3"
# $ANDROID_SDK_HOME/tools/bin/sdkmanager --update

BUILD_MODE="release"

for i in "$@"; do
	case $i in
	-i | --install)
		# for a debug build
		# Install and setup NDK
		$NDK_HOME/build/tools/make_standalone_toolchain.py --api 29 --arch arm64 --install-dir ~/.NDK/arm64
		$NDK_HOME/build/tools/make_standalone_toolchain.py --api 29 --arch arm --install-dir ~/.NDK/arm
		$NDK_HOME/build/tools/make_standalone_toolchain.py --api 29 --arch x86 --install-dir ~/.NDK/x86
		$NDK_HOME/build/tools/make_standalone_toolchain.py --api 29 --arch x86_64 --install-dir ~/.NDK/x86_64
		exit
		;;
	-d | --debug)
		# for a debug build
		BUILD_MODE="debug"
		#build the libraries
		# RUST_LOG=trace RUST_BACKTRACE=full cargo build --target aarch64-linux-android
		# RUST_LOG=trace RUST_BACKTRACE=full cargo build --target armv7-linux-androideabi
		# RUST_LOG=trace RUST_BACKTRACE=full cargo build --target i686-linux-android
		# RUST_LOG=trace RUST_BACKTRACE=full cargo build --target x86_64-linux-android
		# We build using cargo apk as we need the libc++ as well
		RUST_LOG=trace RUST_BACKTRACE=full cargo apk build
		;;
	-r |  --release)
		# for release build
		#build the libraries
		cargo build --target aarch64-linux-android --$BUILD_MODE
		cargo build --target armv7-linux-androideabi --$BUILD_MODE
		cargo build --target i686-linux-android --$BUILD_MODE
		cargo build --target x86_64-linux-android --$BUILD_MODE
		;;

	-b |  --apk-build)
		# APK BUILD
		RUST_LOG=trace RUST_BACKTRACE=full cargo apk build
		exit
		;;

	-r |  --apk-run)
		# APK BUILD
		RUST_LOG=trace RUST_BACKTRACE=full cargo apk run
		exit
		;;

	*)
		# unknown option
		exit
		;;
	esac
done

#NOTE: Dont't forget to modify these vars to your setup
LIBS_DIR=./android/app/src/main/jniLibs
LIB_NAME=rust_jsx_app

#prepare folders...
rm -rf $LIBS_DIR
mkdir $LIBS_DIR
mkdir $LIBS_DIR/arm64-v8a
mkdir $LIBS_DIR/armeabi-v7a
mkdir $LIBS_DIR/x86
mkdir $LIBS_DIR/x86_64

echo
#..and copy the rust library into the android studio project, ready for beeing included into the APK
cp -rf target/$BUILD_MODE/apk/lib/arm64-v8a $LIBS_DIR
cp -rf target/$BUILD_MODE/apk/lib/armeabi-v7a $LIBS_DIR
cp -rf target/$BUILD_MODE/apk/lib/x86 $LIBS_DIR
cp -rf target/$BUILD_MODE/apk/lib/x86_64 $LIBS_DIR

ln -s ./assets ./android/app/src/main/res/assets
