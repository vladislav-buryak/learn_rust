use  std::io ;

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

    let mut s = String::from("Hello");
    change(&mut s); // mutable borrowing of s

    println!("s after change: {}", s);


    
}