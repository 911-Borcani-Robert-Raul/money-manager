use std::io;
use crate::service::Service;
use crate::domain;
use std::num::ParseFloatError;

pub struct Ui<'a>{
    service: &'a mut Service<'a>,
}

impl<'a> Ui<'a> {
    pub fn new(service: &'a mut Service<'a>) -> Ui<'a> {
        Ui {
            service,
        }
    }

    pub fn start(&mut self) {
        println!("Welcome to the Money Manager!\n");

        loop {
            Ui::print_menu();

            let mut option = String::new();
            io::stdin()
                .read_line(&mut option)
                .expect("Failed to read input!");

            match &(option.trim())[..] {
                "0" => break,
                "1" => self.add_currency(),
                "2" => self.print_summary(),
                "3" => self.buy_currency(),
                "4" => self.sell_currency(),
                _ => println!("Please enter a valid option!"),
            };
        }

        println!("Good bye!!");
    }

    fn print_menu() {
        println!("What would you like to do?");
        println!("0. Exit");
        println!("1. Add new currency");
        println!("2. See summary");
        println!("3. Buy currency");
        println!("4. Sell currency");
    }

    fn add_currency(&mut self) {
        println!("Enter the name of the currency: ");

        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input!");
        let name = name.trim();
        println!("Enter the id of the currency: ");
        let id = Ui::read_currency_id();

        let currency = domain::Currency::new(&name[..], &id[..]);
        
        println!("Enter the initial amount: ");
        let amount: Result<f64, _> = Ui::read_currency_amount();

        match amount {
            Ok(amount) => match self.service.add_currency(currency, amount) {
                Ok(()) => (),
                Err(error) => println!("Error: {}", error),
            },
            Err(_) => println!("Error! Please enter a valid number!"),
        };

        println!();
    }

    fn print_summary(&self) {
        let summary = self.service.get_summary();

        for item in summary.iter() {
            println!("You have {} {} ({})", item.amount, item.currency.id, item.currency.name);
        }
        println!();
    }

    fn buy_currency(&mut self) {
        println!("Enter the id of the currency you want to buy: ");
        let id = Ui::read_currency_id();
        println!("Enter the amount: ");
        let amount: Result<f64, _> = Ui::read_currency_amount();

        match amount {
            Ok(amount) => match self.service.buy_currency(&id[..], amount) {
                Ok(()) => (),
                Err(error) => println!("Error: {}", error),
            },
            Err(_) => println!("Error! Please enter a valid number!"),
        };

        println!();
    }

    fn sell_currency(&mut self) {
        println!("Enter the id of the currency you want to sell: ");
        let id = Ui::read_currency_id();
        println!("Enter the amount: ");
        let amount: Result<f64, _> = Ui::read_currency_amount();

        match amount {
            Ok(amount) => match self.service.sell_currency(&id[..], amount) {
                Ok(()) => (),
                Err(error) => println!("Error: {}", error),
            },
            Err(_) => println!("Error! Please enter a valid number!"),
        };

        println!();
    }

    fn read_currency_id() -> String {
        let mut id = String::new();
        io::stdin()
            .read_line(&mut id)
            .expect("Failed to read input!");
        String::from(id.trim())
    }

    fn read_currency_amount() -> Result<f64, ParseFloatError> {
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read input!");
        let amount = amount.trim();
        amount.parse()
    }
}
