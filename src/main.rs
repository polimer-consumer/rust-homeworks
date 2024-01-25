use std::{env, fs};

trait DirWalker {
    fn find(&self, path: String, file_to_find: String);
}

struct FileFinder;

impl DirWalker for FileFinder {
    fn find(&self, path: String, file_to_find: String) {
        match fs::read_dir(&path) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => {
                for found_path in paths {
                    if let Ok(entry) = found_path {
                        let name = entry.file_name();
                        let cur_path = entry.path();

                        if cur_path.is_file() {
                            if file_to_find.is_empty() {
                                println!("> {}", name.to_string_lossy());
                            }
                            if name.to_string_lossy().to_string() == file_to_find {
                                println!("Found file at: {:?}", cur_path);
                            }
                        } else {
                            self.find(cur_path.to_string_lossy().to_string(), file_to_find.clone())
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
        file_finder.find(args[1].clone(), String::new())
    } else if args[2] == "--find" {
        file_finder.find(args[1].clone(), args[3].clone())
    } else {
        eprintln!("Error: no --find option!")
    }
}
