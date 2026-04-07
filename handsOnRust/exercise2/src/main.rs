struct Wallet {
    balance: f64,
}

impl Wallet {
    fn deposit(&mut self, amount: f64)
    {
        if amount > 0f64 {
            self.balance += amount;
        }
    }

    fn withdraw(&mut self, amount: f64) -> bool
    {
        if amount < 0f64 {
            return false;
        }

        if self.balance > amount {
            self.balance -= amount;
            return true;
        }
        else {
            return false;
        }
    }
}

fn main() {
    let mut wallet = Wallet {balance: 0f64};
    println!("{}", wallet.balance);

    wallet.deposit(10f64);
    println!("{}", wallet.balance);

    wallet.withdraw(3f64);
    println!("{}", wallet.balance);

    wallet.withdraw(10f64);
    println!("{}", wallet.balance);

    wallet.deposit(10f64);
    println!("{}", wallet.balance);

    wallet.withdraw(-10f64);
    println!("{}", wallet.balance);

    wallet.deposit(-10f64);
    println!("{}", wallet.balance);
}
