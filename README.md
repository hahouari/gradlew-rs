# gradlew-rs

a command line tool for lazy people that simply work as an alias for your project gradlew file.

## How so?
for example on an Android project I do
```shell
# for GNU Linux or macOS
./gradlew --help
# or for MS Windows
.\gradlew.bat --help
```
I simply now do 
```shell
gradlew --help
```

## Why not use aliases instead?
If you would like to be able to run `gradlew` from subdirectories as well, `gradlew-rs` handles this for you.

The command looks up the correct parent directory and spawns a `gradlew` process within that directory.

## How to install?
Download the [latest release](https://github.com/hahouari/gradlew-rs/releases/latest),
decompress the binary executable and move it to a directory that is visible to your `PATH`.

Example directories to this depending on your OS are: 
* GNU Linux and macOS: `/usr/local/bin`
* MS Windows: `C:\Windows\System32`
