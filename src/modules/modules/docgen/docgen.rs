use std::process::exit;

const DOCGEN_PREFIX: &str = "/**";
const DOCGEN_SUFFIX: &str = "*/";

fn generate_file(path: String) {
    let mut result = String::new();

    let mut is_doc = false;
    for line in std::fs::read_to_string(&path)
        .expect("Something went wrong reading the file")
        .lines()
    {
        if line.starts_with(DOCGEN_PREFIX) {
            is_doc = true;
        }

        if is_doc {
            result.push_str(line);
            result.push_str("\n");
        }

        if line.contains(DOCGEN_SUFFIX) {
            is_doc = false;
        }
    }

    // split the file path by slashes and get the last element
    let file_name = path.split("/").last().unwrap();
    let docs_file_name = String::from(file_name) + ".md";

    let output_path = format!("build/{}", docs_file_name);
    std::fs::write(output_path, result).expect("Unable to write file");

    println!("Generated documentation for {}", path);
}

// this module can either be run as a separate program or as a module in the
// 2lang cli
pub fn main(args: Vec<String>) {
    if args.len() < 1 {
        println!("Usage: {} <command> [options]", args[0]);
        exit(1);
    }

    let files = args[2..].to_vec();
    for file in files {
        generate_file(file);
    }
}
