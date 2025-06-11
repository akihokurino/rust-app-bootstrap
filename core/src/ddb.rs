use crate::domain::time::LocalDateTime;
use crate::domain::Id;
use aws_sdk_dynamodb::operation::query::builders::QueryFluentBuilder;
use aws_sdk_dynamodb::types::{AttributeValue, KeysAndAttributes, Select};
use chrono::{Local, TimeZone};
use std::collections::{HashMap, VecDeque};
use std::iter;
use std::marker::PhantomData;
use std::str::FromStr;

pub mod condition;
pub mod cursor;
pub mod errors;
pub mod index;
pub mod types;

pub trait HasTableName {
    fn table_name() -> String;
}

pub trait HasTypeName {
    fn type_name() -> String;
}

#[derive(Clone, Debug)]
pub struct TableNameProvider {
    prefix: String,
}
impl TableNameProvider {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }

    pub fn get(&self, basename: &str) -> String {
        format!("{}{}", self.prefix, basename)
    }
}

#[derive(Clone, Debug)]
pub struct TableRepository<E> {
    cli: aws_sdk_dynamodb::Client,
    table_name_provider: TableNameProvider,
    _phantom: PhantomData<fn() -> E>,
}
impl<E> TableRepository<E> {
    pub fn new(cli: aws_sdk_dynamodb::Client, table_name_provider: TableNameProvider) -> Self {
        Self {
            cli,
            table_name_provider,
            _phantom: PhantomData,
        }
    }
}
impl<E: HasTableName> TableRepository<E> {
    fn table_name(&self) -> String {
        self.table_name_provider.get(E::table_name().as_str())
    }
}

pub trait ToAttrValue {
    fn into_attr(self) -> AttributeValue;
}
impl ToAttrValue for String {
    fn into_attr(self) -> AttributeValue {
        AttributeValue::S(self)
    }
}
impl ToAttrValue for &str {
    fn into_attr(self) -> AttributeValue {
        AttributeValue::S(self.to_string())
    }
}
impl ToAttrValue for Vec<String> {
    fn into_attr(self) -> AttributeValue {
        AttributeValue::L(self.into_iter().map(|v| v.into_attr()).collect())
    }
}
impl ToAttrValue for LocalDateTime {
    fn into_attr(self) -> AttributeValue {
        let v = self.timestamp_nanos_opt().unwrap();
        AttributeValue::N(v.to_string())
    }
}

pub trait FromAttrValue {
    fn to_s(self) -> Result<String, String>;
    fn to_s_list(self) -> Result<Vec<String>, String>;
    fn to_date_time(self) -> Result<LocalDateTime, String>;
}
impl FromAttrValue for AttributeValue {
    fn to_s(self) -> Result<String, String> {
        self.as_s()
            .map(|v| v.to_string())
            .map_err(|_| "cannot convert string".to_string())
    }

    fn to_s_list(self) -> Result<Vec<String>, String> {
        let v = self
            .as_l()
            .map_err(|_| "cannot convert string list".to_string())?
            .clone();
        v.into_iter()
            .map(|v| v.to_s())
            .collect::<Result<Vec<String>, String>>()
    }

    fn to_date_time(self) -> Result<LocalDateTime, String> {
        let v = self
            .as_n()
            .map_err(|_| "cannot convert timestamp".to_string())?
            .clone();
        Ok(Local.timestamp_nanos(i64::from_str(&v).map_err(|_| "invalid timestamp")?))
    }
}

impl<E: HasTypeName> Into<AttributeValue> for Id<E> {
    fn into(self) -> AttributeValue {
        AttributeValue::S(format!("{}#{}", E::type_name(), self.as_str()))
    }
}
impl<E: HasTypeName> TryFrom<AttributeValue> for Id<E> {
    type Error = String;

    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        let v = value.to_s()?;
        let v = v
            .strip_prefix(&format!("{}#", E::type_name()))
            .ok_or_else(|| "invalid id".to_string())?;
        Ok(Self::new(v))
    }
}
impl<E: HasTypeName> Id<E> {
    fn into_attr_map(self) -> HashMap<String, AttributeValue> {
        [("pk", Some(self.into())), ("sk", Some(anchor_attr_value()))]
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone().unwrap()))
            .collect()
    }
}

#[allow(unused)]
fn anchor_attr_value() -> AttributeValue {
    AttributeValue::S("#".into())
}

