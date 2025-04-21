#![cfg(test)]

use crate as pallet_rsa_verification_parachain;
use frame::deps::sp_core::H256;
use frame::deps::sp_io;
use frame::deps::sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};
use frame::prelude::*;
use frame::testing_prelude::*;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet
frame::deps::frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Template: pallet_rsa_verification_parachain,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type BlockHashCount = BlockHashCount;
    type SS58Prefix = SS58Prefix;
}

impl pallet_rsa_verification_parachain::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_rsa_verification_parachain::weights::SubstrateWeight<Test>;
}

// Build genesis storage according to the mock runtime
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
