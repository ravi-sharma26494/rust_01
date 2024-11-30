fn main() {
    println!("Hello, world!");

    // calling the is_even function
    println!("{}", is_even(0));
    println!("{}", is_even(-5));
    println!("{}", is_even(5));
    // println!("{}", is_even(-5.13)); // floating point nuber to avoid
    // println!("{}", is_even(-5.10)); floating point nuber to avoid
}

// write a function is_even thak takes a number as input and true if its even and false otherwise
// define a function
// define a variable // how to call the defined function
// call the defined function with a value
// if else condition
fn is_even(n: i32) -> bool {
    // return the result of the comparison
    // n % 2 == 0  // on liner of the function

    if n % 2 == 0 {
        return true;
    }
    return false;
}
