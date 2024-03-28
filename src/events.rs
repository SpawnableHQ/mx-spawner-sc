multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    fn emit_fees_deposited_event(&self, caller: ManagedAddress, project: ManagedBuffer, amount: BigUint) {
        self.fees_deposited_event(caller, project, amount);
    }

    fn emit_voucher_redeemed_event(&self, caller: ManagedAddress, nonce: u64) {
        self.voucher_redeemed_event(caller, nonce);
    }

    #[event("fees_deposited")]
    fn fees_deposited_event(&self, #[indexed] caller: ManagedAddress, #[indexed] project: ManagedBuffer, #[indexed] amount: BigUint);

    #[event("voucher_redeemed")]
    fn voucher_redeemed_event(&self, #[indexed] caller: ManagedAddress, #[indexed] nonce: u64);
}
