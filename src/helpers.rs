pub fn is_digit(ch: char) -> bool {
    return ch.is_digit(10);
}

pub fn is_alpha(ch: char) -> bool {
    return ch.is_alphabetic();
}

pub fn is_alpha_num(ch: char) -> bool {
    return ch.is_alphanumeric();
}