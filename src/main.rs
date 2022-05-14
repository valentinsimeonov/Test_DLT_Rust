use blockchainlib::*;

fn main()
{
	let difficulty = 0x000fffffffffffffffffffffffffffff;
	
	
	//index, timestamp,           nonce      Data(transactions stored)
    let mut genesis_block = Block::new(0, now(), vec![0 ; 32], vec![
		Transaction
		{
		inputs: vec![
			transaction::Output
			{
				to_addr: "Chris".to_owned(),
				value: 50,
			}
		],
		outputs: vec![
			transaction::Output
			{
				to_addr:"Alice".to_owned(),
				value: 50,
			},
			transaction::Output
			{
				to_addr:"Bob".to_owned(),
				value: 7,
			},
		],
	},
	], difficulty);

	genesis_block.mine();
	println!("Mined Block {:?}", &genesis_block);

	let mut last_hash = genesis_block.hash.clone();

	let mut blockchain = Blockchain::new();

	blockchain.update_with_block(genesis_block).expect("Failed to Add Genesis Block");




	let mut block = Block::new(1, now(), last_hash, vec![
		Transaction
		{
		inputs: vec![ ],
		outputs: vec![
			transaction::Output
			{
				to_addr:"Chris".to_owned(),
				value: 536,
			},
		],
	},
	Transaction
	{
	inputs: vec![
		blockchain.blocks[0].transactions[0].outputs[0].clone(),
	],
	outputs: vec![
		transaction::Output
		{
			to_addr:"Alice".to_owned(),
			value: 36,
		},
	],
},
	], difficulty);

	block.mine();

	println!("Mined Block {:?}", &block);

	last_hash = block.hash.clone();

	blockchain.update_with_block(block).expect("Failed to Add Block");
}

