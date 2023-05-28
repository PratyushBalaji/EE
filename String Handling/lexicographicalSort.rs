fn sort(arr:Vec<&str>)->Vec<&str>{
    let mut ret = arr;

    for i in 0..ret.len()-1{
        for j in 0..ret.len()-i-1{
            if ret[j] > ret[j+1]{
                let temp = ret[j];
                ret[j] = ret[j+1];
                ret[j+1] = temp;
            }
        }
    }
    ret
}
fn main(){
    let arr:Vec<&str> = ["hello","world","qux","foo","bar"].to_vec();
    println!("{:?}",sort(arr))
}