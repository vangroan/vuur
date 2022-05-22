//! Delimited list

use crate::{stream::TokenStream, Parse, ParseResult};

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

    fn parse(_input: &mut TokenStream) -> ParseResult<Self::Output> {
        todo!("delimited")
    }
}
