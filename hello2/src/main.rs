mod libs; //initialized or imported from libs.rs
use crate::libs::called_by_main_rs;
fn main() {
    println!("Going to second function");
    module_function();
}
fn module_function(){
    println!("{} = value returned by libs.rs",called_by_main_rs(10)); //calling function from libs.rs
}

