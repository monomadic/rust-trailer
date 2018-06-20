use trailer::exchanges::Exchange;

pub enum Command {
    Test,
    SetExchange(Exchange),
}
