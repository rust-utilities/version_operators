# Version Operators
[heading__top]:
  #version-operators
  "&#x2B06; Rust library for comparing and manipulating version numbers"


Rust library for comparing and manipulating version numbers


## [![Byte size of Version][badge__main__version__source_code]][version__main__source_code] [![Open Issues][badge__issues__version]][issues__version] [![Open Pull Requests][badge__pull_requests__version]][pull_requests__version] [![Latest commits][badge__commits__version__main]][commits__version__main] [![Build Status][badge_travis_ci]][build_travis_ci]


---


- [:arrow_up: Top of Document][heading__top]

- [:building_construction: Requirements][heading__requirements]

- [:zap: Quick Start][heading__quick_start]

- [&#x1F9F0; Usage][heading__usage]

- [&#x1F5D2; Notes][heading__notes]

- [:chart_with_upwards_trend: Contributing][heading__contributing]

  - [:trident: Forking][heading__forking]
  - [:currency_exchange: Sponsor][heading__sponsor]

- [:card_index: Attribution][heading__attribution]

- [:balance_scale: Licensing][heading__license]


---



## Requirements
[heading__requirements]:
  #requirements
  "&#x1F3D7; Prerequisites and/or dependencies that this project needs to function properly"


This repository requires [Rust][rust_home] language/compiler to build from source


As of last update to this ReadMe file, the recommended method of installing Rust is via the installer script...


```Bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


______


## Quick Start
[heading__quick_start]:
  #quick-start
  "&#9889; Perhaps as easy as one, 2.0,..."


This repository is a Rust library, define it as a dependency within a project `Cargo.toml` file...


**`Cargo.toml` (snip)**


```toml
[dependencies]
version_operators = "0.0.1"
```


> Check [Rust -- Doc -- Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) for details about defining dependencies.


Then include within a source file via `use` statement...


**`src/main.rs` (snip)**


```Rust
use version_operators::Version;
```


______


## Usage
[heading__usage]:
  #usage
  "&#x1F9F0; How to utilize this repository"


**`src/main.rs` (example)**


```Rust
use version_operators::Version;

fn main() {
    let version = Version::from_str("1.14.3");

    let expected = vec![1, 14, 3];

    assert_eq!(version.to_vector(), expected);
}
```


---


Check the `examples/` directory of this repository for more detailed examples of how this project may be utilized within other applications, use `cargo run --example <name>` to execute, eg...


**`examples/version-compare.rs`**


```Bash
cargo run --example version-compare '1.14.3' '-lt' '1.14.4'
#> true

cargo run --example version-compare '1.14.3' '==' '1.14.4'
#> false
```


**`examples/version-math.rs`**


```Bash
cargo run --example version-math '1.14.4' '-add' '1.14.3'
#> [2, 28, 7]

cargo run --example version-math '1.14.4' '-sub' '1.14.3'
#> [0, 0, 1]
```


______


## Notes
[heading__notes]:
  #notes
  "&#x1F5D2; Additional things to keep in mind when developing"


Until version `0.1.0`; methods, data structure properties, and implementation name(s) may have names and/or behavior change!


This repository may not be feature complete and/or fully functional, Pull Requests that add features or fix bugs are certainly welcomed.


Currently math **and** comparison operators start with the **most** significant bits of each version. This means that when operating on versions of unequal length it may be useful to zero-pad the shorter version...


```Rust
use version_operators::Version;

fn main() {
    let version_one = Version.from_str("1.14.3");
    let version_inc = Version.from_str("0.0.1");
    let new_version = version_one + version_inc;

    assert_eq!(new_version.to_vector(), vec![1, 14, 4]);
}
```


Check source code of `examples/version-increment.rs` for an example of targeting a portion of a version to increment...


```Bash
cargo run --example version-increment '1.14.3' '1'
#> [1, 15, 3]
```


______


## Contributing
[heading__contributing]:
  #contributing
  "&#x1F4C8; Options for contributing to version and rust-utilities"


Options for contributing to version and rust-utilities


---


### Forking
[heading__forking]:
  #forking
  "&#x1F531; Tips for forking version"


Start making a [Fork][version__fork_it] of this repository to an account that you have write permissions for.


- Add remote for fork URL. The URL syntax is _`git@github.com:<NAME>/<REPO>.git`_...


```Bash
cd ~/git/hub/rust-utilities/version_operators

git remote add fork git@github.com:<NAME>/version_operators.git
```


- Test changes via `cargo`...


```Bash
cargo test
```


- Commit your changes and push to your fork, eg. to fix an issue...


```Bash
cd ~/git/hub/rust-utilities/version_operators


git commit -F- <<'EOF'
:bug: Fixes #42 Issue


**Edits**


- `<SCRIPT-NAME>` script, fixes some bug reported in issue
EOF


git push fork main
```


> Note, the `-u` option may be used to set `fork` as the default remote, eg. _`git push -u fork main`_ however, this will also default the `fork` remote for pulling from too! Meaning that pulling updates from `origin` must be done explicitly, eg. _`git pull origin main`_


- Then on GitHub submit a Pull Request through the Web-UI, the URL syntax is _`https://github.com/<NAME>/<REPO>/pull/new/<BRANCH>`_


> Note; to decrease the chances of your Pull Request needing modifications before being accepted, please check the [dot-github](https://github.com/rust-utilities/.github) repository for detailed contributing guidelines.


---


### Sponsor
  [heading__sponsor]:
  #sponsor
  "&#x1F4B1; Methods for financially supporting rust-utilities that maintains version"


Thanks for even considering it!


Via Liberapay you may <sub>[![sponsor__shields_io__liberapay]][sponsor__link__liberapay]</sub> on a repeating basis.


Regardless of if you're able to financially support projects such as version that rust-utilities maintains, please consider sharing projects that are useful with others, because one of the goals of maintaining Open Source repositories is to provide value to the community.


______


## Attribution
[heading__attribution]:
  #attribution
  "&#x1F4C7; Resources that where helpful in building this project so far."


- [GitHub -- `github-utilities/make-readme`](https://github.com/github-utilities/make-readme)

- [Rust Doc -- Function `std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html)

- [Rust Doc -- Testing -- Unit testing](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)

- [Rust Doc -- Book Chapter 11 -- Test Organization -- Integration Tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests)

- [Rust Doc -- Documentation Tests](https://doc.rust-lang.org/rustdoc/documentation-tests.html)

- [Rust Doc -- Trait -- `Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)

- [Rust Doc -- Trait -- `Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html)

- [Rust Doc -- Book 1.3.0 -- Closures -- Taking Closures as Arguments](https://doc.rust-lang.org/1.3.0/book/closures.html#taking-closures-as-arguments)

- [StackOverflow -- How do I run a projects example using Cargo?](https://stackoverflow.com/questions/54469463/)

- [StackOverflow -- How to skip the first items of an iterator in Rust?](https://stackoverflow.com/questions/38826633/)


______


## License
[heading__license]:
  #license
  "&#x2696; Legal side of Open Source"


```LICENSE
Rust library for comparing and manipulating version numbers
Copyright (C) 2021 S0AndS0

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```


For further details review full length version of [AGPL-3.0][branch__current__license] License.



[branch__current__license]:
  /LICENSE
  "&#x2696; Full length version of AGPL-3.0 License"


[badge__commits__version__main]:
  https://img.shields.io/github/last-commit/rust-utilities/version_operators/main.svg

[commits__version__main]:
  https://github.com/rust-utilities/version_operators/commits/main
  "&#x1F4DD; History of changes on this branch"


[version__community]:
  https://github.com/rust-utilities/version_operators/community
  "&#x1F331; Dedicated to functioning code"


[issues__version]:
  https://github.com/rust-utilities/version_operators/issues
  "&#x2622; Search for and _bump_ existing issues or open new issues for project maintainer to address."

[version__fork_it]:
  https://github.com/rust-utilities/version_operators/
  "&#x1F531; Fork it!"

[pull_requests__version]:
  https://github.com/rust-utilities/version_operators/pulls
  "&#x1F3D7; Pull Request friendly, though please check the Community guidelines"

[version__main__source_code]:
  https://github.com/rust-utilities/version_operators/
  "&#x2328; Project source!"

[badge__issues__version]:
  https://img.shields.io/github/issues/rust-utilities/version_operators.svg

[badge__pull_requests__version]:
  https://img.shields.io/github/issues-pr/rust-utilities/version_operators.svg

[badge__main__version__source_code]:
  https://img.shields.io/github/repo-size/rust-utilities/version_operators


[rust_home]:
  https://www.rust-lang.org/
  "Home page for Rust language"

[rust_github]:
  https://github.com/rust-lang
  "Source code for Rust on GitHub"

[sponsor__shields_io__liberapay]:
  https://img.shields.io/static/v1?logo=liberapay&label=Sponsor&message=rust-utilities

[sponsor__link__liberapay]:
  https://liberapay.com/rust-utilities
  "&#x1F4B1; Sponsor developments and projects that rust-utilities maintains via Liberapay"


[badge_travis_ci]:
  https://travis-ci.com/rust-utilities/version_operators.svg?branch=main

[build_travis_ci]:
  https://travis-ci.com/rust-utilities/version_operators

