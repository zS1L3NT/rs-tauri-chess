use proc_macro::TokenStream;

#[proc_macro]
pub fn square(input: TokenStream) -> TokenStream {
    let string = input.to_string();

    if string.len() == 2 {
        let mut chars = string.chars();
        let file = chars.next().unwrap();
        let rank = chars.next().unwrap();

        format!(
            r#"crate::engine::Square {{
				file: crate::engine::File::{},
				rank: crate::engine::Rank::_{}
			}}"#,
            file, rank
        )
        .parse()
        .unwrap()
    } else {
        let mut chars = string.split_whitespace();
        let mut file: String = chars.next().unwrap().into();
        let mut rank: String = chars.next().unwrap().into();

        if file.len() == 1 {
            file = format!("crate::engine::File::{}", file);
        }

        if rank.len() == 1 {
            rank = format!("crate::engine::Rank::_{}", rank);
        }

        format!(
            r#"crate::engine::Square {{
				file: {},
				rank: {},
			}}"#,
            file, rank
        )
        .parse()
        .unwrap()
    }
}
