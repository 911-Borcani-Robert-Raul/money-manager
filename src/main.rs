mod repo;
mod domain;
mod ui;
mod service;

fn main() {
    let mut repo = repo::CurrencyRepo::new(String::from("data.txt"));
    let mut service = service::Service::new(&mut repo);
    let mut user_interface = ui::Ui::new(&mut service);

    user_interface.start();
}
