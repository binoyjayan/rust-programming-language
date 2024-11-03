// References:
// https://without.boats/blog/references-are-like-jumps

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

// Inherent implementation for Account
impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }
    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }
    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }
    fn summary(&self) -> String {
        format!(
            "[{}] {} has balance of {}",
            self.id, self.holder, self.balance
        )
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

// Inherent implementation for Bank
impl Bank {
    fn new() -> Self {
        Bank {
            accounts: Vec::new(),
        }
    }
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }
    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account1 = Account::new(1, "Alice".to_string());
    let mut account2 = Account::new(2, "Bob".to_string());

    account1.deposit(100);
    account2.deposit(200);
    account1.withdraw(50);
    account2.withdraw(100);

    println!("{}", account1.summary());
    println!("{}", account2.summary());

    bank.add_account(account1);
    bank.add_account(account2);

    println!("Bank: {:#?}", bank);
    println!("Summary: {:#?}", bank.summary());
    println!("Total balance: {}", bank.total_balance());
}
