use crate::{state::ChainState, CwOrcError, IndexResponse, Uploadable};
use cosmwasm_std::{Addr, BlockInfo, Coin};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
// Functions that are callable on the cosmwasm chain/mock
pub type TxResponse<Chain> = <Chain as TxHandler>::Response;
/// Signer trait for chains.
/// Accesses the sender information from the chain object to perform actions.
pub trait TxHandler: ChainState + Clone {
    type Response: IndexResponse + Debug;
    type Error: Into<CwOrcError> + Debug;
    type ContractSource;

    // Gets current sender
    fn sender(&self) -> Addr;
    // Skip x amount of blocks
    fn wait_blocks(&self, amount: u64) -> Result<(), Self::Error>;
    fn wait_seconds(&self, secs: u64) -> Result<(), Self::Error>;
    fn next_block(&self) -> Result<(), Self::Error>;
    fn block_info(&self) -> Result<BlockInfo, Self::Error>;
    // Actions //
    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error>;
    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<Self::Response, Self::Error>;
    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, Self::Error>;
    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error>;
}

// Required to be a different trait because it can not be implemented for the generic Mock<...>.
pub trait ChainUpload: TxHandler {
    fn upload(&self, contract_source: &impl Uploadable) -> Result<Self::Response, Self::Error>;
}