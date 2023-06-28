/* Declarative macros with macro_rules! for general meta-programming */

// this annotation makes the macro available whenever the crate in which it's defined is brought into scope
#[macro_export]
macro_rules! jeff_vec {
  // the $ sign means it's a macro var as opposed to a regular Rust var
  // the comma is a literal comma separator char and the asterisk specifies tha pattern matches zero or more of whatever precedes itself (e.g. vec![$x, $x, $x])
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
