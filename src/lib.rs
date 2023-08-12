#![no_std]

multiversx_sc::imports!();

pub mod config;
pub mod events;
pub mod object;

#[multiversx_sc::contract]
pub trait SpawnerContract: config::ConfigModule + object::ObjectModule + events::EventsModule {
    #[init]
    fn init(&self) {
        let caller = self.blockchain().get_caller();

        if !self.admins().contains(&caller) {
            self.admins().insert(caller);
        }
    }

    #[only_owner]
    #[endpoint(addAdmin)]
    fn add_admin(&self, address: ManagedAddress) {
        self.admins().insert(address);
    }

    #[only_owner]
    #[endpoint(removeAdmin)]
    fn remove_admin(&self, address: ManagedAddress) {
        self.admins().swap_remove(&address);
    }

    #[endpoint(spawnContract)]
    fn spawn_contract_endpoint(&self, code: ManagedBuffer, code_metadata: CodeMetadata, gas: u64, args: MultiValueEncoded<ManagedBuffer>) {
        // TODO: guard caller is admin
        let (address, _) = self.send_raw().deploy_contract(gas, &BigUint::zero(), &code, code_metadata, &args.to_arg_buffer());

        self.contracts().insert(address.clone());
        self.contract_spawned_event(address);
    }

    #[endpoint(upgradeContract)]
    fn upgrade_contract_endpoint(&self, address: ManagedAddress, code: ManagedBuffer, code_metadata: CodeMetadata, gas: u64, args: MultiValueEncoded<ManagedBuffer>) {
        // TODO: guard caller is admin
        require!(self.contracts().contains(&address), "contract must be spawned first");

        self.send_raw()
            .upgrade_contract(&address, gas, &BigUint::zero(), &code, code_metadata, &args.to_arg_buffer());

        self.contract_upgraded_event(address);
    }

    #[view(getContracts)]
    #[storage_mapper("contracts")]
    fn contracts(&self) -> UnorderedSetMapper<ManagedAddress>;
}
