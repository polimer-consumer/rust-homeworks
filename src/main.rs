use std::{env, fs};

trait DirWalker {
    fn find(&self, path: &str, file_to_find: &str);
}

struct FileFinder;

impl DirWalker for FileFinder {
    fn find(&self, path: &str, file_to_find: &str) {
        match fs::read_dir(&path) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => {
                for path in paths {
                    if let Ok(entry) = path {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();

                        if entry.path().is_file() {
                            if file_to_find.is_empty() {
                                println!("> {}", name_str);
                            }
                            if name_str == file_to_find {
                                println!("Found file at: {:?}", entry.path());
                                return;
                            }
                        } else {
                            self.find(&name_str, file_to_find)
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
        file_finder.find(&args[1], "");
    } else if args[2] == "--find" {
        file_finder.find(&args[1], &args[3]);
    } else {
        eprintln!("Error: no --find option!");
    }
}
