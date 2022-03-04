mod blockchain;
use blockchain::Blockchain;

fn main() {
    let bc = Blockchain::new("Hello, world!");
    println!("last block: {:?}", bc);
}
