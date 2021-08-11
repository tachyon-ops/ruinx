# #!/usr/bin/env bash

export ANDROID_SDK_ROOT=/Users/$USER/android
export NDK_HOME=$ANDROID_SDK_ROOT/android/android-ndk-r20/
echo "Using the following Android and NDK:"
echo $ANDROID_SDK_ROOT
echo $NDK_HOME

BUILD_MODE="release"

for i in "$@"; do
	case $i in
	-d | --debug)
		# for a debug build
		BUILD_MODE="debug"
		#build the libraries
		# We build using cargo apk as we need the libc++ as well
		RUST_LOG=trace RUST_BACKTRACE=full cargo apk build
		;;
	-r |  --release)
		# for release build
		#build the libraries
		cargo apk build --$BUILD_MODE
		;;

	-b |  --apk-build)
		# APK BUILD
		RUST_LOG=trace RUST_BACKTRACE=full cargo apk build
		exit
		;;

	-s |  --apk-start)
		# APK RUN
		RUST_LOG=trace RUST_BACKTRACE=full cargo apk run
		exit
		;;

	-r |  --apk-release)
		# APK BUILD RELEASE
		cargo apk build --$BUILD_MODE
		exit
		;;

	*)
		# unknown option
		exit
		;;
	esac
done

#NOTE: Dont't forget to modify these vars to your setup
PROJ_MAIN=./android/app/src/main
LIBS_DIR=$PROJ_MAIN/jniLibs
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

# Check whether the root assets folder of our main Android app has a symlink to our shared assets
pushd $PROJ_MAIN
if [ ! -d "assets" ]; then
	echo "Linking assets to ../../../../assets"
	ln -s ../../../../assets .
fi
popd
