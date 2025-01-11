use bank_payments_engine::bank::client::ClientExport;
use bank_payments_engine::bank::export_csv;
use bank_payments_engine::bank::Bank;
use rust_decimal_macros::dec;


fn process_test_file(filename: &str) -> Vec<ClientExport> {
    let mut bank = Bank::new();
    bank.process_csv(&format!("tests/data/{filename}.csv")).expect("Error while processing csv");

    let mut clients = bank.export_clients();

    clients.sort_by_key(|client| client.id);

    clients
}
#[test]
fn single_deposit() {
    let clients = process_test_file("single_deposit");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(1.0), held: dec!(0.0), total: dec!(1.0), locked: false },
    ]);
}

#[test]
fn negative_deposit() {
    let clients = process_test_file("negative_deposit");
    assert_eq!(clients, vec![]);
}

#[test]
fn two_clients_deposit() {
    let clients = process_test_file("two_clients_deposit");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(1.0), held: dec!(0.0), total: dec!(1.0), locked: false },
        ClientExport { id: 2, available: dec!(2.0), held: dec!(0.0), total: dec!(2.0), locked: false },
    ]);
}

#[test]
fn single_withdrawal() {
    let clients = process_test_file("single_withdrawal");
    assert_eq!(clients, vec![]);
}

#[test]
fn withdrawal_after_deposit() {
    let clients = process_test_file("withdrawal_after_deposit");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(0.4), held: dec!(0.0), total: dec!(0.4), locked: false },
    ]);
}

#[test]
fn negative_withdrawal() {
    let clients = process_test_file("negative_withdrawal");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(2.0), held: dec!(0.0), total: dec!(2.0), locked: false },
    ]);
}

#[test]
fn duplicate_transaction_id() {
    let clients = process_test_file("duplicate_transaction_id");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(1.0), held: dec!(0.0), total: dec!(1.0), locked: false },
    ]);
}


#[test]
fn insufficient_funds_for_withdrawal() {
    let clients = process_test_file("insufficient_funds_for_withdrawal");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(1.0), held: dec!(0.0), total: dec!(1.0), locked: false },
    ]);
}


#[test]
fn dispute() {
    let clients = process_test_file("dispute");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(0.0), held: dec!(1.0), total: dec!(1.0), locked: false },
    ]);
}

#[test]
fn double_dispute() {
    let clients = process_test_file("double_dispute");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(10.0), held: dec!(1.0), total: dec!(11.0), locked: false },
    ]);
}

#[test]
fn dispute_resolve() {
    let clients = process_test_file("dispute_resolve");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(1.0), held: dec!(0.0), total: dec!(1.0), locked: false },
    ]);
}

#[test]
fn dispute_chargeback() {
    let clients = process_test_file("dispute_chargeback");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(0.0), held: dec!(0.0), total: dec!(0.0), locked: true },
    ]);
}

#[test]
fn deposit_while_locked() {
    let clients = process_test_file("deposit_while_locked");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(0.0), held: dec!(0.0), total: dec!(0.0), locked: true },
    ]);
}

#[test]
fn withdraw_while_locked() {
    let clients = process_test_file("withdraw_while_locked");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(10.0), held: dec!(0.0), total: dec!(10.0), locked: true },
    ]);
}


#[test]
fn invalid_type() {
    let clients = process_test_file("invalid_type");
    assert_eq!(clients, vec![]);
}


#[test]
fn precision_deposits() {
    let clients = process_test_file("precision_deposits");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(2.0), held: dec!(0.0), total: dec!(2.0), locked: false },
    ]);
}

#[test]
fn precision_withdrawals() {
    let clients = process_test_file("precision_withdrawals");
    assert_eq!(clients, vec![
        ClientExport { id: 1, available: dec!(8.0), held: dec!(0.0), total: dec!(8.0), locked: false },
    ]);
}

#[test]
fn csv_export() {
    let clients = process_test_file("csv_export");
    let data = export_csv(clients).expect("Error while exporting CSV");
    assert_eq!(data, "client,available,held,total,locked
1,1.5,0,1.5,false
2,2,0,2,false
");
}








