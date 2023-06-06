extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn run_day(input: TokenStream) -> TokenStream {
    let mut s = String::new();
    let binding = input.to_string();
    let chunks = binding.split(',').map(|x| x.trim().split('-'));
    for mut chunk in chunks {
        let a = chunk.next().unwrap().trim().parse::<usize>().unwrap();
        let b = chunk.next().map(|x| x.trim().parse::<usize>().unwrap()).unwrap_or(a);
        for x in a..=b {
            let j = format!("day{:02}::run({});\n", x, x);
            s.push_str(j.as_str());
        }
    }
    s.parse().unwrap()
}

#[proc_macro]
pub fn import_day(input: TokenStream) -> TokenStream {
    let mut s = String::new();
    let binding = input.to_string();
    let chunks = binding.split(',').map(|x| x.trim().split('-'));
    for mut chunk in chunks {
        let a = chunk.next().unwrap().trim().parse::<usize>().unwrap();
        let b = chunk.next().map(|x| x.trim().parse::<usize>().unwrap()).unwrap_or(a);
        for x in a..=b {
            let j = format!("mod day{:02};\n", x);
            s.push_str(j.as_str());
        }
    }
    s.parse().unwrap()
}
