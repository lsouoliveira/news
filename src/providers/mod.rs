#[macro_use]
mod tests {
    macro_rules! fixture {
        ($path:expr) => {
            std::fs::read_to_string(concat!("tests/fixtures/", $path)).unwrap()
        };
    }
}

pub mod core;
