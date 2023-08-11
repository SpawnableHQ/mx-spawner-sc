#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait SpawnerContract {
    #[init]
    fn init(&self) {}

    #[endpoint(spawnContract)]
    fn spawn_contract_endpoint(
        &self,
        code: ManagedBuffer,
        code_metadata: CodeMetadata,
        gas: u64,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let (address, _) = self.send_raw().deploy_contract(
            gas,
            &BigUint::zero(),
            &code,
            code_metadata,
            &args.to_arg_buffer(),
        );

        self.contracts().insert(address);
    }

    #[endpoint(spawnObject)]
    fn spawn_object_endpoint(&self) {
        //
    }

    #[view(getContracts)]
    #[storage_mapper("contracts")]
    fn contracts(&self) -> UnorderedSetMapper<ManagedAddress>;
}
