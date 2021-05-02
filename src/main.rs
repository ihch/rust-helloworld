mod modules;

fn main() {
    println!("hello");
    modules::hoge();
    modules::hello_world::greet();
    modules::greet();
}
