mod macros {
    #[macro_export]
    macro_rules! two_plus_two {
        () => { 2 + 2 };
    }
}

macro_rules! one_plus_one {
    () => { 1 + 1 };
}

macro_rules! one_and_one {
    (plus) => { 1 + 1 };
    (minus) => { 1 - 1 };
    (mult) => { 1 * 1 };
}

fn main() {
    println!("1 + 1 = {}", one_plus_one!());
    println!("1 + 1 = {}", one_and_one!(plus));
    println!("1 - 1 = {}", one_and_one!(minus));
    println!("1 * 1 = {}", one_and_one!(mult));

    println!("2 + 2 = {}", two_plus_two!());
}
