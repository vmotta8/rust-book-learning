fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // it is not allowed to do operations with different types
    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("floored: {}", floored);
    println!("remainder: {}", remainder);
    println!("\n\n");

    // just one character
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);
    println!("\n\n");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    println!("\n\n");

    let arr = [10, 20, 30, 40, 50];
    let index = get_index(arr.len());
    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}

fn get_index(array_length: usize) -> usize {
    println!("Please enter an array index.");
    // let mut index = String::new(); 
    loop {
        // index.clear();
        let mut index = String::new();
        std::io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(num) => {
                if num < array_length {
                    num
                } else {
                    println!("Index out of bounds");
                    continue;
                }
            }
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        return index;
    }
}
