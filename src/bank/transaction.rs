use crate::bank::error::BankError;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
    #[serde(untagged)]
    Unknown(String), //Required to handle invalid transaction types within csv file
}


#[derive(Debug, Deserialize)]
pub struct RawTransaction {
    #[serde(rename = "type")]
    pub kind: TransactionKind,
    pub client: u16,
    #[serde(rename = "tx")]
    pub id: u32,
    pub amount: Option<Decimal>, // Only deposits and withdrawals have an amount
}

impl RawTransaction {
    pub fn get_amount(&self) -> Result<Decimal, BankError> {
        // Deposits and Withdrawals must contain a positive amount.
        // All other transactions have no given amount.
        // Calling get_amount on them returns a BankError
        if let Some(amount) = self.amount {
            // Any non positive amount is nonsense
            if amount <= Decimal::zero() {
                return Err(BankError::NegativeAmount);
            }

            Ok(amount)
        } else {
            Err(BankError::MissingAmount)
        }
    }
}

pub enum Transaction {
    Deposit { id: u32, client: u16, amount: Decimal },
    Withdrawal { id: u32, client: u16, amount: Decimal },
    Dispute { client: u16, id: u32 },
    Resolve { client: u16, id: u32 },
    Chargeback { client: u16, id: u32 },
}