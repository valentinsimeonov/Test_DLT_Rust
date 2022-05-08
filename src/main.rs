use blockchainlib::*;


fn main()
{						//index, timestamp,           nonce      Data(transactions stored)
    let mut block = Block::new(0, now(), vec![0 ; 32], 35708, "Genesis Block!".to_owned(), 0x0000ffffffffffffffffffffffffffff);
	
	// println!("{:?}", &block);

	// let h = block.hash();

	// println!("{:?}", &h);

	// block.hash = h;

	block.hash = block.hash();

	println!("{:?}", &block);

	// block.mine();

	// println!("{:?}", &block);
}
