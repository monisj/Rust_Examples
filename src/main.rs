
fn main(){ //Driver Code function 
    //let a =12; // without let mut a=12 the variables are immutable 
    //let b =14;
    //let c = a+b;
    //let mut a: i32 = 42 here mutability is added with the integer 32bit contraint datatype
    //println!("Sum of {} and {} are = {}",a,b,c);
    //{                                                                                         //Uncomment for basic example
    //let c=12; // Values in scope will be deleted after the scope is being utlized
    //println!("C={}",c); // The curly brackets represent the scope
    //}
   // println!("C={}",c); //Value went back to the original c variable
     
   
    println!("First Function");
    
    let  mut missiles: i32 = 8;
    let ready: i32 = 2;
    println!("Firing {} of my Ready {} missiles !!!",missiles,ready);
    missiles = missiles-ready;
    println!("Remaining = {}",missiles);   
    main2(missiles)
    
}
const STARTING_MISSILES: i32 =8;
const READY_AMOUNT: i32 = 2;

fn main2(missiles: i32) {   //Second Function
    println!("Second Function");    
    println!("Remaining = {}",missiles);
    let missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    println!("Firing {} of my Ready {} missiles !!!",missiles,ready);
}

