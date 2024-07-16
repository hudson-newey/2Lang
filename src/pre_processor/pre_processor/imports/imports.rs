use crate::pre_processor::pre_processor::{util, errors};

pub fn interpolate_imports(file_lines: Vec<String>, file_path: String) -> Vec<String> {
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

            let imported_file_lines: Vec<String> = util::util::read_file(imported_file_path);

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

pub fn remove_imports(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();

    for line in file_contents {
        if !line.starts_with("@") {
            new_file_contents.push(line);
        }
    }

    return new_file_contents;
}
