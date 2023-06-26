fn main() {
    let hello = "Здравствуйте";

    let s = &hello[0..2];
    
    println!("hello is {hello}");
    println!("s is {s}");

    for c in "Зд".bytes() {
        println!("{c}");
    }    
}
