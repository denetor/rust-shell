mod ls;

use ls::Ls;

fn main() {
    let command = std::env::args().nth(1).expect("no command given");

    if command == "ls" {
        // println!("trying to execute ls command");
        Ls::run();
    } else if command == "cat" {
        println!("trying to execute cat command");
    } else {
        println!("unknown command: {}", command);
    }
}
