#![allow(
    dead_code,
    unused_variables,
    unused_parens,
    unused_imports,
    unused_mut,
    unused_must_use
)]
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
    for_loops()
}

fn for_loops(){
    for num in 0..50 { //Similar to for i in range(0,50) add (0..=50) for 0 to 50 range
        println!("{}=Value",num);
    }
    clone_strings()
    //`bob: loop{           //identifier with ` sign
       // println!("First Loop");       //Just as an example on how uncoditional loop works
        //loop{
            //println!("Second Loop");
            //loop{
                //break `bob;
           // }
       // }
   // }

   //while dizzy(){
       //println!("Value ={}",a);       //Just as an example on how while loops work on rust
      // a=a+1;
  // }                                  //Will break out once the condition is false same as other languages
}
fn clone_strings(){ //Ownership of variables Function as moving of variable data
    let text1 = String::from("Hello");
    //let text2 = text1;  //Creates an error because of the stack heap assignment rust uses it eliminates the Previous Assignment
    let text2 = text1.clone();
    println!("{} = text1",text1);           
    println!("{} = text2",text2);
    do_stuff(&text2);               //String Function
    println!("{} = text1",text2);
}

fn do_stuff(text3: &String){
    println!("{}",text3);       //Function as to print text as string from above
    let mut v: Vec<i32> = Vec::new();           //Vector usefull for creating lists Also it works as a stack with Push and Pop commands
    v.push(1);
    v.push(2); //Push Function      <> ingle brackets 
    v.push(3);
    v.push(4);
    let x =v.pop(); // Value = 4
    v.pop();   //Pop Function
    println!("{:?} =",v);
    println!("{:?} =",x); //Value = 4
    println!("{}",v[1]); //In This case we get a string value 


    let mut vect = Vec![1,2,3,4]; //Another way to initilize Vector
    println("{} = Value 1",vect[1]);

    let mut h: HashMap<u8,bool> = HashMap::new();    //Create a HashMap sort of a Dictonary with key
    h.insert(1,false); //Inserting Values
    h.insert(2,true);
    let have= h.remove(&2).unwrap(); //to Get and remove value from dictonay
    
}
    



