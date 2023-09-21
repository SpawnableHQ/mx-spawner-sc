#![no_std]

multiversx_sc::imports!();

pub mod blueprint;
pub mod config;
pub mod contract;

#[multiversx_sc::contract]
pub trait SpawnerContract: config::ConfigModule + blueprint::BlueprintModule + contract::ContractModule {
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
}
