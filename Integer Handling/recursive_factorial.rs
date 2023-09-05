fn recursive_factorial(n:i32)->i32{
    if n == 1{
        1
    }else{
        n * recursive_factorial(n-1)
    }
}

fn main(){
    println!("{}",recursive_factorial(10));
}
