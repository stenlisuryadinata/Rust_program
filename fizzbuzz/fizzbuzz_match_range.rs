#[allow(unused_variables)]
fn main() {
    for i in 1..=100 {
        match i {
            n if n % 3 == 0 && n % 5 == 0 => println!("FizzBuzz"),
            n if n % 3 == 0 => println!("Fizz"),
            n if n % 5 == 0 => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}
