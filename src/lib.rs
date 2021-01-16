
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign};


#[derive(Clone, Debug)]
pub struct Version {
    vector: Vec<u64>
}

impl Version {
    /// Create new instance from vector
    ///
    /// ## Example
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let vector = vec![1, 14, 3];
    ///
    /// let version = Version::new(vector.clone());
    ///
    /// assert_eq!(version.to_vector(), vector);
    /// ```
    pub fn new(vector: Vec<u64>) -> Self {
        Self { vector }
    }

    /// Create new instance using a custom spliter callback function or closure
    ///
    /// ## Example with Closure
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let version = Version::from_split_callback("1.14.3", |c: char| !c.is_numeric());
    ///
    /// assert_eq!(version.to_vector(), vec![1, 14, 3]);
    /// ```
    ///
    /// ## Example with Function
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// fn custom_spliter(c: char) -> bool {
    ///     !c.is_numeric()
    /// }
    ///
    /// let version = Version::from_split_callback("1.14.3", &custom_spliter);
    ///
    /// assert_eq!(version.to_vector(), vec![1, 14, 3]);
    /// ```
    pub fn from_split_callback<F>(input: &str, callback: F) -> Self
            where F: Fn(char) -> bool {
        Self {
            vector: input.split(|c: char| callback(c))
                         .map(|c| c.parse::<u64>().unwrap())
                         .collect(),
        }
    }

    /// Convert `&str` to vector of unsigned-integers
    ///
    /// ## Example
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let version = Version::from_str("1.14.3");
    ///
    /// assert_eq!(version.to_vector(), vec![1, 14, 3]);
    /// ```
    pub fn from_str(input: &str) -> Self {
        Self {
            vector: input.split(|c: char| !c.is_numeric())
                         .map(|c| c.parse::<u64>().unwrap())
                         .collect(),
        }
    }

    /// Save vector of unsigned-integers to new instance of Version
    ///
    /// ## Example
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let vector = vec![1, 14, 3];
    /// let version = Version::from_vec(vector.clone());
    ///
    /// assert_eq!(version.to_vector(), vector);
    /// ```
    pub fn from_vec(vector: Vec<u64>) -> Self {
        Self { vector }
    }

    /// Passes ownership of vector within Version instance
    ///
    /// ## Example
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let version = Version::from_str("1.14.3");
    ///
    /// assert_eq!(version.to_vector(), vec![1, 14, 3]);
    /// ```
    pub fn to_vector(self) -> Vec<u64> {
        self.vector
    }
}


/// Implement equal/not-equal (`==`, `!=`) operators
impl PartialEq for Version {
    /// ## Example Equality
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let version_one = Version::from_str("1.14.3");
    /// let version_two = Version::from_str("1.14.3");
    ///
    /// assert!(version_one == version_two);
    /// assert_eq!(version_one, version_two);
    /// ```
    ///
    /// ## Example Inequality
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let version_one = Version::from_str("1.14");
    /// let version_two = Version::from_str("1.14.2");
    ///
    /// assert!(version_one != version_two);
    /// assert_ne!(version_one, version_two);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.vector == other.vector
    }
}

/// See: `PartialEq` implementation
impl Eq for Version {}


/// Implement greater/less-than (`<`, `>`) and operators
impl Ord for Version {
    /// ## Example Greater Than
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let version_one = Version::from_str("1.14.3");
    /// let version_two = Version::from_str("1.14.2");
    ///
    /// assert!(version_one > version_two);
    /// assert_eq!(version_two > version_one, false);
    /// ```
    ///
    /// ## Example Less Than
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let version_one = Version::from_str("1.14.3");
    /// let version_two = Version::from_str("1.14");
    ///
    /// assert!(version_two < version_one);
    /// assert_eq!(version_one < version_two, false);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.vector.iter().cmp(other.vector.iter())
    }
}


impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


/// Implement addition (`+`) operator
impl Add for Version {
    type Output = Self;

