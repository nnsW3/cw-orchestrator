use cosm_script::{
    contract::{Contract, ContractCodeReference},
    index_response::IndexResponse,
    state::StateInterface,
    tx_handler::{TxHandler, TxResponse},
    CosmScriptError, Daemon, Mock,
};
use cosmwasm_std::{Addr, Binary, Empty, Uint128};
use cw20::{Cw20Coin, MinterResponse, BalanceResponse};
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_multi_test::ContractWrapper;
use serde::Serialize;
use std::{fmt::Debug, ops::Deref};

use crate::CwPlusContract;

pub type Cw20<Chain> = CwPlusContract<Chain, ExecuteMsg, InstantiateMsg, QueryMsg, Empty>;

// implement chain-generic functions
impl<Chain: TxHandler + Clone> Cw20<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: &Chain) -> Self {
        Self(Contract::new(name, chain))
    }
    pub fn send(
        &self,
        msg: Binary,
        amount: u128,
        contract: String,
    ) -> Result<TxResponse<Chain>, CosmScriptError> {
        let msg = ExecuteMsg::Send {
            contract,
            amount: Uint128::new(amount),
            msg,
        };

        self.execute(&msg, None)
    }

    pub fn create_new<T: Into<Uint128>>(
        &self,
        minter: &Addr,
        balance: T,
    ) -> Result<TxResponse<Chain>, CosmScriptError> {
        let msg = InstantiateMsg {
            decimals: 6,
            mint: Some(MinterResponse {
                cap: None,
                minter: minter.to_string(),
            }),
            symbol: "TEST".into(),
            name: self.0.name.to_string(),
            initial_balances: vec![Cw20Coin {
                address: minter.to_string(),
                amount: balance.into(),
            }],
            marketing: None,
        };

        self.instantiate(&msg, Some(minter), None)
    }

    pub fn balance(&self, address: &Addr) -> Result<Uint128, CosmScriptError> {
        let bal: BalanceResponse =self.query(&QueryMsg::Balance {
            address: address.to_string(),
        })?;
        Ok(bal.balance)
    }

    pub fn test_generic(&self, sender: Addr) -> Result<(),CosmScriptError> {
        // Instantiate the contract using a custom function
        let resp = self.create_new(&sender, 420u128)?;
        // Access the execution result
        println!("events: {:?}", resp.events());
        // get the user balance and assert for testing purposes
        let new_balance = self.balance(&sender)?;
        // balance == mint balance
        assert_eq!(420u128, new_balance.u128());
        // BURNNNN
        self.execute(
            &cw20::Cw20ExecuteMsg::Burn {
                amount: 96u128.into(),
            },
            None,
        )?;
        let token_info: cw20::TokenInfoResponse =
            self.query(&cw20_base::msg::QueryMsg::TokenInfo {})?;
        println!("token_info: {:?}", token_info);
        Ok(())
    }
}

impl Cw20<Daemon> {
    pub fn source(&self) -> ContractCodeReference {
        ContractCodeReference::WasmCodePath("cw20_base")
    }
}
impl Cw20<Mock> {
    pub fn source(&self) -> ContractCodeReference {
        cosm_script::contract::ContractCodeReference::ContractEndpoints(Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            ),
        ))
    }
}

// fn upload_token<Chain>(token: Cw20<Chain>) -> anyhow::Result<()>
// where
// Chain: TxHandler + Clone,
// <Chain as TxHandler>::Response : IndexResponse,
// Cw20<Chain>: ContractSource
// {
//     token.upload(get_source(&token))?;
//     Ok(())
// }

// impl <S:StateInterface>Cw20<Mock<S>>
// {
//     pub fn source(&self) -> ContractCodeReference<Empty> {
//         let cw20_token_contract = Box::new(ContractWrapper::new_with_empty(
//             cw20_base::contract::execute,
//             cw20_base::contract::instantiate,
//             cw20_base::contract::query,
//         ));
//         ContractCodeReference::ContractEndpoints(cw20_token_contract)
//     }
// }

// impl<Chain: TxRe> CW20<Chain> {
//     /// Send tokens to a contract allong with a contract call
//     pub async fn send(
//         &self,
//         msg: Binary,
//         amount: u128,
//         contract: String,
//     ) -> Result<CosmTxResponse, CosmScriptError> {
//         let msg = ExecuteMsg::Send {
//             contract,
//             amount: Uint128::new(amount),
//             msg,
//         };

//         self.exec(&msg, None).await
//     }

//     /// Instantiate a new token instance with some initial balance given to the minter
//     pub async fn create_new<T: Into<Uint128>>(
//         &self,
//         minter: String,
//         balance: T,
//     ) -> Result<CosmTxResponse, CosmScriptError> {
//         let msg = InstantiateMsg {
//             decimals: 6,
//             mint: Some(MinterResponse {
//                 cap: None,
//                 minter: minter.clone(),
//             }),
//             symbol: self.instance().name.to_ascii_uppercase(),
//             name: self.instance().name.to_string(),
//             initial_balances: vec![Cw20Coin {
//                 address: minter.clone(),
//                 amount: balance.into(),
//             }],
//             marketing: None,
//         };

//         self.init(msg, Some(minter), None).await
//     }
// }