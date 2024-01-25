use std::{env, fs};

trait DirWalker {
    fn find(&self, path: &String, file_to_find: &String);
}

struct FileFinder;

impl DirWalker for FileFinder {
    fn find(&self, path: &String, file_to_find: &String) {
        match fs::read_dir(path) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => {
                for path in paths {
                    if let Ok(entry) = path {
                        let name = entry.file_name().to_string_lossy().to_string();

                        if entry.path().is_file() {
                            if file_to_find.is_empty() {
                                println!("> {}", name);
                            }
                            if name == *file_to_find {
                                println!("Found file at: {:?}", entry.path());
                                return;
                            }
                        } else {
                            self.find(&name, file_to_find);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let file_finder = FileFinder;

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        file_finder.find(&args[1], &String::new());
    } else if args[2] == "--find" {
        file_finder.find(&args[1], &args[3]);
    } else {
        eprintln!("Error: no --find option!");
    }
}
