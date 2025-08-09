mod loops;
mod struct_learn;
mod traits;
mod valid_parenthesis;

use core::panic;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use struct_learn::Rectangle;
use traits::{notify, Summary, Tweet};
use valid_parenthesis::ValidParenthesis;
fn main() -> Result<(), Box<dyn Error>> {
    let username = fs::read_to_string("hello.txt")?;
    println!("Hello, {username}!");
    // ! immutability

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // ! constant

    const CONSTANT_OF_ME: u32 = 10;
    println!("{}", CONSTANT_OF_ME);

    // ! shadowing

    let x = 5;

    // let x = x + 2;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // ! compound types

    let x: (i32, f64, u8) = (500, 6.4, 1);
    // ? here is the right way to print a tuple
    println!("{:?}", x);

    // ! arrays

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    // ! string literals

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    let mut s3 = String::from("hello");
    let s4 = change(&mut s3);
    println!("s3 = {}, s4 = {}", s3, s4);

    // ! the slice type
    let mut numbers = vec![10, 20, 30, 40];
    let slice: &mut [i32] = &mut numbers[0..2];

    // Modify the first element of the slice
    slice[0] = 99;

    // The original `numbers` vector is now changed.
    // `numbers` will be [99, 20, 30, 40]
    println!("{:?}", numbers);

    // ! using struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        username: String::from("alice"),
        email: String::from("alice@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.username = String::from("bob"); // This line will cause an error because user1 is immutable

    println!(
        "User: {}, Email: {}, Sign In Count: {}, Active: {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    // ! tuples
    let point1 = (10, 20);
    let area = surface(point1);
    println!("The area of the rectangle is: {}", area);

    // ! using struct from another file
    let rect = Rectangle {
        width: 50,
        height: 50,
    };

    println!(
        "The are of the rectangle with area function is : {}",
        struct_learn::area(&rect)
    );
    println!("The are of the rectangle is: {:#?}", rect);

    // ! using dbg! macro
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    // ! using methods
    let rect2 = Rectangle {
        width: 50,
        height: 40,
    };
    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));

    // ! automatic referencing and dereferencing
    rect1.can_hold(&rect2);
    (&rect1).can_hold(&rect2);
    //  *rect1 dereferencing &, &mut, and *

    // ! using enums
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let ip4 = IpAddrKind::V4;
    let ip6 = IpAddrKind::V6;
    println!("IP4: {:?}, IP6: {:?}", ip4, ip6);

    impl IpAddrKind {
        fn is_ipv4(&self) -> bool {
            matches!(self, IpAddrKind::V4)
        }
    }
    println!("Is ip4 IPv4 ? {}", ip4.is_ipv4());

    // ! absent value
    let absent_number: Option<i32> = None;
    // let absent_string: Option<String> = None;

    println!("Absent string: {:?}", absent_number);

    // ! if let syntax
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // ! testing the valid parenthesis
    let valid_string = String::from("()[]{}");
    let invalid_string = String::from("([)]");
    println!(
        "Is '{}' a valid parenthesis? {}",
        valid_string,
        ValidParenthesis::is_valid(&valid_string)
    );
    println!(
        "Is '{}' a valid parenthesis? {}",
        invalid_string,
        ValidParenthesis::is_valid(&invalid_string)
    );

    // todo: loops
    loops::for_loop();
    loops::while_loop();

    // todo: master match statement
    // ? you must use Some for all the types that you want to match.
    let number: Option<i32> = Some(7);
    match number {
        Some(number) if number < 5 => println!("The number is less than 5: {}", number),
        Some(number) if number == 5 => println!("The number is equal to 5: {}", number),
        Some(number) if number > 5 => println!("The number is greater than 5: {}", number),
        None => println!("No number provided"),
        _ => println!("Invalid number"),
    }
    // ! strings in depth
    let mut s = String::new();
    s.push('h');
    s.push_str("ello");
    println!("String s: {}", s);

    // ! using hash maps
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    println!("Scores: {:?}", scores);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("Updated Scores: {:?}", scores);

    // ! handling errors
    // panic!("This is a panic message!"); // This will cause the program to panic and exit
    // ? recouvrable errors
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("File opened successfully: {:?}", greeting_file);

    // ! using unwrap and expect
    let greeting_file = File::open("hello.txt").expect("the file must be with .txt extension.");
    println!("File opened successfully: {:?}", greeting_file);

    let username =
        read_username_from_file_shorter().expect("Failed to read the username from the file");
    println!("Username red from file: {}", username);

    // ! Guess number
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}",
                    value
                );
            }
            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    let guess = Guess::new(99);
    println!("Guess value: {}", guess.value());

    // ! generic types
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is: {}", result);
    let chars = vec!['y', 'm', 'a', 'q'];
    let result_char = largest(&chars);
    println!("The largest char is: {}", result_char);

    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let point = Point { x: 5, y: 10.5 };
    println!("The point is: {:?}", point);

    // ! traits
    let author: Tweet = Tweet {
        username: String::from("abdelatif"),
        content: String::from("One of the most"),
    };

    println!("Author: {}", author.summarize());
    // ! the parameter type of the notify function is any type that impl the Summary trait
    notify(&author);
    // ? trait bound syntax
    // pub fn notify<T: Summary>(item1: &T, item2: &T) {}
    // ? where clause
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {
    // }
    // ! lifetimes
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // ? x is out of scope
    // println!("r: {}", r);

    Ok(())
}
// ! functions
fn gives_ownership() -> String {
    let some_string = String::from("ownership");
    some_string // return some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

// ! mutable references and borrowing
fn change(s: &mut String) -> String {
    s.push_str(" world");
    s.to_string()
}

fn surface(dimentions: (i32, i32)) -> i32 {
    dimentions.0 * dimentions.1
}

// ! reading from a file with long error handling
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// ! reading from a file with short error handling
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// ! lifetime annotations

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
