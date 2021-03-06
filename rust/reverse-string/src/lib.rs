use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed_s = String::new();
    for char in input.graphemes(true).rev() {
        reversed_s.push_str(&char.to_string());
    }
    reversed_s
}


