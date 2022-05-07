use blockchainlib::*;


fn main()
{						//index, timestamp, 
    let mut block = Block::new(0, now(), vec![0 ; 32], 1, "Genesis Block!".to_owned());
	
	println!("{:?}", &block);

	let h = block.hash();

	println!("{:?}", &h);

	block.hash = h;

	println!("{:?}", &block);
}
