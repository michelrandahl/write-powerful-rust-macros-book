use private_macro::{private, local_broken, local_fixed, local_fixed2, compose, add_hello, add_hello2};

private!(
    struct Example {
        string_value: String,
        number_value: usize,
    }
);


mod FooFun {
    use private_macro::private2;
    private2!(
        struct Foobar {
            pub stuff: String,
            pub bla: usize,
        }
    );
}

add_hello!(
    struct Foo {}
);

fn add_one(n : usize) -> usize {
    n + 1
}

fn stringify(n : usize) -> String {
    n.to_string()
}

fn prefix_w(prefix : &str) -> impl Fn(String) -> String + '_ {
    move |x| format!("{}{}", prefix, x)
}

fn main() {
    //local_broken!();

    local_fixed!(); // `let greeting = ..` will be in scope
    //local_fixed2!(); // `let greeting = ..` will be in scope

    println!("Hello, world! {}", greeting);

    let foo = Example {string_value: "foobar".to_string(), number_value: 42};
    println!("foo {}", foo.get_string_value());

    let prefix_fn = prefix_w("hello");

    let composed = compose!
        ( add_one
        >> add_one
        >> add_one
        >> stringify
        >> prefix_fn
        );
    let res = composed(42);
    println!("composed {}", res);

    let foo = Foo {};
    foo.hello();

    add_hello2!(Foo);
    foo.hello_world();

    //let x = FooFun::Foobar {stuff: "joe".to_string(), bla: 42};
    let x = FooFun::Foobar::new("joe".to_string(),42);
    println!("{}", x.get_stuff());
    println!("{}", x.get_bla());
}
