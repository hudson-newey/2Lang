pub fn remove_macros(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();

    for line in file_contents {
        if !line.starts_with("#") {
            new_file_contents.push(line);
        }
    }

    return new_file_contents;
}

pub fn get_macros(file_lines: Vec<String>) -> Vec<String> {
    let mut macros: Vec<String> = Vec::new();

    for line in file_lines {
        if line.starts_with("#") {
            // push to the macros vector but remove the # symbol
            macros.push(line[1..].to_string());
        }
    }

    return macros;
}

pub fn replace_macro_parameter(macro_value: String, calling_line: String) -> String {
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

pub fn get_macro_value(macro_value: String) -> String {
    // return everything after the first space
    return macro_value
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1]
        .to_string();
}

pub fn get_macro_key(macro_value: String) -> String {
    // return everything before the first space
    return macro_value
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[0]
        .to_string();
}
