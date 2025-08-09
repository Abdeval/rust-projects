pub fn for_loop() {
    for i in 0..10 {
        print!("i = {}, ", i);
    }
    println!(); // Print a newline after the loop
}

pub fn while_loop() {
    let mut i = 0;
    while i < 10 {
        print!("i = {}, ", i);
        i += 1; 
    }
}