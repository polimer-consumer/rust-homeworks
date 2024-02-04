use std::{env, fs, io::Write, io::Read};

enum Occurrence {
    File(String),
    Directory(String),
    TextFile(String),
}

trait OutputStrategy {
    fn execute(&self, file_list: &[String]);
}

struct ConsoleOutput;

impl OutputStrategy for ConsoleOutput {
    fn execute(&self, file_list: &[String]) {
        for file in file_list {
            println!("{}", file);
        }
    }
}

struct FileOutput {
    file_name: String,
}

impl OutputStrategy for FileOutput {
    fn execute(&self, file_list: &[String]) {
        let mut file = match fs::File::create(&self.file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error creating file '{}': {}", &self.file_name, e);
                return;
            }
        };
        for file_name in file_list {
            if let Err(e) = writeln!(file, "{}", file_name) {
                eprintln!("Error writing to file '{}': {}", &self.file_name, e);
                return;
            }
        }
    }
}

trait DirWalker {
    fn find(&self, path: &str, file_to_find: &str, file_list: &mut Vec<Occurrence>);
    fn print_files(&self, sort_flag: bool, text_flag: bool, text_to_find: &str,
                   file_list: &mut Vec<Occurrence>, strategy: &dyn OutputStrategy);
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

fn find_in_file(path: &str, text_to_find: &str) -> Result<Vec<String>, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let mut found_lines = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        if line.contains(text_to_find) {
            found_lines.push(format!("{}: Line {}: {}", path, line_index + 1, line.trim()));
        }
    }
    Ok(found_lines)
}

impl DirWalker for FileFinder {
    fn find(&self, path: &str, file_to_find: &str, file_list: &mut Vec<Occurrence>) {
        match fs::read_dir(path) {
            Err(_) => eprintln!("Failed to open folder: {}", path),
            Ok(paths) => {
                for path in paths {
                    if let Ok(entry) = path {
                        let cur_path = entry.path();
                        let name = entry.file_name();

                        if cur_path.is_file() {
                            if let Some(extension) = cur_path.extension().and_then(|s| s.to_str()) {
                                let occurrence;

                                if extension == "rs" || extension == "txt" {
                                    occurrence = Occurrence::TextFile(cur_path.display().to_string());
                                } else {
                                    occurrence = Occurrence::File(cur_path.display().to_string());
                                }

                                if !file_to_find.is_empty() {
                                    if name == file_to_find {
                                        file_list.push(occurrence);
                                    }
                                } else {
                                    file_list.push(occurrence);
                                }
                            }
                        } else {
                            file_list.push(Occurrence::Directory(cur_path.display().to_string()));
                            self.find(cur_path.to_str().unwrap(), file_to_find, file_list);
                        }
                    }
                }
            }
        }
    }

    fn print_files(&self, sort_flag: bool, text_flag: bool, text_to_find: &str,
                   file_list: &mut Vec<Occurrence>, strategy: &dyn OutputStrategy) {
        let mut names_list: Vec<String> = if text_flag {
            file_list.iter().filter_map(|occ| {
                if let Occurrence::TextFile(path) = occ {
                    match find_in_file(path, text_to_find) {
                        Ok(found_lines) => Some(found_lines.join("\n")),
                        Err(e) => {
                            eprintln!("Error reading file {}: {}", path, e);
                            None
                        }
                    }
                } else {
                    None
                }
            }).collect()
        } else {
            file_list.iter().map(|occ| {
                match occ {
                    Occurrence::File(path) |
                    Occurrence::Directory(path) |
                    Occurrence::TextFile(path) => path.clone(),
                }
            }).collect()
        };

        if sort_flag {
            bubble_sort(&mut names_list);
        }

        strategy.execute(&names_list);
    }
}

fn main() {
    let file_finder = FileFinder;
    let args: Vec<String> = env::args().collect();
    let start_dir = &args[1];
    let mut file_to_find = "";
    let mut text_to_find = "";
    let mut sort_flag = false;
    let mut find_text_flag = false;
    let mut file_list = Vec::new();

    let mut output_strategy: Box<dyn OutputStrategy> = Box::new(ConsoleOutput);

    for i in 0..args.len() {
        find_text_flag = true;
        if args[i] == "--in-file" {
            if i + 1 < args.len() {
                text_to_find = &args[i + 1];
            }
        }
        else if args[i] == "--find" {
            if i + 1 < args.len() {
                file_to_find = &args[i + 1];
            }
        }
        else if args[i] == "--sort" {
            sort_flag = true;
        }
        else if args[i] == "-f" {
            if i + 1 < args.len() {
                output_strategy = Box::new(FileOutput { file_name: args[i + 1].clone() });
            }
        }
    }

    file_finder.find(start_dir, file_to_find, &mut file_list);
    file_finder.print_files(sort_flag, find_text_flag, text_to_find, &mut file_list, &*output_strategy);
}
