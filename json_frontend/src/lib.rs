use serde;
use serde_json;

use midlang;

pub fn parse_string<'a, T>(str: &'a str) -> serde_json::Result<T>
where
    T: serde::Deserialize<'a>,
{
    serde_json::from_str::<T>(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
