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

use exonum::crypto::{Hash, PublicKey};
use exonum::storage::{Fork, ProofListIndex, ProofMapIndex, Snapshot};

use INITIAL_BALANCE;
///use INITIAL_DETAILS;
///use INITIAL_DEVELOP;
use wallet::Wallet;
use wallet::Detail;

/// Database schema for the cryptocurrency.
#[derive(Debug)]
pub struct CurrencySchema<T> {
    view: T,
}

impl<T> AsMut<T> for CurrencySchema<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.view
    }
}

impl<T> CurrencySchema<T>
where
    T: AsRef<Snapshot>,
{
    /// Constructs schema from the database view.
    pub fn new(view: T) -> Self {
        CurrencySchema { view }
    }

    /// Returns `MerklePatriciaTable` with wallets.
    pub fn wallets(&self) -> ProofMapIndex<&T, PublicKey, Wallet> {
        ProofMapIndex::new("cryptocurrency.wallets", &self.view)
    }

    /// Returns history of the wallet with the given public key.
    pub fn wallet_history(&self, public_key: &PublicKey) -> ProofListIndex<&T, Hash> {
        ProofListIndex::new_in_family("cryptocurrency.wallet_history", public_key, &self.view)
    }

    /// Returns wallet for the given public key.
    pub fn wallet(&self, pub_key: &PublicKey) -> Option<Wallet> {
        self.wallets().get(pub_key)
    }

    /// Returns database state hash.
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.wallets().merkle_root()]
    }
}

/// Implementation of mutable methods.
impl<'a> CurrencySchema<&'a mut Fork> {
    /// Returns mutable `MerklePatriciaTable` with wallets.
    pub fn wallets_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, Wallet> {
        ProofMapIndex::new("cryptocurrency.wallets", &mut self.view)
    }

    /// Returns history for the wallet by the given public key.
    pub fn wallet_history_mut(
        &mut self,
        public_key: &PublicKey,
    ) -> ProofListIndex<&mut Fork, Hash> {
        ProofListIndex::new_in_family("cryptocurrency.wallet_history", public_key, &mut self.view)
    }

    /// Increase balance of the wallet and append new record to its history.
    ///
    /// Panics if there is no wallet with given public key.
    pub fn increase_wallet_balance(&mut self, wallet: Wallet, amount: u64, transaction: &Hash) {
        let wallet = {
            let mut history = self.wallet_history_mut(wallet.pub_key());
            history.push(*transaction);
            let history_hash = history.merkle_root();
            let balance = wallet.balance();
            wallet.set_balance(balance + amount, &history_hash)
        };
        self.wallets_mut().put(wallet.pub_key(), wallet.clone());
    }

    /// Decrease balance of the wallet and append new record to its history.
    ///
    /// Panics if there is no wallet with given public key.
    pub fn decrease_wallet_balance(&mut self, wallet: Wallet, amount: u64, transaction: &Hash) {
        let wallet = {
            let mut history = self.wallet_history_mut(wallet.pub_key());
            history.push(*transaction);
            let history_hash = history.merkle_root();
            let balance = wallet.balance();
            wallet.set_balance(balance - amount, &history_hash)
        };
        self.wallets_mut().put(wallet.pub_key(), wallet.clone());
    }
    
    /// Decrease count of the detail and append new record to its history.
    ///
    /// Panics if there is no wallet with given public key.
    pub fn decrease_detail_count(&mut self, wallet: Wallet, new_count: u64, n_o_type: u8, transaction: &Hash) {
        let wallet = {
            let mut history = self.wallet_history_mut(wallet.pub_key());
            history.push(*transaction);
            let history_hash = history.merkle_root();
            
            
            let mut new_details = wallet.details();
            /*let mut iterator = new_details.iter();*/
            
			/* new_details().iter_mut().find(|v| v.nameoftype() == n_otype).and_then(|v: &mut Detail| v = v.clone().set(self)) */
			let index: Option<usize> = new_details.iter().position(|v| v.name_of_type() == n_o_type);
			match index {
				Some(index) => {
					let detail = new_details[index].clone();
                    let count = detail.count();
					let new_detail = detail.set_count_detail(count - new_count);
					new_details[index] = new_detail;
				},
				None => return,
			}
            wallet.set_details(new_details, &history_hash)
        };
        self.wallets_mut().put(wallet.pub_key(), wallet.clone());
    }
    

