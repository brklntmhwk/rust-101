/* How to make documentation comments */
/* the below comments will be rendered as HTML code and you can see it by cargo doc --open */
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
/// Adds right to left
///
/// # Examples
/// by running cargo test, the below test is also executed
/// ```
/// let answer = my_crate::add(1, 2);
/// assert_eq!(3, answer);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// //! # Art
// //!
// //! A library for modeling artistic concepts.

// pub use self::kinds::PrimaryColor;
// pub use self::kinds::SecondaryColor;
// pub use self::utils::mix;
// // re-exports: a way to make the documentation more user-friendly by making them visible from the top page of the doc
// // also this is ok
// // use art::mix;
// // use art::PrimaryColor;

// pub mod kinds {
//     /// The primary colors according to the RYB color model.
//     pub enum PrimaryColor {
//         Red,
//         Yellow,
//         Blue,
//     }

//     /// The secondary colors according to the RYB color model.
//     pub enum SecondaryColor {
//         Orange,
//         Green,
//         Purple,
//     }
// }

// pub mod utils {
//     use crate::kinds::*;

//     /// Combines two primary colors in equal amounts to create
//     /// a secondary color.
//     pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
//     }
// }
