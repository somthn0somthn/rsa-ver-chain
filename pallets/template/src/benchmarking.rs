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

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
