pub fn reply(message: &str) -> &str {

    let trimmed_msg = message.trim_right();
    let is_all_caps = all_caps(trimmed_msg);

    match trimmed_msg.chars().last() {
        Some(ch) if ch == '?' && is_all_caps => "Calm down, I know what I'm doing!",
        Some(ch) if ch == '?' => "Sure.",
        Some(_) if is_all_caps => "Whoa, chill out!",
        Some(_) => "Whatever.",
        None => "Fine. Be that way!"
    }
}

///
/// Returns true if message consists of capital letters [1, ..) + neutral symbols [0, ...)
///
fn all_caps(message: &str) -> bool {
    message.len() > 0 &&
        message.chars()
            .into_iter()
            .map(|c| if c.is_uppercase() { 1 } else if is_neutral(c) { 0 } else { -1 })
            /* make sure we have at least one uppercase character */
            .fold(0, |acc, item| if (acc) < 0 { acc } else if item < 0 { item } else { acc + item }) > 0
}

///
/// Checks whether character is neutral (does not affect decision that message is all caps)
///
fn is_neutral(c: char) -> bool {
    c.is_whitespace() || c.is_numeric() || c.is_ascii_punctuation()
}
