use std::{env, fs};

trait DirWalker {
    fn find(&self, base_path: &str, path: &str, file_to_find: &str, file_list: &mut Vec<String>);
    fn print_files(&self, sort_flag: bool, file_list: &mut Vec<String>);
}

struct FileFinder;

fn bubble_sort(file_list: &mut Vec<String>) {
    let n = file_list.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if file_list[j] > file_list[j + 1] {
                file_list.swap(j, j + 1);
            }
        }
    }
}

impl DirWalker for FileFinder {
    fn find(&self, base_path: &str, path: &str, file_to_find: &str, file_list: &mut Vec<String>) {
        match fs::read_dir(path) {
            Err(_) => eprintln!("Failed to open folder: {}", path),
            Ok(paths) => {
                for path in paths {
                    if let Ok(entry) = path {
                        let entry_path = entry.path();
                        let relative_path = entry_path.strip_prefix(base_path).unwrap_or(&entry_path);
                        let file_name = relative_path.to_str().unwrap();

                        if entry_path.is_file() {
                            if !file_to_find.is_empty() && file_name == file_to_find {
                                println!("Found file at: {:?}", entry_path);
                                return;
                            } else {
                                file_list.push(file_name.to_string());
                            }
                        } else {
                            self.find(base_path, file_name, file_to_find, file_list);
                        }
                    }
                }
            }
        }
    }

    fn print_files(&self, sort_flag: bool, file_list: &mut Vec<String>) {
        if sort_flag {
            bubble_sort(file_list);
        }
        for file in file_list {
            println!("{}", file);
        }
    }
}

fn main() {
    let file_finder = FileFinder;

    let args: Vec<String> = env::args().collect();
    let base_path = &args[1];
    let mut file_to_find = "";
    let mut sort_flag = false;
    let mut file_list: Vec<String> = Vec::new();

    for i in 0..args.len() {
        if args[i] == "--find" {
            if i + 1 < args.len() {
                file_to_find = &args[i + 1];
            }
        }
        if args[i] == "--sort" {
            sort_flag = true;
        }
    }

    file_finder.find(base_path, base_path, file_to_find, &mut file_list);
    file_finder.print_files(sort_flag, &mut file_list);
}
