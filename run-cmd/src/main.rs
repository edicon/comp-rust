// run system command
// -http://levelup.gitconnected.com/rust-invoking-of-system-commands-f8a8b0ef37ce
//
use std::process::Command;

fn main() {
    // spawn()
    println!("Running via spawn())");
    let mut cmd_spawn = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls commandfailed to start");
    cmd_spawn.wait().expect("Failing while waiting");
    println!("{:?}", cmd_spawn);

    // ooutput()
    println!("Running via ooutput())");
    let cmd_output = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .output()
        .expect("ls commandfailed to start");
    println!("");
    println!("{:?}", cmd_output);

    // spawn()
    println!("Running via spawn())");
    let cmd_status = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .status()
        .expect("ls commandfailed to start");
    println!("");
    println!("{:?}", cmd_status);
}
