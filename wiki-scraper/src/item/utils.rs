use std::str::FromStr;

use super::Rarity;

pub fn parse_opt_leading_number<T: FromStr>(s: &Option<String>) -> Option<T> {
    s.as_ref().and_then(|s| parse_leading_number(s).ok())
}

pub fn parse_leading_number<T: FromStr>(s: &str) -> Result<T, T::Err> {
    s.char_indices()
        .find(|&(_, c)| !c.is_ascii_digit() && !".-".contains(c))
        .map(|(i, _)| T::from_str(&s[..i]))
        .unwrap_or_else(|| T::from_str(s))
}

pub fn parse_rarity(s: &Option<String>) -> Rarity {
    let rare = s.as_ref().expect("Rarity is required");
    dbg!(rare)
        .parse::<i32>()
        .or_else(|_| {
            parse_leading_number(
                rare.split_once("Rarity color ")
                    .expect("Invalid rarity")
                    .1
                    .split_once('.')
                    .unwrap()
                    .0,
            )
        })
        .expect("Invalid rarity")
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_number() {
        assert_eq!(parse_leading_number::<i32>("123"), Ok(123));
        assert!(parse_leading_number::<i32>("123.0").is_err());
        assert_eq!(parse_leading_number::<f32>("123.0"), Ok(123.0));

        assert_eq!(parse_leading_number::<i32>("123 abc"), Ok(123));
        assert_eq!(parse_leading_number::<i32>("123 456"), Ok(123));
        assert_eq!(parse_leading_number::<f32>("123.0 456"), Ok(123.0));
    }
}
