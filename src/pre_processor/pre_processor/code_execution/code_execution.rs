use crate::tokens::tokens;
use std::borrow::Cow;
use std::io::Write;
use std::process::{exit, Child, Command, Output, Stdio};

pub fn run_code_execution(
    contents: Vec<String>,
    debug: bool,
    suppress_errors: &bool,
) -> Vec<String> {
    return interpreter_code_execution(contents.clone(), debug, suppress_errors);
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
    let mut result: Vec<String> = Vec::new();

    let mut in_code_execution_block: bool = false;
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

fn execute_code(
    interpreter: String,
    code: Vec<String>,
    debug: bool,
    suppress_errors: &bool,
) -> Vec<String> {
    let stringified_code: String = code.join("\n");

    if debug {
        println!("'{}': '{}'", interpreter.to_string(), stringified_code);
    }

    // we use a heredoc to execute the command through an interpreter
    // this means that the interpreter that you are using must support running
    // programs from STDIN
    let heredoc_code: String = format!(r#"{}"#, stringified_code);

    // run the file and capture the standard output
    let mut output: Vec<String> = Vec::new();
    let mut execution_child: Child = Command::new(&interpreter)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    if let Some(mut stdin) = execution_child.stdin.take() {
        stdin
            .write_all(heredoc_code.as_bytes())
            .expect("Failed to write to stdin");
    }

    let code_std_out: Output = execution_child
        .wait_with_output()
        .expect("Failed to capture output");

    // convert the standard output to a string and split it by newlines
    let code_output: Cow<'_, str> = String::from_utf8_lossy(&code_std_out.stdout);
    let errors: Cow<'_, str> = String::from_utf8_lossy(&code_std_out.stderr);

    if !errors.is_empty() {
        println!("Failed to run pre-processor interpreted macro:\n{}", errors);

        if !*suppress_errors {
            exit(1);
        }
    }

    for line in code_output.split("\n") {
        output.push(line.to_string());
    }

    if debug {
        println!(
            "Interpreter '{}' output: '{}'",
            interpreter.to_string(),
            code_output
        );
    }

    return output;
}

fn interpreter_code_execution(
    contents: Vec<String>,
    debug: bool,
    suppress_errors: &bool,
) -> Vec<String> {
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
                .replace("{", "")
                .trim()
                .to_string();
        }

        if in_code_execution_block {
            let stripped_line = line
                .replace(&code_interpreter, "")
                .replace("{", "")
                .replace(tokens::PRE_PROCESSOR_DIRECTIVE_INTERPRETER, "")
                .replace(tokens::PRE_PROCESSOR_DIRECTIVE_END, "");

            code_buffer.push(stripped_line.clone());
        } else {
            result.push(line.clone());
        }

        if line.contains(tokens::PRE_PROCESSOR_DIRECTIVE_END) {
            in_code_execution_block = false;

            let execution_output = execute_code(
                code_interpreter.to_string(),
                code_buffer,
                debug,
                suppress_errors,
            );

            for output_line in execution_output {
                result.push(output_line);
            }

            code_buffer = Vec::new();
        }
    }

    return result;
}
