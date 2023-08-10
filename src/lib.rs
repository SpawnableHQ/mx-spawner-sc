#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait MinterContract {
    #[init]
    fn init(&self) {}

    #[endpoint(mintObject)]
    fn mint_object_endpoint(&self) {
        //
    }
}
