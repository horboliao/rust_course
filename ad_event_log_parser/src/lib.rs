use nom::error::Error;
use nom::{
    character::complete::{alphanumeric1, char, digit1},
    combinator::map_res,
    sequence::tuple,
    IResult,
};
use std::str::FromStr;

// Parsing date in `dd-mm-yyyy` format
pub fn parse_date(input: &str) -> IResult<&str, &str> {
    let (input, (_day, _, _month, _, _year)) = tuple((digit1, char('-'), digit1, char('-'), digit1))(input)?;
    Ok((input, input))
}

// Parsing time in `hh:mm:ss` format
pub fn parse_time(input: &str) -> IResult<&str, &str> {
    let (input, (_hour, _, _minute, _, _second)) = tuple((digit1, char(':'), digit1, char(':'), digit1))(input)?;
    Ok((input, input))
}

// Parsing a UUID (for Transaction ID)
pub fn parse_uuid(input: &str) -> IResult<&str, String, Error<&str>> {
    map_res(
        tuple((
            alphanumeric1, char('-'), alphanumeric1, char('-'),
            alphanumeric1, char('-'), alphanumeric1, char('-'),
            alphanumeric1
        )),
        |(p1, _, p2, _, p3, _, p4, _, p5)| -> Result<String, nom::Err<()>> {
            Ok(format!("{}-{}-{}-{}-{}", p1, p2, p3, p4, p5))
        }
    )(input)
}

// Parsing an IP address
pub fn parse_ip(input: &str) -> IResult<&str, String, Error<&str>> {
    map_res(
        tuple((
            digit1, char('.'), digit1, char('.'),
            digit1, char('.'), digit1
        )),
        |(p1, _, p2, _, p3, _, p4)| -> Result<String, nom::Err<()>> {
            Ok(format!("{}.{}.{}.{}", p1, p2, p3, p4))
        }
    )(input)
}

// Parse floating numbers for price and realPrice fields
pub fn parse_price(input: &str) -> IResult<&str, f32> {
    map_res(digit1, |price: &str| f32::from_str(price))(input)
}
