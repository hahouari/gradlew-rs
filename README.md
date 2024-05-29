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

## Why not alias on my own it instead?
If you would like to be able to run `gradlew` from subdirectories as well,
here are your options with your local `gradlew` file:
* In case your active terminal is within a subdirectory, you need to do something like this:
```shell
cd ../ # go back to the project root directory
./gradlew --help 
```
* If you don't want to `cd` back, you have to run something like:
```shell
# -p or --project-dir to specify where your root build files are
# if you don't use it the command will fail.
../gradlew -p ../ --help
```

The `gradlew-rs` handles this for you,
the tool looks back the parent directories in case the gradlew isn't present in current directory,
when it finds the gradlew file it doesn't have to cd back to it,
and it doesn't use the `-p` argument internally to keep low footprint on the command itself,
so you can use `-p` yourself without worrying about conflicting arguments,
the command tool here simply spawn a process with `./gradlew` command within the correct directory.
