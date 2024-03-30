#[allow(unused_variables)]

fn main() {
    let x = 3;
    let y = 5;
    for i in 1..=100 {
        if i % x == 0 && i % y == 0 {
            println!("fizzbuzz");
        } else if i % x == 0 {
            println!("fizz");
        } else if i % y == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }

    
}



