fn split_str<'a>(string: &'a str, splitter: &'a str) -> Vec<&'a str> {
    string.split(splitter).collect()
}

fn main() {
    let string = "hello how are you doing";
    let splitter = " ";
    println!("{:?}", split_str(string, splitter));
}
