fn main() {
    let a = String::from("abc");
    let b = &a as *mut String;
    println!("{:p}", b);
    println!("{:p}", &b);
    println!("{:p}", &a);
    // println!("{:p}", a);

}
