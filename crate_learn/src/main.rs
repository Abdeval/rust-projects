pub use crate_learn::addition::add;
use crate_learn::multiplication::multiply;
use rand::Rng;
// use std::{cmp::Ordering, io};

fn main() {
    // let somme = add(2, 3);
    let result = multiply(2, 3);
    println!("Multiplication : {}", result);
    // println!("Somme : {}", somme);

    // ! external dependancies
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number: {}", secret_number);

    // ! front of house
    crate_learn::front_of_house::hosting::add_to_waitlist();

    // ! using vectors
    // let mut v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector: {:?}", v);

    // todo: how to access elements in a vector
    let i: i32 = v[0];
    println!("First element: {}", i);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("Third element: {}", third),
        None => println!("No third element"),
    }
}
