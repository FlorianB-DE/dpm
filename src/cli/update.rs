use crate::{sources::update_source, Errors};

pub fn run() -> Result<(), Errors> {
    update_source(None)
}
