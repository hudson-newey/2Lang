use super::generic::generic_error;

pub fn file_error(file_path: String) {
    let error_message: String = format!("File not found: '{}'", file_path);
    generic_error(error_message, 0, file_path);
}

pub fn import_error(file_path: String, line_number: usize, import_path: String) {
    let error_message: String = format!("Import '{}' could not be resolved", import_path);
    generic_error(error_message, line_number, file_path);
}
