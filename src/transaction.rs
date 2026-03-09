#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub date: String,
    pub account: Account,
    pub description: String,
    pub category: String,
    pub amount: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Account {
    Chase(ChaseAccount),
    CapitalOne(CapitalOneAccount),
    Amex(AmexAccount),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChaseAccount {
    Deposit1199,
    CreditCard9055,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CapitalOneAccount {
    VentureX,
    Checking360,
    AashishRainyDay,
    ParentsRainyDay,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmexAccount {
    Gold,
}
