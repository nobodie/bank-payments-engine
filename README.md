# Bank payments engine

Implements a simple toy payments engine that reads a series of transactions
from a CSV, updates client accounts, handles disputes and chargebacks, and then outputs
the state of clients accounts as a CSV

## Description

Below are the key steps I followed to approach this test :

* First handling of deposits and withdrawals to get a good overview of the challenge.
* Creation of multiple tests to handle edge cases (negative amounts, duplicate transaction ids, withdrawal as first
  transaction, ...)
* Rework to use rust_decimal instead of simple f64 values (due to precision limitations and bank-specific requirements)
* Creation of tests to handle disputes, resolves and chargeback (TDD approach)
* Handling of locked accounts due to chargeback
* General cleanup and clippy checks
* General documentation (readme + comments)

## How-to

### Usage

`cargo run -- tests/data/transactions.csv`

### Input

The input is read from a CSV file provided as a parameter.

```
type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 3.0
```

### Output

The output is a CSV file containing a list of client accounts with the following data:

* client id
* available funds
* held funds
* total (available + held)
* whether the client account is locked due to a chargeback transaction

The output is written to stdout. Errors are written to stderr.

```
client,available,held,total,locked
2,2,0,2,false
1,1.5,0,1.5,false
```

## Features

* Deposits
* Withdrawals
* Disputes (deposited funds are held until resolution)
* Resolves (client's held funds are released)
* Chargebacks (deposit gets reverted and account gets locked)

## Potential improvements

* Redefine specific types to handle client id (u16), transaction id (u32), ...
* Separate internal errors from publicly exposed ones

## Authors

[@Bastien Scheurer](https://www.linkedin.com/in/bastienscheurer/)