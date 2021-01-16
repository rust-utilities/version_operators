
use version_operators::Version;


#[test]
fn lt_from_str() {
    let version_one = Version::from_str("1.14.3");
    let version_two = Version::from_str("1.14");
    assert!(version_two < version_one);
    assert_eq!(version_one < version_two, false);
}


#[test]
fn le_from_str() {
    let version_one = Version::from_str("1.14.3");
    let version_two = Version::from_str("1.14");
    assert!(version_two < version_one);
    assert_eq!(version_one < version_two, false);
}

