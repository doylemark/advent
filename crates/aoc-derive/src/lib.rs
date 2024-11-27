use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenTree};
use quote::quote;

fn build_idents(days: i8) -> Vec<proc_macro2::Ident> {
    let mut parts = vec![];
    for day in 1..=days {
        let ident = Ident::new(&["Day", &day.to_string()].join(""), Span::call_site());
        parts.push(ident);
    }
    return parts;
}

fn get_year(input: TokenTree) -> proc_macro2::Ident {
    match input {
        TokenTree::Literal(lit) => {
            return Ident::new(&["Year20", &lit.to_string()].join(""), Span::call_site())
        }
        _ => panic!("Expected literal"),
    }
}

fn get_day(input: TokenTree) -> i8 {
    match input {
        TokenTree::Literal(lit) => lit.to_string().parse().unwrap(),
        _ => panic!("Expected literal"),
    }
}

#[proc_macro_attribute]
pub fn year(attributes: TokenStream, item: TokenStream) -> TokenStream {
    let attr_input: proc_macro2::TokenStream = attributes.into();
    let mut attrs = attr_input.into_iter();
    let cur_year = get_year(attrs.next().expect("expected (year, days completed)"));
    // skip comma in #[year(2023, 1)], just want to match the <year> and <n> ident
    attrs.next();
    let days_completed = get_day(attrs.next().expect("expected (year, days completed)"));

    let idents = build_idents(days_completed);

    let input: proc_macro2::TokenStream = item.into();

    let mut traits = vec![];
    let mut match_arms = vec![];

    for (i, ident) in idents.into_iter().enumerate() {
        let i: i8 = (i + 1).try_into().unwrap();
        traits.push(quote! {
            pub trait #ident {
                fn part1(input: String) -> impl std::fmt::Display;
                fn part2(input: String) -> impl std::fmt::Display;
            }
        });

        if i <= days_completed {
            match_arms.push(quote! {
                 #i => {
                    match part {
                         1 => <#cur_year as #ident>::part1(input).to_string(),
                         2 => <#cur_year as #ident>::part2(input).to_string(),
                         _ => panic!("part not implemented"),
                    }
                }
            });
        }
    }

    let toks = quote! {
        #input
        pub struct #cur_year;

        #(#traits)*

        pub fn run(day: i8, part: i8, input: String) -> String {
            match day {
                #(#match_arms)*
                _ => panic!("day not implemented"),
            }
        }
    };

    toks.into()
}
