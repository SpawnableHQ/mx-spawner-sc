#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait MinterContract {
    #[init]
    fn init(&self) {}
}
