


fn main() {
    println!("Hello World!");
}
struct Bot {
    Commands: Vec<fn( msg: String, sender: String) -> (String)> 
}