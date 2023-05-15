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

#[test]
fn delete_campaign_works() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = Referral::create_campaign(RuntimeOrigin::signed(1), 1, metadata);

		assert_ok!(Referral::delete_campaign(RuntimeOrigin::signed(1), 1));

		assert_eq!(Campaigns::<Test>::get(1), None);
	})
}

#[test]
fn delete_campaign_failed_when_campaign_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Referral::delete_campaign(RuntimeOrigin::signed(1), 1),
			Error::<Test>::CampaignNotExists,
		);
	});
}

#[test]
fn delete_campaign_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = Referral::create_campaign(RuntimeOrigin::signed(1), 1, metadata);

		assert_noop!(
			Referral::delete_campaign(RuntimeOrigin::signed(2), 1),
			Error::<Test>::NotCampaignOwner,
		);
	});
}

#[test]
fn create_referral_works() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		let referral_code = BoundedVec::try_from(vec![68, 69]).unwrap(); // code: hi
		let campaign_owner = 1;
		let referral_account = 2;

		let _ = Referral::create_campaign(RuntimeOrigin::signed(campaign_owner), 1, metadata);

		assert_ok!(Referral::register_referral(
			RuntimeOrigin::signed(referral_account),
			1,
			referral_code.clone()
		));

		assert_eq!(Referrals::<Test>::get(1, &referral_code).unwrap(), 2);
	});
}

#[test]
fn create_referral_failed_when_campaign_not_exists() {
	new_test_ext().execute_with(|| {
		let referral_code = BoundedVec::try_from(vec![68, 69]).unwrap(); // code: hi
		let referral_account = 2;

		assert_noop!(
			Referral::register_referral(RuntimeOrigin::signed(referral_account), 1, referral_code),
			Error::<Test>::CampaignNotExists,
		);
	});
}

#[test]
fn create_referral_failed_when_referral_code_already_exists() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		let referral_code = BoundedVec::try_from(vec![68, 69]).unwrap(); // code: hi
		let campaign_owner = 1;
		let referral_account = 2;
		let campaign_id = 1;

		let _ =
			Referral::create_campaign(RuntimeOrigin::signed(campaign_owner), campaign_id, metadata);

		let _ = Referral::register_referral(
			RuntimeOrigin::signed(referral_account),
			campaign_id,
			referral_code.clone(),
		);

		assert_noop!(
			Referral::register_referral(
				RuntimeOrigin::signed(referral_account),
				campaign_id,
				referral_code
			),
			Error::<Test>::ReferralAlreadyExists,
		);
	});
}

#[test]
fn delete_referral_works() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		let referral_code = BoundedVec::try_from(vec![68, 69]).unwrap(); // code: hi
		let campaign_owner = 1;
		let referral_account = 2;
		let campaign_id = 1;

		let _ =
			Referral::create_campaign(RuntimeOrigin::signed(campaign_owner), campaign_id, metadata);

		let _ = Referral::register_referral(
			RuntimeOrigin::signed(referral_account),
			campaign_id,
			referral_code.clone(),
		);

		assert_ok!(Referral::delete_referral(
			RuntimeOrigin::signed(referral_account),
			campaign_id,
			referral_code.clone()
		));

		assert_eq!(Referrals::<Test>::get(1, &referral_code), None);
	});
}

#[test]
fn delete_referral_failed_when_campaign_not_exists() {
	new_test_ext().execute_with(|| {
		let referral_code = BoundedVec::try_from(vec![68, 69]).unwrap(); // code: hi
		let referral_account = 2;

		assert_noop!(
			Referral::delete_referral(RuntimeOrigin::signed(referral_account), 1, referral_code),
			Error::<Test>::CampaignNotExists,
		);
	});
}

#[test]
fn delete_referral_failed_when_referral_not_exists() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		let referral_code = BoundedVec::try_from(vec![68, 69]).unwrap(); // code: hi
		let campaign_owner = 1;
		let referral_account = 2;
		let campaign_id = 1;

		let _ =
			Referral::create_campaign(RuntimeOrigin::signed(campaign_owner), campaign_id, metadata);

		assert_noop!(
			Referral::delete_referral(
				RuntimeOrigin::signed(referral_account),
				campaign_id,
				referral_code
			),
			Error::<Test>::ReferralNotExist,
		);
	});
}

#[test]
fn delete_referral_failed_when_not_referral_owner() {
	new_test_ext().execute_with(|| {
		let metadata = BoundedVec::try_from(vec![0, 1]).unwrap();
		let referral_code = BoundedVec::try_from(vec![68, 69]).unwrap(); // code: hi
		let campaign_owner = 1;
		let referral_account = 2;
		let invalid_referral_account = 3;
		let campaign_id = 1;

		let _ =
			Referral::create_campaign(RuntimeOrigin::signed(campaign_owner), campaign_id, metadata);

		let _ = Referral::register_referral(
			RuntimeOrigin::signed(referral_account),
			campaign_id,
			referral_code.clone(),
		);

		assert_noop!(
			Referral::delete_referral(
				RuntimeOrigin::signed(invalid_referral_account),
				campaign_id,
				referral_code
			),
			Error::<Test>::NotReferralOwner,
		);
	});
}
