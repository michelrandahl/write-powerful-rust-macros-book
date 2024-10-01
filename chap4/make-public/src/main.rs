use make_public_macro::public;

#[public]
struct Example {
    first: String,
    pub second: u32,
}

fn main() {
    let foo = Example {first: String::from("hello"), second: 42};
    println!("{}", foo.first);
    println!("{}", foo.second);
}
