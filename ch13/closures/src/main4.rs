fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("Hello"));
    let n = example_closure(5);

    println!("s = {s}");
    println!("n = {n}");
}