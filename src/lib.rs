extern crate day_01;
pub extern crate utils;

#[allow(dead_code)]
const VERSION: &'static str = env!("CARGO_PKG_VERSION");


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_version() {
        assert_eq!(VERSION, "0.1.0");
    }

}