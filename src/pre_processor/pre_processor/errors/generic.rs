use colored::Colorize;

pub fn generic_error(message: String, line_number: usize, file_name: String) {
    println!("{}", "\nCompiler Error".red());
    println!("{}:{}", file_name, line_number);
    println!("\t{:?}\n", message);
}
