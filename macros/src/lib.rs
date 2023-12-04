extern crate proc_macro;

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitInt,
};

#[proc_macro]
pub fn mod_years(_: TokenStream) -> TokenStream {
    let years = get_years();

    TokenStream::from(
        years
            .iter()
            .map(|(year, _)| {
                let year = format_ident!("y{year}");
                quote! {
                    pub(crate) mod #year;
                }
            })
            .collect::<proc_macro2::TokenStream>(),
    )
}

struct ModDaysArgs {
    year: LitInt,
}

impl Parse for ModDaysArgs {
    fn parse(input: ParseStream) -> syn::Result<ModDaysArgs> {
        Ok(ModDaysArgs {
            year: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn mod_days(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ModDaysArgs);

    let year = input.year.to_string();
    let days = get_days(&PathBuf::from("src").join(format!("y{year}")));

    TokenStream::from(
        days.into_iter()
            .map(|day| {
                let day = format_ident!("day{day}");
                quote! {
                    pub(crate) mod #day;
                }
            })
            .collect::<proc_macro2::TokenStream>(),
    )
}

#[proc_macro]
pub fn solutions(_: TokenStream) -> TokenStream {
    let years = get_years();

    let solutions_count = years.iter().map(|(_, days)| days.len()).sum::<usize>();

    let solution_entries = years
        .iter()
        .flat_map(|(year, days)| {
            days.iter().map(move |day| {
                quote! {
                    Solution {
                        year: Year::new(#year),
                        day: Day::new(#day),
                    },
                }
            })
        })
        .collect::<Vec<_>>();

    let solution_runners = years
        .iter()
        .flat_map(|(year, days)| {
            days.iter().map(move |day| {
                let year_module = format_ident!("y{year}");
                let day_module = format_ident!("day{day}");
                quote! {
                    (#year, #day) => {
                        let result = crate::#year_module::#day_module::solve(input);
                        (result.0.to_string(), result.1.to_string())
                    }
                }
            })
        })
        .collect::<Vec<_>>();

    TokenStream::from(quote! {
        pub const SOLUTIONS: [Solution; #solutions_count] = [
            #(#solution_entries)*
        ];

        impl Solution {
            pub fn run(&self, input: &str) -> (String, String) {
                match (self.year.0, self.day.0) {
                    #(#solution_runners)*
                    _ => unreachable!(),
                }
            }
        }
    })
}

fn get_years() -> Vec<(i64, Vec<u8>)> {
    let mut years = Vec::new();
    let src_dir = PathBuf::from("src");
    for child in src_dir.read_dir().unwrap() {
        let child = child.unwrap();
        if !child.file_type().unwrap().is_dir() {
            continue;
        }
        let Some(year) = year_from_path(&child.file_name()) else {
            continue;
        };

        years.push((year, get_days(&child.path())));
    }
    years.sort();

    years
}

fn get_days(year_path: &Path) -> Vec<u8> {
    let mut days = Vec::new();
    for year_child in year_path.read_dir().unwrap() {
        let year_child = year_child.unwrap();
        if !year_child.file_type().unwrap().is_file() {
            continue;
        }
        let Some(day) = day_from_path(&year_child.file_name()) else {
            continue;
        };
        days.push(day);
    }

    days.sort();
    days
}

fn year_from_path(value: &OsStr) -> Option<i64> {
    let value = value.to_str().unwrap();
    if &value[..1] != "y" || !value[1..].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    Some(value[1..].parse().unwrap())
}

fn day_from_path(value: &OsStr) -> Option<u8> {
    let value = value.to_str().unwrap();
    if &value[..3] != "day" || &value[(value.len() - 3)..] != ".rs" {
        return None;
    }

    Some(value[3..(value.len() - 3)].parse().unwrap())
}
