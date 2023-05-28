fn concat(a:&str,b:&str)->String{
    let concatenated_string = a.to_string() + &b.to_string();
    concatenated_string
}

fn main(){
    let a: &str = "Hello ";
    let b: &str = "World!";
    println!("{}",concat(a,b));
}
