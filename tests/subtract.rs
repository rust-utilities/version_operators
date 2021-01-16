
use version_operators::Version;


#[test]
fn subtract_from_str() {
    let version = Version::from_str("1.14.3") - Version::from_str("1.14.2");
    assert!(version.to_vector() == vec![0, 0, 1]);
}


#[test]
fn subtract_assign_from_str() {
    let mut version = Version::from_str("1.14.3");
    version -= Version::from_str("1.14.2");
    assert!(version.to_vector() == vec![0, 0, 1]);
}

