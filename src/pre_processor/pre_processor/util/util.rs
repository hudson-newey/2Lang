use std::{fs::File, io::Write};

pub fn read_file(file_path: String) -> Vec<String> {
    let file: String = std::fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let lines: Vec<String> = file
        .split("\n")
        .map(|s: &str| s.to_string())
        .map(|s: String| s.to_string())
        .collect();

    return lines;
}

pub fn write_to_file(file_path: String, contents: Vec<String>) {
    let mut file: File =
        std::fs::File::create(file_path).expect("Something went wrong creating the file");

    for line in contents {
        file.write_all((line + "\n").as_bytes())
            .expect("Something went wrong writing the file");
    }
}
