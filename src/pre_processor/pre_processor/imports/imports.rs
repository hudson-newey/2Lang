use crate::pre_processor::pre_processor::{errors, util};
use crate::tokens::tokens;

pub struct InterpolateImportReturn {
    pub file_content: Vec<String>,
    pub imported_files: Vec<String>,
}

pub fn interpolate_imports(
    file_lines: Vec<String>,
    file_path: String,
    suppress_errors: &bool,
) -> InterpolateImportReturn {
    let mut throw_errors: bool = false;
    let mut file_content: Vec<String> = Vec::new();

    let mut line_number: usize = 0;

    let mut imported_files: Vec<String> = Vec::new();

    let mut evaluated_imports: Vec<String> = Vec::new();
    for line in file_lines {
        line_number += 1;

        if line.starts_with(tokens::IMPORT)
            && !line.starts_with(tokens::PRE_PROCESSOR_DIRECTIVE)
            && !line.starts_with(tokens::PRE_PROCESSOR_DIRECTIVE_INTERPRETER)
        {
            // we do not want to import the same file multiple times
            // however, we want to allow using the same template multiple times
            if evaluated_imports.contains(&line) && !line.contains(".template.2") {
                print!("Skipping import {}", line);
                continue;
            }

            // remove the @ symbol and read the file at the remaining file path
            // replace the line with the contents of the file
            let imported_file_path: String = line[1..].to_string();

            imported_files.push(imported_file_path.clone());

            // check if the file exists
            if !std::path::Path::new(&imported_file_path).exists() {
                errors::files::import_error(
                    file_path.to_string(),
                    line_number,
                    imported_file_path.clone(),
                );

                throw_errors = true;
            } else {
                let imported_file_lines: Vec<String> = util::util::read_file(imported_file_path);

                for imported_line in imported_file_lines {
                    file_content.push(imported_line);
                }

                file_content.push("\n".to_string());

                evaluated_imports.push(line);
            }
        } else {
            file_content.push(line);
        }
    }

    // TODO: we should probably return a Result<T> type here so that once macro
    // errors are added, we can have multiple error types emitted
    if throw_errors && !*suppress_errors {
        std::process::exit(1);
    }

    return InterpolateImportReturn {
        file_content,
        imported_files,
    };
}

pub fn has_imports(file_contents: Vec<String>) -> bool {
    for line in file_contents {
        if line.starts_with(tokens::IMPORT) {
            if !line.starts_with(tokens::PRE_PROCESSOR_DIRECTIVE)
                && !line.starts_with(tokens::PRE_PROCESSOR_DIRECTIVE_INTERPRETER)
            {
                return true;
            }
        }
    }

    return false;
}

pub fn remove_imports(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();

    for line in file_contents {
        if !line.starts_with(tokens::IMPORT)
            || line.starts_with(tokens::PRE_PROCESSOR_DIRECTIVE)
            || line.starts_with(tokens::PRE_PROCESSOR_DIRECTIVE_INTERPRETER)
        {
            new_file_contents.push(line);
        }
    }

    return new_file_contents;
}
