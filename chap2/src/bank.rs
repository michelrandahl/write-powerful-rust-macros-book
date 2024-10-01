struct FirstName { val : String}
struct LastName { val : String}
struct Age { val : i32 }
struct Pay { val : i32 }

macro_rules! gen_get_val {
    ($struct_type : ident) => (
        impl $struct_type {
            pub fn get_value(&self) -> &String {
                &self.val
            }
        }
    );
    ($struct_type : ident, $return_type : ty) => (
        impl $struct_type {
            pub fn get_value(&self) -> $return_type {
                self.val
            }
        }
    );
}

//impl From<i32> for Age {
//    fn from(val: i32) -> Self {
//        Self { val }
//    }
//}

macro_rules! gen_from {
    ($struct_type : ident) => (
        impl From<&str> for $struct_type {
            fn from(val : &str) -> Self {
                Self { val: String::from(val) }
            }
        }
    );

    ($struct_type : ident, $from_type : ty) => (
        impl From<$from_type> for $struct_type {
            fn from(val : $from_type) -> Self {
                Self { val }
            }
        }
    )
}

gen_get_val!(FirstName);
gen_from!(FirstName);

gen_get_val!(LastName);
gen_from!(LastName);

gen_get_val!(Age, i32);
gen_from!(Age, i32);

gen_get_val!(Pay, i32);
gen_from!(Pay, i32);

pub fn test_fn() {
    let a = Age {val : 42};
    println!("{}", a.get_value());

    let n = FirstName {val : "joe".to_string()};
    println!("{}", n.get_value());

    let foo = FirstName::from("billy");
    println!("{}", foo.get_value());
}
