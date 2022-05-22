//! Delimited list

use crate::stream::TokenStream;
use crate::{Parse, ParseError, ParseResult};

#[derive(Debug)]
pub struct Delimited<T: Parse<Output = T>, U: Parse<Output = U>> {
    pub pairs: Vec<Pair<T, U>>,
}

#[derive(Debug)]
pub struct Pair<T, U> {
    pub item: T,
    pub delimiter: Option<U>,
}

impl<T, U> Parse for Delimited<T, U>
where
    T: Parse<Output = T>,
    U: Parse<Output = U>,
{
    type Output = Self;

    fn parse(input: &mut TokenStream) -> ParseResult<Self::Output> {
        let mut pairs: Vec<Pair<T, U>> = vec![];

        loop {
            match T::parse(input) {
                Ok(item) => {
                    match U::parse(input) {
                        Ok(delim) => {
                            pairs.push(Pair {
                                item,
                                delimiter: Some(delim),
                            });
                        }
                        Err(ParseError::Token(_)) => {
                            // Stop parsing at unexpected token.
                            pairs.push(Pair { item, delimiter: None })
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                Err(ParseError::Token(_)) => break, // stop at unexpected token
                Err(err) => return Err(err),
            }
        }

        Ok(Delimited { pairs })
    }
}

impl<T, U> Parse for Pair<T, U>
where
    T: Parse<Output = T>,
    U: Parse<Output = U>,
{
    type Output = Self;

    fn parse(_input: &mut TokenStream) -> ParseResult<Self::Output> {
        todo!("Pair")
    }
}
