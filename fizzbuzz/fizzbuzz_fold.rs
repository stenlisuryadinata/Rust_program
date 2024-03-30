#[allow(unused_variables)]
fn main() {
    let fizzbuzz = (1..=100).fold(String::new(), |mut acc, i| {
        if i % 3 == 0 && i % 5 == 0 {
            acc.push_str("FizzBuzz\n");
        } else if i % 3 == 0 {
            acc.push_str("Fizz\n");
        } else if i % 5 == 0 {
            acc.push_str("Buzz\n");
        } else {
            acc.push_str(&i.to_string());
            acc.push_str("\n");
        }
        acc
    });
    println!("{}", fizzbuzz);
}
