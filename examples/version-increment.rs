
use std::env;

use version_operators::Version;


fn main() {
    let mut iter = env::args().skip(1);
    let mut version_args: Vec<String> = vec![];
    loop {
        if version_args.len() < 2 {
            let arg = iter.next();
            match arg {
                Some(value) => version_args.push(value),
                _ => break,
            }
        } else {
            let mut version = Version::from_str(&version_args[0]);
            let position: &u8 = &version_args[1].to_owned()
                                                .parse::<u8>()
                                                .unwrap();

            let mut vector: Vec<u64> = vec![];
            for _ in 0..position.to_owned() {
                vector.push(0);
            }
            vector.push(1);
            println!("{:?}", vector);

            version += Version::from_vec(vector);
            println!("{:?}", version.to_vector());

            version_args = vec![];
        }
    }
}

