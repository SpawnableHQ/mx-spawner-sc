#![allow(non_snake_case)]

pub mod config;
mod proxy;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";

pub async fn spawner_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let config = Config::new();
    let mut interact = ContractInteract::new(config).await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "upgrade" => interact.upgrade().await,
        "addAdmin" => interact.add_admin().await,
        "removeAdmin" => interact.remove_admin().await,
        "depositFees" => interact.deposit_fees_endpoint().await,
        "redeemVoucher" => interact.redeem_voucher_endpoint().await,
        "setVoucherCollection" => interact.set_voucher_collection().await,
        "getAdmins" => interact.admins().await,
        "getManager" => interact.manager().await,
        "getVoucherCollection" => interact.voucher_collection().await,
        "setBlueprintRoyalties" => interact.set_blueprint_royalties_endpoint().await,
        "spawnBlueprint" => interact.spawn_blueprint_endpoint().await,
        "issueBlueprintCollection" => interact.issue_blueprint_collection_endpoint().await,
        "setBlueprintCollectionLocalRoles" => interact.set_collection_local_roles_endpoint().await,
        "spawnContract" => interact.spawn_contract_endpoint().await,
        "respawnContract" => interact.respawn_contract_endpoint().await,
        "callContract" => interact.call_contract_endpoint().await,
        "getContracts" => interact.contracts().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>
}

impl State {
        // Deserializes state from file
        pub fn load_state() -> Self {
            if Path::new(STATE_FILE).exists() {
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            } else {
                Self::default()
            }
        }
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {
            self.contract_address = Some(address);
        }
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }
    }
    
    impl Drop for State {
        // Serializes state to file
        fn drop(&mut self) {
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
            file.write_all(toml::to_string(self).unwrap().as_bytes())
                .unwrap();
        }
    }

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("spawner");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/spawner.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }
    }

    pub async fn deploy(&mut self) {
        let manager = bech32::decode("");

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .init(manager)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

        println!("new address: {new_address_bech32}");
    }

    pub async fn upgrade(&mut self) {
        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .upgrade()
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_admin(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .add_admin(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn remove_admin(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .remove_admin(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn deposit_fees_endpoint(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let opt_project = OptionalValue::Some(ManagedBuffer::new_from_bytes(&b""[..]));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .deposit_fees_endpoint(opt_project)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn redeem_voucher_endpoint(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .redeem_voucher_endpoint()
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_voucher_collection(&mut self) {
        let collection = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .set_voucher_collection(collection)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn admins(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SpawnerContractProxy)
            .admins()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn manager(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SpawnerContractProxy)
            .manager()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn voucher_collection(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SpawnerContractProxy)
            .voucher_collection()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn set_blueprint_royalties_endpoint(&mut self) {
        let royalties = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .set_blueprint_royalties_endpoint(royalties)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn spawn_blueprint_endpoint(&mut self) {
        let receiver = bech32::decode("");
        let name = ManagedBuffer::new_from_bytes(&b""[..]);
        let hash = ManagedBuffer::new_from_bytes(&b""[..]);
        let attributes = ManagedBuffer::new_from_bytes(&b""[..]);
        let uris = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .spawn_blueprint_endpoint(receiver, name, hash, attributes, uris)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn issue_blueprint_collection_endpoint(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .issue_blueprint_collection_endpoint()
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_collection_local_roles_endpoint(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .set_collection_local_roles_endpoint()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn spawn_contract_endpoint(&mut self) {
        let code = ManagedBuffer::new_from_bytes(&b""[..]);
        let code_metadata = CodeMetadata::<StaticApi>::default();
        let gas = 0u64;
        let args = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .spawn_contract_endpoint(code, code_metadata, gas, args)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn respawn_contract_endpoint(&mut self) {
        let address = bech32::decode("");
        let code = ManagedBuffer::new_from_bytes(&b""[..]);
        let code_metadata = CodeMetadata::<StaticApi>::default();
        let gas = 0u64;
        let args = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .respawn_contract_endpoint(address, code, code_metadata, gas, args)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn call_contract_endpoint(&mut self) {
        let contract = bech32::decode("");
        let endpoint = ManagedBuffer::new_from_bytes(&b""[..]);
        let args = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::SpawnerContractProxy)
            .call_contract_endpoint(contract, endpoint, args)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn contracts(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SpawnerContractProxy)
            .contracts()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

}
