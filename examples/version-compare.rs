
use std::env;

use version_operators::Version;


/// Parses command-line arguments into version comparison(s)
///
/// ## Example
///
/// ```bash
/// cargo run --example compare-args '1.14.3' '-lt' '1.14.4'
/// #> true
/// ```
fn main() {
    let mut iter = env::args().skip(1);
    let mut version_args: Vec<String> = vec![];
    loop {
        if version_args.len() < 3 {
            let arg = iter.next();
            match arg {
                Some(value) => version_args.push(value),
                _ => break,
            }
        } else {
            let left = Version::from_str(&version_args[0]);
            let comparator: &str = &version_args[1].to_owned();
            let right = Version::from_str(&version_args[2]);
            let result = match comparator {
                "==" | "-eq" => left == right,
                "!=" | "-ne" => left != right,
                ">=" | "-ge" => left >= right,
                ">" | "-gt" => left > right,
                "<=" | "-le" => left <= right,
                "<" | "-lt" => left < right,
                _ => panic!("Unrecognized operator! Try '==' or '!=' or '-lt' or '-gt'"),
            };
            println!("{:?}", result);
            version_args = vec![];
        }
    }
}

