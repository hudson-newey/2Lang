use std::io::Write;

pub fn read_file(file_path: String) -> Vec<String> {
    let file = std::fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let lines: Vec<String> = file
        .split("\n")
        .map(|s| s.to_string())
        .map(|s| s.to_string())
        .collect();

    return lines;
}

pub fn write_to_file(file_path: String, contents: Vec<String>) {
    let mut file =
        std::fs::File::create(file_path).expect("Something went wrong creating the file");

    for line in contents {
        file.write_all((line + "\n").as_bytes())
            .expect("Something went wrong writing the file");
    }
}

pub fn find_text_between_quotation_marks(input: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut inside_quotes = false;
    let mut current_text = String::new();

    for c in input.chars() {
        match c {
            '"' => {
                // Toggle the inside_quotes flag when encountering a double quotation mark.
                inside_quotes = !inside_quotes;

                // If we've just closed a quotation, add the captured text to the result.
                if !inside_quotes {
                    result.push(current_text.clone());
                    current_text.clear();
                }
            }
            _ if inside_quotes => {
                // If we're inside quotation marks, append the character to the current_text.
                current_text.push(c);
            }
            _ => {}
        }
    }

    return result;
}
