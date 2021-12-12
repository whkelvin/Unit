use ic_cdk::export::{candid::{CandidType, Deserialize}};
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::collections::BTreeMap;
use std::vec::Vec;

type ExpensesStorage = BTreeMap<String, Vec<Expense>>;

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
    let expenses_storage = storage::get::<ExpensesStorage>();

    expenses_storage.get(&date).cloned().unwrap_or_else(|| Vec::new())
}

#[update(name = "addExpense")]
fn add_expense(expense: Expense) -> () {
    let date = expense.date.clone();
    let expenses_storage = storage::get_mut::<ExpensesStorage>();
    let mut list: Vec<Expense> = expenses_storage.get_mut(&date).cloned().unwrap_or_else(|| Vec::new());
    list.push(expense);
    expenses_storage.insert(date, list);
}
