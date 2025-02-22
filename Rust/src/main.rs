// src/main.rs
#[derive(Debug)]
struct Deposit {
    amount: f64,
}

#[derive(Debug)]
struct Withdrawal {
    amount: f64,
}
#[derive(Debug)]
struct Account {
    owner: String,
    balance: f64,
    deposits: Vec<Deposit>,
    withdrawals: Vec<Withdrawal>,
}

impl Account {
    fn new(owner: String) -> Account {
        Account {
            owner,
            balance: 0.0,
            deposits: Vec::new(),
            withdrawals: Vec::new(),
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        self.deposits.push(Deposit { amount });
        println!("Deposited ${:.2}", amount);
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            self.withdrawals.push(Withdrawal { amount });
            println!("Withdrew ${:.2}", amount);
            Ok(())
        } else {
            Err(String::from("Insufficient funds"))
        }
    }

    fn check_balance(&self) {
        println!("Balance: ${:.2}", self.balance);
    }

    fn print_transactions(&self) {
        println!("Deposits:");
        for deposit in &self.deposits {
            println!(" - ${:.2}", deposit.amount);
        }

        println!("Withdrawals:");
        for withdrawal in &self.withdrawals {
            println!(" - ${:.2}", withdrawal.amount);
        }
    }
}
fn main() {
    let mut account = Account::new(String::from("John Doe"));

    account.deposit(1000.0);
    account.check_balance();
    account.print_transactions();

    match account.withdraw(500.0) {
        Ok(_) => account.check_balance(),
        Err(e) => println!("{}", e),
    }
    account.print_transactions();

    match account.withdraw(600.0) {
        Ok(_) => account.check_balance(),
        Err(e) => println!("{}", e),
    }
    account.print_transactions();
}
