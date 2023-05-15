#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The maximum length of a metadata string.
		#[pallet::constant]
		type StringLimit: Get<u32>;
		/// The maximu length of the referral code.
		#[pallet::constant]
		type MaxCodeLength: Get<u32>;
		/// The runtime event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(
		Clone, Encode, Decode, PartialEqNoBound, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct Campaign<T: Config> {
		pub owner: T::AccountId,
		pub metadata: BoundedVec<u8, T::StringLimit>,
	}

	#[pallet::storage]
	pub type Campaigns<T: Config> = StorageMap<_, Blake2_128Concat, u32, Campaign<T>>;

	#[pallet::storage]
	pub type Referrals<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxCodeLength>,
		T::AccountId,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CampaignCreated(T::AccountId, u32),
		CampaignUpdated(T::AccountId, u32),
		CampaignDeleted(T::AccountId, u32),
		ReferralRegistered(T::AccountId, u32, BoundedVec<u8, T::MaxCodeLength>),
		ReferralDeleted(T::AccountId, u32, BoundedVec<u8, T::MaxCodeLength>),
	}

	#[pallet::error]
	pub enum Error<T> {
		CampaignAlreadyExists,
		CampaignNotExists,
		NotCampaignOwner,
		ReferralAlreadyExists,
		NotReferralOwner,
		ReferralNotExist,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_campaign(
			origin: OriginFor<T>,
			id: u32,
			metadata: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(!Campaigns::<T>::contains_key(id), Error::<T>::CampaignAlreadyExists);

			let campaign = Campaign { owner: sender.clone(), metadata };
			Campaigns::<T>::insert(id, campaign);

			Self::deposit_event(Event::CampaignCreated(sender, id));
			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn update_campaign(
			origin: OriginFor<T>,
			id: u32,
			metadata: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			Campaigns::<T>::try_mutate(id, |maybe_campaign| {
				let campaign = maybe_campaign.as_mut().ok_or(Error::<T>::CampaignNotExists)?;
				ensure!(campaign.owner == sender, Error::<T>::NotCampaignOwner);
				campaign.metadata = metadata;

				Self::deposit_event(Event::CampaignUpdated(sender, id));
				Ok(().into())
			})
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn delete_campaign(origin: OriginFor<T>, id: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			Campaigns::<T>::try_mutate_exists(id, |maybe_campaign| {
				let campaign = maybe_campaign.as_ref().ok_or(Error::<T>::CampaignNotExists)?;
				ensure!(campaign.owner == sender, Error::<T>::NotCampaignOwner);
				*maybe_campaign = None;

				Self::deposit_event(Event::CampaignDeleted(sender, id));
				Ok(().into())
			})
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn register_referral(
			origin: OriginFor<T>,
			id: u32,
			code: BoundedVec<u8, T::MaxCodeLength>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(Campaigns::<T>::contains_key(id), Error::<T>::CampaignNotExists);
			ensure!(!Referrals::<T>::contains_key(id, &code), Error::<T>::ReferralAlreadyExists);

			Referrals::<T>::insert(id, &code, &sender);

			Self::deposit_event(Event::ReferralRegistered(sender, id, code));
			Ok(().into())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn delete_referral(
			origin: OriginFor<T>,
			id: u32,
			code: BoundedVec<u8, T::MaxCodeLength>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(Campaigns::<T>::contains_key(id), Error::<T>::CampaignNotExists);

			match Referrals::<T>::get(id, &code) {
				Some(account) => ensure!(account == sender, Error::<T>::NotReferralOwner),
				None => return Err(Error::<T>::ReferralNotExist.into()),
			};

			Referrals::<T>::remove(id, &code);

			Self::deposit_event(Event::ReferralDeleted(sender, id, code));
			Ok(().into())
		}
	}
}
