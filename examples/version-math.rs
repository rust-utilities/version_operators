
use std::env;

use version_operators::Version;


/// Parses command-line arguments and returns vector from math operation(s)
///
/// ## Example
///
/// ```bash
/// cargo run --example version-math '1.14.3' '-add' '1.14.4'
/// #> [2, 28, 4]
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
            let operator: &str = &version_args[1].to_owned();
            let right = Version::from_str(&version_args[2]);
            let result = match operator {
                "+" | "-add" => left + right,
                "-" | "-sub" => left - right,
                _ => panic!("Unrecognized operator! Try '+'/'-add' or '-'/'-sub'"),
            };
            println!("{:?}", result.to_vector());
            version_args = vec![];
        }
    }
}

