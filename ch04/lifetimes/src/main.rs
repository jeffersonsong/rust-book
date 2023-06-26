use std::fmt::Display;

fn longest_with_an_annoucement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where 
    T: Display{
    println!("Annoucement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = "abcd";
    let y = "xyz";
    let ann = "An important message";

    let result = longest_with_an_annoucement(x, y, ann);
    println!("the longest is '{}'", result);
}