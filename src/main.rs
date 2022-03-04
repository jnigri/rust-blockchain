mod blockchain;
use blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new("Hello, world!");
    bc.add(format!("new block"));
    println!("block chain: {:?}", bc);
}
