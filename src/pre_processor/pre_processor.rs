mod minification;
mod errors;
mod imports;
mod macros;
mod code_execution;
mod util;

use util::util::{read_file, write_to_file};
use imports::imports::{remove_imports, interpolate_imports};
use macros::user::{remove_macros, interpolate_macros};
use macros::inbuilt::convert_strings_to_binary;
use minification::comments::remove_comments;
use errors::files::file_error;
use std::path::Path;

pub fn pre_process(file_path: String, should_expand_strings: &bool) -> String {
    // check if the input file exists
    if !Path::new(&file_path).exists() {
        file_error(file_path.clone());
    }

    // by removing comments first we ensure that macros and imports that are
    // commented out do not get processed by the pre-processor
    let no_unit_comments = remove_comments(read_file(file_path.clone()));

    // we interpolate imports first so that we can remove imports
    // and perform inbuilt macro operations to a single file
    let import_interpolated = interpolate_imports(no_unit_comments.clone(), file_path.clone());

    // we remove comments from the import expanded file so that macros that are
    // commented out both in source, and library source files are not
    // expanded during pre-processing
    let no_comments = remove_comments(import_interpolated.clone());

    let binary_strings = match *should_expand_strings {
        true => convert_strings_to_binary(no_comments.clone()),
        false => no_comments.clone()
    };

    // a fully interpolated file contains the maximum compilation unit possible
    // with all macros fully expanded, and imports interpolated in a single
    // source file
    let full_interpolated: Vec<String> = interpolate_macros(&binary_strings);

    // by this point the file all macros should have been fully expanded
    // and we can start removing parts of the file that are not ones or zeros
    // therefore reducing the size that the compiler needs to process
    let no_imports_file = remove_imports(full_interpolated.clone());
    let result = remove_macros(no_imports_file.clone());

    let new_file_path: String = format!("{}.bin", file_path);
    write_to_file(new_file_path.clone(), result.clone());

    return new_file_path;
}
