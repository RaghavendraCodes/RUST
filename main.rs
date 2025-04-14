use std::time::Instant;
use std::io; 

fn main() {

//  VARIABLES AND MUTABILITY.  
/*
    by default, variables are immutable. 
    -> when u run try to change a variable when it is immutable, 
    the rust compiler throws an error telling u r trying to change
    a immutable variable. 

    you can make them mutable by adding mut in front of the variable name

    constants: 
    -> like immutable variables, constants are values that are bound to name and immutability and are not allowed to change. 
    -> you are not allowed to use mut with constants, u cannot change constants. 
*/
    let mut x = 5; 
    println!("x value is {}", x); 
    x = 6; 
    println!("x value is {}", x); 

    // shadowing - it creates a new space in memory with let keyword for the specified variable and takes the value from the previously defined variable and then updates it and stores it in the new space. 

    // so all the 3 y variables are stored in different areas of the memory.
    let y = 3; // y = 3 
    let y = y + 1; // y = 4

    {
        let y = y * 2; // y = 8 
        println!("the value of y in inner scope: {}", y); // prints 8
    }

    println!("the value of y is {}", y); // prints 4

    // using mut - here, we are creating only one variable in memory with name z and with mut keyword, so when value of z is changed, its changed in the actual memory address of z. 

    // so, theres only 1 memory address is created here and only that is changed over the time. 
    let mut z = 3; // z = 3
    z = z + 1;  // z = 4 
    {
        z = z * 2; // z = 8
        println!("the value of z in inner scope: {}", z); // prints 8
    }

    println!("the value of z is {}", z); // prints 8

    // let spaces = "    "; 
    // let spaces = spaces.len();

    let ans = another_function(x, y, z); 
    // ans = ans * 2; 

    //&str is an immutable reference to a string slice, meaning it doesn’t own the string data — it just points to it.
    //String is a heap-allocated, growable string. This is the type that owns the data, so it can be modified or passed around.

    let myname = "raghu"; 

    // passing the string address and not the string. 
    let greetings = get_greeting(&myname); 

    println!("{}", greetings); 

    println!("{}", ans); 

    // difference between a expression and a statement.
    
    let a = 2; // statement; 

    let b = {
        let c = a + 2; // statement 
        c * 2 // expression 
    }; // returns a value. 

    println!("b : {}", b);


    // if else control. 
    if_else_function(); 

    // loops. 
    loops_function(); 

    // exercise 1 
    convertor(); 

    // exercise 2 
    fibonacci(); 

    let fib = fibonacci_rec(5);
    println!("{}", fib); 
}

fn another_function(x:i32, y:i32, z:i32) -> i32 {
    let total:i32 = x + y + z; 
    return total; 
}

// parameter - its not the string itself but the address of it. 
// return - string format. 
// When you try to concatenate strings, the left-hand side of the   operation must be a String (because it's owned and can be modified), while the right side can be a &str (borrowed data).

fn get_greeting(name:&str) -> String {
    // let hello = "hello" 
    // let greet:String = hello+name;  
    // the above cannot be done because the left side is a &str (address) and right is also a &str. 
    // obviously, u cannot alter upon the address of a string. 
    // that means, the left side should always be a String type so that the data can be modified. 
    // the right side can be anything, it can be of type &str or String. 
    // u cannot return an &str because it would go out of scope. 
    let hello = "hello ".to_owned();
    let greet:String = hello+name; 
    return greet;
}

fn if_else_function() {
    let number = 5; 
    if number < 4 {
        println!("number is lesser than 4"); 
    } else if number == 5 {
        println!("number is equal to 5"); 
    } else {
        println!("number is greater than 4");
    }

    let condition = false; 

    let number = if condition {5} else {6}; 

    println!("{number}");
}

fn loops_function() {
    let mut count = 0; 
    let start = Instant::now();
    loop {
        println!("again. ");
        count+=1; 

        if count >= 1 {
            break; 
        }
    }
    let duration = start.elapsed();
    println!("count : {count}");
    println!("duration : {:?}", duration);
}

// convertor 
fn convertor() {

        let mut option = String::new();
        println!("Enter the option 1 or 2 : "); 
        io::stdin().read_line(&mut option).expect("error"); 
        let option:i8 = option.trim().parse().expect("error");

        println!("option : {}", option); 

        if option == 1 {
            let mut input = String::new(); 
            println!("Enter the value of temperature in celcius : "); 
            io::stdin().read_line(&mut input).expect("error"); 
            let input:f32 = input.trim().parse().expect("error"); 
            // (0°C × 9/5) + 32 = 32F
            let farhenheit:f32 = (input * 9.0/5.0) + 32.0; 
            println!("{} Deg C in farhanheit is {}", input, farhenheit); 
        } else if option == 2 {
            let mut input = String::new(); 
            println!("Enter the value of temperature in Farhenheit : "); 
            io::stdin().read_line(&mut input).expect("error"); 
            let input:f32 = input.trim().parse().expect("error"); 
            let celcius:f32 = (input - 32.0)*(5.0/9.0); 
            println!("{}F in Celcius is {}", input, celcius);
            // (32°F − 32) × 5/9
        } else {
            println!("error: choose only between 1 or 2");
        }
    
}

// Generate the nth Fibonacci number.
fn fibonacci() {
    // fn(n) = fn(n-1) + fn(n-2); 
    let num = 6; 
    let mut i = 2; 
    let mut superprevious = 0; 
    let mut previous = 1; 
    let mut res = 0; 
    while i < num {
        res = previous + superprevious; 
        superprevious = previous; 
        previous = res; 
        i += 1; 
    }

    println!("fibonacci of {} is {}", num, res); 
}

fn fibonacci_rec(num:i32) -> i32 {
    if num == 0 || num == 1 {
        return num; 
    }

    return fibonacci_rec(num - 1) + fibonacci_rec(num - 2); 
}

