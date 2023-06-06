struct Person {
    name: String,
}

fn main() {
    let mut x: i32 = 100;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();

    let mut p = Person{
        name: String::from("Jeremy"),
    };

    println!("{} was here!", p.name);
}
