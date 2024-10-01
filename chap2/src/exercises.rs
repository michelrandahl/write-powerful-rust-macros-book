macro_rules! hello_world {
    ($something : ident) => (
        impl $something {
            fn hello_world(&self) {
                println!("hello world!");
            }
        }
    );
}


struct Foo;
hello_world!(Foo);

pub fn test_fn() {
    let x = Foo;
    x.hello_world();
}
