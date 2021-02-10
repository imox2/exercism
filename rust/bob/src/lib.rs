use regex::Regex;

pub fn reply(message: &str) -> String {
    let answer;
    let all_spaces = Regex::new(r"^\s+$").unwrap();
    let all_one_uppercase = Regex::new(r".*[A-Z]").unwrap();
    if message.is_empty() || all_spaces.is_match(message) {
        answer = "Fine. Be that way!".to_string();
    } else {
        let last_char = message.trim().chars().last().unwrap();
        let re = Regex::new(r"^[^a-z]*$").unwrap();
        if re.is_match(message) && all_one_uppercase.is_match(message) {
            if last_char == '?' {
                answer = "Calm down, I know what I'm doing!".to_string();
            } else {
                answer = "Whoa, chill out!".to_string();
            }
        } else if last_char == '?' {
            answer = "Sure.".to_string();
        } else {
            answer = "Whatever.".to_string();
        }
    }
    answer
}
