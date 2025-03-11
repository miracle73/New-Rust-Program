#[derive(Debug)]
struct BankAccount {
    bank_name: String,
    account_number: String,
    account_type: String,
    balance: f64,
    is_active: bool,
}

impl BankAccount {
    fn new(
        bank_name: String,
        account_number: String,
        account_type: String,
        balance: f64,
        is_active: bool,
    ) -> Self {
        BankAccount {
            bank_name,
            account_number,
            account_type,
            balance,
            is_active,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if self.is_active {
            self.balance += amount;
            println!("Deposit successful! New balance: {}", self.balance);
        } else {
            println!("Account is not active.");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if self.is_active {
            if amount <= self.balance {
                self.balance -= amount;
                println!("Withdrawal successful! New balance: {}", self.balance);
            } else {
                println!("Insufficient funds!");
            }
        } else {
            println!("Account is not active.");
        }
    }

    fn display(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    let mut my_account = BankAccount::new(
        String::from("Union Bank"),
        String::from("1234567890"),
        String::from("Savings"),
        1000.0,
        true,
    );

    my_account.display();

    my_account.deposit(500.0);
    my_account.withdraw(300.0);
    my_account.withdraw(1500.0);
}
