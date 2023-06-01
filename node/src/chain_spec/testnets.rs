use std::collections::HashMap;

use da_primitives::currency::AVL;
use da_runtime::{AccountId, Balance};

use crate::chain_spec::{AuthorityKeys, GenesisConfig};

pub mod kate;

const MIN_VALIDATOR_BOND: Balance = 10 * AVL;
const MIN_NOMINATOR_BOND: Balance = 1 * AVL;

fn make_genesis(
	sudo: AccountId,
	authorities: Vec<AuthorityKeys>,
	council: Vec<AccountId>,
	tech_committee_members: Vec<AccountId>,
	endowed_accounts: HashMap<AccountId, Balance>,
) -> GenesisConfig {
	crate::chain_spec::make_genesis(
		sudo,
		authorities,
		council,
		tech_committee_members,
		endowed_accounts,
		MIN_VALIDATOR_BOND,
		MIN_NOMINATOR_BOND,
	)
}
