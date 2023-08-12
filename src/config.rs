multiversx_sc::imports!();

pub const ROYALTIES_MAX: u32 = 100_00;

#[multiversx_sc::module]
pub trait ConfigModule {
    fn require_caller_is_admin(&self) {
        let caller = self.blockchain().get_caller();
        require!(self.admins().contains(&caller), "not allowed for user");
    }

    #[view(getAdmins)]
    #[storage_mapper("admins")]
    fn admins(&self) -> UnorderedSetMapper<ManagedAddress>;
}
