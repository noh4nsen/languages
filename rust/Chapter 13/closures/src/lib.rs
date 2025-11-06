use application::closure_examples;
use application::giveaway;
use helpers::setup;

mod application;
mod domain;
mod helpers;

pub fn run_giveaway() {
    for shirt in setup::shirts() {
        giveaway::run(&shirt, &setup::store());
    }
}

pub fn run_closures(numbers: Vec<u64>, mutable_numbers: &mut Vec<u64>) {
    for i in 0..numbers.len() {
        closure_examples::expensive_closure()(numbers[i]);
        closure_examples::example_closure_1(numbers[i]);
    }
    closure_examples::example_closure_2(&numbers);
    closure_examples::example_closure_3(mutable_numbers);
    closure_examples::example_closure_4(numbers.clone());
}
