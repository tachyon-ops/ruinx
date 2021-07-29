#!/usr/bin/env bash

# export ANDROID_SDK_ROOT=/Users/nmpribeiro/Library/Android/sdk
# export NDK_HOME=/Users/nmpribeiro/Library/Android/sdk/ndk/22.1.7171670

# create a folder for all the android pre-requisites stuff
mkdir android

# cd android
# wget -q https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip
# unzip -q sdk-tools-linux-4333796.zip
# rm sdk-tools-linux-4333796.zip
# tools/bin/sdkmanager "platform-tools"
# tools/bin/sdkmanager "platforms;android-29"
# tools/bin/sdkmanager "build-tools;29.0.0"
# tools/bin/sdkmanager --update

# cd ..

export ANDROID_SDK_ROOT=/Users/$USER/Library/Android/sdk
export ANDROID_SDK_HOME=/Users/$USER/Library/Android/sdk
NDK_VERSION=21.4.7075529
export NDK_HOME=$ANDROID_SDK_ROOT/ndk/$NDK_VERSION
echo $ANDROID_SDK_ROOT
ls $ANDROID_SDK_ROOT
echo $NDK_HOME
ls $NDK_HOME

# export JAVA_OPTS='-XX:+IgnoreUnrecognizedVMOptions --add-modules java.se.ee'
# export JAVA_OPTS='-XX:+IgnoreUnrecognizedVMOptions --add-modules java.xml.bind'

# sudo $ANDROID_SDK_HOME/tools/bin/sdkmanager "platform-tools" "platforms;android-30" "build-tools;30.0.0"
$ANDROID_SDK_HOME/tools/bin/sdkmanager "platform-tools"
$ANDROID_SDK_HOME/tools/bin/sdkmanager "platforms;android-30"
$ANDROID_SDK_HOME/tools/bin/sdkmanager "build-tools;30.0.3"
$ANDROID_SDK_HOME/tools/bin/sdkmanager --update

for i in "$@"; do
	case $i in
	-b | --build)
		# for a debug build
		cargo quad-apk build
		exit
		;;
	-r | --release)
		# for a release build
		cargo quad-apk build --release
		exit
		;;
	*)
		# unknown option
		exit
		;;
	esac
done
