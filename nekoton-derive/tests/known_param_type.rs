use ton_abi::{Param, ParamType};
use ton_block::MsgAddressInt;
use ton_types::UInt256;

use nekoton_abi::{uint256_bytes, KnownParamType, KnownParamTypePlain};

#[derive(KnownParamTypePlain, Debug)]
pub struct PlainStruct {
    #[abi]
    pub tokens: u128,
    #[abi(with = "uint256_bytes")]
    pub sender_public_key: UInt256,
    #[abi(address)]
    pub sender_address: MsgAddressInt,
    #[abi(array)]
    pub array: Vec<u32>,
    #[abi(array, uint32)]
    pub array_explicit: Vec<u32>,
}

#[derive(KnownParamType)]
pub struct Struct {
    #[abi]
    pub tokens: u128,
    #[abi(with = "uint256_bytes")]
    pub sender_public_key: UInt256,
    #[abi(address)]
    pub sender_address: MsgAddressInt,
    #[abi(array)]
    pub array: Vec<u32>,
    #[abi(array, uint32)]
    pub array_explicit: Vec<u32>,
}

fn main() {
    let params = vec![
        Param::new("tokens", ParamType::Uint(128)),
        Param::new("sender_public_key", ParamType::Uint(256)),
        Param::new("sender_address", ParamType::Address),
        Param::new("array", ParamType::Array(Box::new(ParamType::Uint(32)))),
        Param::new(
            "array_explicit",
            ParamType::Array(Box::new(ParamType::Uint(32))),
        ),
    ];

    assert_eq!(params, PlainStruct::param_type());
    assert_eq!(ParamType::Tuple(params), Struct::param_type());
}