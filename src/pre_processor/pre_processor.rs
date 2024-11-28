mod code_execution;
mod errors;
mod imports;
mod macros;
mod minification;
mod util;

use code_execution::code_execution::{
    has_code_execution_statements, remove_interpreter_code_blocks, run_code_execution,
};
use errors::files::file_error;
use imports::imports::{has_imports, interpolate_imports, remove_imports};
use macros::user::{interpolate_macros, remove_macros};
use minification::comments::remove_comments;
use minification::whitespace::remove_whitespace;
use std::path::Path;
use util::util::{read_file, write_to_file};

pub fn pre_process(file_path: String, preserve_linked: &bool, debug: bool) -> String {
    // check if the input file exists
    if !Path::new(&file_path).exists() {
        file_error(file_path.clone());
    }

    // by removing comments first we ensure that macros and imports that are
    // commented out do not get processed by the pre-processor
    let no_unit_comments = remove_comments(read_file(file_path.clone()));

    // we interpolate imports first so that we can remove imports
    // and perform inbuilt macro operations to a single file

    let mut import_interpolated: Vec<String> = no_unit_comments.clone();
    while has_imports(import_interpolated.clone()) {
        let intermediate_import_interpolate =
            interpolate_imports(import_interpolated.clone(), file_path.clone());

        // we remove comments from the import expanded file so that macros that are
        // commented out both in source, and library source files are not
        // expanded during pre-processing
        import_interpolated = remove_comments(intermediate_import_interpolate.clone());
    }

    let mut code_execute_interpolated = import_interpolated.clone();
    while has_code_execution_statements(code_execute_interpolated.clone()) {
        code_execute_interpolated = run_code_execution(code_execute_interpolated.clone(), debug)
    }

    code_execute_interpolated = remove_interpreter_code_blocks(code_execute_interpolated);

    if *preserve_linked {
        let preserved_linked_file_path = format!("{}.linked", file_path);
        write_to_file(
            preserved_linked_file_path,
            code_execute_interpolated.clone(),
        );
    }

    let mut macro_interpolate = interpolate_macros(&code_execute_interpolated.clone());
    macro_interpolate = interpolate_macros(&macro_interpolate.clone());

    let macro_complete = remove_comments(macro_interpolate.clone());

    // by this point the file all macros should have been fully expanded
    // and we can start removing parts of the file that are not ones or zeros
    // therefore reducing the size that the compiler needs to process
    let no_imports_file = remove_imports(macro_complete.clone());
    let result = remove_macros(no_imports_file.clone());
    let minified_result = remove_whitespace(result.clone());

    let new_file_path: String = format!("{}.bin", file_path);
    write_to_file(new_file_path.clone(), minified_result.clone());

    return new_file_path;
}
