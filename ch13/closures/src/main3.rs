fn add_one_v1  (x: u32) -> u32 {x + 1}

fn main() {
    let add_one_v2 = |x: u32| -> u32 { x + 1};
    let add_one_v3 = |x|        { x + 1};
    let add_one_v4 = |x|          x + 1;

    println!("add_one_v1(9) = {}", add_one_v1(9));
    println!("add_one_v2(9) = {}", add_one_v2(9));
    println!("add_one_v3(9) = {}", add_one_v3(9));
    println!("add_one_v4(9) = {}", add_one_v4(9));
}
