extern "C" {
    fn foo();
}

fn main() {
    println!("Hello Rusty world!");
    unsafe { foo() };
}
