use proc_macro::TokenStream;

#[proc_macro]
pub fn square(input: TokenStream) -> TokenStream {
    let string = input.to_string();
    let file = string.chars().nth(0).unwrap();
    let rank = string.chars().nth(1).unwrap();

    format!(
        r#"crate::engine::square::Square {{
		file: crate::engine::square::File::{},
		rank: crate::engine::square::Rank::_{}
	}}"#,
        file, rank
    )
    .parse()
    .unwrap()
}
