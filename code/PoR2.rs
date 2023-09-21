use substrate_primitives::{crypto::Pair, ed25519};
use substrate_primitives::{Blake2Hasher, H256};
use codec::{Encode, Decode};
use rstd::vec::Vec;
use support::{decl_module, decl_storage, StorageMap, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ProofOfReserves<AccountId, Balance> {
    pub accounts: Vec<AccountId>,
    pub balances: Vec<Balance>,
}

decl_storage! {
    trait Store for Module<T: Trait> as ProofOfReserves {
        Proofs get(fn proofs): map hasher(blake2_256) H256 => ProofOfReserves<T::AccountId, T::Balance>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn create_proof(origin, accounts: Vec<T::AccountId>, balances: Vec<T::Balance>) -> Result {
            let sender = ensure_signed(origin)?;
            let proof = ProofOfReserves { accounts, balances };
            let proof_hash = Blake2Hasher::hash(&proof.encode());
            Proofs::<T>::insert(proof_hash, proof);
            Self::deposit_event(RawEvent::NewProof(sender, proof_hash));
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, Test};
    use substrate_primitives::{H256, crypto::UncheckedFrom};

    #[test]
    fn test_create_proof() {
        new_test_ext().execute_with(|| {
            let accounts = vec![
                AccountId::unchecked_from("Alice"),
                AccountId::unchecked_from("Bob"),
                AccountId::unchecked_from("Charlie"),
            ];
            let balances = vec![10, 20, 30];
            assert_ok!(Test::create_proof(Origin::signed(accounts[0]), accounts.clone(), balances.clone()));
            let proof_hash = Blake2Hasher::hash(&ProofOfReserves { accounts, balances }.encode());
            assert_eq!(Test::proofs(proof_hash), ProofOfReserves { accounts, balances });
        });
    }
}

