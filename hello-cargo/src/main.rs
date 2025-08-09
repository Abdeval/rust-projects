fn main() {
    greet();
    another_function(greet());
    println!("************");
    let x = {
        let a = 5;
        let b = 10;
        a + b  // No semicolon → returns 15
    };
    println!("{}", x);

}


fn greet() -> i32 {
    let x = 34;
    x
}

fn another_function(x: i32) {
    println!("here is the value of {x}");
}
