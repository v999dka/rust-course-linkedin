pub fn celsius_to_fahrenheit(degrees: f64) -> f64 {
    (1.8 * degrees) + 32.0
}

pub fn trim_spaces(s: &str) -> &str {
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}

pub fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a + *b)
}
