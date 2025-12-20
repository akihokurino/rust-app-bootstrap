pub mod admin_user;
pub mod order;
pub mod types;
pub mod user;

use rand::random;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
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
impl<E> Hash for Id<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
impl<E> Display for Id<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.id, f)
    }
}
impl<E> AsRef<str> for Id<E> {
    fn as_ref(&self) -> &str {
        &self.id
    }
}
impl<E> Serialize for Id<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de, E> Deserialize<'de> for Id<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(String::deserialize(deserializer)?))
    }
}

fn generate_id_str() -> String {
    base_62::encode(&random::<[u8; 16]>())
}

pub trait IntoIdMap<T> {
    fn into_id_map(self) -> HashMap<Id<T>, T>;
}

impl<T, I> IntoIdMap<T> for I
where
    T: HasId<T>,
    I: IntoIterator<Item = T>,
{
    fn into_id_map(self) -> HashMap<Id<T>, T> {
        let mut map = HashMap::new();
        for item in self {
            map.insert(item.id().clone(), item);
        }
        map
    }
}
