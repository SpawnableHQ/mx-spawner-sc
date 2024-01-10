#![no_std]

multiversx_sc::imports!();

pub mod blueprint;
pub mod config;
pub mod contract;
pub mod events;

#[multiversx_sc::contract]
pub trait SpawnerContract: config::ConfigModule + blueprint::BlueprintModule + contract::ContractModule + events::EventsModule {
    #[init]
    fn init(&self, manager: ManagedAddress) {
        let caller = self.blockchain().get_caller();

        self.manager().set(&manager);

        if !self.admins().contains(&caller) {
            self.admins().insert(caller);
        }
    }

    #[endpoint]
    fn upgrade(&self) {}

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

    #[payable("EGLD")]
    #[endpoint(depositFees)]
    fn deposit_fees_endpoint(&self, opt_project: OptionalValue<ManagedBuffer>) {
        let caller = self.blockchain().get_caller();
        let value = self.call_value().egld_value();
        let manager = self.manager().get();

        self.send().direct_egld(&manager, &value);

        self.emit_fees_deposited_event(caller, opt_project.into_option().unwrap_or_default(), value.clone_value());
    }
}
