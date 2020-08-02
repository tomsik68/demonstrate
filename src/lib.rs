//! Declarative testing framework

extern crate proc_macro;

use crate::block::Root;
use crate::generate::Generate;

mod block;
mod generate;

/// Allows for tests to be defined in a declarative manner, hiding repetitive code before
/// generation.
///
/// `describe`/`context` blocks define a scope such as a `mod` block and can be nested as such.
/// ```
/// # use demonstrate::demonstrate;
/// demonstrate! {
///     describe outer {
///         describe inner {}
///     }
/// }
/// ```
/// This is generated into:
/// ```
/// #[cfg(test)]
/// mod outer {
///     mod inner {}
/// }
/// ```
///
/// <br />
/// `it`/`test` blocks define a unit test.
/// ```
/// # use demonstrate::demonstrate;
/// demonstrate! {
///     describe tests {
///         it asserts {
///             assert!(true)
///         }
///     }
/// }
/// ```
/// This is generated into:
/// ```
/// #[cfg(test)]
/// mod tests {
///     #[test]
///     fn asserts() {
///         assert!(true)
///     }
/// }
/// ```
///
/// <br />
/// `before` and `after` blocks prevent shared starting and ending sequences of code from being
/// written for each test within a the `describe`/`context` block it is contained in and each
/// nested `describe`/`context` block.
/// ```
/// # use demonstrate::demonstrate;
/// demonstrate! {
///     describe tests {
///         before {
///             let one = 1;
///         }
///
///         it one {
///             assert_eq!(one, 1)
///         }
///
///         it zero {
///             assert_eq!(one - 1, 0)
///         }
///
///         describe nested {
///             before {
///                 let two = 2;
///             }
///
///             it two {
///                 assert_eq!(one + 1, two)
///             }
///         }
///     }
/// }
/// ```
/// This is generated into:
/// ```
/// #[cfg(test)]
/// mod tests {
///     #[test]
///     fn one() {
///         let one = 1;
///         assert_eq!(one, 1)
///     }
///
///     #[test]
///     fn zero() {
///         let one = 1;
///         assert_eq!(one - 1, 1)
///     }
///
///     mod nested {
///         #[test]
///         fn two() {
///             let one = 1;
///             let two = 2;
///             assert_eq!(one + 1, two)
///         }
///     }
/// }
/// ```
///
/// <br />
/// Outer attributes, returning result types, and async tokens are all valid for `it`/`test` blocks, and can
/// be applied to `describe`/`context` blocks as well which will affect all descendant tests.
/// ```
/// # use demonstrate::demonstrate;
/// demonstrate! {
///     describe returnable -> Result<(), &'static str> {
///         it succeeds {
///             Ok(())
///         }
///
///         #[should_panic]
///         it fails {
///             Err("I failed!")
///         }
///     }
/// }
/// ```
/// This is generated into:
/// ```
/// #[cfg(test)]
/// mod returnable {
///     #[test]
///     fn succeeds() -> Result<(), &'static str> {
///         Ok(())
///     }
///
///     #[test]
///     #[should_panic]
///     fn fails() -> Result<(), &'static str> {
///         Err("I failed!")
///     }
/// }
/// ```
#[proc_macro]
pub fn demonstrate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let root = syn::parse2::<Root>(input).unwrap();

    proc_macro::TokenStream::from(root.generate(None))
}
