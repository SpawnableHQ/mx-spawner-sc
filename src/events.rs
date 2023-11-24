multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    fn emit_fees_deposited_event(&self, caller: ManagedAddress, project: ManagedBuffer, amount: BigUint) {
        self.fees_deposited_event(caller, project, amount);
    }

    #[event("fees_deposited")]
    fn fees_deposited_event(&self, #[indexed] caller: ManagedAddress, #[indexed] project: ManagedBuffer, #[indexed] amount: BigUint);
}
