fn main() {

    //immutable variable
    let x = 5; // can't change the value 
    
    println!("Immutable variable {x}");

    //mutable variable
    let mut y = 10;
    y += 10;

    println!("Mutable variable {y}");

    const SEC_IN_MIN: u32 = 60;

    println!("Constant Variable {SEC_IN_MIN}");

}
