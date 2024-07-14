mod minification;
mod errors;
mod imports;
mod macros;
mod code_execution;
mod util;

pub fn pre_process(file_path: String) -> String {
    let mut new_file_lines: Vec<String> = Vec::new();

    // check if the input file exists
    if !std::path::Path::new(&file_path).exists() {
        errors::files::file_error(file_path.clone());
    }

    let original_file_lines: Vec<String> = imports::imports::remove_imports(macros::inbuilt::convert_strings_to_binary(
        macros::user::remove_macros(minification::comments::remove_comments(util::util::read_file(file_path.clone()))),
    ));
    let interpolated_file_lines: Vec<String> = imports::imports::interpolate_imports(file_path.clone());
    let macros: Vec<String> = macros::user::get_macros(interpolated_file_lines.clone());

    for line in original_file_lines {
        let mut line_replaced = false;
        for searching_macro in &macros {
            let searching_macro_key: String = macros::user::get_macro_key(searching_macro.clone());

            if line.contains(&*searching_macro_key) {
                let macro_value: String = macros::user::get_macro_value(searching_macro.clone());

                let interpolated_macro_value: String =
                    macros::user::replace_macro_parameter(macro_value, line.clone());

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

    util::util::write_to_file(new_file_path, new_file_lines.clone());

    let new_file_path: String = format!("{}{}", file_path, ".bin");

    util::util::write_to_file(new_file_path.clone(), new_file_lines);

    return new_file_path;
}
