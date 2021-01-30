pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb: String = "".to_owned();
    let list_size = list.len();

    for i in 0..list_size {
        let mut result: String   = "And all for the want of a {what}.".to_owned();
        if i+1 <list_size {
            let mut phrase_template: String  = "For want of a {first} the {second} was lost.\n".to_owned();
            phrase_template = phrase_template.replace("{first}", list[i]);
            phrase_template = phrase_template.replace("{second}", list[i+1]);
            proverb.push_str(&phrase_template);
        } else {
            result = result.replace("{what}", list[0]);
            proverb.push_str(&result);
        } 
    }
    proverb
}