// https://docs.aws.amazon.com/ja_jp/amazondynamodb/latest/developerguide/Query.Pagination.html
// データ量によって分割した結果が返ってくるので、limitに達していないがhas_nextがtrueになることがある
#[allow(unused)]
async fn query<T>(
    q: QueryFluentBuilder,
    limit: Option<i32>,
    conv: impl Fn(HashMap<String, AttributeValue>) -> Result<T, String>,
) -> Result<Vec<T>, String> {
    let mut limit = limit;
    let mut q = q.set_limit(limit);

    let mut items: Vec<T> = vec![];
    while {
        let query_res = q.clone().send().await.unwrap();
        let mut partial_items = query_res
            .items
            .ok_or_else(|| "result items missing")?
            .into_iter()
            .map(|v| conv(v))
            .collect::<Result<Vec<_>, _>>()?;

        let has_next = query_res.last_evaluated_key.is_some();
        limit = limit.map(|limit| limit - partial_items.len() as i32);
        items.append(&mut partial_items);
        q = q
            .set_exclusive_start_key(query_res.last_evaluated_key)
            .set_limit(limit);
        limit.map(|v| v > 0).unwrap_or(true) && has_next
    } {}

    Ok(items)
}

// https://docs.aws.amazon.com/ja_jp/amazondynamodb/latest/developerguide/Query.Pagination.html
// データ量によって分割した結果が返ってくるので、whileでhas_nextを見る必要がある
#[allow(unused)]
async fn count(q: QueryFluentBuilder) -> Result<usize, String> {
    let mut q = q.select(Select::Count);

    let mut count: usize = 0;
    let mut has_next = true;
    while has_next {
        let query_res = q.clone().send().await.unwrap();
        count += query_res.count as usize;
        has_next = query_res.last_evaluated_key.is_some();
        q = q.set_exclusive_start_key(query_res.last_evaluated_key);
    }
    Ok(count)
}

#[allow(unused)]
async fn batch_get<T>(
    cli: &aws_sdk_dynamodb::Client,
    table_name: impl Into<String>,
    keys: &[HashMap<String, AttributeValue>],
    conv: impl Fn(HashMap<String, AttributeValue>) -> Result<T, String>,
) -> Result<Vec<T>, String> {
    let table_name = table_name.into();

    match keys.len() {
        0 => return Ok(vec![]),
        1 => {
            return cli
                .get_item()
                .table_name(table_name)
                .set_key(Some(keys[0].clone()))
                .send()
                .await
                .map_err(|e| e.to_string())?
                .item
                .map(&conv)
                .into_iter()
                .collect::<Result<Vec<_>, String>>();
        }
        _ => (),
    }

    const CHUNK_SIZE: usize = 100;
    let mut res = Vec::<T>::with_capacity(keys.len());
    let mut req_keys = VecDeque::from_iter(keys.into_iter().cloned());
    let mut unprocessed_keys = Vec::<HashMap<String, AttributeValue>>::with_capacity(CHUNK_SIZE);
    let mut next_keys = Vec::<HashMap<String, AttributeValue>>::with_capacity(CHUNK_SIZE);

    while {
        next_keys.truncate(0);
        next_keys.append(&mut unprocessed_keys);
        next_keys.extend(
            (0..(CHUNK_SIZE - next_keys.len()))
                .map(|_| req_keys.pop_front())
                .take_while(Option::is_some)
                .flatten(),
        );
        next_keys.len() > 0
    } {
        match next_keys.len() {
            1 => res.extend(
                cli.get_item()
                    .table_name(&table_name)
                    .set_key(Some(next_keys.pop().unwrap()))
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .item
                    .map(&conv)
                    .transpose()?
                    .into_iter(),
            ),
            _ => {
                let api_res = cli
                    .batch_get_item()
                    .set_request_items(Some(
                        iter::once((
                            table_name.clone(),
                            KeysAndAttributes::builder()
                                .set_keys(Some(next_keys.to_vec()))
                                .build()
                                .unwrap(),
                        ))
                        .collect(),
                    ))
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(mut responses) = api_res.responses {
                    if let Some(items) = responses.remove(&table_name) {
                        res.append(
                            items
                                .into_iter()
                                .map(&conv)
                                .collect::<Result<Vec<_>, String>>()?
                                .as_mut(),
                        )
                    }
                }

                if let Some(mut upks) = api_res.unprocessed_keys {
                    if let Some(KeysAndAttributes { mut keys, .. }) = upks.remove(&table_name) {
                        unprocessed_keys.append(&mut keys);
                    }
                }
            }
        };
    }

    Ok(res)
}
