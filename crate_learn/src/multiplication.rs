use crate::addition::add;


pub fn multiply(a: i32, b: i32) -> i32 {
    let result = add(a, a);
    result * b
}