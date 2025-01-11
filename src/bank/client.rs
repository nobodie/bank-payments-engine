use crate::bank::error::BankError;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Client {
    pub available: Decimal,
    pub held: Decimal,
    pub locked: bool,
    // map of deposits identified by transaction id
    // contains the amount of the deposit and whether the deposit is already disputed or not
    pub deposits: HashMap<u32, (Decimal, bool)>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct ClientExport {
    #[serde(rename = "client")]
    pub id: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

impl Client {
    fn ensure_unlocked(&self) -> Result<(), BankError> {
        if self.locked {
            Err(BankError::ClientLocked)
        } else {
            Ok(())
        }
    }
    pub fn deposit(&mut self, transaction_id: u32, amount: Decimal) -> Result<(), BankError> {
        self.ensure_unlocked()?;
        self.available += amount;
        self.deposits.insert(transaction_id, (amount, false));
        Ok(())
    }

    pub fn withdraw(&mut self, amount: Decimal) -> Result<(), BankError> {
        self.ensure_unlocked()?;
        if self.available < amount {
            return Err(BankError::InsufficientFunds { expected: amount, available: self.available });
        }

        self.available -= amount;

        Ok(())
    }

    pub fn dispute(&mut self, transaction_id: u32) -> Result<(), BankError> {
        self.ensure_unlocked()?;
        // Retrieve deposit data for the corresponding transaction id
        let (ref dispute_amount, disputed) = self.deposits.get_mut(&transaction_id).ok_or(BankError::MissingTransaction { id: transaction_id })?;

        if *disputed {
            return Err(BankError::AlreadyDisputed { id: transaction_id });
        }

        if self.available < *dispute_amount {
            return Err(BankError::InsufficientFunds { expected: *dispute_amount, available: self.available });
        }

        self.available -= dispute_amount;
        self.held += dispute_amount;
        *disputed = true;

        Ok(())
    }

    pub fn resolve(&mut self, transaction_id: u32) -> Result<(), BankError> {
        self.ensure_unlocked()?;
        let (ref dispute_amount, disputed) = self.deposits.get_mut(&transaction_id).ok_or(BankError::MissingTransaction { id: transaction_id })?;
        if !*disputed {
            return Err(BankError::NotDisputed { id: transaction_id });
        }

        self.available += dispute_amount;
        self.held -= dispute_amount;
        *disputed = false;

        Ok(())
    }

    pub fn chargeback(&mut self, transaction_id: u32) -> Result<(), BankError> {
        self.ensure_unlocked()?;
        let (dispute_amount, disputed) = self.deposits.get(&transaction_id).ok_or(BankError::MissingTransaction { id: transaction_id })?;
        if !*disputed {
            return Err(BankError::NotDisputed { id: transaction_id });
        }

        self.held -= dispute_amount;
        self.locked = true;

        Ok(())
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            available: Decimal::zero(),
            held: Decimal::zero(),
            locked: false,
            deposits: HashMap::new(),
        }
    }
}