multiversx_sc::imports!();

use crate::config;

#[multiversx_sc::module]
pub trait ContractModule: config::ConfigModule {
    #[endpoint(spawnContract)]
    fn spawn_contract_endpoint(&self, code: ManagedBuffer, code_metadata: CodeMetadata, gas: u64, args: MultiValueEncoded<ManagedBuffer>) {
        self.require_caller_is_admin();
        let (address, _) = self.send_raw().deploy_contract(gas, &BigUint::zero(), &code, code_metadata, &args.to_arg_buffer());

        self.contracts().insert(address.clone());
        self.contract_spawned_event(address);
    }

    #[endpoint(respawnContract)]
    fn respawn_contract_endpoint(&self, address: ManagedAddress, code: ManagedBuffer, code_metadata: CodeMetadata, gas: u64, args: MultiValueEncoded<ManagedBuffer>) {
        self.require_caller_is_admin();
        require!(self.contracts().contains(&address), "contract must be spawned first");

        self.send_raw()
            .upgrade_contract(&address, gas, &BigUint::zero(), &code, code_metadata, &args.to_arg_buffer());

        self.contract_respawned_event(address);
    }

    #[endpoint(callContract)]
    fn call_contract_endpoint(&self, contract: ManagedAddress, endpoint: ManagedBuffer, args: MultiValueEncoded<ManagedBuffer>) {
        self.require_caller_is_admin();
        require!(self.contracts().contains(&contract), "contract must be spawned first");

        let gas_left = self.blockchain().get_gas_left();

        self.send()
            .contract_call::<()>(contract, endpoint)
            .with_gas_limit(gas_left)
            .with_raw_arguments(args.to_arg_buffer())
            .execute_on_dest_context::<()>();
    }

    #[view(getContracts)]
    #[storage_mapper("contracts")]
    fn contracts(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[event("contract_spawned")]
    fn contract_spawned_event(&self, #[indexed] address: ManagedAddress);

    #[event("contract_respawned")]
    fn contract_respawned_event(&self, #[indexed] address: ManagedAddress);
}
