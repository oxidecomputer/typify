// Copyright 2023 Oxide Computer Company

use std::collections::BTreeMap;

use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};
use serde_json::Value;

use crate::RefKey;

pub(crate) fn schema_value_validate(
    schema: &Schema,
    value: &Value,
    defs: &BTreeMap<RefKey, Schema>,
) -> Result<(), String> {
    match schema {
        Schema::Bool(false) => Err("never schema".to_string()),
        Schema::Bool(true) => Ok(()),
        Schema::Object(object) => schema_object_value_validate(object, value, defs),
    }
}

fn schema_object_value_validate(
    object: &SchemaObject,
    value: &Value,
    _defs: &BTreeMap<RefKey, Schema>,
) -> Result<(), String> {
    if let Some(const_value) = &object.const_value {
        if value != const_value {
            return Err(format!(
                "{} does not match the const value {}",
                value, const_value,
            ));
        }
    }
    if let Some(enum_values) = &object.enum_values {
        if !enum_values.contains(value) {
            return Err(format!(
                "{} does not match the enum values {}",
                value,
                serde_json::to_string(enum_values).unwrap(),
            ));
        }
    }

    let SchemaObject { instance_type, .. } = object;

    schema_object_value_validate_instance_type(instance_type.as_ref(), value)?;

    Ok(())
}

fn schema_object_value_validate_instance_type(
    instance_type: Option<&SingleOrVec<InstanceType>>,
    value: &Value,
) -> Result<(), String> {
    match instance_type {
        None => Ok(()),
        Some(SingleOrVec::Single(it)) => check_instance(it, value),
        Some(SingleOrVec::Vec(its)) => its
            .iter()
            .any(|it| check_instance(it, value).is_ok())
            .then_some(())
            .ok_or_else(|| "no valid instance type".to_string()),
    }
}

fn check_instance(it: &InstanceType, value: &Value) -> Result<(), String> {
    match it {
        InstanceType::Null => value
            .is_null()
            .then_some(())
            .ok_or_else(|| "not null".to_string()),
        InstanceType::Boolean => value
            .is_boolean()
            .then_some(())
            .ok_or_else(|| "not boolean".to_string()),
        InstanceType::Object => value
            .is_object()
            .then_some(())
            .ok_or_else(|| "not object".to_string()),
        InstanceType::Array => value
            .is_array()
            .then_some(())
            .ok_or_else(|| "not array".to_string()),
        InstanceType::Number => value
            .is_number()
            .then_some(())
            .ok_or_else(|| "not number".to_string()),
        InstanceType::String => value
            .is_string()
            .then_some(())
            .ok_or_else(|| "not string".to_string()),
        InstanceType::Integer => (value.is_i64() || value.is_u64())
            .then_some(())
            .ok_or_else(|| "not integer".to_string()),
    }
}
