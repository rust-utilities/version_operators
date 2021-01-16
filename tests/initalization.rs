
use version_operators::Version;


/// Custom callback function for splitting version string
fn custom_spliter(c: char) -> bool {
    !c.is_numeric()
}


#[test]
fn from_str() {
    let expected = vec![1, 14, 3];

    let version = Version::from_str("1.14.3");
    assert_eq!(version.to_vector(), expected);
}


#[test]
fn from_split_callback() {
    let expected = vec![1, 14, 3];

    let version = Version::from_split_callback("1.14.3", |c: char| !c.is_numeric());
    assert_eq!(version.to_vector(), expected);

    let version = Version::from_split_callback("1.14.3", &custom_spliter);
    assert_eq!(version.to_vector(), expected);
}


#[test]
fn from_vec() {
    let expected = vec![1, 14, 3];

    let version = Version::from_vec(expected.clone());
    assert_eq!(version.to_vector(), expected);
}


#[test]
#[should_panic]
fn from_str_panic() {
    let version = Version::from_str("");
    println!("version -> {:?}", version);
}

