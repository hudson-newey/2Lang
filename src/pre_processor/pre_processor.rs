mod minification;
mod errors;
mod imports;
mod macros;
mod code_execution;
mod util;

pub fn pre_process(file_path: String, should_expand_strings: &bool) -> String {
    let mut new_file_lines: Vec<String> = Vec::new();

    // check if the input file exists
    if !std::path::Path::new(&file_path).exists() {
        errors::files::file_error(file_path.clone());
    }

    // create an "original file" which contains no comments, macros, and has
    // strings automatically expanded to their binary representation
    let no_comments_file = minification::comments::remove_comments(util::util::read_file(file_path.clone()));
    let no_macros_file = macros::user::remove_macros(no_comments_file.clone());
    let pre_processed_strings = match *should_expand_strings {
        true => macros::inbuilt::convert_strings_to_binary(no_macros_file.clone()),
        false => no_macros_file.clone()
    };

    // we create a "target" which does not contain any macros, imports, or comments
    // the source file will contain a single file that contains all the imports
    // from this file, we generate all the macros keys that we need to replace
    let target: Vec<String> = imports::imports::remove_imports(pre_processed_strings.clone());
    let source: Vec<String> = imports::imports::interpolate_imports(file_path.clone());
    let macros: Vec<String> = macros::user::get_macros(source.clone());

    for line in target {
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
