multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    fn emit_fees_deposited_event(&self, caller: ManagedAddress, amount: BigUint) {
        self.fees_deposited_event(caller, amount);
    }

    #[event("fees_deposited")]
    fn fees_deposited_event(&self, #[indexed] caller: ManagedAddress, #[indexed] amount: BigUint);
}
