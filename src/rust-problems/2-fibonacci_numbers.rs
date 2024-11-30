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

fn fib(num: i32) -> i32 {
    let mut first = 0; // mutable variables whose values can change
    let mut second = 1;

    if (num == 0) {
        return first;
    }
    if (num == 1) {
        return second;
    }

    for i in 1..num - 2 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}

// Code to Generate Fibonacci Series in Rust:

fn fibonacci_series(n: usize) -> Vec<u32> {
    let mut series = vec![0, 1]; // Starting values for the Fibonacci series

    for i in 2..n {
        let next = series[i - 1] + series[i - 2];
        series.push(next);
    }

    series
}

// fn main() {
//     let n = 10; // Number of terms to generate
//     let series = fibonacci_series(n);
//     println!("Fibonacci series ({} terms): {:?}", n, series);
// }
