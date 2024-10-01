// disallow for example using `$(..)+` in the matcher while using `$(..)?` in the transcriber
// (might trigger false positives)
#![deny(meta_variable_misuse)]
// trace_macros requires nightly unstable channel
//#![feature(trace_macros)] 

#[macro_use]
mod greeting;
use crate::greeting::base_greeting_fn;

#[macro_use]
mod bank;

#[macro_use]
mod compose;

mod exercises;

macro_rules! my_vec {
    () => [ // we can use square brackets for the transcriber
        Vec::new()
    ];

    // we can pattern match on a long arbitrary sentence like the following
    (make me an empty vector please!) => ( // we can use parens for the transcriber
        Vec::new()
    );
    //
    // we use $x to capture the argument, and for this case it can be any expression
    // `expr` is a keyword for denoting expression
    {$x:expr} => { // using { for the transcriber
        { // since we have multiple lines we have to create a scope, just like with anon funs
            let mut v = Vec::new();
            v.push($x);
            v
        }
    };

    // varargs matching
    ($($x:expr),+$(,)?) => (
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    );
}

fn main() {
    let empty : Vec<i32> = my_vec!();
    println!("{:#?}", empty);

    let empty : Vec<i32> = my_vec!(make me an empty vector please!);
    println!("{:#?}", empty);

    let stuff : Vec<i32> = my_vec!(42);
    println!("{:#?}", stuff);

    let stuff : Vec<i32> = my_vec!(42 + 42);
    println!("{:#?}", stuff);

    let stuff : Vec<i32> = my_vec!(32,32,43,42);
    println!("{:#?}", stuff);

    let stuff : Vec<i32> = my_vec!(32,32,43,42,);
    println!("{:#?}", stuff);

    // greeting tests
    let greet = greeting!("sam", "hi");
    println!("{:#?}", greet);

    let greet = greeting!("sam");
    println!("{:#?}", greet);

    println!("testing bank stuff");
    bank::test_fn();

    println!();

    println!("testing compose");
    compose::test_fn();

    println!();

    println!("Exercises:");
    exercises::test_fn();
}
