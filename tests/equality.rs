
use version_operators::Version;


#[test]
fn equality_from_str() {
    let version_one = Version::from_str("1.14.3");
    let version_two = Version::from_str("1.14.3");
    assert_eq!(version_one == version_two, true);
    assert_eq!(version_one, version_two);
}


#[test]
fn inequality_from_str() {
    let version_one = Version::from_str("1.14.3");
    let version_two = Version::from_str("1.14.2");
    assert_eq!(version_one == version_two, false);
    assert_ne!(version_one, version_two);

    let version_one = Version::from_str("1.14");
    let version_two = Version::from_str("1.14.2");
    assert_eq!(version_one == version_two, false);
    assert_ne!(version_one, version_two);
}

