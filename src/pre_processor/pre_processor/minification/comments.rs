pub fn remove_comments(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();

    for line in file_contents {
        if !line.starts_with("//") {
            new_file_contents.push(line);
        }
    }

    return new_file_contents;
}
