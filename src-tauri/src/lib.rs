use proc_macro::TokenStream;

#[proc_macro]
pub fn square(input: TokenStream) -> TokenStream {
    let string = input.to_string();

    if string.len() == 2 {
        let file = string.chars().nth(0).unwrap();
        let rank = string.chars().nth(1).unwrap();

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
        let mut file: String = string.split_whitespace().nth(0).unwrap().into();
        let mut rank: String = string.split_whitespace().nth(1).unwrap().into();

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
