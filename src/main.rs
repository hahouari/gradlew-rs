use std::env;
use std::process::Command;

const SYS_CMD: &str = if cfg!(windows) { "gradlew.bat" } else { "./gradlew" };
const GRADLEW_FILENAME: &str = if cfg!(windows) { "gradlew.bat" } else { "gradlew" };

fn locate_gradlew() -> Option<String> {
    let work_dir = env::current_dir().expect("failed to get current working directory");
    let mut cur_dir = Some(work_dir.as_path());

    while cur_dir.is_some() {
        let cur_dir_path = cur_dir.unwrap();
        let path = cur_dir_path.join(GRADLEW_FILENAME);
        if path.exists() {
            return Some(cur_dir_path.as_os_str().to_os_string().into_string().unwrap());
        }
        cur_dir = cur_dir_path.parent()
    }
    return None;
}

fn main() {
    let sys_cmd_dir = match locate_gradlew() {
        Some(cmd) => cmd,
        None => {
            println!("{} file not found in this directory or any parent directories", GRADLEW_FILENAME);
            return;
        }
    };
    let args = env::args().collect::<Vec<String>>();
    // first arg is executable path, we need to delete that.
    let sys_args = &args[1..].join(" ");

    let mut child = Command::new(SYS_CMD)
        // run command from gradlew directory for gradle to correctly fetch settings and build files.
        .current_dir(sys_cmd_dir)
        .arg(sys_args)
        .spawn()
        .expect("failed to execute child process");
    child.wait().expect("failed to wait on child process");
}
