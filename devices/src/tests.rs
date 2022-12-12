use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_onboard_device() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Devices::onboard_device(Origin::root(), 1 , b"Device1".to_vec()));
		// Read pallet storage and assert an expected result.
		assert_eq!(Devices::get_device_count(), 1u32);
	});
}

#[test]
fn it_fails_for_onboard_device() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Devices::onboard_device(Origin::root(), 1 , b"Device1".to_vec()));
		assert_eq!(Devices::get_device_count(), 1u32);

		assert_noop!(Devices::onboard_device(Origin::root(), 1 , b"Device1".to_vec()), Error::<Test>::DeviceAlreadyExists);
	});
}

#[test]
fn it_works_for_remove_device() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Devices::onboard_device(Origin::root(), 1 , b"Device1".to_vec()));
		// Read pallet storage and assert an expected result.
		assert_ok!(Devices::remove_device(Origin::root(), 1));
	});
}

#[test]
fn it_fails_for_remove_device() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Devices::onboard_device(Origin::root(), 1 , b"Device1".to_vec()));
		assert_eq!(Devices::get_device_count(), 1u32);

		assert_noop!(Devices::remove_device(Origin::root(), 2 ), Error::<Test>::DeviceDoesNotExist);
	});
}

#[test]
fn it_adds_new_record() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Devices::onboard_device(Origin::root(), 1 , b"Device1".to_vec()));
		assert_eq!(Devices::get_device_count(), 1u32);

		assert_ok!(Devices::record(Origin::signed(1),b"1.0".to_vec(),b"1.0".to_vec(),b"1.0".to_vec(),b"1.0".to_vec()));
		assert_eq!(Devices::get_device_buffer_index(1),(1u64,0u64));
	});
}

#[test]
fn it_adds_new_record_fails() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Devices::onboard_device(Origin::root(), 1 , b"Device1".to_vec()));
		assert_eq!(Devices::get_device_count(), 1u32);

		assert_noop!(Devices::record(Origin::signed(2),b"1.0".to_vec(),b"1.0".to_vec(),b"1.0".to_vec(),b"1.0".to_vec()), Error::<Test>::DeviceDoesNotExist);
	});
}
