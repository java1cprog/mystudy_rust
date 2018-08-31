/*
////////////////////////////////////////////////////////////////
class BankAccount
{
private double balance; // Баланс счета
public BankAccount(double openingBalance) // Конструктор
{
balance = openingBalance;
}
public void deposit(double amount) // Внесение средств
{
balance = balance + amount;
}
public void withdraw(double amount) // Снятие средств
{
balance = balance - amount;
}
public void display() // Вывод баланса
{
System.out.println("balance=" + balance);
}
// Конец класса BankAccount
////////////////////////////////////////////////////////////////
class BankApp
{
public static void main(String[] args)
{
BankAccount ba1 = new BankAccount(100.00); // Создание счета
System.out.print("Before transactions, ");
ba1.display(); // Вывод баланса
ba1.deposit(74.35); // Внесение средств
ba1.withdraw(20.00); // Снятие средств
System.out.print("After transactions, ");
ba1.display(); // Вывод баланса
} // Конец main()
} // Конец класса BankApp

*/

struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn display(&self) {
        println!("balance = {}", self.balance);
    }

    fn deposit(&mut self, amount: f64) {
        self.balance = self.balance + amount;
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance = self.balance - amount;
    }
}
fn main() {
    let mut bank_account: BankAccount = BankAccount { balance: 100.0 };
    print!("Before transactions, ");
    bank_account.display();
    bank_account.deposit(74.35);
    bank_account.withdraw(20.00);
    print!("After transactions, ");
    bank_account.display();
}
