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

    let input: proc_macro2::TokenStream = item.into();

    let mut attrs = attr_input.into_iter();

    let y = get_year(attrs.next().expect("expected (year, days completed)"));
    attrs.next();
    let days_completed = get_day(attrs.next().expect("expected (year, days completed)"));

    let idents = build_idents(days_completed);

    let mut traits = vec![];
    let mut match_arms = vec![];

    for (i, ident) in idents.into_iter().enumerate() {
        let i: i8 = (i + 1).try_into().unwrap();
        traits.push(quote! {
            pub trait #ident {
                fn part1(input: String) -> String;
                fn part2(input: String) -> String;
            }
        });

        if i <= days_completed {
            match_arms.push(quote! {
                 #i => {
                    match part {
                         1 => <#y as #ident>::part1(input),
                         2 => <#y as #ident>::part2(input),
                         _ => panic!("part not implemented"),
                    }
                }
            });
        }
    }

    let toks = quote! {
        #input
        pub struct #y;

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
