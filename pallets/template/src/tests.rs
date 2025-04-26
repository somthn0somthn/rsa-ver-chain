#![cfg(test)]

use crate::mock::*;
use crate::{Error, Pallet as Template};
use base64::prelude::*;
use frame::testing_prelude::*;
use frame_system::Pallet as System;
use rand::rngs::ThreadRng;
use rsa::pkcs8::DecodePrivateKey;
use rsa::signature::SignatureEncoding;
use rsa::{pkcs1v15::SigningKey, signature::RandomizedSigner};
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;

const TEST_PRIVATE_KEY_BASE64: &str =
    "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDSPWDk+tMHNeRH
mlF0E+WP4d6KbmLI0xHUW6+v5mdmsnC2mdsniuZyOEvnaXjtSXFLdjUStJB479XU
l7quaj+cENi+MkuIBnKK+NxDmJLFF3dL29dntOLS+PW3sCT56icMoqv40pbGoZZU
jmvzyVkHUUc2lP9LOKVcV77dHAA9OCh7wzb1b/ejXnlouKPALLisPtDPfTYMX2Zd
5jPVqjEJPLoLV+GaFpBI4KqURE6EE3GrYQyHGPrtjpLppf8g0Nx4Uni3l5HjLRYy
YcPOEc9Plff32WF1cDjow3ZePt1Oz9GWdwCcCXo4hfZzUB2t9czMiqKxemd8P/4I
Vf3b0uxnAgMBAAECggEAGsP8q3oxVpgqVWFUoGIOFkjLs7UE94tDmFUJ42uCW4U5
K04gJAMffogjRbVcU11HwrMsY5LAV900zGl6t0zIh15vsZsmmY00lPPE2wg4G0Z3
Bh3ZaRcSz4gphDa1JmoEiavGGBn7XxNgV6iAHtvR7UgQOGc0r0dpaf2zy2OOtjxO
SLJUjvL7UjHQEmkqONpYhe9xMZWcQ/2WnKO8zZDXzSP/S4BJdBynJHkqcIeqwrQ7
hIouk1Iw/XPZoq+upF2NqI4vb1ST8kpQFlvoC0+PFJ3vdggn7m4Xo9dn0uMIxBRk
C+6zSk6tDJQjlatAIDfD+AhtKua9GX9AjotkEVnxmQKBgQD8FoeTSvFbb7ZEfsdx
QYMKK1TuTjNbDRVMdHzoeNTisHdTLxODdHYthvvMJqpHBFnj99Nty87wYS/GnmVx
iGnkkk64pNY1ClSM/oROeAeBQcfIZEdjFH6+HWcVwcahJviuUwJgBFOp5HCxmsWc
1rMJ5Tk++4MOW6ltGoxL83bkfwKBgQDVgJkq1FgoF/JT3Z2o7c7vILWg38v8Ha9d
qs7i2TdDo5jCuVz2Eyb/azDjOVSlp/sF9/wLBo+LAwZSd7ttPTUni5E2alqJcr3c
eEWk21J5P+TrUvbs01ubx7k7rP8gTsWRb0ct+kdsHILcrzd3mzvSRDKAV8n39rYD
nHcf7YtkGQKBgDJnjtQ0t+3AeIFZmCddIgZ2g5TirgIWq5uN7yvMfUSWZEWNtCkk
sDyKMO6xDHsGs7KRawUZK1eFFzhvk3kP/ZxmZLcmvm38ksteWLYX6eg+buPyqRN2
laEN3/7JsB2o3TB+CIuKUfA2bRIWEE074LCp1LhzMdE9IrNeC49kLvGhAoGAQIJ8
xT4n7OVfaQu7SYGhVwuRFazJW8I0bJ7hqBVt51gVHjEX75pV3DMtMFrwQqWA6YrG
cyZmVOBzFpPq49TsrKg1FjYIaSNsTqNgl7FVOCGgL6J/zYsHUQiF5YweAksK0yjG
XxQSZI4w1DF1FwSmjJ/iinj8I5o94NV0AQeraHECgYEAgIegHmZajeDcvRotHN4x
+CVGUfDlouvQbmJ2uCqQ8BAL6xoQv786JqZ2R3TnMqehl8wsFV3uLqfwb3qi+aoI
A/kHbIz/lUx7G1WZFijEa97RCSFUWK6Sc2Y7UBXTa5OPwnqIb25GnDWAacFf/rSq
UDYhGgAwb+EKfvDP1GziH0k=";

const TEST_PUBLIC_KEY_BASE64: &str =
    "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0j1g5PrTBzXkR5pRdBPl
