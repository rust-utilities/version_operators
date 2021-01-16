
use version_operators::Version;


#[test]
fn add_from_str() {
    let version = Version::from_str("1.14.3") + Version::from_str("1.14.2");
    assert_eq!(version.to_vector(), vec![2, 28, 5]);

    let version = Version::from_str("1.14") + Version::from_str("1.14.2");
    assert_eq!(version.to_vector(), vec![2, 28, 2]);
}


#[test]
fn add_assign_from_str() {
    let mut version = Version::from_str("1.14.3");
    version += Version::from_str("1.14.2");
    assert_eq!(version.to_vector(), vec![2, 28, 5]);
}