    /// Create composite detail.
    ///
    /// Panics if there is no wallet with given public key.
    pub fn create_composite_detail(&mut self, wallet: Wallet, n_o_type: u8, req_details: Vec<Detail>, transaction: &Hash) {
        let wallet = {
            let mut history = self.wallet_history_mut(wallet.pub_key());
            history.push(*transaction);
            let history_hash = history.merkle_root();
            
            
            let mut curr_details = wallet.details();
            /*let mut iterator = new_details.iter();*/
            
			/* new_details().iter_mut().find(|v| v.nameoftype() == n_otype).and_then(|v: &mut Detail| v = v.clone().set(self)) */
			let len_req_details = req_details.len();			
			
			for req_detail in req_details
			{
			    let req_name_of_type = req_detail.name_of_type();
			    let req_count = req_detail.count();
			    let index: Option<usize> = curr_details.iter().position(|v| v.name_of_type() == req_name_of_type);
			    match index {
				    Some(index) => {
					    let detail = curr_details[index].clone();
                        let count = detail.count();
                        if count < req_count 
                        { 
							return; 
						} 
						else
						{
					        let curr_detail = detail.set_count_detail(count - req_count);
					        curr_details[index] = curr_detail;
					    }
				    },
				    None => return,
			   }	
			}
			
			let index: Option<usize> = curr_details.iter().position(|v| v.name_of_type() == n_o_type);
			match index {
				Some(index) => {
					let detail = curr_details[index].clone();
                    let count = detail.count();
					let curr_detail = detail.set_count_detail(count + 1);
					curr_details[index] = curr_detail;
				},
				None => { let i = 1; }, 	
			}
            wallet.set_details(curr_details, &history_hash)
        };
        self.wallets_mut().put(wallet.pub_key(), wallet.clone());
    }
        
    
    
    
    
    /// Insrease count of the detail and append new record to its history.
    ///
    /// Panics if there is no wallet with given public key.
    pub fn increase_detail_count(&mut self, wallet: Wallet, new_count: u64, n_o_type: u8, transaction: &Hash) {
        let wallet = {
            let mut history = self.wallet_history_mut(wallet.pub_key());
            history.push(*transaction);
            let history_hash = history.merkle_root();
            
            
            let mut new_details = wallet.details();
            /*let mut iterator = new_details.iter();*/
            
			/* new_details().iter_mut().find(|v| v.nameoftype() == n_otype).and_then(|v: &mut Detail| v = v.clone().set(self)) */
			let index: Option<usize> = new_details.iter().position(|v| v.name_of_type() == n_o_type);
			match index {
				Some(index) => {
					let detail = new_details[index].clone();
                    let count = detail.count();
					let new_detail = detail.set_count_detail(count + new_count);
					new_details[index] = new_detail;
				},
				None => {
					new_details.push(Detail::new(n_o_type, new_count));
				},
			}
            wallet.set_details(new_details, &history_hash)
        };
        self.wallets_mut().put(wallet.pub_key(), wallet.clone());
    }
    
    

    /// Create new wallet and append first record to its history.
    pub fn create_wallet(&mut self, key: &PublicKey, name: &str, transaction: &Hash) {
        let wallet = {
            let mut history = self.wallet_history_mut(key);
            //Detail.init_details(0, 0);
            let mut balance = 33;
            
            //let test_details = vec![Detail::new(0, 0)];
            //let test_develop = vec![Detail::new(0, 0)];
            
            let mut details = vec![];
            let mut develop = vec![]; 
             
            if name.contains("plain_") {
				develop.push(Detail::new(0, 0));
				details.push(Detail::new(0, 0));
			} 
			
			if name.contains("engine_") {
				develop.push(Detail::new(1, 0));
				details.push(Detail::new(1, 0));
			}
			
			if name.contains("chassis_") {
				develop.push(Detail::new(2, 0));
				details.push(Detail::new(2, 0));
			}
			
			if name.contains("corpus_") {
				develop.push(Detail::new(3, 0));
				details.push(Detail::new(3, 0));
			}
			
			if name.contains("turbine_") {
				develop.push(Detail::new(4, 2));
				details.push(Detail::new(4, 2));
			}
			
			if name.contains("compressor_") {
				develop.push(Detail::new(5, 2));
				details.push(Detail::new(5, 2));
			}
			
			if name.contains("wheel_") {
				develop.push(Detail::new(6, 3));
				details.push(Detail::new(6, 3));
			}
			
			if name.contains("bolt_") {
				develop.push(Detail::new(7, 15));
				details.push(Detail::new(7, 15));
			}
			
			if name.contains("wing_") {
				develop.push(Detail::new(8, 2));
				details.push(Detail::new(8, 2));
			}
			
			if name.contains("glue_") {
				develop.push(Detail::new(9, 10));
				details.push(Detail::new(9, 10));
			}
			
			if name.contains("fuselage_") {
				develop.push(Detail::new(10, 1));
				details.push(Detail::new(10, 1));
			}          
            
            history.push(*transaction);  
            let history_hash = history.merkle_root();
            Wallet::new(key, name, balance, details, develop, history.len(), &history_hash)
        };
        self.wallets_mut().put(key, wallet);
    }
}
