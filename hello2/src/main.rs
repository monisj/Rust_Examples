mod libs; //initialized or imported from libs.rs
use crate::libs::called_by_main_rs;

trait Bite{
    fn bite (self: &mut Self);
}


// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//
#[derive(Debug)] // include this line right before your struct definition
struct Grapes{
    amount_left: i32,
}


// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//
impl Bite for Grapes{
    fn bite (self : & mut Self){
        self.amount_left -= 2;
    }
}




fn main() {
    println!("Going to second function");
    module_function();
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);
}
fn module_function(){
    println!("{} = value returned by libs.rs",called_by_main_rs(10)); //calling function from libs.rs
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}




impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}