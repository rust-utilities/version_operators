

use version_operators::Version;


#[test]
fn gt_from_str() {
    let version_one = Version::from_str("1.14.3");
    let version_two = Version::from_str("1.14.2");
    assert!(version_one > version_two);
    assert_eq!(version_two > version_one, false);
}


#[test]
fn ge_from_str() {
    let version_one = Version::from_str("1.14.3");
    let version_two = Version::from_str("1.14.2");
    assert!(version_one > version_two);
    assert_eq!(version_two > version_one, false);
}

