pub mod hello_world;

pub use hello_world::greet;

pub fn hoge() {
    println!("hello hoge!");
    hello_world::greet();
}
