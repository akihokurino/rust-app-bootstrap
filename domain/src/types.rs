pub mod time;
pub mod user;

use rand::random;
use std::marker::PhantomData;

pub trait HasId<T = Self>: Sized {
    fn id(&self) -> &Id<T>;
}

#[derive(Debug, Ord, PartialOrd)]
pub struct Id<E> {
    id: String,
    _phantom: PhantomData<E>,
}
impl<E> Id<E> {
    pub fn new<I: Into<String>>(id: I) -> Self {
        Self {
            id: id.into(),
            _phantom: PhantomData,
        }
    }

    pub fn generate() -> Self {
        Self::new(generate_id_str())
    }

    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }
}
impl<E> PartialEq<Self> for Id<E> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
impl<E> Eq for Id<E> {}
impl<E> Clone for Id<E> {
    fn clone(&self) -> Self {
        self.id.clone().into()
    }
}
impl<E> From<String> for Id<E> {
    fn from(id: String) -> Self {
        Self::new(id)
    }
}
impl<E> From<&str> for Id<E> {
    fn from(id: &str) -> Self {
        Self::new(id.to_string())
    }
}
impl<E> Into<String> for Id<E> {
    fn into(self) -> String {
        self.id
    }
}

fn generate_id_str() -> String {
    base_62::encode(&random::<[u8; 16]>())
}

#[allow(unused)]
trait FromUnchecked<T> {
    fn from_unchecked(value: T) -> Self;
}

#[allow(unused)]
macro_rules! define_len_restricted_string_model {
    ($typ:ident, $display_name:literal, $min:literal, $max:literal) => {
        $crate::types::define_string_model!($typ);
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
#[allow(unused)]
use define_len_restricted_string_model;

#[allow(unused)]
macro_rules! define_string_model {
    ($typ:ident) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
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
        impl crate::types::FromUnchecked<String> for $typ {
            fn from_unchecked(value: String) -> Self {
                Self(value)
            }
        }
    };
}
#[allow(unused)]
use define_string_model;
