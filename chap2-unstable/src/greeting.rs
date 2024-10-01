
pub fn base_greeting(name : &str, greeting : &str) -> String {
    format!("{}, {}!", greeting, name)
}

macro_rules! greeting {
    ($name : literal) => (
        base_greeting($name, "Hello")
    );
    ($name : literal, $greeting : literal) => (
        base_greeting($name, $greeting)
    );
    (test $name : literal) => (
        {
            log_syntax!("name passed to test is ", $name);
            println!("default greeting....");
            base_greeting($name, "hello")
        }
    )
}
