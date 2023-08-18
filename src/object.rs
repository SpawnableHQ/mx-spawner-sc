multiversx_sc::imports!();

use crate::config;
use crate::config::ROYALTIES_MAX;

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

    #[storage_mapper("object:collection")]
    fn object_collection(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("object:royalties")]
    fn object_royalties(&self) -> SingleValueMapper<u32>;

    #[event("objectSpawned")]
    fn object_spawned_event(&self, #[indexed] collection_id: TokenIdentifier, #[indexed] nonce: u64);
}
