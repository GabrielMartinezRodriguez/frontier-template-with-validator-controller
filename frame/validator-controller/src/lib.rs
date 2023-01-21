#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use pallet_session::SessionManager;
use sp_runtime::traits::Convert;
use sp_std::prelude::*;

#[frame_support::pallet]

pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn validators)]
	pub type Validators<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub initial_validators: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				initial_validators: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize_validators(&self.initial_validators);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn add_validator(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			<Validators<T>>::mutate(|validators| {
				if !validators.contains(&validator) {
					validators.push(validator);
				}
			});
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn remove_validator(origin: OriginFor<T>, validator: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			<Validators<T>>::mutate(|validators| {
				if let Some(index) = validators.iter().position(|v| v == &validator) {
					validators.remove(index);
				}
			});
			Ok(())
		}
	}
}

impl<T: Config> SessionManager<T::AccountId> for Pallet<T> {
	fn new_session(_new_index: u32) -> Option<Vec<T::AccountId>> {
		Some(Self::validators())
	}

	fn end_session(_end_index: u32) {}

	fn start_session(_start_index: u32) {}
}

impl<T: Config> Pallet<T> {
	fn initialize_validators(validators: &[T::AccountId]) {
		assert!(
			<Validators<T>>::get().is_empty(),
			"Validators are already initialized!"
		);

		<Validators<T>>::put(validators);
	}
}

pub struct ValidatorOf<T>(sp_std::marker::PhantomData<T>);

impl<T: pallet_session::Config> Convert<T::ValidatorId, Option<T::ValidatorId>> for ValidatorOf<T> {
	fn convert(account: T::ValidatorId) -> Option<T::ValidatorId> {
		Some(account)
	}
}
