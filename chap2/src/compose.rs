fn add_one(n : i32) -> i32 {
    n + 1
}

fn stringify(n : i32) -> String {
    n.to_string()
}

fn prefix_w(prefix : &str) -> impl Fn(String) -> String + '_ {
    move |x| format!("{}{}", prefix, x)
}

fn fwrd_comp_two<Fst_arg, Snd_arg, Res, F, G>
    (f : F, g : G) -> impl Fn(Fst_arg) -> Res
where
    F : Fn(Fst_arg) -> Snd_arg,
    G : Fn(Snd_arg) -> Res,
{
    move |x| g(f(x))
}

macro_rules! fwrd_comp {
    ($last : expr) => ( $last );
    ($head : expr, $($tail : expr),+ ) => (
        fwrd_comp_two($head, fwrd_comp!( $($tail),+ ))
    );
}

macro_rules! fwrd_comp2 {
    ($last : expr) => ( $last );
    // NOTE: adding the separator `;` inside the inner match is mmuch easier to read
    ($head : expr $(; $tail : expr)+ ) => (
        fwrd_comp_two($head, fwrd_comp2!( $($tail);+ ))
    );
}

macro_rules! fwrd_comp3 {
    ($last : expr) => ( $last );
    // NOTE: adding the separator `=>` inside the inner match is mmuch easier to read
    ($head : expr $(=> $tail : expr)+ ) => (
        fwrd_comp_two($head, fwrd_comp3!( $($tail)=>+ ))
    );
}

pub fn test_fn() {
    let composed = fwrd_comp_two::<i32, _, String, _, _>(add_one, stringify);
    let x = 42;
    let r = composed(x);
    println!("{}", r);

    let more_comp = fwrd_comp!(
        add_one,
        stringify,
        prefix_w("stuff: "),
        prefix_w("more"),
        prefix_w("more")
    );
    let x = 42;
    let r = more_comp(x);
    println!("{}", r);

    let more_comp2 = fwrd_comp2!(
        add_one;
        stringify;
        prefix_w("stuff: ");
        prefix_w("more");
        prefix_w("more");
        prefix_w("more")
    );
    let x = 42;
    let r = more_comp2(x);
    println!("{}", r);

    let more_comp3 = fwrd_comp3!(
        add_one
        => stringify
        => prefix_w("stuff: ")
        => prefix_w("more")
        => prefix_w("more")
    );
    let x = 42;
    let r = more_comp3(x);
    println!("{}", r);
}
