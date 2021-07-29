# rust-jsx-ui

A cool experiment to create a Rust UI using JSX

## To build iOS

` cd ios``  `./build.sh``

Open xcode, open the project and build.

## To build android

Go to `Cargo.toml` witch `macroquad = {git = "https://github.com/not-fl3/macroquad", version = "*", default-features = false}` off and `# macroquad = "0.3.1"` on. It will now build for android.

`./android.sh`
If should leave an APK in `target/android-artifacts/apk`
