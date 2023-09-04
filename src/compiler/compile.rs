use std::io::Write;

fn interpolate_imports(file_path: String) -> Vec<String> {
    let file_lines: Vec<String> = read_file(file_path);
    let mut new_file_lines: Vec<String> = Vec::new();

    for line in file_lines {
        if line.starts_with("@") {
            // remove the @ symbol and read the file at the remaining file path
            // replace the line with the contents of the file
            let imported_file_path: String = line[1..].to_string();
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

fn get_macro_value(macro_value: String) -> String {
    // return everything after the first space
    return macro_value.split(" ").map(|s| s.to_string()).collect::<Vec<String>>()[1].to_string();
}

fn get_macro_key(macro_value: String) -> String {
    // return everything before the first space
    return macro_value.split(" ").map(|s| s.to_string()).collect::<Vec<String>>()[0].to_string();
}

fn read_file(file_path: String) -> Vec<String> {
    let file = std::fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = file
        .split("\n")
        .map(|s| s.to_string())
        .map(|s| s.to_string())
        .collect();

    return lines;
}

fn write_to_file(file_path: String, contents: Vec<String>) {
    let mut file = std::fs::File::create(file_path)
        .expect("Something went wrong creating the file");

    for line in contents {
        file.write_all((line + "\n").as_bytes())
            .expect("Something went wrong writing the file");
    }
}

pub fn compile_file(file_path: String) -> String {
    let mut new_file_lines: Vec<String> = Vec::new();

    let original_file_lines: Vec<String> = remove_imports(
        remove_macros(
            remove_comments(
                read_file(file_path.clone())
            )
        )
    );
    let interpolated_file_lines: Vec<String> = interpolate_imports(file_path.clone());
    let macros: Vec<String> = get_macros(interpolated_file_lines.clone());

    for line in original_file_lines {
        let mut line_replaced = false;
        for searching_macro in &macros {
            let searching_macro_key: String = get_macro_key(searching_macro.clone());

            if line.contains(&*searching_macro_key) {
                let macro_value: String = get_macro_value(searching_macro.clone());
                // replace the macro key in the string with the macro value
                let new_line: String = line.replace(&*searching_macro_key, &macro_value);

                new_file_lines.push(new_line);
                line_replaced = true;
                break;
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
