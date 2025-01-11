pub mod error;
pub mod transaction;
pub mod client;

use crate::bank::client::{Client, ClientExport};
use crate::bank::error::BankError;
use crate::bank::transaction::{RawTransaction, Transaction, TransactionKind};
use csv::{ReaderBuilder, Writer};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Bank {
    clients: HashMap<u16, Client>, // Map of clients identified by their ID
    transactions: HashSet<u32>, // Set of transaction IDs (used to check for duplicates)
}

impl Bank {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn export_clients(&self) -> Vec<ClientExport> {
        self.clients.iter().map(|(id, client)| ClientExport {
            id: *id,
            available: client.available,
            held: client.held,
            total: client.available + client.held,
            locked: client.locked,
        }).collect()
    }

    fn apply_transaction(&mut self, transaction: Transaction) -> Result<(), BankError> {
        match transaction {
            Transaction::Deposit { id, client, amount } => {
                // Assumption : a client gets created only when he makes a deposit.
                // All other cases require client to exist
                let client = self.clients.entry(client).or_default();
                client.deposit(id, amount)?;
                self.transactions.insert(id);
            }
            Transaction::Withdrawal { id, client, amount } => {
                let client = self.clients.get_mut(&client).ok_or(BankError::InvalidClient { id: client })?;
                client.withdraw(amount)?;
                self.transactions.insert(id);
            }
            Transaction::Dispute { client, id } => {
                let client = self.clients.get_mut(&client).ok_or(BankError::InvalidClient { id: client })?;
                client.dispute(id)?;
            }
            Transaction::Resolve { client, id } => {
                let client = self.clients.get_mut(&client).ok_or(BankError::InvalidClient { id: client })?;
                client.resolve(id)?;
            }
            Transaction::Chargeback { client, id } => {
                let client = self.clients.get_mut(&client).ok_or(BankError::InvalidClient { id: client })?;
                client.chargeback(id)?;
            }
        }
        Ok(())
    }
    pub fn handle_transaction(&mut self, transaction: RawTransaction) -> Result<(), BankError> {
        let transaction = self.validate_transaction(&transaction)?;
        self.apply_transaction(transaction)
    }

    fn ensure_transaction_is_unique(&self, id: u32) -> Result<(), BankError> {
        if self.transactions.contains(&id) {
            return Err(BankError::DuplicateTransaction { id });
        }
        Ok(())
    }

    fn ensure_transaction_exists(&self, id: u32) -> Result<(), BankError> {
        if !self.transactions.contains(&id) {
            return Err(BankError::MissingTransaction { id });
        }
        Ok(())
    }

    fn validate_transaction(&self, transaction: &RawTransaction) -> Result<Transaction, BankError> {
        match transaction.kind {
            TransactionKind::Deposit => {
                self.ensure_transaction_is_unique(transaction.id)?;
                // amount must respect a precision of 4 digits
                let amount = transaction.get_amount()?.trunc_with_scale(4).normalize();

                Ok(Transaction::Deposit { id: transaction.id, client: transaction.client, amount })
            }
            TransactionKind::Withdrawal => {
                self.ensure_transaction_is_unique(transaction.id)?;
                let amount = transaction.get_amount()?.trunc_with_scale(4).normalize();

                Ok(Transaction::Withdrawal { id: transaction.id, client: transaction.client, amount })
            }
            TransactionKind::Dispute => {
                self.ensure_transaction_exists(transaction.id)?;

                Ok(Transaction::Dispute { client: transaction.client, id: transaction.id })
            }
            TransactionKind::Resolve => {
                self.ensure_transaction_exists(transaction.id)?;

                Ok(Transaction::Resolve { client: transaction.client, id: transaction.id })
            }
            TransactionKind::Chargeback => {
                self.ensure_transaction_exists(transaction.id)?;

                Ok(Transaction::Chargeback { client: transaction.client, id: transaction.id })
            }
            TransactionKind::Unknown(ref name) => {
                Err(BankError::UnknownTransactionType { name: name.clone() })
            }
        }
    }

    pub fn process_csv(&mut self, csv_file: &str) -> Result<(), csv::Error> {
        let mut reader = ReaderBuilder::new().trim(csv::Trim::All).from_path(csv_file)?;

        for record in reader.deserialize() {
            let transaction: RawTransaction = record?;

            // Any invalid transaction handling results in an error caught here and displayed.
            // We keep processing the rest of the file.
            if let Err(e) = self.handle_transaction(transaction) {
                eprintln!("{}", e);
            }
        }

        Ok(())
    }
}


pub fn export_csv(clients: Vec<ClientExport>) -> Result<String, BankError> {
    let mut wtr = Writer::from_writer(vec![]);

    for client in clients {
        wtr.serialize(client)?;
    }

    let data = String::from_utf8(wtr.into_inner().map_err(|_| { BankError::CsvExportError })?)?;
    Ok(data)
}
