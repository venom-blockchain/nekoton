use anyhow::Result;
use nekoton_abi::num_bigint::BigUint;
use nekoton_abi::VmGetterOutput;
use nekoton_jetton::{JettonMetaData, MetaDataContent};
use thiserror::Error;
use ton_block::{Deserializable, MsgAddressInt};
use ton_types::{Cell, SliceData};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct JettonRootData {
    pub total_supply: BigUint,
    pub mintable: bool,
    pub admin_address: MsgAddressInt,
    pub content: JettonMetaData,
    pub wallet_code: Cell,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct JettonRootMeta {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub base_chain_id: String,
    pub base_token: String,
}

pub fn get_jetton_data(res: VmGetterOutput) -> Result<JettonRootData> {
    if !res.is_ok {
        return Err(RootContractError::ExecutionFailed {
            exit_code: res.exit_code,
        }
        .into());
    }

    const JETTON_DATA_STACK_ELEMENTS: usize = 5;

    let stack = res.stack;
    if stack.len() != JETTON_DATA_STACK_ELEMENTS {
        return Err(RootContractError::InvalidMethodResultStackSize {
            actual: stack.len(),
            expected: JETTON_DATA_STACK_ELEMENTS,
        }
        .into());
    }

    let total_supply = stack[0].as_integer()?.into(0..=u128::MAX)?;

    let mintable = stack[1].as_bool()?;

    let mut address_data = stack[2].as_slice()?.clone();

    let admin_address = MsgAddressInt::construct_from(&mut address_data).unwrap_or_default();

    let content = stack[3].as_cell()?;
    let content = MetaDataContent::parse(content)?;

    let wallet_code = stack[4].as_cell()?.clone();

    let dict = match content {
        MetaDataContent::Internal { dict } => dict,
        MetaDataContent::Unsupported => {
            return Err(RootContractError::UnsupportedContentType.into())
        }
    };

    Ok(JettonRootData {
        total_supply: BigUint::from(total_supply),
        mintable,
        admin_address,
        wallet_code,
        content: (&dict).into(),
    })
}

pub fn get_jetton_meta(res: VmGetterOutput) -> Result<JettonRootMeta> {
    if !res.is_ok {
        return Err(RootContractError::ExecutionFailed {
            exit_code: res.exit_code,
        }
        .into());
    }

    const JETTON_META_STACK_ELEMENTS: usize = 5;

    let stack = res.stack;
    if stack.len() != JETTON_META_STACK_ELEMENTS {
        return Err(RootContractError::InvalidMethodResultStackSize {
            actual: stack.len(),
            expected: JETTON_META_STACK_ELEMENTS,
        }
        .into());
    }

    let slice = SliceData::load_cell_ref(stack[0].as_cell()?)?;
    let name = String::from_utf8_lossy(slice.remaining_data().data()).to_string();

    let slice = SliceData::load_cell_ref(stack[1].as_cell()?)?;
    let symbol = String::from_utf8_lossy(slice.remaining_data().data()).to_string();

    let decimals = stack[2].as_integer()?.into(0..=u8::MAX)?;

    let base_chain_id = stack[3].as_integer()?.to_string();
    let base_token = stack[4].as_integer()?.to_string();

    Ok(JettonRootMeta {
        name,
        symbol,
        decimals,
        base_chain_id,
        base_token,
    })
}

pub fn get_wallet_address(res: VmGetterOutput) -> Result<MsgAddressInt> {
    if !res.is_ok {
        return Err(RootContractError::ExecutionFailed {
            exit_code: res.exit_code,
        }
        .into());
    }

    const WALLET_ADDRESS_STACK_ELEMENTS: usize = 1;

    let stack = res.stack;
    if stack.len() != WALLET_ADDRESS_STACK_ELEMENTS {
        return Err(RootContractError::InvalidMethodResultStackSize {
            actual: stack.len(),
            expected: WALLET_ADDRESS_STACK_ELEMENTS,
        }
        .into());
    }

    let mut address_data = stack[0].as_slice()?.clone();
    let address = MsgAddressInt::construct_from(&mut address_data)?;

    Ok(address)
}

#[derive(Error, Debug)]
pub enum RootContractError {
    #[error("ExecutionFailed (exit_code: {exit_code})")]
    ExecutionFailed { exit_code: i32 },
    #[error("Invalid method result stack size (actual: {actual}, expected {expected})")]
    InvalidMethodResultStackSize { actual: usize, expected: usize },
    #[error("Unsupported metadata content type")]
    UnsupportedContentType,
}
