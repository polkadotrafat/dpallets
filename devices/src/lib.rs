#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::Time};
	use frame_system::pallet_prelude::*;
	use scale_info::{
		TypeInfo,
	};
	use sp_runtime::ArithmeticError;
	use sp_std::vec::Vec;


	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	#[pallet::type_value]
    pub fn MaximumDataSize<T: Config>() -> u32
    {
        512u32
    }


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Time: Time;
	}

	#[pallet::storage]
	#[pallet::getter(fn get_device_count)]
	pub(super) type DeviceCount<T:Config> = StorageValue<
		_,
		u32,
		ValueQuery,
	>;

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Device<T:Config> {
		pub hash: Vec<u8>,
		pub block: T::BlockNumber,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct EnergyData<T:Config> {
		pub voltage: Vec<u8>,
		pub current: Vec<u8>,
		pub energy: Vec<u8>,
		pub energyacum: Vec<u8>,
		pub block: T::BlockNumber,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum DeviceStatus {
		Up,
		Down,
		DoesNotExist,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_device)]
	pub(super) type Devices<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Device<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_energydata_item)]
	pub(super) type EnergyDataItem<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AccountId,u64),
		EnergyData<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_device_data_index)]
	pub(super) type DeviceDataIndex<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		u64,
		OptionQuery,
	>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewDeviceAdded(T::AccountId),
		DeviceRemoved(T::AccountId),
		NewRecord(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Device already exists
		DeviceAlreadyExists,
		/// Device doesn't exist
		DeviceDoesNotExist,
		/// UnAuthorized device
		UnauthorizedDevice,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(3) + T::DbWeight::get().reads(2))]
		pub fn onboard_device(origin: OriginFor<T>, address: T::AccountId, info: Vec<u8>) -> DispatchResult {
			
			ensure_root(origin)?;
			ensure!(!Devices::<T>::contains_key(&address.clone()), Error::<T>::DeviceAlreadyExists);

			let device = Device::<T> {
				block: <frame_system::Pallet<T>>::block_number(),
				hash: info.clone(),
			};

			let count = Self::get_device_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			Devices::<T>::insert(address.clone(),device);
			DeviceCount::<T>::put(count);

			// Emit an event.
			Self::deposit_event(Event::NewDeviceAdded(address));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2) + T::DbWeight::get().reads(2))]
		pub fn remove_device(origin: OriginFor<T>, address: T::AccountId) -> DispatchResult {
			
			ensure_root(origin)?;
			ensure!(Devices::<T>::contains_key(&address.clone()), Error::<T>::DeviceDoesNotExist);

			let count = Self::get_device_count().checked_sub(1).ok_or(ArithmeticError::Overflow)?;

			Devices::<T>::remove(address.clone());
			DeviceCount::<T>::put(count);

			// Emit an event.
			Self::deposit_event(Event::DeviceRemoved(address));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(3) + T::DbWeight::get().reads(2))]
		pub fn record(origin: OriginFor<T>, voltage: Vec<u8>, current: Vec<u8>, energy: Vec<u8>, energyacum: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Devices::<T>::contains_key(who.clone()), Error::<T>::DeviceDoesNotExist);
			let count = Self::get_device_data_index(who.clone()).unwrap().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let data = EnergyData::<T> {
				voltage: voltage.clone(),
				current: current.clone(),
				energy: energy.clone(),
				energyacum: energyacum.clone(),
				block: <frame_system::Pallet<T>>::block_number(),
			};

			EnergyDataItem::<T>::insert((who.clone(),count.clone()),&data);
			DeviceDataIndex::<T>::insert(who.clone(),count);

			Self::deposit_event(Event::NewRecord(who));
			Ok(())
		}
	}
}
