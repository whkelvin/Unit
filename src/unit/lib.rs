use ic_cdk::export::{candid::{CandidType, Deserialize}, Principal};
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::collections::BTreeMap;
use std::vec::Vec;

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Expense {
    pub name: String,
    pub description: String,
    pub amount: u64,
    pub date: String,
}

#[query(name = "getExpenses")]
fn get_expenses(date: String) -> Vec<Expense> {
    ic_cdk::print(format!("hello {}", date));
    return Vec::new();
}

#[update(name = "addExpense")]
fn add_expense(expense: Expense) -> () {
    ic_cdk::print(format!("hello {}", expense.date));
}
