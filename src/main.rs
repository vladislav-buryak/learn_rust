use  std::io ;
//module from another file
mod new_math;

//macros 

macro_rules! my_print {
    ($msg:expr) => {
        println!("{}" , $msg);
    };
}

//Modules
mod math {
    fn add(a: f32, b: f32) -> f32 {
        a + b
    }

    fn subtract(a: f32, b: f32) -> f32 {
        a - b
    }

    fn multiply(a: f32, b: f32) -> f32 {
        a * b
    }

    fn divide(a: f32, b: f32) -> f32 {
        a / b
    }

}

fn main() {
    //integer inizialization
    let number : i16 = 100;
    //float inizialization
    let float_number : f32 = 100.566785;
    //boolean inizialization
    let boolean_value : bool = true;
    //character inizialization
    let character_value : char = 'A';
    //string inizialization
    let string_value : &str = "Hello, World!";

    //printing the values
    println!("Integer value: {}", number);
    println!("Float value: {}", float_number);
    println!("Boolean value: {}", boolean_value);
    println!("Character value: {}", character_value);
    println!("String value: {}", string_value);

    //const

    const IP_ADDRESS: &str = "192.168.1.1";
    println!("IP Address(const): {}", IP_ADDRESS);

    // Tuple 
    
    let mut user : (&str, i32, f32, char) = (
        "John Wick",
        42,
        1.75 ,
        'M'        
    );

    user.2 = 1.84 ;//changing the height value of the user tuple

    println!("User: {}, Age: {}, Height: {}, Gender: {}", user.0, user.1, user.2, user.3);

    // Array 

    let mut list : [i32; 5] = [1, 2, 3, 4, 5];
    list[0]  = 10; //changing the first value of the array

    println!("Array value , index 2: {:?}", list[2]);
    println!("Array values : {:?}", list);

    // User input 

    let mut user_data = String::new();
    println!("Enter your name : ");
    io::stdin().read_line(&mut user_data).expect("Failed to read line");
    println!("Hello, {}!", user_data);

    // User input practice programm 

    let mut num1 = String::new();
    let mut num2 = String::new(); 

    println!("Enter fist number : ");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    
    println!("Enter second number : ");
    io::stdin().read_line(&mut num2).expect("Failed to read line");

    let data1 : i32 = num1.trim().parse().expect("Please type a number!");
    let data2 : i32 = num2.trim().parse().expect("Please type a number!");  
    
    println!("Sum of {} and {} is : {}", data1, data2, data1 + data2);

    //Rust Calculator 
    let mut programm : bool = true;

    while programm {
        
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut symbol = String::new();
    
    println!("Enter first number : ");
    io::stdin().read_line(&mut first_number).expect("Failed to read line");
    
    println!("Enter second number : ");
    io::stdin().read_line(&mut second_number).expect("Failed to read line");

    println!("Enter the operation symbol (+, -, *, /) For exit write (0): ");
    io::stdin().read_line(&mut symbol).expect("Failed to read line");
    
    let num1 : f32 = first_number.trim().parse().expect("Please type a number!");
    let num2 : f32 = second_number.trim().parse().expect("Please type a number!");
    let operation : char = symbol.trim().parse().expect("Please type a symbol!");

    let result = match operation {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        '0' => {
            println!("Exiting the calculator...");
            programm = false;
            continue;
        }
        _ => {
            println!("Invalid operation symbol!");
            continue;
        }
    };
    println!("Result of {} {} {} = {}", num1, operation, num2, result);

    }

    //ownership and borrowing
    let s1 = String::from("Hello");
    let s2 = s1; // ownership of s1 is moved to s2

    // println!("s1: {}", s1); // This would cause a compile error because s1 is no longer valid
    println!("s2: {}", s2);

    let s1 = String::from("Hello");
    let s2 = &s1; // borrowing s1, s1 is still valid
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    //mutabble borrowing

    let s = String::from("Hello");
    //change(&mut s); // mutable borrowing of s

    println!("s after change: {}", s);
    
    //LOOPS
    //for loop
    for number in 1..4 {//without including 4
        println!("Number: {}", number);
    }

    for number in 1..=4 { //with including 4
        println!("Number: {}", number);
    }

    for number in (1..=4).rev() { //reversing the range and including 4
        println!("Number: {}", number);
    }   

    for number in (1..=10).rev().step_by(2) { //reversing the range and including 10 and stepping by 2
        println!("Number: {}", number);
    }   

    //while loop

    let mut number = 5;

    while number  > 0 {
        println!("Number: {}", number);
        number -= 1;
    }

    //example for with if statement
    for number in 1..21 {
        if number % 2 == 0 {
            continue; //skip the even numbers
        } 

        if number >= 7 {
            break; //break the loop if number is greater than or equal to 7
        }

        println!("Odd Number: {}", number);
    }

    //loop 

    let mut count = 0 ;

    loop {
        count += 1;
        println!("Count: {}", count);
        
        if count == 5 {
            break; //break the loop if count is equal to 5
        }
    }

    // array with for loop 

    let array = [10 , 20 ,30 , 40 , 50];

    for el in array {
        println!("Array element: {}", el);
    }

    //tuple with for loop 

    //let tuple = ("John Wick", 42, 1.75 , 'M');

    //for el in tuple {
    //    println!("Tuple element: {}", el);
    //}

    // Error because tuple is not iterable

    //Functions work example
    test(); //calling the test function

    add(10, 20); //calling the add function with arguments 10 and 20


    let user = "BorschT";
    greet_user(user); //calling the greet_user function with argument user

    let mut username = String::from("Liquor");
    modern_greet_user(&mut username); //calling the modern_greet_user function with mutable reference to username

    let sum = modern_add(10, 20); //calling the modern_add function with arguments 10 and 20 and storing the return value in sum
    println!("Sum returned from modern_add function: {}", sum);

    let sum2 = change_modern_add(&mut 30, &mut 40); //calling the modern_add function with arguments 30 and 40 and storing the return value in sum2
    println!("Sum returned from change_modern_add function: {}", sum2);

    let data = (10, 20);
    let product = mult(&data); //calling the mult function with reference to data tuple and storing the return value in product
    println!("Product of {} and {} is : {}", data.0, data.1, product);

    //Macros example of work 
    my_print!("Hello, this is a macro example!");


    //Modules example of work
    let sum = math::add(10.0, 20.0);
    let difference = math::subtract(20.0, 10.0);
    let product = math::multiply(10.0, 20.0);
    let quotient = math::divide(20.0, 10.0);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);

    //Modules example of work from another file
    let sum = new_math::add(10.0, 20.0);
    let difference = new_math::subtract(20.0, 10.0);
    let product = new_math::multiply(10.0, 20.0);
    let quotient = new_math::divide(20.0, 10.0);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}

//Function 
fn test() {
    println!("Hello , i'm test fn")
}

fn add(a : i16 , b: i16 ) {
    let res = a+b;
    println!("Sum of {} and {} is : {}", a, b, res);

}

fn greet_user(name : &str) {
    println!("Hello, {}!", name);
}

fn modern_greet_user(name :&mut String) {
    *name = String::from("Borschiha");
    println!("Hello, {}!", name);
}

//Functions with return values
fn modern_add(a : i16 , b: i16 ) -> i16 {
    let res = a + b;
    println!("Sum of {} and {} is : {}", a, b, res);
    res //returning the sum of a and b
}

fn change_modern_add(a : &mut i16 , b: &mut i16 ) ->     i16 {
    let res = *a + *b;
    println!("Sum of {} and {} is : {}", a, b, res);
    res//returning the sum of a and b
}

//example fn with tuple 

fn mult(data : &(i32 , i32)) -> i32 {
    data.0 * data.1 //returning the product of the two elements in the tuple
}