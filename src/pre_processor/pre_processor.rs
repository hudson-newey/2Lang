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
use imports::imports::{has_imports, interpolate_imports, remove_imports, InterpolateImportReturn};
use macros::user::{interpolate_macros, remove_macro_definitions};
use minification::whitespace::remove_whitespace;
use minification::{comments::remove_comments, minification::minify_linked_file};
use std::path::Path;
use util::util::{read_file, write_to_file};

pub struct PreProcessorOutput {
    pub output_path: String,
    pub source_files: Vec<String>,
}

pub fn pre_process(
    file_path: String,
    output_file_path: &String,
    preserve_linked: &bool,
    _processor_comments: &bool,
    debug: bool,
    suppress_errors: &bool,
) -> PreProcessorOutput {
    // check if the input file exists
    if !Path::new(&file_path).exists() {
        file_error(file_path.clone());
    }

    let mut linked_source_files: Vec<String> = Vec::new();

    // by removing comments first we ensure that macros and imports that are
    // commented out do not get processed by the pre-processor
    let no_unit_comments: Vec<String> = remove_comments(read_file(file_path.clone()));

    // we interpolate imports first so that we can remove imports
    // and perform inbuilt macro operations to a single file

    let mut import_interpolated: Vec<String> = no_unit_comments.clone();
    while has_imports(import_interpolated.clone()) {
        let import_output: InterpolateImportReturn = interpolate_imports(
            import_interpolated.clone(),
            file_path.clone(),
            suppress_errors,
        );

        let intermediate_import_interpolate: Vec<String> = import_output.file_content;

        for source_file in import_output.imported_files {
            linked_source_files.push(source_file.clone());
        }

        // we remove comments from the import expanded file so that macros that are
        // commented out both in source, and library source files are not
        // expanded during pre-processing
        import_interpolated = remove_comments(intermediate_import_interpolate.clone());
    }

    let minified_linked_source: Vec<String> = minify_linked_file(import_interpolated);

    let mut code_execute_interpolated: Vec<String> = minified_linked_source.clone();
    while has_code_execution_statements(code_execute_interpolated.clone()) {
        code_execute_interpolated =
            run_code_execution(code_execute_interpolated.clone(), debug, suppress_errors)
    }

    code_execute_interpolated = remove_interpreter_code_blocks(code_execute_interpolated);

    if *preserve_linked {
        let preserved_linked_file_path: String = format!("{}.2.o", output_file_path);
        write_to_file(
            preserved_linked_file_path,
            code_execute_interpolated.clone(),
        );
    }

    let mut macro_interpolate: Vec<String> = interpolate_macros(&code_execute_interpolated.clone());
    macro_interpolate = interpolate_macros(&macro_interpolate.clone());

    let macro_complete: Vec<String> = remove_comments(macro_interpolate.clone());

    // by this point the file all macros should have been fully expanded
    // and we can start removing parts of the file that are not ones or zeros
    // therefore reducing the size that the compiler needs to process
    let no_imports_file: Vec<String> = remove_imports(macro_complete.clone());
    let result: Vec<String> = remove_macro_definitions(no_imports_file.clone());
    let minified_result: Vec<String> = remove_whitespace(result.clone());

    let new_file_path: String = format!("{}.2.bin", output_file_path);
    write_to_file(new_file_path.clone(), minified_result.clone());

    return PreProcessorOutput {
        output_path: new_file_path,
        source_files: linked_source_files,
    };
}
