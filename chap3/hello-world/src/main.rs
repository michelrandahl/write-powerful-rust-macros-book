#[macro_use]
extern crate hello_world_macro;

// requires unstable/nightly
// #![feature(trace_macros)]

#[derive(Hello)]
struct Example;

#[derive(Hello)]
struct Foobar;

// To see expanded code produced by the macros:
// make sure cargo-expand is installed: `$ cargo install cargo-expand`
// run: `$ cargo expand` in the project

fn main() {
    println!("Hello, world!");

    let e = Example {};
    e.hello_world();
    e.uppercase();
    Example::testing_testing();
    e.greet("joe");

    // requires unstable/nightly
    //trace_macros!(true);
    let f = Foobar {};
    f.hello_world();
    // requires unstable/nightly
    //trace_macros!(false);
}
