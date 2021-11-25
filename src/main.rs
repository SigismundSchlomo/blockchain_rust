use blockchainlib::*;

fn main() {
    let mut block = Block::new(0,0,vec![0; 32],0,"0".to_string());

    println!("{:?}", &block );

    let h = block.hash();

    println!("{:?}", &h);

    block.hash = h;

    println!("{:?}", &block);
}
