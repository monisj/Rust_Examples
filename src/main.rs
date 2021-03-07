
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
    main3()
    }

fn main3(){
    let x= 5;
    let y = { 
        let x =13;
        x+1
    };
    println!("Value of y = {}", y);
    let f_return = main4(4);
    println!("Returned Value = {}",f_return); //Return value function
    array()
}
fn main4(x: i32) -> i32 { //Return Parameter as per data type
    return x;
    
}

fn array(){
    let list= [1,2,3];
    let list2 = [0; 3];
    let list3: [u8; 3] = [1,2,3]; //Array are size limited to 32
    println!("{}",list[0]); // Single array valye
    println!("{}",list2[1]); // Print same value 
    println!("{0:?}",list3); // Print entire array
    if_else_statement()
}

fn if_else_statement(){
    let variable1: i32 = 32;
    let variable2: i32 = 32;
    if variable1+variable2 == 64 {
        println!("Congrats you got 64!!!");
    }                                           //If Else Statement
    else if variable1+variable2 != 64 {
        //println!("I do not know what i got");
    }
    else{
        println!("Value i got is {}",variable1+variable2);
    }

    
    
}


