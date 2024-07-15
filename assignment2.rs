struct BankAccount {
    account_number: u32,
    balance: f64,
}

impl BankAccount {
    // Constructor for creating a new BankAccount
    fn new(account_number: u32, initial_balance: f64) -> BankAccount {
        BankAccount {
            account_number,
            balance: initial_balance,
        }
    }

    // Getter method for the balance
    fn get_balance(&self) -> f64 {
        self.balance
    }

    // Method to deposit an amount and return a new BankAccount with updated balance
    fn deposit(self, amount: f64) -> BankAccount {
        if amount > 0.0 {
            BankAccount {
                account_number: self.account_number,
                balance: self.balance + amount,
            }
        } else {
            // If the amount is not positive, return the original account unchanged
            self
        }
    }

    // Method to withdraw an amount and return a new BankAccount with updated balance
    fn withdraw(self, amount: f64) -> BankAccount {
        if self.balance >= amount {
            BankAccount {
                account_number: self.account_number,
                balance: self.balance - amount,
            }
        } else {
            // If there are insufficient funds, return the original account unchanged
            self
        }
    }

    // Method to transfer an amount to another account and return the updated accounts
    fn transfer(self, to_account: BankAccount, amount: f64) -> (BankAccount, BankAccount) {
        if self.balance >= amount {
            let updated_self = BankAccount {
                account_number: self.account_number,
                balance: self.balance - amount,
            };
            let updated_to_account = BankAccount {
                account_number: to_account.account_number,
                balance: to_account.balance + amount,
            };
            (updated_self, updated_to_account)
        } else {
            // If there are insufficient funds, return the original accounts unchanged
            (self, to_account)
        }
    }
}

fn main() {
    let account1 = BankAccount::new(123456, 1000.0);
    let account2 = BankAccount::new(654321, 500.0);

    let (updated_account1, updated_account2) = account1.transfer(account2, 200.0);
    println!("Account 1 balance after transfer: {}", updated_account1.get_balance());
    println!("Account 2 balance after transfer: {}", updated_account2.get_balance());

    let (updated_account1, updated_account2) = updated_account1.transfer(updated_account2, 1200.0);
    println!("Account 1 balance after failed transfer: {}", updated_account1.get_balance());
    println!("Account 2 balance after failed transfer: {}", updated_account2.get_balance());
}
