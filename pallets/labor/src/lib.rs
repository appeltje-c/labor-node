#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use frame_support::inherent::Vec;
    use scale_info::TypeInfo;
    use codec::{Decode, Encode};

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    #[derive(Debug, Encode, Decode, Clone, TypeInfo, PartialEq)]
    pub struct StructValue {
        number: u8,
        string: Vec<u8>,
    }

    #[derive(Debug, Encode, Decode, Clone, TypeInfo, PartialEq)]
    pub enum Enum {
        Value1,
        Value2,
        Value3,
    }

    #[pallet::storage]
    #[pallet::getter(fn some_number)]
    pub type SomeNumber<T> = StorageValue<_, u32>;

    #[pallet::storage]
    #[pallet::getter(fn some_string)]
    pub type SomeString<T> = StorageValue<_, Vec<u8>>;

    #[pallet::storage]
    #[pallet::getter(fn some_struct)]
    pub type SomeStruct<T> = StorageValue<_, StructValue>;

    #[pallet::storage]
    #[pallet::getter(fn some_enum)]
    pub type SomeEnum<T> = StorageValue<_, Enum>;

    #[pallet::storage]
    #[pallet::getter(fn some_map)]
    pub type SomeMap<T> = StorageMap<_, Blake2_256, u32, StructValue>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomeNumberStored(u32, T::AccountId),
        SomeStringStored(Vec<u8>, T::AccountId),
        SomeStructStored(StructValue, T::AccountId),
        SomeEnumStored(Enum, T::AccountId),
        SomeMapStored(u32, StructValue, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn do_some_number(origin: OriginFor<T>, something: u32) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/v3/runtime/origins
            let who = ensure_signed(origin)?;

            // Update storage.
            <SomeNumber<T>>::put(something);

            // Emit an event.
            Self::deposit_event(Event::SomeNumberStored(something, who));

            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn do_some_string(origin: OriginFor<T>, something: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <SomeString<T>>::put(&something);

            Self::deposit_event(Event::SomeStringStored(something, who));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn do_some_struct(origin: OriginFor<T>, something: StructValue) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <SomeStruct<T>>::put(&something);

            Self::deposit_event(Event::SomeStructStored(something, who));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn do_some_enum(origin: OriginFor<T>, something: Enum) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <SomeEnum<T>>::put(&something);

            Self::deposit_event(Event::SomeEnumStored(something, who));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn do_some_map(origin: OriginFor<T>, key: u32, something: StructValue) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <SomeMap<T>>::insert(key, &something);

            Self::deposit_event(Event::SomeMapStored(key, something, who));

            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match <SomeNumber<T>>::get() {
                // Return an error if the value has not been set.
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    <SomeNumber<T>>::put(new);
                    Ok(())
                }
            }
        }
    }
}