j+Heim5iyNMR1Fuvr+ZnZrJwtpnbJ4rmcjhL52l47UlxS3Y1ErSQeO/V1Je6rmo/
nBDYvjJLiAZyivjcQ5iSxRd3S9vXZ7Ti0vj1t7Ak+eonDKKr+NKWxqGWVI5r88lZ
B1FHNpT/SzilXFe+3RwAPTgoe8M29W/3o155aLijwCy4rD7Qz302DF9mXeYz1aox
CTy6C1fhmhaQSOCqlEROhBNxq2EMhxj67Y6S6aX/INDceFJ4t5eR4y0WMmHDzhHP
T5X399lhdXA46MN2Xj7dTs/RlncAnAl6OIX2c1AdrfXMzIqisXpnfD/+CFX929Ls
ZwIDAQAB";

#[test]
fn validate_string_works_with_valid_input() {
    new_test_ext().execute_with(|| {
        // Setup system for testing
        System::<Test>::set_block_number(1);

        // Arrange
        let account = 1;
        let valid_string = b"hello".to_vec();

        // Act
        assert_ok!(Template::<Test>::validate_string(
            RuntimeOrigin::signed(account),
            valid_string
        ));

        // Assert
        System::<Test>::assert_last_event(RuntimeEvent::Template(crate::Event::ValidString {
            who: account,
        }));
    });
}

#[test]
fn validate_string_works_with_invalid_input() {
    new_test_ext().execute_with(|| {
        // Setup system for testing
        System::<Test>::set_block_number(1);

        // Arrange
        let account = 1;
        let invalid_string = b"world".to_vec();

        // Act
        assert_ok!(Template::<Test>::validate_string(
            RuntimeOrigin::signed(account),
            invalid_string
        ));

        // Assert
        System::<Test>::assert_last_event(RuntimeEvent::Template(crate::Event::InvalidString {
            who: account,
        }));
    });
}

#[test]
fn verify_rsa_signature_works() {
    new_test_ext().execute_with(|| {
        // Setup system for testing
        System::<Test>::set_block_number(1);

        // Arrange
        let account = 1;
        let message = b"hello world".to_vec();

        // Decode the base64 keys
        let private_key_der = BASE64_STANDARD
            .decode(TEST_PRIVATE_KEY_BASE64.replace('\n', ""))
            .unwrap();
        let public_key_der = BASE64_STANDARD
            .decode(TEST_PUBLIC_KEY_BASE64.replace('\n', ""))
            .unwrap();

        // Parse the private key for signing
        let private_key = RsaPrivateKey::from_pkcs8_der(&private_key_der).unwrap();

        // Create a signing key and sign the message
        let signing_key = SigningKey::<Sha256>::new(private_key);
        let mut rng = rand::thread_rng();
        let signature = signing_key.sign_with_rng(&mut rng, &message);

        // Convert the signature to bytes
        let signature_bytes = signature.to_bytes().to_vec();

        // Act - Call the verify_rsa_signature function
        assert_ok!(Template::<Test>::verify_rsa_signature(
            RuntimeOrigin::signed(account),
            public_key_der,
            message,
            signature_bytes
        ));

        // Assert - Check that the event was emitted
        System::<Test>::assert_last_event(RuntimeEvent::Template(
            crate::Event::SignatureVerified { who: account },
        ));
    });
}

#[test]
fn verify_rsa_signature_fails_with_wrong_message() {
    new_test_ext().execute_with(|| {
        // Setup system for testing
        System::<Test>::set_block_number(1);

        // Arrange
        let account = 1;
        let original_message = b"hello world".to_vec();
        let tampered_message = b"hello world!".to_vec(); // Different message

        // Decode the base64 keys
        let private_key_der = BASE64_STANDARD
            .decode(TEST_PRIVATE_KEY_BASE64.replace('\n', ""))
            .unwrap();
        let public_key_der = BASE64_STANDARD
            .decode(TEST_PUBLIC_KEY_BASE64.replace('\n', ""))
            .unwrap();

        // Parse the private key for signing
        let private_key = RsaPrivateKey::from_pkcs8_der(&private_key_der).unwrap();

        // Create a signing key and sign the original message
        let signing_key = SigningKey::<Sha256>::new(private_key);
        let mut rng = rand::thread_rng();
        let signature = signing_key.sign_with_rng(&mut rng, &original_message);

        // Convert the signature to bytes
        let signature_bytes = signature.to_bytes().to_vec();

        // Act - Try to verify with the tampered message
        let result = Template::<Test>::verify_rsa_signature(
            RuntimeOrigin::signed(account),
            public_key_der,
            tampered_message,
            signature_bytes,
        );

        // Assert - Verification should fail
        assert_noop!(result, Error::<Test>::SignatureVerificationFailed);
    });
}
