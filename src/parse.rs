use crate::ast::Item;
use crate::tokenize::Tokens;

pub type ParseResult<T> = Result<T, &'static str>;

pub fn parse_items(tokens: &mut Tokens) -> ParseResult<Vec<Item>> {
    let mut items = Vec::new();
    
	while tokens.len() > 0 {
		items.push(Item::parse(tokens)?);
	}
	
	Ok(items)
}

pub trait Parse {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self>
	where Self: std::marker::Sized;
}