    /// ## Example
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let version_one = Version::from_str("1.14.3");
    /// let version_two = Version::from_str("1.14.2");
    ///
    /// let version_new = version_one + version_two;
    ///
    /// assert!(version_new.to_vector() == vec![2, 28, 5]);
    /// ```
    fn add(self, other: Self) -> Self {
        let mut vector: Vec<u64> = vec![];
        let mut other_iter = other.vector.iter();
        let mut self_iter = self.vector.iter();
        loop {
            let self_next = self_iter.next();
            let other_next = other_iter.next();
            match (self_next, other_next) {
                (Some(self_value), Some(other_value)) => {
                    vector.push(self_value + other_value);
                },
                (Some(self_value), _) => {
                    vector.push(self_value + 0);
                },
                (_, Some(other_value)) => {
                    vector.push(0 + other_value);
                },
                (_, _) => {
                    break;
                },
            };
        }
        Self { vector }
    }
}


/// Implement add assignment (`+=`) operator
impl AddAssign for Version {
    /// ## Example
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let mut version_one = Version::from_str("1.14.3");
    /// let version_two = Version::from_str("1.14.2");
    ///
    /// version_one += version_two;
    ///
    /// assert!(version_one.to_vector() == vec![2, 28, 5]);
    /// ```
    fn add_assign(&mut self, other: Self) {
        let mut vector: Vec<u64> = vec![];
        let mut other_iter = other.vector.iter();
        let mut self_iter = self.vector.iter();
        loop {
            let self_next = self_iter.next();
            let other_next = other_iter.next();
            match (self_next, other_next) {
                (Some(self_value), Some(other_value)) => {
                    vector.push(self_value + other_value);
                },
                (Some(self_value), _) => {
                    vector.push(self_value + 0);
                },
                (_, Some(other_value)) => {
                    vector.push(0 + other_value);
                },
                (_, _) => {
                    break;
                },
            };
        }
        *self = Self { vector };
    }
}


/// Implement subtraction (`-`) operator
impl Sub for Version {
    type Output = Self;

    /// ## Example
    ///
    /// ```rust
    /// use version_operators::Version;
    /// 
    /// let version_one = Version::from_str("1.14.3");
    /// let version_two = Version::from_str("1.14.2");
    ///
    /// let version_new = version_one - version_two;
    ///
    /// assert!(version_new.to_vector() == vec![0, 0, 1]);
    /// ```
    fn sub(self, other: Self) -> Self {
        let mut vector: Vec<u64> = vec![];
        let mut other_iter = other.vector.iter();
        let mut self_iter = self.vector.iter();
        loop {
            let self_next = self_iter.next();
            let other_next = other_iter.next();
            match (self_next, other_next) {
                (Some(self_value), Some(other_value)) => {
                    vector.push(self_value - other_value);
                },
                (Some(self_value), _) => {
                    vector.push(self_value - 0);
                },
                (_, Some(other_value)) => {
                    vector.push(0 - other_value);
                },
                (_, _) => {
                    break;
                },
            };
        }
        Self { vector }
    }
}


/// Implement subtract assignment (`-=`) operator
impl SubAssign for Version {
    /// ## Example
    ///
    /// ```rust
    /// use version_operators::Version;
    ///
    /// let mut version = Version::from_str("1.14.3");
    ///
    /// version -= Version::from_str("1.14.2");
    ///
    /// assert!(version.to_vector() == vec![0, 0, 1]);
    /// ```
    fn sub_assign(&mut self, other: Self) {
        let mut vector: Vec<u64> = vec![];
        let mut other_iter = other.vector.iter();
        let mut self_iter = self.vector.iter();
        loop {
            let self_next = self_iter.next();
            let other_next = other_iter.next();
            match (self_next, other_next) {
                (Some(self_value), Some(other_value)) => {
                    vector.push(self_value - other_value);
                },
                (Some(self_value), _) => {
                    vector.push(self_value - 0);
                },
                (_, Some(other_value)) => {
                    vector.push(0 - other_value);
                },
                (_, _) => {
                    break;
                },
            };
        }
        *self = Self { vector };
    }
}

