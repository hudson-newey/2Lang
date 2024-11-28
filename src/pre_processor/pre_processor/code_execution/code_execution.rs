use crate::tokens::tokens;
use tempfile::NamedTempFile;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;


pub fn run_code_execution(contents: Vec<String>, debug: bool) -> Vec<String> {
    return interpreter_code_execution(contents.clone(), debug);
}

pub fn has_code_execution_statements(contents: Vec<String>) -> bool {
    for line in contents {
        if line.starts_with(tokens::PRE_PROCESSOR_DIRECTIVE_INTERPRETER) {
            return true;
        }
    }

    return false;
}

pub fn remove_interpreter_code_blocks(contents: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    let mut in_code_execution_block = false;
    for line in contents {
        if line.starts_with(tokens::PRE_PROCESSOR_DIRECTIVE_INTERPRETER) {
            in_code_execution_block = true;
        }

        if !in_code_execution_block {
            result.push(line.clone());
        }

        if line.contains(tokens::PRE_PROCESSOR_DIRECTIVE_END) {
            in_code_execution_block = false;
        }
    }

    return result;
}

fn execute_code(interpreter: String, code: Vec<String>) -> Vec<String> {
    let named_temp_file = NamedTempFile::new().unwrap();
    let mut file = named_temp_file.as_file();
    let path = named_temp_file.path();

    // add a shebang to the file to execute, then write all the code contents
    // to a temporary file
    writeln!(file, "#!{}", interpreter).unwrap();
    for line in code {
        writeln!(file, "{}", line).unwrap();
    }

    // make the file executable
    let mut perms = file.metadata().unwrap().permissions();
    perms.set_mode(0o755);
    file.set_permissions(perms).unwrap();

    // run the file and capture the standard output
    let mut output = Vec::new();
    let code_std_out = std::process::Command::new(path)
        .output()
        .expect("failed to execute process");

    // convert the standard output to a string and split it by newlines
    let code_output = String::from_utf8_lossy(&code_std_out.stdout);
    for line in code_output.split("\n") {
        output.push(line.to_string());
    }

    return output;
}

fn interpreter_code_execution(contents: Vec<String>, debug: bool) -> Vec<String> {
    let mut result = Vec::new();

    let mut code_interpreter = String::new();
    let mut code_buffer = Vec::new();
    let mut in_code_execution_block = false;
    for line in contents.clone() {
        if line.starts_with(tokens::PRE_PROCESSOR_DIRECTIVE_INTERPRETER) {
            in_code_execution_block = true;

            code_interpreter = line
                .replace(tokens::PRE_PROCESSOR_DIRECTIVE_INTERPRETER, "")
                .replace(tokens::PRE_PROCESSOR_DIRECTIVE_END, "")
                .trim()
                .to_string();
        }

        if !in_code_execution_block {
            result.push(line.clone());
        } else {
            code_buffer.push(line.clone());
        }

        if line.contains(tokens::PRE_PROCESSOR_DIRECTIVE_END) {
            in_code_execution_block = false;

            if debug {
                println!("'{}': '{}'", code_interpreter.to_string(), line);
            }

            let execution_output = execute_code(code_interpreter.to_string(), code_buffer);
            for output_line in execution_output {
                result.push(output_line);
            }

            code_buffer = Vec::new();
        }
    }

    return result;
}
