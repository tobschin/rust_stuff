mod moo;
mod trait_me;
mod examples;

fn main() {
    let f = moo::nested::foo();
    println!("{}", f);
    println!("Hello, world!");
    println!("{:?}", examples::closure::closure_example(vec![0,81]));

    let mut my_num = 2;
    examples::reference::quadrat(&mut my_num);
    println!("{:?}", my_num);
}
