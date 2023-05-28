fn reverse(string:String)->String{
    let mut rev = "".to_string();
    for i in 0..string.len(){
        let index = string.len()-i-1;
        rev.push(string.chars().nth(index).unwrap());
    }
    rev
}

fn main(){
    let string = "hello".to_string();
    println!("{}",reverse(string));
}