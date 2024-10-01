#![feature(trace_macros)]
#![feature(log_syntax)]

use std::ops::{Add, Sub};

#[macro_use]
mod greeting;
use crate::greeting::base_greeting;

#[macro_use]
mod alias_stuff;

#[macro_use]
mod banksters;

#[macro_use]
mod compose;
use crate::compose::compose_two;



// (matcher) => (transcriber)
// any of `()`, `{}`, `[]` can be used for the transcriber
macro_rules! my_vec {
    () => (
        Vec::new()
    );
    (make me an empty vector please!) => (
        Vec::new()
    );
    ($x : expr) => (
        {
            let mut v = Vec::new();
            v.push($x);
            v
        }
    );
    ($($x : expr), +) => (
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    )
}

struct FirstName {
    value : String
}
generate_get_value!(FirstName);

struct Age {
    value : u32
}
generate_get_value!(Age, u32);

#[derive(Debug)]
struct Account {
    money : u32
}

impl Account {
    fn add(&mut self, money : u32) {
        self.money = self.money.add(money)
    }
    fn subtract(&mut self, money : u32) {
        self.money = self.money.sub(money)
    }
}

fn add_one(n : i32) -> i32 {
    n + 1
}

fn stringify(n : i32) -> String {
    n.to_string()
}

fn prefix_with<'a>(prefix : &'a str) -> impl Fn(String) -> String + 'a {
    move |x| format!("{}{}", prefix, x)
}


fn main() {
    let empty : Vec<i32> = my_vec![];
    println!("{:?}", empty);
    let also_empty : Vec<i32> = my_vec!(make me an empty vector please!);
    println!("{:?}", also_empty);
    let three_nums = my_vec!(1, 2, 3);
    println!("{:?}", three_nums);

    println!("{}", greeting!("foo"));
    println!("{}", greeting!("bar", "foo"));

    let first_name = FirstName { value : String::from("foo") };
    println!("first name {}", first_name.get_value());

    let age = Age { value : 32 };
    println!("age {}", age.get_value());

    let mut the_poor = Account {
        money : 0
    };
    let mut the_rich = Account {
        money : 200
    };
    exchange!(Give 20 to the_poor);
    exchange!(Take 10 from the_rich);
    exchange!(Give 30 from the_rich to the_poor);
    println!("poor {:?}, rich: {:?}", the_poor, the_rich);

    let composed = compose!(add_one, stringify, prefix_with("Result is: "));
    let res = composed(42);
    println!("{}", res);

    trace_macros!(true);
    let right = compose_right!(prefix_with("right res: "), stringify, add_one);
    let res = right(43);
    println!("{}", res);

    let _greet = greeting!("sam", "heya");
    let _greet_def = greeting!("sam");
    let _greet_def_test = greeting!(test "sam");
    trace_macros!(false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vector() {
        let v: Vec<i32> = my_vec![];
        assert!(v.is_empty());
    }

    #[test]
    fn test_single_element_vector() {
        let v = my_vec![42];
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_multiple_element_vector() {
        let v = my_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_foobar_syntax() {
        let v: Vec<i32> = my_vec!(this is foobar);
        assert!(v.is_empty());
    }
}
