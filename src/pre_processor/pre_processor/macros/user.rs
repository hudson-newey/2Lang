use crate::tokens::tokens;

pub fn remove_macros(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();

    for line in file_contents {
        if !line.starts_with(tokens::MACRO) {
            new_file_contents.push(line);
        }
    }

    return new_file_contents;
}

pub fn get_macros(file_lines: Vec<String>) -> Vec<String> {
    let mut macros: Vec<String> = Vec::new();

    for line in file_lines {
        if line.starts_with(tokens::MACRO) {
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
    return macro_value.replacen(tokens::MACRO_PARAMETER, &parameter_value, 1);
}

// a macro value is defined as everything after a space or tab character
// up to the first newline character
pub fn get_macro_value(macro_value: String) -> String {
    let start_index = match macro_value.find(" ") {
        Some(index) => index,
        None => 0,
    };

    if start_index == 0 {
        return String::from("");
    }

    let end_index = match macro_value.find("\n") {
        Some(index) => index,
        None => macro_value.len(),
    };

    let result = macro_value[start_index..end_index].to_string();
    return result;
}

pub fn get_macro_key(macro_value: String) -> String {
    // return everything before the first space
    return macro_value
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[0]
        .to_string();
}

pub fn interpolate_macros(target: &Vec<String>) -> Vec<String> {
    let mut new_file_lines: Vec<String> = Vec::new();
    let macros: Vec<String> = get_macros(target.clone());

    for line in target {
        let mut line_replaced = false;
        for searching_macro in &macros {
            let searching_macro_key: String = get_macro_key(searching_macro.clone());

            if line.contains(&*searching_macro_key) && !line.starts_with(tokens::MACRO) {
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
            new_file_lines.push(line.clone());
        }
    }

    return new_file_lines.clone();
}
