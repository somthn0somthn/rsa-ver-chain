#![cfg(test)]

use crate::mock::*;
use crate::Pallet as Template;
use frame::testing_prelude::*;
use frame_system::Pallet as System;
use rsa::{RsaPrivateKey, RsaPublicKey};
//use spki::{DecodePrivateKey, DecodePublicKey};

const TEST_PRIVATE_KEY_BASE64: &str =
    "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC8o+pwitCfTxre
EO2qKYC2Brl9wQasR9K7QXSc9EPQNh6Wlb2rSuoYtTdmb/3rFCXH0WcEdfWhTdGt
hzWkmdGIMTjk87AyHBj9Y/XacfB2Bx4vM/iM1U8s9A3HnYm0hCGMXxcSOvKJ2rjK
dQVAUcFjFr3nzbElw+OSXlfhtZmUsz/tfaijOTlHg5F8tLZkz8l19N9D7y3dW6TE
OJxg24diiuzmy5DlPu9iC6Z7gX8W34tNxcHfDBfcBb2IhULJJsmRWDG1zHDDoa6H
CCSZju/KNX+61AXf+FIMn/Hlzd/2M2PLooiQBAf0kjnCMrAYhsnBy/pzB//Idiew
6KAXaus1AgMBAAECggEAW3jUfrwhS47NRW9J/6BeZGXAXoVSGgo92kV0KYOcuoLz
tCtqLHyIJXECKy8VHn/HjiwxWw1kDVKx4QZWcbmQzAyqPQMZpkLPr1dgaQulAWaU
MWlvFcN9wBub7vaqRplntyyJAHMLFuOzhaDW6gh60PS/I38vblOdr2UGElG/5kVq
Il96qiBJjU57LXQIH/KB6dGQYsYoxz5inLrpBCgOZE1H2CUzfuLZt1UGi2C9/de4
Llw0HdWfnwh41Oqn/o27bDI6tipGzQfzvcHkP5Sm4gUP/SazZ0s/YlrnCodg+Imt
GmEBhmLznO1WERDxVVU18nuPVBlbR0FftF5ZRImsZQKBgQD7g6iMZdB/QJwaC/cL
7BHdyiXKp50YHE0SufTjuT617LcZ4IPHaS7ZGg5Ou6PyD51PFTXoAUG+aM8eA2h+
sbyZBqLYg3eeaXoDl4GtoKkIBL+XyvH7uDLkdXYbhxvZ+1xSs92DeqWNF+M8Kujv
38UGnfFBlhB3XAEx2STbZdTBIwKBgQDAATFhFiV056PV6trEKUaoddkyXYv3435R
w2UZDERjGF5dKsvWgHdre/NWsvEfg+4YDO3z/6BNqtxmrzDp8XV3uuRWcksM7BWy
iuGQ2/iZR6Xq4jkd2fby26h+MIBCenPGolmM6Acyd4/EWjnvN09/t4SkIXVd7u3B
P+pj1WsjxwKBgQDmDL6V7GqQemBN0rPcy8hrvQJkInumGfzFLCG6SS366NFxcdIX
CzY2jYaYrTh/p5iTRStRUhT4PJV1NljVyWxf65n3wAghdQiNUH7keRcipnU4OgCr
5Qdnv6iP9TycXTVM0AfEwuG59Cd8WSNGhL6KFt0cyE23XwgYuCzmIHZBUQKBgEQx
6ETwZbtg7qry8GKNun/o44zk9urDjUEjIkxENCHyLMnwglecZ03wx1v60HN+G1nF
ddqc52XOktuRy9C7pxowchtWGtvrGkXA2B7oUiK29PI5CUpV+UUGyZCQT23NKkPZ
s+69FplRJ+n4le0l+wGEwAK/s3z6eM2Tkv+EdLpPAoGAG+CWMNu/qrSdSlBkOryw
19TIIk4Bb8wcaudcCLbMU1qphnJetKErdb6wIXOWEcJQUqkqFfI0Of094WR57gAF
rGwZ7M5CMcJeq+1AaknC7bwJkZv+EPIb83bNCKfZGtQz505zxqJwSSuwrbGZsJd9
puDoN7v6WDcjweQyWM4DwUI=";

const TEST_PUBLIC_KEY_BASE64: &str =
    "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvKPqcIrQn08a3hDtqimA
tga5fcEGrEfSu0F0nPRD0DYelpW9q0rqGLU3Zm/96xQlx9FnBHX1oU3RrYc1pJnR
iDE45POwMhwY/WP12nHwdgceLzP4jNVPLPQNx52JtIQhjF8XEjryidq4ynUFQFHB
Yxa9582xJcPjkl5X4bWZlLM/7X2oozk5R4ORfLS2ZM/JdfTfQ+8t3VukxDicYNuH
Yors5suQ5T7vYgume4F/Ft+LTcXB3wwX3AW9iIVCySbJkVgxtcxww6GuhwgkmY7v
yjV/utQF3/hSDJ/x5c3f9jNjy6KIkAQH9JI5wjKwGIbJwcv6cwf/yHYnsOigF2rr
NQIDAQAB";

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
