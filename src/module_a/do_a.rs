use crate::{common::TestCase, module_b::do_b};

use super::error::ErrorA;

pub fn do_a(case: TestCase) -> Result<bool, ErrorA> {
    match do_b(&case) {
        Ok(_) => (),
        // handling error from other module
        Err(error) => return Err(error.into()),
    }

    // Do something from this module
    match case {
        TestCase::FailFromTop1 => Err(ErrorA::ErrorA1(format!("Error A1 from do_a({:?}).", &case))),
        TestCase::FailFromTop2 => Err(ErrorA::ErrorA2(format!("Error A2 from do_a({:?}).", &case))),
        _ => Ok(true),
    }
}
