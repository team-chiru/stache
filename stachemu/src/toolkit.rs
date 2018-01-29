pub fn is_matching(template: &str, to_match: &str) -> bool {
    let mut value_to_match = template.chars();
    let mut chars_to_match = to_match.chars();

    while let Some(value_c) = value_to_match.next() {
        if let Some(match_c) = chars_to_match.next() {
            if value_c != match_c {
                return false;
            }
        }
    }

    true
}

pub fn interpolate(context: &String) -> Option<String> {
    let mut value = String::default();
    let mut context = context.chars().rev().collect::<String>();

    while let Some(c) = context.pop() {
        if c.is_whitespace() {
            break;
        } else {
            value.push(c);
        }
    }

    if value != String::default() {
        Some(value)
    } else {
        None
    }
}