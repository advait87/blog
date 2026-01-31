use std::fs::{File, read_dir};
use std::fs;
use std::io::{BufReader, BufRead};

use chrono::Local;

fn main() {
    let dir = read_dir("blogs");

    match dir {
        Ok(dir) => {
            for entry in dir {
                let entry = entry.unwrap();
                let file_path = entry.path();
                let file_name = file_path.file_name().unwrap().to_str().unwrap();

                let file = File::open(&file_path);

                let mut title = String::new();
                let mut content = String::new();
                let mut created_at = String::new();

                if let Ok(f) = file {
                    let buf = BufReader::new(f);
                    for (line_number, line) in buf.lines().enumerate() {
                        if let Ok(l) = line {
                            if line_number == 0 {
                                title = l;
                            } else if line_number == 1 {
                                created_at = l;
                            } else {
                                content.push_str(&l);
                                content.push('\n');
                            }
                        }
                    }
                }

                // Format current date like "Jan 25, 2026"
                let date = Local::now().format("%b %d, %Y").to_string();

                let html = include_str!("template.html")
                    .replace("__TITLE__", &title)
                    .replace("__DATE__", &created_at)
                    .replace("__CONTENT__", &content);

                fs::write(format!("sites/{}.html", file_name), html)
                    .expect("Failed to write file");
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
