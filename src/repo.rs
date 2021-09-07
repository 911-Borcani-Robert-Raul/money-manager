use crate::domain;
use std::collections::HashMap;
use std::collections::hash_map;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::num::ParseFloatError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct RepoError {
    message: String,
}

impl RepoError {
    pub fn new(message: String) -> RepoError {
        RepoError {
            message,
        }
    }

    #[allow(dead_code)]
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct CurrencyRepo {
    currencies: HashMap<String, domain::CurrencyBalance>,
    file_name: String,
}

impl CurrencyRepo {
    pub fn new(file_name: String) -> CurrencyRepo {
        let mut repo = CurrencyRepo {
            currencies: HashMap::new(),
            file_name,
        };

        match repo.read_data() {
            Ok(()) => (),
            Err(error) => eprintln!("{}", error),
        };

        repo
    }

    pub fn add_currency(&mut self, currency: domain::Currency, amount: f64) -> Result<(), RepoError> {
        if self.currencies.contains_key(&currency.id) {
            return Err(RepoError::new(String::from("Currency with given ID already exists")));
        }

        self.currencies.insert(currency.id.clone(), domain::CurrencyBalance::new(currency, amount));
        match self.write_data() {
            Ok(()) => (),
            Err(error) => eprintln!("{}", error.to_string()),
        };
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_number_of_currencies(&self) -> usize {
        self.currencies.len()
    }

    pub fn get_amount(&self, id: &str) -> Option<f64> {
        let currency = self.currencies.get(&String::from(id));
        match currency {
            Some(currency) => Some(currency.amount),
            None => None
        }
    }

    pub fn modify_amount(&mut self, id: &str, new_amount: f64) {
        let currency = self.currencies.get_mut(&String::from(id));
        match currency {
            Some(currency) => currency.amount = new_amount,
            None => ()
        };
        match self.write_data() {
            Ok(()) => (),
            Err(error) => eprintln!("{}", error.to_string()),
        };
    }

    pub fn remove_currency(&mut self, id: &str) -> Result<(), RepoError> {
        match self.currencies.remove(&String::from(id)) {
            Some(_) => {
                match self.write_data() {
                    Ok(()) => (),
                    Err(error) => eprintln!("{}", error.to_string()),
                };
                Ok(())
            },
            None => Err(RepoError::new(String::from("No such currency found!"))),
        }
    }

    pub fn iter(&self) -> hash_map::Values<'_, String, domain::CurrencyBalance> {
        self.currencies.values()
    }

    fn read_data(&mut self) -> Result<(), RepoError> {
        let file = match File::open(&self.file_name[..]) {
            Ok(file) => file,
            Err(error) => { return Err(RepoError::new(error.to_string())); },
        };

        let buffered = BufReader::new(file);
        for line in buffered.lines() {
            let current_line: String = match line {
                Ok(line) => line,
                Err(error) => { return Err(RepoError::new(error.to_string())); },
            };
            let items: Vec<&str> = current_line.split("|").collect();

            let current_currency = domain::Currency::new(items[0], items[1]);
            let amount: Result<f64, ParseFloatError> = items[2].parse();
            
            match amount {
                Ok(amount) => {
                    let current_currency_balance = domain::CurrencyBalance::new(current_currency, amount);
                    self.currencies.insert(current_currency_balance.currency.id.clone(), current_currency_balance);
                },
                Err(error) => { return Err(RepoError::new(error.to_string())); },
            };
        }

        Ok(())
    }

    fn write_data(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.file_name[..])?;
        
        for item in self.iter() {
            let line = item.currency.name.clone() + "|" + &item.currency.id[..] + "|" + &item.amount.to_string()[..] + "\n";
            match write!(file, "{}", line) {
                Ok(()) => (),
                Err(error) => return Err(error),
            };
        }

        Ok(())
    }
}
