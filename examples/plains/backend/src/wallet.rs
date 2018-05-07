// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Wallet.

use exonum::crypto::{Hash, PublicKey};

encoding_struct! {
    struct Detail {
	    name_of_type: u8,
	    count: u64,
    }
}

encoding_struct! {
    /// Wallet information stored in the database.
    struct Wallet {
        pub_key:            &PublicKey,
        name:               &str,
        balance:            u64,
        details:            Vec<Detail>, 
        develop:            Vec<Detail>,
        history_len:        u64,
        history_hash:       &Hash,
    }
}

impl Wallet {
    /// Returns a copy of this wallet with updated balance.
    pub fn set_balance(self, balance: u64, history_hash: &Hash) -> Self {
        Self::new(
            self.pub_key(),
            self.name(),
            balance,
            self.details(),
            self.develop(),
            self.history_len() + 1,
            history_hash,
        )
    }
    
    pub fn set_details(self, details: Vec<Detail>, history_hash: &Hash) -> Self {
		Self::new(
		    self.pub_key(),
            self.name(),
            self.balance(),
            details,
            self.develop(),
            self.history_len() + 1,
            history_hash,
        )
    }
}

impl Detail {
	pub fn set_count_detail(self, count: u64) -> Self {
    	Self::new(
		    self.name_of_type(),
		    count,
		)
    }
} 
