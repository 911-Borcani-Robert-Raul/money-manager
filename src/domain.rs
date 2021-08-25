pub struct Currency{
    pub name: String,
    pub id: String,
}

impl Currency{
    pub fn new(name: &str, id: &str) -> Currency {
        Currency {
            name: String::from(name),
            id: String::from(id),
        }
    }
}

pub struct CurrencyBalance {
    pub currency: Currency,
    pub amount: f64,
}

impl CurrencyBalance {
    pub fn new(currency: Currency, initial_amount: f64) -> CurrencyBalance {
        CurrencyBalance {
            currency: currency,
            amount: initial_amount,
        }
    }
}