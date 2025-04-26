#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Required for no_std environments
extern crate alloc;

// Import RSA libraries
use rsa::pkcs1v15::Signature;
use rsa::signature::Verifier;
use rsa::{pkcs1v15::VerifyingKey, RsaPublicKey};
use sha2::Sha256;
use sp_std::prelude::*;
use spki::DecodePublicKey;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html>
// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>
//
// To see a full list of `pallet` macros and their use cases, see:
// <https://paritytech.github.io/polkadot-sdk/master/pallet_example_kitchensink/index.html>
// <https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/index.html>
#[frame::pallet]
pub mod pallet {
    use crate::weights::WeightInfo;
    use frame::prelude::*;
    // Import the RSA functionality into the pallet module
    use rsa::pkcs1v15::Signature;
    use rsa::signature::Verifier;
    use rsa::{pkcs1v15::VerifyingKey, RsaPublicKey};
    use sha2::Sha256;
    use sp_std::prelude::*;
    use spki::DecodePublicKey;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: crate::weights::WeightInfo;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// A struct to store a single block-number. Has all the right derives to store it in storage.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_storage_derives/index.html>
    #[derive(
        Encode, Decode, MaxEncodedLen, TypeInfo, CloneNoBound, PartialEqNoBound, DefaultNoBound,
    )]
    #[scale_info(skip_type_params(T))]
    pub struct CompositeStruct<T: Config> {
        /// A block number.
        pub(crate) block_number: BlockNumberFor<T>,
    }

    /// The pallet's storage items.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#storage>
    /// <https://paritytech.github.io/polkadot-sdk/master/frame_support/pallet_macros/attr.storage.html>
    #[pallet::storage]
    pub type Something<T: Config> = StorageValue<_, CompositeStruct<T>>;

    /// Pallets use events to inform users when important changes are made.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// We usually use passive tense for events.
        SomethingStored {
            block_number: BlockNumberFor<T>,
            who: T::AccountId,
        },
        ValidString {
            who: T::AccountId,
        },
        InvalidString {
            who: T::AccountId,
        },
        SignatureVerified {
            who: T::AccountId,
        },
        ValidSignedString {
            who: T::AccountId,
        },
        InvalidSignedString {
            who: T::AccountId,
        },
    }

    /// Errors inform users that something went wrong.
    /// <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error>
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        /// Invalid RSA public key format
        InvalidPublicKey,
        /// Invalid signature format
        InvalidSignature,
        /// Signature verification failed
        SignatureVerificationFailed,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Validates if the input string equals "hello"
        #[pallet::call_index(0)] 
        #[pallet::weight(T::WeightInfo::validate_string())]
        pub fn validate_string(
            origin: OriginFor<T>,
            input: sp_std::vec::Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // Convert the byte array to a string and check if it equals "hello"
            let is_valid = input == b"hello".to_vec();

            // Emit the appropriate event based on the validation result
            if is_valid {
                Self::deposit_event(Event::ValidString { who });
            } else {
                Self::deposit_event(Event::InvalidString { who });
            }

            Ok(().into())
        }

        #[pallet::call_index(1)] 
        #[pallet::weight(T::WeightInfo::validate_string())]
        pub fn verify_rsa_signature(
            origin: OriginFor<T>,
            public_key: Vec<u8>,
            message: Vec<u8>,
            signature: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Parse the public key
            let public_key = RsaPublicKey::from_public_key_der(&public_key)
                .map_err(|_| Error::<T>::InvalidPublicKey)?;

            // Create the verifying key
            let verifying_key = VerifyingKey::<Sha256>::new(public_key);

            // Convert to signature type
            let signature = Signature::try_from(signature.as_slice())
                .map_err(|_| Error::<T>::InvalidSignature)?;

            // Verify the signature
            verifying_key
                .verify(&message, &signature)
                .map_err(|_| Error::<T>::SignatureVerificationFailed)?;

            // Emit success event
            Self::deposit_event(Event::SignatureVerified { who });

            Ok(())
        }
    }
}
