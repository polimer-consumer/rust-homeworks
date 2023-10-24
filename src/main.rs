use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    match fs::read_dir(&args[1]) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                if let Ok(entry) = path {
                    let name = entry.file_name();
                    if entry.path().is_dir() {
                        println!("> [ {} ]", name.to_string_lossy())
                    } else {
                        println!("> {}", name.to_string_lossy());
                    }
                }
            }
        }
    }
}
