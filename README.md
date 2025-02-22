# Banking Application in Rust

## Overview

This project implements a simple banking application written in Rust. The application allows users to perform basic banking operations such as making deposits, making withdrawals, checking the account balance, and viewing transaction history. It is a console-based application intended for educational purposes to demonstrate Rust's struct handling, error management, and object-oriented programming features.

---

## Features

- **Deposit**: Add funds to the account.
- **Withdrawal**: Withdraw funds from the account with error handling for insufficient funds.
- **Balance Check**: View the current account balance.
- **Transaction History**: View all deposits and withdrawals made to the account.

---

## Installation

1. Make sure that [Rust](https://www.rust-lang.org/) is installed on your machine. You can verify this by running the following command in your terminal:

    ```bash
    rustc --version
    ```

    If you don't have Rust installed, please follow the installation instructions on the [official website](https://www.rust-lang.org/tools/install).

2. Clone this repository to your local machine:

    ```bash
    git clone https://github.com/yourusername/banking-app-rust.git
    ```

3. Navigate to the project directory:

    ```bash
    cd banking-app-rust
    ```

4. Build and run the project using Cargo (Rust’s package manager and build system):

    ```bash
    cargo run
    ```

    This will compile and run the banking application, allowing you to see the various banking operations in action.

---

## Usage

Once the application is running, the following operations are available:

1. **Depositing Money**: Use the `deposit` method to add funds to the account.

    Example:
    ```rust
    account.deposit(1000.0);
    ```

2. **Withdrawing Money**: Use the `withdraw` method to withdraw money from the account. If the balance is insufficient,

# Youtube: https://youtu.be/3lhKE2MfSmQ
