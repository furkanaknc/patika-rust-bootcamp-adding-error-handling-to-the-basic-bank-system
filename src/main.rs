fn main() {
    let mut account1 = BankAccount {
        account_number: String::from("123456"),
        holder_name: String::from("Furkan"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: String::from("135790"),
        holder_name: String::from("Joseph"),
        balance: 2000.0,
    };

    match account1.deposit(1000.0) {
        Ok(_) => println!("Deposit successful for Account 1"),
        Err(err) => println!("Deposit error for Account 1: {}", err),
    }

    match account2.withdraw(2000.0) {
        Ok(_) => println!("Withdrawal successful for Account 2"),
        Err(err) => println!("Withdrawal error for Account 2: {}", err),
    }

    println!("Account 1 Balance: ${:.2}", account1.balance());
    println!("Account 2 Balance: ${:.2}", account2.balance());
}

trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: String,
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
    fn withdraw(&mut self, amount: f64)-> Result<(), String>  {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Withdrawal amount must be greater than zero and less than or equal to the balance.".to_string())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
