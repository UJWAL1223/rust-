fn main() {
    println!("Hello, world!");

    another_function();

    one_more(12);

    multiple(6,'m');

    let y = {
        let x = 2;
        x + 1
    };

    println!("The value of y is {y}");

    let x = five();

    println!("The value of x is {x}");
}

fn another_function(){
    println!("This is another function");
}

fn one_more(x: i32){
    println!("The value of x is {x}");
}

fn multiple(value: i32,unit: char){
    println!("The length is {value}{unit}");
}

fn five() -> i32{
    5
}


