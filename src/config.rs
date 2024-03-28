multiversx_sc::imports!();

pub const ROYALTIES_MAX: u32 = 100_00;

#[multiversx_sc::module]
pub trait ConfigModule {
    #[endpoint(setVoucherCollection)]
    fn set_voucher_collection(&self, collection: TokenIdentifier) {
        self.require_caller_is_admin();

        self.voucher_collection().set(&collection);
    }

    fn require_caller_is_admin(&self) {
        let caller = self.blockchain().get_caller();
        let is_admin = self.admins().contains(&caller);

        require!(is_admin, "not allowed for user");
    }

    #[view(getAdmins)]
    #[storage_mapper("admins")]
    fn admins(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getManager)]
    #[storage_mapper("manager")]
    fn manager(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getVoucherCollection)]
    #[storage_mapper("voucher_collection")]
    fn voucher_collection(&self) -> SingleValueMapper<TokenIdentifier>;
}
