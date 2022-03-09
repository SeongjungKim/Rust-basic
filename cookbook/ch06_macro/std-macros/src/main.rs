#[derive(Debug)]
pub struct MyStruct(usize);

fn main() {
    println!("Hello, world!");
    println!("a vec: {:?}", vec!{1, 2, 3});
    println!("concat: {}", concat!(0, 'x', "5ff"));
    println!("MyStruct stringified: {}", stringify!(MyStruct(10)));
    println!("some random word stringified: {}", stringify!(helloworld));

    println!("Running on Linux? {}", cfg!(target_os = "linux"));
    println!("$PATH: {:?}", option_env!("PATH"));

    eprintln!("Oh no");
    debug_assert!(true);
}
