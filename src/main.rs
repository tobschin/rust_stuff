mod moo;
mod trait_me;
mod examples;

fn main() {
    let f = moo::nested::foo();
    println!("{}", f);
    println!("Hello, world!");
    print!("{:?}", examples::closure::closure_example(vec![0,81]));
}
