pub fn compose_two<A, B, C, F, G>(f : F, g : G) -> impl Fn(A) -> C
    where F : Fn(A) -> B,
          G : Fn(B) -> C
{
    move |x| g(f(x))
}


macro_rules! compose {
    ($last : expr) => ($last);
    ($head : expr, $($tail : expr), +) => (
        compose_two($head, compose!($($tail), +))
    )
}

macro_rules! compose_right {
    ($last : expr) => ($last);
    ($head : expr, $($tail : expr), +) => (
        compose_two(compose_right!($($tail), +), $head)
    )
}
