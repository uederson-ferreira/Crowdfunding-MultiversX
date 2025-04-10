#![no_std]

pub mod crowdfunding_proxy;

#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Crowdfunding {
    #[init]
    fn init(&self, target: BigUint) {
        self.target().set(&target);
    }
    
    #[upgrade]
    fn upgrade(&self) {}

    #[view]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;
}
