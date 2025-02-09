//! Repetitions
//! Matchers can contain repetitions.
//! These allow a sequence of tokens to be matched.
//! These have the general form $ ( ... ) sep rep.
//! $ is a literal dollar token.
//! ( ... ) is the paren-grouped matcher being repeated.
//! sep is an optional separator token. It may not be a delimiter or one of the repetition operators. Common examples are , and ;.
//! rep is the required repeat operator (?,*,+)

macro_rules! vec_strs {
    (
        // match repetition separated by commas ending with repeat operator
        $($e:expr),*
    ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $e));
            )*
            v
        }
    };
}

pub fn repetitions() {
    let v = vec_strs!("a", "b", "c", "d", "e");
    println!("{:?}", v);
}
