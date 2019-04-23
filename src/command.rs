use std::process::Command;
fn command_wrapper(test_command: &str, command_directory: &str) -> Command {
    let mut command = if cfg!(target_os = "windows") {
        {
            let mut c = Command::new("cmd");
            c.args(&["/C", test_command]);
            c
        }
    } else {
        {
            let mut c = Command::new("sh");
            c.arg("-c");
            c.arg(test_command);
            c
        }
    };
    command.current_dir(command_directory);
    command
}

pub fn get_output(password:&str,path:&str)->String{
    let mut c = command_wrapper("echo docker_pid", ".");


    let buf = c.output().unwrap().stdout;
    let s = String::from_utf8_lossy(&buf);

    s.to_string().trim().to_string()
}