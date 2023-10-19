pub fn is_prime(num:u32)->bool{
    for i in 1..num/2{
        if num%(i+1) == 0{
            return false;
        }
    }
    true
}

pub fn list_primes(n:u32){
    let mut i = 0;
    let mut num = 1;
    let mut arr:Vec<u32>= vec![];
    while i < n{
        if is_prime(num){
            i+=1;
            arr2.push(num);
        }
        num+=1;
    }
}
