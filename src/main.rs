mod moo;
mod trait_me;

fn main() {
    let f = moo::nested::foo();
    println!("{}", f);
    println!("Hello, world!");
}
