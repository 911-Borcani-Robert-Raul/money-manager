use crate::repo;
use crate::domain;

pub struct Service<'a> {
    currency_repo: &'a mut repo::CurrencyRepo,
}

impl<'a> Service<'a> {
    pub fn new(currency_repo: &'a mut repo::CurrencyRepo) -> Service<'a> {
        Service {
            currency_repo
        }
    }

    pub fn add_currency(&mut self, currency: domain::Currency, initial_amount: f64) -> Result<(), &str> {
        self.currency_repo.add_currency(currency, initial_amount)
    }

    pub fn get_summary(&self) -> Vec<&domain::CurrencyBalance> {
        let mut result = Vec::new();
        for item in self.currency_repo.iter() {
            result.push(item);
        }

        result
    }

    pub fn buy_currency(&mut self, id: &str, amount: f64) -> Result<(), &str> {
        let initial_amount = self.currency_repo.get_amount(id);
        
        match initial_amount {
            Some(initial_amount) => { 
                self.currency_repo.modify_amount(id, initial_amount + amount);
                Ok(())
            },
            None => Err("No such currency found!"),
        }
    }

    pub fn sell_currency(&mut self, id: &str, amount: f64) -> Result<(), &str> {
        let initial_amount = self.currency_repo.get_amount(id);
        
        match initial_amount {
            Some(initial_amount) => { 
                self.currency_repo.modify_amount(id, initial_amount - amount);
                Ok(())
            },
            None => Err("No such currency found!"),
        }
    }
}