use crate::pre_processor::pre_processor::util;

pub fn string_to_binary(input_string: String) -> String {
    let mut binary_string = String::new();

    for c in input_string.chars() {
        let binary_representation = format!("{:08b}", c as u8);
        binary_string.push_str(&binary_representation);
    }

    return binary_string;
}

// allows you to use strings in your code e.g. "Hello World!"
// instead of having to remember the binary values
pub fn convert_strings_to_binary(content: Vec<String>) -> Vec<String> {
    // strings are defined as anything in between double or single quotation marks
    // we should therefore search for quotation marks, set a flag saying that we are in quotation marks, until we see another
    // get all characters that were found in between the quotation marks by removing the quotation marks
    // and converting the string to ASCII binary
    // we use ASCII binary because that's what processors natively support
    // I might add support for UTF-8/16 in the future

    let mut all_lines: Vec<String> = Vec::new();

    for line in content {
        let mut modified_line = line.clone();

        let text_in_between_quotation_marks: Vec<String> = util::util::find_text_between_quotation_marks(line);

        for text in text_in_between_quotation_marks {
            modified_line = modified_line.replace(&text, &string_to_binary(text.clone()));
        }

        all_lines.push(modified_line);
    }

    return all_lines;
}
