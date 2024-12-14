use std::process::exit;

mod docgen;

pub fn run_module(name: String, args: Vec<String>) {
    let arguments: Vec<String> = args[1..].to_vec();
    match name.as_str() {
        "docgen" => docgen::docgen::main(arguments),
        _ => {
            println!("Module {} not found", name);
            exit(1);
        }
    }
}
