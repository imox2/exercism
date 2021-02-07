use std::collections::HashMap;
pub fn convert_option_to_value(option: Option<&char>) -> String {
    match option {
        Some(option) => option.to_string(),
        None => "Not Found".to_string(),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let allowed_char: [char; 6] = ['[',']','{','}','(',')'];
    let mut char_inverse_map_start: HashMap<char, char> = HashMap::new();
	char_inverse_map_start.insert('[' , ']');
	char_inverse_map_start.insert('{' , '}');
    char_inverse_map_start.insert('(' , ')');
    let mut stack = vec!['n'];
    for char in string.chars() {
        if allowed_char.iter().any(|&i| i==char) {
            // check start
            if char_inverse_map_start.contains_key(&char) {
                // push to stack
                stack.push(convert_option_to_value(char_inverse_map_start.get(&char)).chars().next().unwrap());
            } else {
                let stack_top = convert_option_to_value(stack.last());
                if stack_top != "Not Found" && stack_top.chars().next().unwrap() == char {
                    stack.pop();
                } else {
                    stack.push(convert_option_to_value(char_inverse_map_start.get(&char)).chars().next().unwrap());
                }
            }
        }
    }
    stack.len() == 1
}