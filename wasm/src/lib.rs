// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           17
// Async Callback:                       1
// Total number of exported functions:  19

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    spawner
    (
        init => init
        upgrade => upgrade
        addAdmin => add_admin
        removeAdmin => remove_admin
        depositFees => deposit_fees_endpoint
        redeemVoucher => redeem_voucher_endpoint
        setVoucherCollection => set_voucher_collection
        getAdmins => admins
        getManager => manager
        getVoucherCollection => voucher_collection
        setBlueprintRoyalties => set_blueprint_royalties_endpoint
        spawnBlueprint => spawn_blueprint_endpoint
        issueBlueprintCollection => issue_blueprint_collection_endpoint
        setBlueprintCollectionLocalRoles => set_collection_local_roles_endpoint
        spawnContract => spawn_contract_endpoint
        respawnContract => respawn_contract_endpoint
        callContract => call_contract_endpoint
        getContracts => contracts
    )
}

multiversx_sc_wasm_adapter::async_callback! { spawner }
