#[cfg(test)]
mod main_test;

fn main() {
    println!("Hello, world!");

    println!("10 + 5 equals : {}", add_two_numbers(10,5));
    println!("10 - 5 equals : {}", subtract_two_numbers(10,5));
}

pub fn add_two_numbers( a:i32, b:i32 ) -> i32 {
    a + b
}

pub fn subtract_two_numbers( a:i32, b:i32 ) -> i32 {
    a - b
}