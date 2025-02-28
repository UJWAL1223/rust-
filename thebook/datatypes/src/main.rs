fn main() {

    let x = 2.0; //float 64 bit

    let y: f32 = 3.0; //float 32 bit

    let sum = 1+2; //addition

    let difference = 95.5 - 4.3; //subtraction
    
    let product = 9 * 8; //multiplication

    let quotient = 56.7 / 32.2; // division
    let truncated = -5 / 3; //result in -1

    let remainder = 21 % 2; //reminder
                            
    let t =  true; //boolean

    let f: bool = false; // with explicit type annotation 

    let c = 'z'; //character
    
    let z: char = 'Z'; //Explicit type 
    

    let tupl = (12,3.1,11); //tuple

    let (f, g, h) = tupl;

    println!("The value of g is {g}");

    let tup: (i32, f64, u8) = (500, 6.4, 1); //tuple with explicit

    //accesing the value of tuple

    let one = tupl.2;

    let six_point_four = tupl.1;

    let five_hundred = tupl.0;

    //array 
    let a = [1,2,35,5,4];

    let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];

    let ar: [i32; 5] = [1,3,4,6,7];

    let s = [3; 5]; //Same value (3) five times

    let first = a[0];
    let last = a[4];

    

}
