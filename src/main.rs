use blockchainlib::*;

fn main() {
  let mut block = Block::new(0, 0, vec![0; 32], 0, "0".to_string(), 0x0fffffffffffffffffffffffffffffff);

  block.hash = block.hash();

  println!("{:?}", &block);

  block.mine();

  println!("{:?}", &block);
}
