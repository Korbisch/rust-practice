mod fahrenheit_celsius;
mod fibonacci;
mod christmas;

fn main() {
    println!("{}", fahrenheit_celsius::convert_to_celsius(32));
    println!("{}", fahrenheit_celsius::convert_to_fahrenheit(0));

    println!("{}", fibonacci::fibonacci(100));

    christmas::twelve_days_of_christmas();
}
