pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut total_score = 0;
    let mut current_score = 0;
    let mut total_garbage = 0;
    let mut ignore = false;
    let mut garbage = false;
    for c in input.trim_end().chars() {
        if ignore {
            ignore = false;
        } else if c == '!' {
            ignore = true;
        } else if garbage {
            if c == '>' {
                garbage = false;
            } else {
                total_garbage += 1;
            }
        } else if c == '<' {
            garbage = true;
        } else if c == '{' {
            current_score += 1;
        } else if c == '}' {
            total_score += current_score;
            current_score -= 1;
        }
    }
    assert!(current_score == 0, "Invalid current_score: {current_score}");

    (total_score.to_string(), total_garbage.to_string())
}
