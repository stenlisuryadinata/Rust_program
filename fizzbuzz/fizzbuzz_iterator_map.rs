#[allow(unused_variables)]
fn main() {
    (1..=100).map(|i| {
        if i % 3 == 0 && i % 5 == 0 {
            "FizzBuzz".to_string()
        } else if i % 3 == 0 {
            "Fizz".to_string()
        } else if i % 5 == 0 {
            "Buzz".to_string()
        } else {
            i.to_string()
        }
    }).for_each(|s| println!("{}", s));
}