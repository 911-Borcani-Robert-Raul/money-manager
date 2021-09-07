pub struct Currency{
    pub name: String,
    pub id: String,
}

impl Currency{
    pub fn new(name: String, id: String) -> Currency {
        Currency {
            name,
            id,
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