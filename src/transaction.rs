use super::*;
use std::collections::HashSet;



/////////////////////////////////////////////////////////////////
/*	Data Structure */
////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Output
{
	pub to_addr: Address,
	pub value: u64,
}

/*	Implementations  */

impl Hashable for Output
{
	fn bytes (&self) -> Vec<u8>
	{
		let mut bytes = vec![];

		bytes.extend(self.to_addr.as_bytes());
		bytes.extend(&u64_bytes(&self.value));

		return bytes;
	}
}
/////////////////////////////////////////////////////////////////
/*	Data Structure */
////////////////////////////////////////////////////////////////
pub struct Transaction
{
	pub inputs: Vec<Output>,
	pub outputs: Vec<Output>,
}

/*	Implementations  */
impl Transaction
{
	/*	Sum up all the values of the inputs	 */
	pub fn input_value (&self) -> u64
	{
		self.inputs
			.iter()
			.map(|input| input.value)
			.sum()
	}
	
	/*	Sum up all the values of the outputs	*/
	pub fn output_value (&self) -> u64
	{
		self.outputs
			.iter()
			.map(|output| output.value)
			.sum()
	}

	/* Return a Set of Hashes corresponding to the inputs */
	pub fn input_hashes (&self) -> HashSet<Hash>
	{
		self.inputs
			.iter()
			.map(|input| input.hash())
			.collect::<HashSet<Hash>>()

	}

	/* Return a Set of Hashes corresponding to the outputs */
	pub fn output_hashes (&self) -> HashSet<Hash>
	{
		self.outputs
			.iter()
			.map(|output| output.hash())
			.collect::<HashSet<Hash>>()

	}

	/*	Checking if Transaction is of type Coinbase */
	pub fn is_coinbase(&self) -> bool
	{
		self.inputs.len() == 0
	}
}


impl Hashable for Transaction
{
	fn bytes (&self) -> Vec<u8>
	{
		let mut bytes = vec![];

		bytes.extend(self.inputs
			.iter()
			.flat_map(|input| input.bytes())
			.collect::<Vec<u8>>());

		bytes.extend(self.outputs
			.iter()
			.flat_map(|output| output.bytes())
			.collect::<Vec<u8>>());
		return bytes
	}
}