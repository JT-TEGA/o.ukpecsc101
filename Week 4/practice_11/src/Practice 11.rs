fn main() {
    let a:i32 = 2;    // Bit presentation 10
    let b:i32 = 3;    // Bit presentation 11

    let mut result:i32;

    result = a & b;
    println!("(a & b) => {} ",result);

    reult = a | b;
    println!("(a | b) => {} ",result);

    result = a ^ b;
    println!("(a ^ b) => {} ",reslt);

    result = !b;
    println!("(!b) => {}", reslt);

    result = a << b;
    println!("(a << b ) => {}",result);    

    result = a >> b;
    println!("(a >> b ) => {}",result;
}
