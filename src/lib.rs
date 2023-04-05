use std::str::FromStr;

pub fn parse_comma_seperated<T: FromStr>(csl: &str) -> Result<Vec<T>, T::Err> {
    csl.split(",")
        .filter(|item| item.len() > 0)
        .map(|item| {
            let item = item.parse()?;
            Ok::<T, T::Err>(item)
        })
        .collect()
}
