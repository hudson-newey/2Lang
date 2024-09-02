use crate::tokens::tokens;

pub fn remove_comments(file_contents: Vec<String>) -> Vec<String> {
    let mut new_file_contents = file_contents;

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
    let mut new_file_contents = Vec::new();

    let mut is_inside_block_comment = false;
    for line in file_contents {
        if line.starts_with(tokens::BLOCK_COMMENT_START) {
            is_inside_block_comment = true;
        }

        if !is_inside_block_comment {
            new_file_contents.push(line.clone());
        }

        if line.contains(tokens::BLOCK_COMMENT_END) {
            is_inside_block_comment = false;
        }
    }

    return new_file_contents;
}
