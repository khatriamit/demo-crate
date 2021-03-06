#[allow(unused)]
use clap::{Arg, Command};
pub fn this_test() -> [Command<'static>; 2] {
    [
        Command::new("true").about("does nothing successfully"),
        Command::new("false").about("does nothing unsuccessfully"),
    ]
}
fn main() {
    let m = Command::new("My Program")
        .author("Me, me@mail.com")
        .version("1.0.2")
        .about("Explains in brief what the program does")
        .arg(Arg::new("in_file"))
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .get_matches();
    println!("{:?}", m);
    print!("hello world");
}
