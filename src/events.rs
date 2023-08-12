multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("contractSpawned")]
    fn contract_spawned_event(&self, #[indexed] address: ManagedAddress);

    #[event("contractUpgraded")]
    fn contract_upgraded_event(&self, #[indexed] address: ManagedAddress);
}
