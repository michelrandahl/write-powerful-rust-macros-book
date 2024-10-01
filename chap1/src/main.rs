
trait Hello {
    fn hello(&self);
}

// blanket implementation (inline trait bound syntax)
//impl<T : Copy> Hello for T {
//    fn hello(&self) {
//        println!("helloooo");
//    }
//}

// blanket implementation (where cluase trait bound syntax)
impl<T> Hello for T
where
    T : Copy
{
    fn hello(&self) {
        println!("helloooo");
    }
}

fn main() {
    println!("Hello, world!");
    2.hello();
    true.hello();
    "hi".hello();
}
