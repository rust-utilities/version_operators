
use version_operators::Version;


#[test]
fn to_vector() {
    let version = Version::from_str("1.14.3");
    let expected = vec![1, 14, 3];
    assert_eq!(version.to_vector(), expected);
}

