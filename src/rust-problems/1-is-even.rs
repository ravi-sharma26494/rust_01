fn main() {
    println!("{}", is_even(number: 50))
}

fn is_even_number(number: i32) -> bool {
    if number % 2 == 0 {
        return false;
    } else {
        return true;
    }
}
