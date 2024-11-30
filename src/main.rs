fn main() {
    println!("Hello, world!");

    // calling the is_even function
    // println!("{}", is_even(0));
    // println!("{}", is_even(-5));
    // println!("{}", is_even(5));
    // println!("{}", is_even(-5.13)); // floating point nuber to avoid
    // println!("{}", is_even(-5.10)); floating point nuber to avoid

    // fibonacci numbers
    // print!("{}", fibonacci(6)); // 8
    print!("{}", fib(8)); // 8
}

// write a function is_even thak takes a number as input and true if its even and false otherwise
// define a function
// define a variable // how to call the defined function
// call the defined function with a value
// if else condition

//<======================================== Problem 1 =================================================>

fn is_even(n: i32) -> bool {
    // return the result of the comparison
    // n % 2 == 0  // on liner of the function

    if n % 2 == 0 {
        return true;
    }
    return false;
}
//<======================================== Problem 1 =================================================>

//<======================================== Problem 2 =================================================>
// function that find the fibonacci of the number that takes it as input
fn fibonacci(n: u32) -> u32 {
    // 4 // 3
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// Dry-Run Step-by-Step:
// Call fibonacci(4):
// n = 4 → 4 > 1, so we calculate fibonacci(3) + fibonacci(2).
// Call fibonacci(3):
// n = 3 → 3 > 1, so we calculate fibonacci(2) + fibonacci(1).
// Call fibonacci(2) (from fibonacci(3)):
// n = 2 → 2 > 1, so we calculate fibonacci(1) + fibonacci(0).
// Call fibonacci(1) (from fibonacci(2)):
// n = 1 → 1 <= 1, return 1.
// Call fibonacci(0) (from fibonacci(2)):
// n = 0 → 0 <= 1, return 0.
// Now, fibonacci(2) = fibonacci(1) + fibonacci(0) = 1 + 0 = 1.
// Call fibonacci(1) (from fibonacci(3)):
// n = 1 → 1 <= 1, return 1.
// Now, fibonacci(3) = fibonacci(2) + fibonacci(1) = 1 + 1 = 2.
// Call fibonacci(2) (from fibonacci(4)):
// n = 2 → Already calculated, fibonacci(2) = 1.
// Now, fibonacci(4) = fibonacci(3) + fibonacci(2) = 2 + 1 = 3.
// Final Result:
// fibonacci(4) = 3
//<======================================== Problem 2 =================================================>

//<======================================== Problem 3 =================================================>
fn fib(num: i32) -> i32 {
    let mut first = 0; // mutable variables whose values can change
    let mut second = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }

    for _ in 0..num - 1 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}
//<======================================== Problem 3 =================================================>
