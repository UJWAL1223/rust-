fn main() {
    

    let number = 6;
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    if number % 4 == 0{
        println!("number is divisible by 4");
    } else if number % 3 == 0{
        println!("number is divisible with 3");
    } else if number % 2 == 0{
        println!("number is divisible by 2");
    } else {
        println!("none of the above");
    }

    println!("\n");
    loops();    
    
    println!("\n");
    anotherLoop();

    println!("\n");
    whileLoop();

    println!("\n");
    arrayLoop();

    println!("\n");
    forArrayLoop();
}

fn forArrayLoop(){
    let a = [11,12,13,14,15,16,17,18,19,20];

    for element in a{
        println!("the value is: {element}");
    }
}

fn arrayLoop(){
    let a = [11,12,13,14,15,16,17,18,19,20];
    let mut index = 0;

    while index < 5{
        println!("thevalue is: {}", a[index]);
        index += 1;
    }
}

fn whileLoop(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loops(){


    let mut count = 0;

    let result = loop{
        count += 1;

        if count == 10{
            break count * 2;
        }
    };

    println!("The result is {result}");

}

fn anotherLoop(){
    
    let mut count = 0;
    'countingUp: loop {

        println!("count = {count}");
        let mut remaining = 10;
        

        loop{

            println!("remaining = {remaining}");
            if remaining == 9 {

                break;
            }
            if count == 2{

                break 'countingUp;

    
            }

            remaining -= 1;

        }

        count += 1;

    }

    println!("End count = {count}");

}
