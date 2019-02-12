use crate::{
	contract::ContractDecl,
	state::ContractState,
	msg::Message,
};
use pdsl_core::{
	env::{
		Env,
		ContractEnv,
		srml::Address,
		srml::Balance,
	},
	storage::{self, alloc::Initialize},
};

state! {
	/// A simple implementation of a rudimentary Erc20 token contract.
	struct State {
		/// The balance for an address.
		balances: storage::HashMap<Address, Balance>,
		/// The total supply.
		total: storage::Value<Balance>,
	}
}

impl Initialize for State {
	type Args = Balance;

	fn initialize(&mut self, total_supply: Self::Args) {
		self.balances[ContractEnv::caller()] = total_supply;
		self.total.set(total_supply);
	}
}

messages! {
	/// Returns the total supply.
	TotalSupply() -> Balance;
	/// Returns the balance of the given address.
	BalanceOf(owner: Address) -> Balance;
	/// Transfers balance from the caller to the given address.
	///
	/// Returns `true` if the transfer was successful.
	Transfer(to: Address, amount: Balance) -> bool;
}

fn instantiate() {
	Contract::new("Erc20Token")
		.using_state::<State>()
		.on_msg::<TotalSupply>(|env, _| {
			*env.state.total.get()
		})
		.on_msg::<BalanceOf>(|env, owner| {
			env.state.balances[owner]
		})
		.on_msg_mut::<Transfer>(|env, (to, amount)| {
			if amount == Address::from(0x0) {
				return false;
			}
			let from = env.caller();
			let balance_from = env.state.balances[from];
			let balance_to = env.state.balances[to];

			if balance_from >= amount {
				env.state.balances[from] = existing_from - amount;
				env.state.balances[to] = existing_to + amount;
				return true
			}

			false
		});
}