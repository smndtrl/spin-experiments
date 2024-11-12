#[allow(warnings)]
mod bindings;

use crate::bindings::exports::smndtrl::experiments::component::{Guest, Counter, Error};

struct Component;

impl Guest for Component {
    fn modify() -> Result<Counter, Error> {
        Ok(Counter::new(27))
    }
}

bindings::export!(Component with_types_in bindings);
