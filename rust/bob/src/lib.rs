const SIMPLE_QUESTION: &str = "Sure.";
const YELL: &str = "Whoa, chill out!";
const YELL_QUESTION: &str = "Calm down, I know what I'm doing!";
const EMPTY: &str = "Fine. Be that way!";
const DEFAULT: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    match message.trim() {
        m if m == "" => EMPTY,
        m if is_yelling(m) && is_question(m) => YELL_QUESTION,
        m if is_yelling(m) => YELL,
        m if is_question(m) => SIMPLE_QUESTION,
        _ => DEFAULT,
    }
}

fn is_yelling(message: &str) -> bool {
    message.chars().any(|c| c.is_alphabetic()) && message.chars().all(|c| !c.is_lowercase())
}
fn is_question(message: &str) -> bool {
    message.chars().last() == Some('?')
}