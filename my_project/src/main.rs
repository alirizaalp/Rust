
#[allow(dead_code)] //I USED IT TO INDICATE THAT IT SHOULD NOT BE USED ANYWHERE.
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds");
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    
    let mut account1 = BankAccount {
        account_number: 1,
        holder_name: String::from("ALİ RIZA ALP"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 2,
        holder_name: String::from("ALİ RIZA ALP"),
        balance: 500.0,
    };

 
    account1.deposit(200.0);

  
    account2.withdraw(100.0);

    println!("Account 1 Balance: {}", account1.balance());
    println!("Account 2 Balance: {}", account2.balance());
}
