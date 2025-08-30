pub use crate_learn::addition::add;
use crate_learn::{multiple_clone::multiple_clone, multiplication::multiply};
use rand::Rng;
use std::ops::Deref;
// use std::{cmp::Ordering, io};

#[allow(dead_code)]
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use crate::List::{Cons, Nil};

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

    // ! Boxes: data is stored in the heap
    let b = Box::new(4);
    println!("b = {}", b);
    let nested_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    if let Cons(value, _) = nested_list {
        println!("Premier élément : {}", value);
    }
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // ! using multiple clones 
    multiple_clone();

}
