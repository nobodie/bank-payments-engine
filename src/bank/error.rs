use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BankError {
    #[error("Invalid parameters")]
    InvalidParameters,
    #[error("Invalid CSV")]
    CsvError(#[from] csv::Error),
    #[error("Utf8 error")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Csv export error")]
    CsvExportError,
    #[error("Invalid Csv Data")]
    InvalidCsvData,
    #[error("Insufficient funds (expected: {expected}, available: {available})")]
    InsufficientFunds { expected: Decimal, available: Decimal },
    #[error("Duplicate transaction {id}")]
    DuplicateTransaction { id: u32 },
    #[error("Missing transaction {id}")]
    MissingTransaction { id: u32 },
    #[error("Invalid client {id}")]
    InvalidClient { id: u16 },
    #[error("Negative amount not allowed")]
    NegativeAmount,
    #[error("Missing amount value for transaction")]
    MissingAmount,
    #[error("Transaction {id} already disputed")]
    AlreadyDisputed { id: u32 },
    #[error("Transaction {id} is not disputed")]
    NotDisputed { id: u32 },
    #[error("Client is locked")]
    ClientLocked,
    #[error("Unknown transaction type {name}")]
    UnknownTransactionType { name: String },
}