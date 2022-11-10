#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PuzzleAnswer(pub Option<String>, pub Option<String>);

pub trait AsAnswer {
    fn as_answer(self) -> Option<String>;
}

macro_rules! to_string_conv {
    ($ty:ty) => {
        impl AsAnswer for $ty {
            fn as_answer(self) -> Option<String> {
                Some(self.to_string())
            }
        }
        impl AsAnswer for Option<$ty> {
            fn as_answer(self) -> Option<String> {
                self.map(|x| x.to_string())
            }
        }
        impl Into<PuzzleAnswer> for $ty {
            fn into(self) -> PuzzleAnswer {
                PuzzleAnswer(self.as_answer(), None)
            }
        }
    };
}

to_string_conv!(usize);
to_string_conv!(u8);
to_string_conv!(i8);
to_string_conv!(u16);
to_string_conv!(i16);
to_string_conv!(u32);
to_string_conv!(i32);
to_string_conv!(u64);
to_string_conv!(i64);
to_string_conv!(u128);
to_string_conv!(i128);
to_string_conv!(&str);
to_string_conv!(String);

impl<T: AsAnswer, J: AsAnswer> Into<PuzzleAnswer> for (T, J) {
    fn into(self) -> PuzzleAnswer {
        let (a, b) = self;
        PuzzleAnswer(a.as_answer(), b.as_answer())
    }
}
