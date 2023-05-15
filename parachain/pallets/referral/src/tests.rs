use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_campaign_works() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_ok!(Referral::create_campaign(RuntimeOrigin::signed(1), 1, metadata.clone()));

		assert_eq!(Campaigns::<Test>::get(1).unwrap(), Campaign { owner: 1, metadata });
	});
}

#[test]
fn create_campaign_failed_when_id_already_exists() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_ok!(Referral::create_campaign(RuntimeOrigin::signed(1), 1, metadata.clone()));

		assert_noop!(
			Referral::create_campaign(RuntimeOrigin::signed(1), 1, metadata.clone()),
			Error::<Test>::CampaignAlreadyExists,
		);
	});
}

#[test]
fn update_campaign_works() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = Referral::create_campaign(RuntimeOrigin::signed(1), 1, metadata);

		let new_metadata = BoundedVec::try_from(vec![0, 1, 2]).unwrap();
		assert_ok!(Referral::update_campaign(RuntimeOrigin::signed(1), 1, new_metadata.clone()));

		assert_eq!(
			Campaigns::<Test>::get(1).unwrap(),
			Campaign { owner: 1, metadata: new_metadata }
		)
	})
}

#[test]
fn update_campaign_failed_when_campaign_not_exists() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_noop!(
			Referral::update_campaign(RuntimeOrigin::signed(1), 1, metadata.clone()),
			Error::<Test>::CampaignNotExists,
		);
	});
}

#[test]
fn update_campaign_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = Referral::create_campaign(RuntimeOrigin::signed(1), 1, metadata);

		let new_metadata = BoundedVec::try_from(vec![0, 1, 2]).unwrap();

		assert_noop!(
			Referral::update_campaign(RuntimeOrigin::signed(2), 1, new_metadata),
			Error::<Test>::NotCampaignOwner,
		);
	});
}
