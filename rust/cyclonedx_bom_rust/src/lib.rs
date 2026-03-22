use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

use jsonschema::{Resource, Validator};
use once_cell::sync::Lazy;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use serde_json::Value;

const SPDX_SCHEMA_URI: &str = "http://cyclonedx.org/schema/spdx.SNAPSHOT.schema.json";
const CRYPTOGRAPHY_DEFS_SCHEMA_URI: &str = "http://cyclonedx.org/schema/cryptography-defs.SNAPSHOT.schema.json";
const JSF_SCHEMA_URI: &str = "http://cyclonedx.org/schema/jsf-0.82.SNAPSHOT.schema.json";

static VALIDATORS: Lazy<Mutex<HashMap<String, Arc<Validator>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[pyfunction]
fn validate_bom_json(
    instance_json: &str,
    bom_schema_path: &str,
    spdx_schema_path: &str,
    cryptography_defs_schema_path: &str,
    jsf_schema_path: &str,
) -> PyResult<Option<String>> {
    let validator = get_validator(
        bom_schema_path,
        spdx_schema_path,
        cryptography_defs_schema_path,
        jsf_schema_path,
    )?;
    let instance: Value = serde_json::from_str(instance_json)
        .map_err(|e| PyValueError::new_err(format!("invalid JSON instance: {e}")))?;
    if validator.is_valid(&instance) {
        return Ok(None);
    }
    let first_error = validator
        .iter_errors(&instance)
        .next()
        .expect("invalid instance has at least one error");
    Ok(Some(format_error(first_error)))
}

fn get_validator(
    bom_schema_path: &str,
    spdx_schema_path: &str,
    cryptography_defs_schema_path: &str,
    jsf_schema_path: &str,
) -> PyResult<Arc<Validator>> {
    let key = format!(
        "{bom_schema_path}\n{spdx_schema_path}\n{cryptography_defs_schema_path}\n{jsf_schema_path}"
    );
    if let Ok(cache) = VALIDATORS.lock() {
        if let Some(existing) = cache.get(&key) {
            return Ok(existing.clone());
        }
    }

    let validator = build_validator(
        bom_schema_path,
        spdx_schema_path,
        cryptography_defs_schema_path,
        jsf_schema_path,
    )
    .map_err(PyRuntimeError::new_err)?;
    let validator = Arc::new(validator);
    if let Ok(mut cache) = VALIDATORS.lock() {
        cache.insert(key, validator.clone());
    }
    Ok(validator)
}

fn build_validator(
    bom_schema_path: &str,
    spdx_schema_path: &str,
    cryptography_defs_schema_path: &str,
    jsf_schema_path: &str,
) -> Result<Validator, String> {
    let bom_schema = read_json_file(bom_schema_path)?;
    let spdx_schema = read_json_file(spdx_schema_path)?;
    let cryptography_defs_schema = read_json_file(cryptography_defs_schema_path)?;
    let jsf_schema = read_json_file(jsf_schema_path)?;

    jsonschema::options()
        .with_resources(
            [
                (SPDX_SCHEMA_URI, Resource::from_contents(spdx_schema)),
                (
                    CRYPTOGRAPHY_DEFS_SCHEMA_URI,
                    Resource::from_contents(cryptography_defs_schema),
                ),
                (JSF_SCHEMA_URI, Resource::from_contents(jsf_schema)),
            ]
            .into_iter(),
        )
        .should_validate_formats(true)
        .build(&bom_schema)
        .map_err(|e| format!("failed to build schema validator: {e}"))
}

fn read_json_file(path: &str) -> Result<Value, String> {
    let raw = fs::read(path).map_err(|e| format!("failed reading {path}: {e}"))?;
    serde_json::from_slice(&raw).map_err(|e| format!("failed parsing {path}: {e}"))
}

fn format_error(error: jsonschema::ValidationError<'_>) -> String {
    format!(
        "{error} (instance_path={}, schema_path={})",
        error.instance_path(),
        error.schema_path()
    )
}

#[pymodule]
fn cyclonedx_bom_rust(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(validate_bom_json, module)?)?;
    Ok(())
}

