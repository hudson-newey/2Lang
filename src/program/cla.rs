pub fn preserve_intermediate(cla: Vec<String>) -> bool {
    const PRESERVE_INTERMEDIATE: &str = "--preserve-intermediate";
    const PRESERVE_INTERMEDIATE_SHORT: &str = "-p";
    return cla.contains(&PRESERVE_INTERMEDIATE.to_string())
        || cla.contains(&PRESERVE_INTERMEDIATE_SHORT.to_string());
}

pub fn generate_intermediate(cla: Vec<String>) -> bool {
    const GENERATE_INTERMEDIATE: &str = "--generate-intermediate";
    const GENERATE_INTERMEDIATE_SHORT: &str = "-b";
    return !(
        cla.contains(&GENERATE_INTERMEDIATE.to_string())
        || cla.contains(&GENERATE_INTERMEDIATE_SHORT.to_string())
    );
}
