fn looping_factorial(n:i32)->i32{
    let mut fact:i32 = 1;
    for i in 1..n+1{
        fact*=i;
    }    
    fact
}

fn main(){
    println!("{}",looping_factorial(100));
}
