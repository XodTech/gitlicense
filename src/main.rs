use std::{
    env,
    mem
};

struct CommandArguments {
    license_name:String,
    directory:String
}

fn main() {
    let mut cmd: Vec<String> = env::args().collect();
    let args = CommandArguments {
        license_name: mem::take(&mut cmd[1]),
        directory: mem::take(&mut cmd[2])
    };
}
