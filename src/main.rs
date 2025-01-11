use bank_payments_engine::bank::error::BankError;
use bank_payments_engine::bank::{export_csv, Bank};

fn main() -> Result<(), BankError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(BankError::InvalidParameters);
    }

    let mut bank = Bank::new();

    bank.process_csv(&args[1])?;

    let clients = bank.export_clients();

    let data = export_csv(clients)?;
    print!("{}", data);

    Ok(())
}
