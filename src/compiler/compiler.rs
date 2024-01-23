use std::io::Write;

mod errors {
    pub mod files;
    pub mod generic;
}

fn interpolate_imports(file_path: String) -> Vec<String> {
    let file_lines: Vec<String> = read_file(file_path.clone());
    let mut new_file_lines: Vec<String> = Vec::new();

    let mut line_number: usize = 0;

    for line in file_lines {
        line_number += 1;

        if line.starts_with("@") {
            // remove the @ symbol and read the file at the remaining file path
            // replace the line with the contents of the file
            let imported_file_path: String = line[1..].to_string();

            // check if the file exists
            if !std::path::Path::new(&imported_file_path).exists() {
                errors::files::import_error(
                    file_path.to_string(),
                    line_number,
                    imported_file_path.clone(),
                );
            }

            let imported_file_lines: Vec<String> = read_file(imported_file_path);

            for imported_line in imported_file_lines {
                new_file_lines.push(imported_line);
            }

            new_file_lines.push("\n".to_string());
        } else {
            new_file_lines.push(line);
        }
    }

    return new_file_lines;
}

fn remove_comments(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();

    for line in file_contents {
        if !line.starts_with("//") {
            new_file_contents.push(line);
        }
    }

    return new_file_contents;
}

fn remove_imports(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();

    for line in file_contents {
        if !line.starts_with("@") {
            new_file_contents.push(line);
        }
    }

    return new_file_contents;
}

fn remove_macros(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();

    for line in file_contents {
        if !line.starts_with("#") {
            new_file_contents.push(line);
        }
    }

    return new_file_contents;
}

fn get_macros(file_lines: Vec<String>) -> Vec<String> {
    let mut macros: Vec<String> = Vec::new();

    for line in file_lines {
        if line.starts_with("#") {
            // push to the macros vector but remove the # symbol
            macros.push(line[1..].to_string());
        }
    }

    return macros;
}

fn replace_macro_parameter(macro_value: String, calling_line: String) -> String {
    // split the calling_line by spaces
    // get the 2nd item in the array if it exists. If not, return the original macro_value
    // if the 2nd item in the split array exists, replace all occurrences of the '$' character with the 2nd item
    let split_macro = calling_line
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // TODO: I should throw a compiler error here
    if split_macro.len() < 3 {
        return macro_value;
    }

    let parameter_value: String = split_macro[2].to_string();

    if parameter_value == "" {
        return macro_value;
    }

    // replace all occurrences of $ with the macro_value
    return macro_value.replacen("$", &parameter_value, 1);
}

fn get_macro_value(macro_value: String) -> String {
    // return everything after the first space
    return macro_value
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1]
        .to_string();
}

fn get_macro_key(macro_value: String) -> String {
    // return everything before the first space
    return macro_value
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[0]
        .to_string();
}

fn find_text_between_quotation_marks(input: String) -> Vec<String> {
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

fn string_to_binary(input_string: String) -> String {
    let mut binary_string = String::new();

    for c in input_string.chars() {
        let binary_representation = format!("{:08b}", c as u8);
        binary_string.push_str(&binary_representation);
    }

    return binary_string;
}

// allows you to use strings in your code e.g. "Hello World!"
// instead of having to remember the binary values
fn convert_strings_to_binary(content: Vec<String>) -> Vec<String> {
    // strings are defined as anything in between double or single quotation marks
    // we should therefore search for quotation marks, set a flag saying that we are in quotation marks, until we see another
    // get all characters that were found in between the quotation marks by removing the quotation marks
    // and converting the string to ASCII binary
    // we use ASCII binary because that's what processors natively support
    // I might add support for UTF-8/16 in the future

    let mut all_lines: Vec<String> = Vec::new();

    for line in content {
        let mut modified_line = line.clone();

        let text_in_between_quotation_marks: Vec<String> = find_text_between_quotation_marks(line);

        for text in text_in_between_quotation_marks {
            modified_line = modified_line.replace(&text, &string_to_binary(text.clone()));
        }

        all_lines.push(modified_line);
    }

    return all_lines;
}

fn read_file(file_path: String) -> Vec<String> {
    let file = std::fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let lines: Vec<String> = file
        .split("\n")
        .map(|s| s.to_string())
        .map(|s| s.to_string())
        .collect();

    return lines;
}

fn write_to_file(file_path: String, contents: Vec<String>) {
    let mut file =
        std::fs::File::create(file_path).expect("Something went wrong creating the file");

    for line in contents {
        file.write_all((line + "\n").as_bytes())
            .expect("Something went wrong writing the file");
    }
}

pub fn compile_file(file_path: String) -> String {
    let mut new_file_lines: Vec<String> = Vec::new();

    // check if the input file exists
    if !std::path::Path::new(&file_path).exists() {
        errors::files::file_error(file_path.clone());
    }

    let original_file_lines: Vec<String> = remove_imports(convert_strings_to_binary(
        remove_macros(remove_comments(read_file(file_path.clone()))),
    ));
    let interpolated_file_lines: Vec<String> = interpolate_imports(file_path.clone());
    let macros: Vec<String> = get_macros(interpolated_file_lines.clone());

    for line in original_file_lines {
        let mut line_replaced = false;
        for searching_macro in &macros {
            let searching_macro_key: String = get_macro_key(searching_macro.clone());

            if line.contains(&*searching_macro_key) {
                let macro_value: String = get_macro_value(searching_macro.clone());

                let interpolated_macro_value: String =
                    replace_macro_parameter(macro_value, line.clone());

                // replace the macro key in the string with the macro value
                let new_line: String =
                    line.replace(&*searching_macro_key, &interpolated_macro_value);

                new_file_lines.push(new_line);
                line_replaced = true;
            }
        }

        if !line_replaced {
            new_file_lines.push(line);
        }
    }

    let new_file_path: String = format!("{}{}", file_path, ".bin");

    write_to_file(new_file_path, new_file_lines.clone());

    let new_file_path: String = format!("{}{}", file_path, ".bin");

    write_to_file(new_file_path.clone(), new_file_lines);

    return new_file_path;
}
