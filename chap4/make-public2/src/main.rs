use make_public_macro::{public, delete};

#[public]
#[derive(Debug, Clone)]
#[repr(C)]
#[doc = "This is an example struct with multiple attributes."]
struct Example {
    first: String,
    pub second: u32,
}

#[delete]
struct EmptyStruct {}

#[public]
struct ExampleUnnamed(usize);

mod my_mod {
    // the unnamed values must be `pub` if we wish to access them outside the module
    pub struct Blah(pub usize);
}

mod my_mod2 {
    use make_public_macro::{public, delete};

    #[public]
    struct Foo(usize);

    #[public]
    enum Bar {
        Hello,
        World,
    }
}

fn main() {
    let foo = Example {first: String::from("hello"), second: 42};
    println!("{}", foo.first);
    println!("{}", foo.second);
    println!("{:?}", foo);

    let foo = ExampleUnnamed(42);
    let x = foo.0;
    println!("{}", x);

    let foo = my_mod::Blah(42);
    println!("{}", foo.0);

    let foo = my_mod2::Foo(11);
    println!("{}", foo.0);

    let foo = my_mod2::Bar::Hello;
}
