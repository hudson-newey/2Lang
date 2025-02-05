use crate::tokens::tokens;

pub fn remove_comments(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = file_contents;

    new_file_contents = remove_single_line_comments(new_file_contents);
    new_file_contents = remove_block_comments(new_file_contents);

    return new_file_contents;
}

fn remove_single_line_comments(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();

    for line in file_contents {
        if !line.starts_with(tokens::LINE_COMMENT) {
            new_file_contents.push(line);
        }
    }

    return new_file_contents;
}

fn remove_block_comments(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents: Vec<String> = Vec::new();
    let mut is_inside_block_comment: bool = false;

    for line in file_contents {
        let mut processed_line: String = String::new();
        let mut current: usize = 0;
        let line_len: usize = line.len();

        while current < line_len {
            if is_inside_block_comment {
                if let Some(end) = line[current..].find(tokens::BLOCK_COMMENT_END) {
                    current += end + tokens::BLOCK_COMMENT_END.len();
                    is_inside_block_comment = false;
                } else {
                    current = line_len;
                }
            } else {
                if let Some(start) = line[current..].find(tokens::BLOCK_COMMENT_START) {
                    processed_line.push_str(&line[current..current + start]);

                    current += start + tokens::BLOCK_COMMENT_START.len();

                    is_inside_block_comment = true;
                } else {
                    processed_line.push_str(&line[current..]);

                    current = line_len;
                }
            }
        }

        if !processed_line.is_empty() || line.is_empty() {
            new_file_contents.push(processed_line);
        }
    }

    return new_file_contents;
}
