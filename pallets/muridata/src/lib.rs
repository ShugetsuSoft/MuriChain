#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	#[derive(Clone, Eq, PartialEq, Default, RuntimeDebug, Encode, Decode, TypeInfo)]
	pub struct Magu {
		dimension: u32,
		group_size: u32,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn maguw)]
	pub type Magus<T: Config> = StorageMap<Hasher = Blake2_128Concat, Key = T::AccountId, Value = Magu>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MaguCreated(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		MaguDoesNotExist,
		MaguAlreadyExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_magu(origin: OriginFor<T>, dimension: u32, group_size: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let magu = Magu {
				dimension,
				group_size,
			};
			Magus::<T>::insert(&who, magu);

			Self::deposit_event(Event::MaguCreated(who));

			Ok(())
		}	
		
	}
}
