multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("contractSpawned")]
    fn contract_spawned_event(&self, #[indexed] address: ManagedAddress);
}
