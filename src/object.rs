multiversx_sc::imports!();

use crate::config;
use crate::config::ROYALTIES_MAX;

const COLLECTION_NAME: &[u8] = b"Objects";
const COLLECTION_TICKER: &[u8] = b"OBJECT";

#[multiversx_sc::module]
pub trait ObjectModule: config::ConfigModule {
    #[endpoint(setObjectRoyalties)]
    fn set_object_royalties_endpoint(&self, royalties: u32) {
        self.require_caller_is_admin();
        require!(royalties <= ROYALTIES_MAX, "invalid amount");

        self.object_royalties().set(royalties);
    }

    #[endpoint(spawnObject)]
    fn spawn_object_endpoint(&self, receiver: ManagedAddress, name: ManagedBuffer, hash: ManagedBuffer, attributes: ManagedBuffer, uri: ManagedBuffer) {
        self.require_caller_is_admin();

        let collection_id = self.object_collection().get();
        let one_big = BigUint::from(1u8);
        let royalties = BigUint::from(self.object_royalties().get());
        let uris = ManagedVec::from_single_item(uri);

        let nonce = self.send().esdt_nft_create(&collection_id, &one_big, &name, &royalties, &hash, &attributes, &uris);

        self.send().direct_esdt(&receiver, &collection_id, nonce, &one_big);

        self.object_spawned_event(collection_id, nonce);
    }

    #[payable("EGLD")]
    #[endpoint(issueObjectCollection)]
    fn issue_object_collection_endpoint(&self) {
        self.require_caller_is_admin();
        require!(self.object_collection().is_empty(), "object collection already set");

        let payment_value = self.call_value().egld_value();
        let properties = SemiFungibleTokenProperties {
            can_freeze: false,
            can_wipe: false,
            can_pause: false,
            can_transfer_create_role: true,
            can_change_owner: true,
            can_upgrade: true,
            can_add_special_roles: true,
        };

        let name = ManagedBuffer::from(COLLECTION_NAME);
        let ticker = ManagedBuffer::from(COLLECTION_TICKER);

        self.send()
            .esdt_system_sc_proxy()
            .issue_semi_fungible(payment_value.clone_value(), &name, &ticker, properties)
            .async_call()
            .with_callback(self.callbacks().collection_issue_callback())
            .call_and_exit();
    }

    #[payable("*")]
    #[callback]
    fn collection_issue_callback(&self, #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => self.object_collection().set(&token_id.unwrap_esdt()),
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.send().direct(&caller, &returned.token_identifier, 0, &returned.amount);
                }
            }
        }
    }

    #[storage_mapper("object:collection")]
    fn object_collection(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("object:royalties")]
    fn object_royalties(&self) -> SingleValueMapper<u32>;

    #[event("objectSpawned")]
    fn object_spawned_event(&self, #[indexed] collection_id: TokenIdentifier, #[indexed] nonce: u64);
}
