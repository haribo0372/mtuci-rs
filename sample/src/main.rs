fn main(){
    let mut v: Vec<i32> = Vec::with_capacity(8);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(1);
    v.push(5);
    v.push(6);
    v.push(1);
    v.push(3);
    // v.resize(4, 228);
    println!("{:?}\n{:?}", v.pop(), v.remove(5))
}