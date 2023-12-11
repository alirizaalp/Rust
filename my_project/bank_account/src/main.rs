trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount > 0.0 {
            self.balance += amount;
            Ok(())
        } else {
            Err("Deposit amount must be greater than zero.".to_string())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Invalid withdrawal amount or insufficient funds.".to_string())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 1,
        holder_name: String::from("ALİRIZA ALP"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 2,
        holder_name: String::from("ALİRIZA"),
        balance: 500.0,
    };

 
    match account1.deposit(200.0) {
        Ok(()) => println!("Deposit successful for Account 1"),
        Err(err) => println!("Deposit error for Account 1: {}", err),
    }


    match account2.withdraw(600.0) {
        Ok(()) => println!("Withdrawal successful for Account 2"),
        Err(err) => println!("Withdrawal error for Account 2: {}", err),
    }

    println!("Account 1 Balance: {}", account1.balance());
    println!("Account 2 Balance: {}", account2.balance());
}

