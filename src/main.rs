use std::io::{self, Write} ;
use std::env ;
use dirs::home_dir;
use std::path::PathBuf;

fn main() {
    let prefix:String = get_prefix();
    print!("{}",prefix);
    io::stdout().flush().unwrap();
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    print!("{}",input);



    println!("exit")
}

fn get_prefix() -> String{
    // Get the current directory
    let current_dir = env::current_dir().unwrap();

    // Get the user's home directory
    let home_dir = home_dir().expect("Could not find home directory");

    // Replace the home directory path with '~'
    let display_path = if current_dir.starts_with(&home_dir) {
        let relative_path = current_dir.strip_prefix(&home_dir).unwrap();
        PathBuf::from("~").join(relative_path)
    } else {
        current_dir
    };
    let current_dir_str = display_path.display().to_string();
    format!("\x1b[1;96m󰉋 {}\n❯ \x1b[0m",current_dir_str)
}
