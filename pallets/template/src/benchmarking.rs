//! Benchmarking setup for pallet-template

use super::*;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet as Template;
    use frame_system::RawOrigin;

    #[benchmark]
    fn validate_string() {
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        validate_string(RawOrigin::Signed(caller), b"hello".to_vec());
    }

    #[benchmark]
    fn verify_rsa_signature_valid() {
        let caller: T::AccountId = whitelisted_caller();

        let public_key = hex::decode("30820122300d06092a864886f70d01010105000382010f003082010a0282010100d23d60e4fad30735e4479a517413e58fe1de8a6e62c8d311d45bafafe66766b270b699db278ae672384be76978ed49714b763512b49078efd5d497baae6a3f9c10d8be324b8806728af8dc439892c517774bdbd767b4e2d2f8f5b7b024f9ea270ca2abf8d296c6a196548e6bf3c9590751473694ff4b38a55c57bedd1c003d38287bc336f56ff7a35e7968b8a3c02cb8ac3ed0cf7d360c5f665de633d5aa31093cba0b57e19a169048e0aa94444e841371ab610c8718faed8e92e9a5ff20d0dc785278b79791e32d163261c3ce11cf4f95f7f7d961757038e8c3765e3edd4ecfd19677009c097a3885f673501dadf5cccc8aa2b17a677c3ffe0855fddbd2ec670203010001").unwrap();

        let message = b"hello".to_vec();

        let signature = hex::decode("19ced4730aadf63b370046fbeb4f450bb2105ea1975b438868f22d595179955e12a9f3e2fd8232de69c2c8ebac6c3b9570f3fa503eea92a65fe430c7490a8a08862a9a01dddae937e34d335845386062252f4a65524aa99b5966fd1d73dada048036ef0eba473ad85e0870026c3688325c3977fa81e572d91139a049c9bcbf8ad2c8e113a343ca38d52e736ce41cfcfcf4e4d998a498c282785b8e1c440b366d25101e5f01de42d493f215bb8a82d3706199268119ef14cec50a52c88a4cff228c6aab7773c7d7adce01013c111ee300bc354876ce05d15538c6f07ba88f9db324351e677585ed87fd768ebd01a50eb24254986ed14661c5f279b37f59d44329").unwrap();

        #[extrinsic_call]
        verify_rsa_signature(RawOrigin::Signed(caller), public_key, message, signature);
    }

    impl_benchmark_test_suite!(TemplatePallet, crate::mock::new_test_ext(), crate::mock::Test);
}
