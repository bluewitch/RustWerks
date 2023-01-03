use frame_support::{decl_module, decl_storage, decl_event, StorageMap, ensure};
use frame_system::{self, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::{Hash};

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ProofOfReserves {
        // Total amount of reserves held by the system
        TotalReserves get(fn total_reserves): T::Balance;
        // Mapping of user accounts to their reserves
        UserReserves get(fn user_reserves): map hasher(blake2_128_concat) T::AccountId => T::Balance;
    }
}

decl_event! {
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId, Balance = <T as balances::Trait>::Balance {
        // Event triggered when the total reserves are updated
        TotalReservesUpdated(Balance),
        // Event triggered when a user's reserves are updated
        UserReservesUpdated(AccountId, Balance),
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // Function to update the total reserves of the system
        pub fn update_total_reserves(origin, new_reserves: T::Balance) -> frame_support::dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;
            // Ensure that the new reserves are greater than the current reserves
            ensure!(new_reserves > TotalReserves::get(), "New reserves must be greater than current reserves");
            // Update the total reserves
            TotalReserves::put(new_reserves);
            // Emit an event to notify of the update
            Self::deposit_event(Event::TotalReservesUpdated(new_reserves));
            Ok(())
        }

        // Function to update a user's reserves
        pub fn update_user_reserves(origin, user: T::AccountId, new_reserves: T::Balance) -> frame_support::dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;
            // Ensure that the new reserves are greater than the current reserves
            ensure!(new_reserves > UserReserves::get(user), "New reserves must be greater than current reserves");
            // Update the user's reserves
            UserReserves::insert(user, new_reserves);
            // Emit an event to notify of the update
            Self::deposit_event(Event::UserReservesUpdated(user, new_reserves));
            Ok(())
        }
    }
}
