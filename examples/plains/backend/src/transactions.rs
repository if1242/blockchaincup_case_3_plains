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

use exonum::blockchain::{ExecutionError, ExecutionResult, Transaction};
use exonum::crypto::{CryptoHash, PublicKey};
use exonum::messages::Message;
use exonum::storage::Fork;

use CRYPTOCURRENCY_SERVICE_ID;
use schema::CurrencySchema;
use wallet::Detail;
/// Error codes emitted by wallet transactions during execution.
#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    /// Wallet already exists.
    ///
    /// Can be emitted by `CreateWallet`.
    #[fail(display = "Wallet already exists")]
    WalletAlreadyExists = 0,

    /// Sender doesn't exist.
    ///
    /// Can be emitted by `Transfer`.
    #[fail(display = "Sender doesn't exist")]
    SenderNotFound = 1,

    /// Receiver doesn't exist.
    ///
    /// Can be emitted by `Transfer` or `Issue`.
    #[fail(display = "Receiver doesn't exist")]
    ReceiverNotFound = 2,

    /// Insufficient currency amount.
    ///
    /// Can be emitted by `Transfer`.
    #[fail(display = "Insufficient currency amount")]
    InsufficientCurrencyAmount = 3,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}

transactions! {
    pub WalletTransactions {
        const SERVICE_ID = CRYPTOCURRENCY_SERVICE_ID;

        /// Transfer `amount` of the currency from one wallet to another.
        struct Transfer {
            from:    &PublicKey,
            to:      &PublicKey,
            amount:  u64,
            seed:    u64,
        }
        
        struct TransferDetails {
            from:    &PublicKey,
            to:      &PublicKey,
            name_of_type: u8,
            count:   u64,
            seed:    u64,
        }

        /// Issue `amount` of the currency to the `wallet`.
        struct Issue {
            pub_key:  &PublicKey,
            amount:  u64,
            seed:    u64,
        }
        
        /// Issue `amount` of the currency to the `wallet`.
        struct IssueDetail {
            pub_key:  &PublicKey,
            name_of_type: u8,
            seed:    u64,
        }

        /// Create wallet with the given `name`.
        struct CreateWallet {
            pub_key: &PublicKey,
            name:    &str,
        }
    }
}

impl Transaction for TransferDetails {
    fn verify(&self) -> bool {
        self.verify_signature(self.from())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = CurrencySchema::new(fork);

        let from = self.from();
        let to = self.to();
        let hash = self.hash();
        let name_of_type = self.name_of_type();
        let count = self.count();

        let sender = schema.wallet(from).ok_or(Error::SenderNotFound)?;

        let receiver = schema.wallet(to).ok_or(Error::ReceiverNotFound)?;

        let sender_details = sender.details();

        let index: Option<usize> = sender_details.iter().position(|v| v.name_of_type() == name_of_type);
		match index {
		    Some(index) => {
					let detail = sender_details[index].clone();
                    let count_sender = detail.count();
					if count_sender < count {
                        Err(Error::InsufficientCurrencyAmount)?
                    }
				},
				None => Err(Error::InsufficientCurrencyAmount)?
			}

        schema.decrease_detail_count(sender, count, name_of_type, &hash);
        schema.increase_detail_count(receiver, count, name_of_type, &hash);

        Ok(())
    }
}


impl Transaction for Transfer {
    fn verify(&self) -> bool {
        self.verify_signature(self.from())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = CurrencySchema::new(fork);

        let from = self.from();
        let to = self.to();
        let hash = self.hash();
        let amount = self.amount();

        let sender = schema.wallet(from).ok_or(Error::SenderNotFound)?;

        let receiver = schema.wallet(to).ok_or(Error::ReceiverNotFound)?;

        if sender.balance() < amount {
            Err(Error::InsufficientCurrencyAmount)?
        }

        schema.decrease_wallet_balance(sender, amount, &hash);
        schema.increase_wallet_balance(receiver, amount, &hash);

        Ok(())
    }
}

impl Transaction for IssueDetail {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = CurrencySchema::new(fork);
        let pub_key = self.pub_key();
        let hash = self.hash();
        let name_of_type = self.name_of_type();
        let mut count_create = 0;

        if let Some(wallet) = schema.wallet(pub_key) {
			
			let our_details = wallet.details();
            let our_develop = wallet.develop();

            let index: Option<usize> = our_develop.iter().position(|v| v.name_of_type() == name_of_type);
		    match index {
                Some(index) => {
					let detail = our_develop[index].clone();
                    count_create = detail.count();
				},
				None => Err(Error::InsufficientCurrencyAmount)?
			}
			if name_of_type > 3 { /********************* Простые детали *******************/
                schema.increase_detail_count(wallet, count_create, name_of_type, &hash);
            }
            else if name_of_type == 0 { /********************* Собираем самолет *******************/
				
				let mut req_details = vec![];
				req_details.push(Detail::new(2, 1));
				req_details.push(Detail::new(3, 2));
				req_details.push(Detail::new(1, 3));
		        schema.create_composite_detail(wallet, 0, req_details, &hash);   
			}
			else if name_of_type == 1 { /********************* Собираем двигатель *******************/
				
				let mut req_details = vec![];
				req_details.push(Detail::new(4, 1));
				req_details.push(Detail::new(5, 1));
		        schema.create_composite_detail(wallet, 1, req_details, &hash);   
			}
			else if name_of_type == 2 { /********************* Собираем шасси *******************/
				
				let mut req_details = vec![];
				req_details.push(Detail::new(6, 1));
				req_details.push(Detail::new(7, 5));
		        schema.create_composite_detail(wallet, 2, req_details, &hash);   
			}
			else if name_of_type == 3 { /********************* Собираем корпус *******************/
				
				let mut req_details = vec![];
				req_details.push(Detail::new(8, 2));
				req_details.push(Detail::new(9, 10));
				req_details.push(Detail::new(10, 1));
		        schema.create_composite_detail(wallet, 3, req_details, &hash);   
			}
			
			
			
				    
            Ok(())
        } else {
            Err(Error::ReceiverNotFound)?
        }
    }
}



impl Transaction for Issue {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = CurrencySchema::new(fork);
        let pub_key = self.pub_key();
        let hash = self.hash();

        if let Some(wallet) = schema.wallet(pub_key) {
            let amount = self.amount();
            schema.increase_wallet_balance(wallet, amount, &hash);
            Ok(())
        } else {
            Err(Error::ReceiverNotFound)?
        }
    }
}

impl Transaction for CreateWallet {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, fork: &mut Fork) -> ExecutionResult {
        let mut schema = CurrencySchema::new(fork);
        let pub_key = self.pub_key();
        let hash = self.hash();

        if schema.wallet(pub_key).is_none() {
            let name = self.name();
            schema.create_wallet(pub_key, name, &hash);
            Ok(())
        } else {
            Err(Error::WalletAlreadyExists)?
        }
    }
}
