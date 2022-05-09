use blockchainlib::*;

fn main()
{
	let difficulty = 0x000fffffffffffffffffffffffffffff;
						//index, timestamp,           nonce      Data(transactions stored)
    let mut block = Block::new(0, now(), vec![0 ; 32], 0, "Genesis Block!".to_owned(), difficulty);
	
	// println!("{:?}", &block);

	// let h = block.hash();

	// println!("{:?}", &h);

	// block.hash = h;

	// block.hash = block.hash();

	// println!("{:?}", &block);

	block.mine();
	println!("Mined Block {:?}", &block);
	// println!("{:?}", &block);

	let mut last_hash = block.hash.clone();

	let mut blockchain = Blockchain 
	{
		blocks: vec![block],
	};

	for i in 1..= 10
	{
		let mut block = Block::new(i, now(), last_hash, 0, "Second Block".to_owned(), difficulty);
		
		block.mine();
		println!("Mined Block {:?}", &block);

		last_hash = block.hash.clone();

		blockchain.blocks.push(block);
	}
}
