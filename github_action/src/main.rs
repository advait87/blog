use std::fs::{File, read_dir};
use std::fs;
use std::io::{BufReader, BufRead};

use serde_json::json;
use chrono::Local;

fn main() {
    gen_static_files();
    gen_index();
}

fn gen_static_files() {

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

                let html = include_str!("templates/blog.html")
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

fn gen_index() -> std::io::Result<()> {
    let mut posts = Vec::new();

    for entry in read_dir("blogs")? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_stem().unwrap().to_str().unwrap();

        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut title = String::new();
        let mut content = String::new();
        let mut created_at = String::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if i == 0 {
                title = line;
            } else if i == 1 {
                created_at = line;
            } else {
                content.push_str(&line);
                content.push(' ');
            }
        }

        let excerpt: String = content.chars().take(200).collect();

        posts.push(json!({
            "title": title,
            "created_at": created_at,
            "content": excerpt.trim(),
            "filename": filename,
        }));
    }

    let posts_json = serde_json::to_string_pretty(&posts).unwrap();
    let template = include_str!("templates/index.html");
    let final_html = template.replace("__POSTS_JSON__", &posts_json);

    fs::write("sites/index.html", final_html)?;
    Ok(())
}
