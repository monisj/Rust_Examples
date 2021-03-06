pub fn called_by_main_rs(x: i32) -> i32{
    println!("Entered into a new file via the Rust Module System");
    let b=x*4;
    return b;
}