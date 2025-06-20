use crate::ddb::{HasTypeName, ToAttrValue};
use aws_sdk_dynamodb::types::{AttributeValue, ComparisonOperator, Condition};

#[allow(unused)]
fn condition_sk_type<T: HasTypeName>() -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::BeginsWith)
        .attribute_value_list(AttributeValue::S(format!("{}#", T::type_name())))
        .build()
        .unwrap()
}

#[allow(unused)]
fn condition_eq(v: AttributeValue) -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::Eq)
        .attribute_value_list(v)
        .build()
        .unwrap()
}

#[allow(unused)]
fn condition_gt(v: AttributeValue) -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::Gt)
        .attribute_value_list(v)
        .build()
        .unwrap()
}

#[allow(unused)]
fn condition_ge(v: AttributeValue) -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::Ge)
        .attribute_value_list(v)
        .build()
        .unwrap()
}

#[allow(unused)]
fn condition_lt(v: AttributeValue) -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::Lt)
        .attribute_value_list(v)
        .build()
        .unwrap()
}

#[allow(unused)]
fn condition_le(v: AttributeValue) -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::Le)
        .attribute_value_list(v)
        .build()
        .unwrap()
}

#[allow(unused)]
fn condition_contains(v: impl Into<String>) -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::Contains)
        .attribute_value_list(v.into().into_attr())
        .build()
        .unwrap()
}

#[allow(unused)]
fn condition_begins_with(v: impl Into<String>) -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::BeginsWith)
        .attribute_value_list(v.into().into_attr())
        .build()
        .unwrap()
}

#[allow(unused)]
fn condition_between(a: AttributeValue, b: AttributeValue) -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::Between)
        .attribute_value_list(a)
        .attribute_value_list(b)
        .build()
        .unwrap()
}
