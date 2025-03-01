use std::io;

fn main() {
    loop{
        println!("Enter the operation accordingly\n + for addition\n - for subtraction\n * for multiplication\n / for division\n e for exit"); 
        
        let mut userInput = String::new();


        io::stdin()
            .read_line(&mut userInput);

        if userInput.len() != 1{
            if userInput == "+"{
                
                let result = add(3,2);
                
                println!("{result}");

            } else if userInput == "-"{

                let result = subtract(3,2);

                println!("{result}");
            
            }else if userInput == "*"{

                let result = multiply(3,2);

                println!("{result}");

            }else if userInput == "/"{

                let result = divide(4,2);

                println!("{result}");
            
            }else if userInput == "e"{
            
                break;

            }else {

                println!("Choose correct oprions");
            }
  }

    }
}

fn add(x: i32, y: i32) -> i32{
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32{
    return x - y;
}

fn multiply(x: i32, y: i32) -> i32{
    return x * y;
}

fn divide(x: i32, y: i32) -> i32{
    return x / y;
}

