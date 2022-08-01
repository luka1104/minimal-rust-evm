// extern crate minimal_rust_evm;

// use minimal_rust_evm::{run, BlockContext, Bytecode, CallContext};
mod bytecode;
mod context;
mod evm;
mod execution_error;
mod i256;
mod memory;
mod opcode_handlers;
mod opcodes;
mod stack;
mod vm;

pub use crate::context::{BlockContext, CallContext};
pub use bytecode::Bytecode;
pub use bytecode::Instruction;
pub use evm::run;
use ethereum_types::{H160, U256};
use ethabi_contract::use_contract;
use ethabi::{decode, ParamType};
use std::str::FromStr;

use_contract!(helloworld, "./src/build/HelloWorld.abi");

// #[test]
fn main() {
	let address = H160::from_str("e7f1725E7734CE288F8367e1Bb143E90bb3F0512").unwrap();
	let alice_addr = H160::from_str("e7f1725E7734CE288F8367e1Bb143E90bb3F0513").unwrap();
	let mut callcontext = CallContext {
	value: U256::zero(),
	calldata: &[],
	contract_address: address,
	caller_address: alice_addr,
	origin_address: alice_addr,
	gas_price: U256::zero(),
	};

	let result = evm::run(
	&Bytecode::new(
		hex::decode(&include_bytes!("build/HelloWorld.bin").to_vec())
		.unwrap()
		.as_slice(),
	),
	&callcontext,
	&BlockContext::default(),
	);
	println!("DEPLOY RESULT: {:?}", hex::encode(&result.return_data));

	let (input, _decoder) = helloworld::functions::add_message::call("Hello Alice");

    callcontext.calldata = &input;
	let tx = evm::run(
		&Bytecode::new(&result.return_data),
		&callcontext,
		&BlockContext::default(),
	);
	println!("{:?}", hex::encode(&callcontext.calldata));

	let (input, _decoder) = helloworld::functions::messages::call(1);

    callcontext.calldata = &input;
	let message = evm::run(
		&Bytecode::new(&result.return_data),
		&callcontext,
		&BlockContext::default(),
	);
	println!("{:?}", hex::encode(&message.return_data));

	// let config = Config::istanbul();
	// let contract_addr = H160::from_str("0x1000000000000000000000000000000000000000").unwrap();
	// let alice_addr = H160::from_str("0xf000000000000000000000000000000000000000").unwrap();

	// let vicinity = MemoryVicinity {
	// 	gas_price: U256::zero(),
	// 	origin: H160::default(),
	// 	block_hashes: Vec::new(),
	// 	block_number: Default::default(),
	// 	block_coinbase: Default::default(),
	// 	block_timestamp: Default::default(),
	// 	block_difficulty: Default::default(),
	// 	block_gas_limit: Default::default(),
	// 	chain_id: U256::one(),
	// 	block_base_fee_per_gas: U256::zero(),
	// };

	// let state = BTreeMap::new();
	// let backend = MemoryBackend::new(&vicinity, state);
	// let metadata = StackSubstateMetadata::new(u64::MAX, &config);
	// let state = MemoryStackState::new(metadata, &backend);
	// let precompiles = BTreeMap::new();
	// let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

	// let new_contract_addr = executor.create_address(evm::CreateScheme::Legacy { caller: contract_addr });
	// println!("Contract Address: {:?}", new_contract_addr);
	// // Deploy the smart contract
	// let _reason = executor.transact_create(contract_addr, U256::zero(), hex::decode(&include_bytes!("build/HelloWorld.bin").to_vec()).unwrap(), u64::MAX, Vec::new());

	// // Add a new message transaction
	// let (input, _decoder) = helloworld::functions::add_message::call("Hello Alice");
	// let _reason = executor.transact_call(alice_addr, new_contract_addr,U256::zero(), input, u64::MAX, Vec::new());

	// let (input, _decoder) = helloworld::functions::messages::call(1);
	// let _reason = executor.transact_call(alice_addr, new_contract_addr, U256::zero(), input, u64::MAX, Vec::new());
	// println!("Calldata: {:?}", decode(&[ParamType::String], &_reason.1).unwrap());
}