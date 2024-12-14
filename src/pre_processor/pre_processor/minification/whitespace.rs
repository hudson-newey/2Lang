pub fn remove_whitespace(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();

    for line in file_contents {
        let new_line: String = line.trim().to_string();
        if new_line != "" {
            new_file_contents.push(new_line);
        }
    }

    return new_file_contents;
}
