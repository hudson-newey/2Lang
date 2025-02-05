use crate::tokens::argument_flags::{
    AUTO_RUN, GENERATE_INTERMEDIATE, GENERATE_INTERMEDIATE_SHORT, LOG_DEBUG, LOG_DEBUG_SHORT,
    OUTPUT_FILE_PATH, OUTPUT_FILE_PATH_SHORT, OUTPUT_TO_STDOUT, OUTPUT_TO_STDOUT_SHORT,
    PRESERVE_INTERMEDIATE, PRESERVE_INTERMEDIATE_SHORT, PRESERVE_LINKED, PRESERVE_LINKED_SHORT,
    PROCESSOR_COMMENTS, PROCESSOR_COMMENTS_SHORT, WATCH,
};

pub fn preserve_intermediate(args: Vec<String>) -> bool {
    return args.contains(&PRESERVE_INTERMEDIATE.to_string())
        || args.contains(&PRESERVE_INTERMEDIATE_SHORT.to_string());
}

pub fn preserve_linked(args: Vec<String>) -> bool {
    return args.contains(&PRESERVE_LINKED.to_string())
        || args.contains(&PRESERVE_LINKED_SHORT.to_string());
}

pub fn generate_intermediate(args: Vec<String>) -> bool {
    return !(args.contains(&GENERATE_INTERMEDIATE.to_string())
        || args.contains(&GENERATE_INTERMEDIATE_SHORT.to_string()));
}

pub fn processor_comments(args: Vec<String>) -> bool {
    return args.contains(&PROCESSOR_COMMENTS.to_string())
        || args.contains(&PROCESSOR_COMMENTS_SHORT.to_string());
}

pub fn log_debug(args: Vec<String>) -> bool {
    return args.contains(&LOG_DEBUG.to_string()) || args.contains(&LOG_DEBUG_SHORT.to_string());
}

pub fn output_file_path(args: Vec<String>) -> String {
    let default_file_path: String = "build/a.out".to_string();

    // find the position of the output flag if it exists
    // if it does, return the next argument as the output file path
    // if not return the default file path
    let output_file_path_position: Option<usize> = args
        .iter()
        .position(|x: &String| x == &OUTPUT_FILE_PATH || x == &OUTPUT_FILE_PATH_SHORT);

    if output_file_path_position.is_none() {
        return default_file_path;
    }

    let output_file_path_position: usize = output_file_path_position.unwrap();

    if output_file_path_position + 1 >= args.len() {
        return default_file_path;
    }

    return args[output_file_path_position + 1].clone();
}

pub fn output_to_stdout(args: Vec<String>) -> bool {
    return args.contains(&OUTPUT_TO_STDOUT.to_string())
        || args.contains(&OUTPUT_TO_STDOUT_SHORT.to_string());
}

pub fn auto_run(args: Vec<String>) -> bool {
    return args.contains(&AUTO_RUN.to_string());
}

pub fn watch_source(args: Vec<String>) -> bool {
    return args.contains(&WATCH.to_string());
}
