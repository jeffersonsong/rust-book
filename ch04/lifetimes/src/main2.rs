/*fn longest<'a>(x1: &'a str, x2: &'a str) -> &'a str {
    if x1.len() > x2.len() {
        x1
    } else {
        x2
    }
}*/

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_ptr()
}

fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}