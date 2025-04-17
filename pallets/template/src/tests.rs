#![cfg(test)]

use crate::mock::*;
use crate::Pallet as Template;
use frame::testing_prelude::*;
use frame_system::Pallet as System;

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
        System::<Test>::assert_has_event(RuntimeEvent::Template(crate::Event::ValidString {
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
        System::<Test>::assert_has_event(RuntimeEvent::Template(crate::Event::InvalidString {
            who: account,
        }));
    });
}
