macro_rules! impl_len_restricted_string_model {
    ($typ:ident, $display_name:literal, $min:literal, $max:literal) => {
        $crate::domain::string::impl_string_model!($typ);

        impl std::convert::TryFrom<String> for $typ {
            type Error = String;
            fn try_from(v: String) -> std::result::Result<Self, Self::Error> {
                use unicode_segmentation::UnicodeSegmentation;

                let len = v.graphemes(true).count();
                #[allow(unused_comparisons)]
                if len < $min {
                    return Err(concat!(
                        $display_name,
                        "は",
                        stringify!($min),
                        "文字以上である必要があります"
                    )
                    .into());
                }
                if len > $max {
                    return Err(concat!(
                        $display_name,
                        "は",
                        stringify!($max),
                        "文字以下である必要があります"
                    )
                    .into());
                }
                Ok(Self(v))
            }
        }
    };
}
pub(crate) use impl_len_restricted_string_model;

macro_rules! impl_string_model {
    ($typ:ident) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct $typ(String);
        impl std::fmt::Display for $typ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl std::convert::Into<String> for $typ {
            fn into(self) -> String {
                self.0
            }
        }
        impl std::convert::AsRef<String> for $typ {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
        impl crate::domain::string::FromUnchecked<String> for $typ {
            fn from_unchecked(value: String) -> Self {
                Self(value)
            }
        }
    };
}
pub(crate) use impl_string_model;

#[allow(unused)]
pub trait FromUnchecked<T> {
    fn from_unchecked(value: T) -> Self;
}