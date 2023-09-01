use crate::{common::TestCase, external_module::do_external};

use super::error::ErrorB;

pub fn do_b(case: &TestCase) -> Result<bool, ErrorB> {
    // Using external module
    match do_external(case) {
        Ok(_) => (),
        // handling error from external module
        Err(error) => return Err(error.into()),
    }

    // Do something from this module
    match case {
        TestCase::FailFromMiddle1 => Err(ErrorB::ErrorB1("Error B1 from do_b.".to_string())),
        TestCase::FailFromMiddle2 => Err(ErrorB::ErrorB2("Error B2 from do_b.".to_string())),
        _ => Ok(true),
    }
}
