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
		/// The runtime event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(Clone, Encode, Decode, PartialEqNoBound, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Campaign<T: Config> {
		pub owner: T::AccountId,
		pub metadata: BoundedVec<u8, T::StringLimit>,
	}

	#[pallet::storage]
	pub type Campaigns<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32,
		Campaign<T>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CampaignCreated(T::AccountId, u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		CampaignAlreadyExists,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_campaign(origin: OriginFor<T>, id: u32, metadata: BoundedVec<u8, T::StringLimit>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(!Campaigns::<T>::contains_key(id), Error::<T>::CampaignAlreadyExists);

			let campaign = Campaign {
				owner: sender.clone(),
				metadata,
			};
			Campaigns::<T>::insert(id, campaign);

			Self::deposit_event(Event::CampaignCreated(sender, id));

			Ok(().into())
		}
	}
}
