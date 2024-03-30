fn main() {
    const ARRAY_REPEAT_VALUE: &str = ""; //This constant is defined as an empty string "". It is immutable and its type is a reference to a string slice &str.
    const ARRAY_SIZE: usize = 100; //This constant is defined as 100. It represents the size of the array we will create later. Its type is usize, which is an unsigned integer type representing the size of memory in bytes.
    let arr: [String; ARRAY_SIZE] = std::array::from_fn(|_| ARRAY_REPEAT_VALUE.to_string()); //Here, we declare a mutable variable arr of type array [String; ARRAY_SIZE]. This means arr is an array of String type with a size of ARRAY_SIZE, which is 100.
    //This initializes the array arr using the from_fn function from the std::array module. It creates an array where each element is initialized by invoking the closure |_| ARRAY_REPEAT_VALUE.to_string() for each index. In this closure, ARRAY_REPEAT_VALUE (which is an empty string) is converted to a String.

    for (i, _item) in arr.iter().enumerate() //This iterates over each element of the array arr along with its index. We use _item to indicate that we're not using the value of the item inside the loop.
    {
        let num = i + 1;
        //This block calculates the result based on the FizzBuzz logic. If num is divisible by both 3 and 5, set result to "FizzBuzz". If divisible only by 3, "Fizz", by 5, "Buzz", else it's the string representation of num.
        let result = if num % 3 == 0 && num % 5 == 0 {
            "FizzBuzz".to_string()
        } else if num % 3 == 0 {
            "Fizz".to_string()
        } else if num % 5 == 0 {
            "Buzz".to_string()
        } else {
            num.to_string()
        };

        println!("{}", result); //print the result for each iteration.
    }
}
