fn fibonacci(n:i32){

    let mut x:i32 = 0;
    let mut y:i32 = 1;

    println!("{}","0");
    println!("{}","1");
    for i in 0..n-2{
        let z:i32 = x+y;
        println!("{}",z);
        x = y;
        y = z;
    }
}
fn main(){
    fibonacci(1000);
}