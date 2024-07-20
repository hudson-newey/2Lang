pub fn preserve_intermediate(cla: Vec<String>) -> bool {
    const PRESERVE_INTERMEDIATE: &str = "--preserve-intermediate";
    const PRESERVE_INTERMEDIATE_SHORT: &str = "-p";
    return cla.contains(&PRESERVE_INTERMEDIATE.to_string())
        || cla.contains(&PRESERVE_INTERMEDIATE_SHORT.to_string());
}

pub fn preserve_linked(cla: Vec<String>) -> bool {
    const PRESERVE_LINKED: &str = "--preserve-linked";
    return cla.contains(&PRESERVE_LINKED.to_string());
}

pub fn generate_intermediate(cla: Vec<String>) -> bool {
    const GENERATE_INTERMEDIATE: &str = "--generate-intermediate";
    const GENERATE_INTERMEDIATE_SHORT: &str = "-b";
    return !(cla.contains(&GENERATE_INTERMEDIATE.to_string())
        || cla.contains(&GENERATE_INTERMEDIATE_SHORT.to_string()));
}

pub fn log_debug(cla: Vec<String>) -> bool {
    const LOG_DEBUG: &str = "--debug";
    const LOG_DEBUG_SHORT: &str = "-d";
    return cla.contains(&LOG_DEBUG.to_string()) || cla.contains(&LOG_DEBUG_SHORT.to_string());
}

pub fn output_file_path(cla: Vec<String>) -> String {
    let default_file_path: String = "a.out".to_string();

    const OUTPUT_FILE_PATH: &str = "--output";
    const OUTPUT_FILE_PATH_SHORT: &str = "-o";

    // find the position of the output flag if it exists
    // if it does, return the next argument as the output file path
    // if not return the default file path
    let output_file_path_position = cla
        .iter()
        .position(|x| x == &OUTPUT_FILE_PATH || x == &OUTPUT_FILE_PATH_SHORT);

    if output_file_path_position.is_none() {
        return default_file_path;
    }

    let output_file_path_position = output_file_path_position.unwrap();

    if output_file_path_position + 1 >= cla.len() {
        return default_file_path;
    }

    return cla[output_file_path_position + 1].clone();
}

pub fn output_to_stdout(cla: Vec<String>) -> bool {
    const OUTPUT_TO_STDOUT: &str = "--stdout";
    const OUTPUT_TO_STDOUT_SHORT: &str = "-s";
    return cla.contains(&OUTPUT_TO_STDOUT.to_string())
        || cla.contains(&OUTPUT_TO_STDOUT_SHORT.to_string());
}

pub fn no_expand_strings(cla: Vec<String>) -> bool {
    const NO_EXPAND_STRINGS: &str = "--no-expand-strings";
    return cla.contains(&NO_EXPAND_STRINGS.to_string());
}

pub fn auto_run(cla: Vec<String>) -> bool {
    const AUTO_RUN: &str = "--run";
    return cla.contains(&AUTO_RUN.to_string());
}
