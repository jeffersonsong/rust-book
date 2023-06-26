fn main() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("Some numbers: {first}, {last}"),
    }

    match numbers {
        (.., second, ..) => println!("Some numbers: {}", second),
    }
}