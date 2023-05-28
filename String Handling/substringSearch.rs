fn search(substring: &str, string: &str) -> bool {
    for i in 0..=(string.len() - substring.len()) {
        if &string[i..(i + substring.len())] == substring {
            return true;
        }
    }
    false
}

fn main() {
    let string = "hello";
    let substring = "llo";
    println!("{}", search(substring, string));
}