use crate::repo;
use crate::domain;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ServiceError {
    message: String,
}

impl ServiceError {
    pub fn new(message: String) -> ServiceError {
        ServiceError {
            message,
        }
    }

    #[allow(dead_code)]
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct Service<'a> {
    currency_repo: &'a mut repo::CurrencyRepo,
}

impl<'a> Service<'a> {
    pub fn new(currency_repo: &'a mut repo::CurrencyRepo) -> Service<'a> {
        Service {
            currency_repo
        }
    }

    pub fn add_currency(&mut self, currency: domain::Currency, initial_amount: f64) -> Result<(), ServiceError> {
        match self.currency_repo.add_currency(currency, initial_amount) {
            Ok(()) => Ok(()),
            Err(error) => Err(ServiceError::new(String::from(format!("Repo error: {}", error)))),
        }
    }

    pub fn get_summary(&self) -> Vec<&domain::CurrencyBalance> {
        self.currency_repo.iter().collect()
    }

    pub fn buy_currency(&mut self, id: &str, amount: f64) -> Result<(), ServiceError> {
        let initial_amount = self.currency_repo.get_amount(id);
        
        match initial_amount {
            Some(initial_amount) => { 
                self.currency_repo.modify_amount(id, initial_amount + amount);
                Ok(())
            },
            None => Err(ServiceError::new(String::from("No such currency found!"))),
        }
    }

    pub fn sell_currency(&mut self, id: &str, amount: f64) -> Result<(), ServiceError> {
        let initial_amount = self.currency_repo.get_amount(id);
        
        match initial_amount {
            Some(initial_amount) => { 
                self.currency_repo.modify_amount(id, initial_amount - amount);
                Ok(())
            },
            None => Err(ServiceError::new(String::from("No such currency found!"))),
        }
    }

    pub fn remove_currency(&mut self, id: &str) -> Result<(), ServiceError> {
        match self.currency_repo.remove_currency(id) {
            Ok(()) => Ok(()),
            Err(error) => Err(ServiceError::new(String::from(format!("Repo error: {}", error)))),
        }
    }
}