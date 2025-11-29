use super::print_enum::Print;
use std::collections::HashMap;
use std::process;

pub struct Term {
    a: f64,
    b: f64,
    c: f64,
}

/*  keep only the non null terms and return the largest degree
or exit if solutions are infinite   */
pub(super) fn check_equation_degree(terms: &mut HashMap<i32, f64>) -> i32 {
    terms.retain(|_, &mut value| value != 0.0 || value != -0.0);

    if let Some(&key) = terms.keys().max() {
        return key;
    }
    println!("There are an infinity of solutions, the equation is true for all x.");
    process::exit(0)
}

/*  assigns to a Term structure the values ​​of the equation
a = the coef for exponent 2, b = the coef for exponent 1,
c = the coef for exponent 0 */
pub(super) fn fill_equation_struct(terms: &HashMap<i32, f64>) -> Term {
    let eq_terms = Term {
        a: get_coef(terms, 2),
        b: get_coef(terms, 1),
        c: get_coef(terms, 0),
    };
    eq_terms
}

/*  give the coef for the corresponding exponent key value,
or 0.0   */
fn get_coef(terms: &HashMap<i32, f64>, exponent_to_find: i32) -> f64 {
    if let Some((_exponent, _coef)) = terms
        .iter()
        .find(|(&_exponent, &_coef)| _exponent == exponent_to_find)
    {
        *_coef
    } else {
        0.0
    }
}

/*  compute and print the degre 0 solution  */
pub(super) fn resolve_degre_0(term: &Term) {
    if term.c == 0.0 {
        Print::Solution(term.c).display();
    } else {
        Print::NoSolution.display();
    }
}

/*  compute and print the degre 1 solution  */
pub(super) fn resolve_degre_1(term: &Term) {
    let result = -term.c / term.b;
    Print::Solution(result).display();
}

/*  compute the discriminant, and depending on its value
solve and display the solutions */
pub(super) fn resolve_degre_2(term: &Term) {
    let discriminant = compute_discriminant(&term);

    if discriminant > 0.0 {
        let solution = compute_positive_discriminant_solutions(&term, discriminant);
        Print::SolutionPositiveDiscriminant(solution.0, solution.1).display();
    } else if discriminant == 0.0 {
        let solution = compute_zero_discrimiannt_solution(term);
        Print::SolutionNullDiscriminant(solution).display();
    } else if discriminant < 0.0 {
        let solution = compute_negative_discriminant_solutions(term, discriminant);
        Print::SolutionNegativeDiscriminant(solution.0, solution.1).display();
    }
}
// solve d =  b^2 - 4ac
fn compute_discriminant(term: &Term) -> f64 {
    (term.b * term.b) - (4.0 * term.a * term.c)
}
// solve s1 = -b + sqrt(d) / 2*a, s2 = -b - sqrt(d) / 2*a
fn compute_positive_discriminant_solutions(term: &Term, discriminant: f64) -> (f64, f64) {
    let sqrt_result = sqrttt(discriminant);

    let positive_solution = (-term.b + sqrt_result) / (2.0 * term.a);
    let negative_solution = (-term.b - sqrt_result) / (2.0 * term.a);

    (positive_solution, negative_solution)
}
// solve s = -b / 2a
fn compute_zero_discrimiannt_solution(term: &Term) -> f64 {
    (-term.b) / (2.0 * term.a)
}

// solve s1 = (-b + i*sqrt(d)) / 2a, s2 = (-b - i*sqrt(d)) / 2a
fn compute_negative_discriminant_solutions(term: &Term, discriminant: f64) -> (f64, f64) {
    let nb = (-term.b) / (2.0 * term.a);
    let complex = sqrttt(discriminant.abs()) / (2.0 * term.a);

    (nb, complex)
}


fn sqrttt(nb: f64) -> f64 {

    let result: f64;

    unsafe{
        std::arch::asm!(
            "sqrtsd {result}, {input}",
            input = in(xmm_reg) nb,
            result = out(xmm_reg) result
        )
    }
    result
}