//! Common types, utilities, and constants for African domain models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard parameter definition with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDef {
    /// Parameter name (used as key)
    pub name: String,
    /// Human-readable label
    pub label: String,
    /// Description
    pub description: String,
    /// Parameter type
    pub param_type: ParameterType,
    /// Default value
    pub default: serde_json::Value,
    /// Minimum value (for numeric types)
    pub min: Option<f64>,
    /// Maximum value (for numeric types)
    pub max: Option<f64>,
    /// Step increment (for numeric types)
    pub step: Option<f64>,
    /// Unit of measurement
    pub unit: Option<String>,
    /// Whether this parameter is required
    pub required: bool,
    /// Dependencies on other parameters
    pub depends_on: Option<Vec<String>>,
    /// Validation expression (e.g., "width > height")
    pub validation: Option<String>,
}

/// Parameter types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterType {
    Float,
    Integer,
    Boolean,
    String,
    Enum,
    Length,
    Angle,
    Ratio,
    Count,
}

/// Create a length parameter definition
pub fn length_param(
    name: &str,
    label: &str,
    description: &str,
    default_mm: f64,
    min_mm: Option<f64>,
    max_mm: Option<f64>,
) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: description.to_string(),
        param_type: ParameterType::Length,
        default: serde_json::json!(default_mm),
        min: min_mm,
        max: max_mm,
        step: Some(1.0),
        unit: Some("mm".to_string()),
        required: true,
        depends_on: None,
        validation: None,
    }
}

/// Create a float parameter definition
pub fn float_param(
    name: &str,
    label: &str,
    description: &str,
    default: f64,
    min: Option<f64>,
    max: Option<f64>,
    unit: Option<&str>,
) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: description.to_string(),
        param_type: ParameterType::Float,
        default: serde_json::json!(default),
        min,
        max,
        step: Some(0.1),
        unit: unit.map(|s| s.to_string()),
        required: true,
        depends_on: None,
        validation: None,
    }
}

/// Create an integer parameter definition
pub fn int_param(
    name: &str,
    label: &str,
    description: &str,
    default: i64,
    min: Option<i64>,
    max: Option<i64>,
) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: description.to_string(),
        param_type: ParameterType::Integer,
        default: serde_json::json!(default),
        min: min.map(|v| v as f64),
        max: max.map(|v| v as f64),
        step: Some(1.0),
        unit: None,
        required: true,
        depends_on: None,
        validation: None,
    }
}

/// Create a boolean parameter definition
pub fn bool_param(
    name: &str,
    label: &str,
    description: &str,
    default: bool,
) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: description.to_string(),
        param_type: ParameterType::Boolean,
        default: serde_json::json!(default),
        min: None,
        max: None,
        step: None,
        unit: None,
        required: false,
        depends_on: None,
        validation: None,
    }
}

/// Create an enum parameter definition
pub fn enum_param(
    name: &str,
    label: &str,
    description: &str,
    default: &str,
    options: Vec<&str>,
) -> ParameterDef {
    ParameterDef {
        name: name.to_string(),
        label: label.to_string(),
        description: description.to_string(),
        param_type: ParameterType::Enum,
        default: serde_json::json!(default),
        min: None,
        max: None,
        step: None,
        unit: None,
        required: true,
        depends_on: None,
        validation: None,
    }
}

/// Standard African context defaults by country
pub fn get_country_defaults(country: &str) -> Option<HashMap<String, serde_json::Value>> {
    let mut defaults = HashMap::new();

    match country.to_uppercase().as_str() {
        "KE" | "KENYA" => {
            defaults.insert("climate".to_string(), serde_json::json!("tropical_savanna"));
            defaults.insert("seismic_zone".to_string(), serde_json::json!(2));
            defaults.insert("materials".to_string(), serde_json::json!(["concrete", "steel", "corrugated_iron", "stone"]));
            defaults.insert("currency".to_string(), serde_json::json!("KES"));
        }
        "NG" | "NIGERIA" => {
            defaults.insert("climate".to_string(), serde_json::json!("tropical_monsoon"));
            defaults.insert("seismic_zone".to_string(), serde_json::json!(1));
            defaults.insert("materials".to_string(), serde_json::json!(["concrete", "cement_block", "steel", "burnt_brick"]));
            defaults.insert("currency".to_string(), serde_json::json!("NGN"));
        }
        "ZA" | "SOUTH_AFRICA" => {
            defaults.insert("climate".to_string(), serde_json::json!("mediterranean"));
            defaults.insert("seismic_zone".to_string(), serde_json::json!(1));
            defaults.insert("materials".to_string(), serde_json::json!(["concrete", "steel", "brick", "timber"]));
            defaults.insert("currency".to_string(), serde_json::json!("ZAR"));
        }
        "EG" | "EGYPT" => {
            defaults.insert("climate".to_string(), serde_json::json!("hot_desert"));
            defaults.insert("seismic_zone".to_string(), serde_json::json!(2));
            defaults.insert("materials".to_string(), serde_json::json!(["concrete", "steel", "stone", "brick"]));
            defaults.insert("currency".to_string(), serde_json::json!("EGP"));
        }
        "ET" | "ETHIOPIA" => {
            defaults.insert("climate".to_string(), serde_json::json!("subtropical_highland"));
            defaults.insert("seismic_zone".to_string(), serde_json::json!(3));
            defaults.insert("materials".to_string(), serde_json::json!(["stone", "concrete", "compressed_earth_block", "eucalyptus"]));
            defaults.insert("currency".to_string(), serde_json::json!("ETB"));
        }
        "GH" | "GHANA" => {
            defaults.insert("climate".to_string(), serde_json::json!("tropical_monsoon"));
            defaults.insert("seismic_zone".to_string(), serde_json::json!(1));
            defaults.insert("materials".to_string(), serde_json::json!(["concrete", "cement_block", "steel", "bamboo"]));
            defaults.insert("currency".to_string(), serde_json::json!("GHS"));
        }
        "UG" | "UGANDA" => {
            defaults.insert("climate".to_string(), serde_json::json!("tropical_savanna"));
            defaults.insert("seismic_zone".to_string(), serde_json::json!(2));
            defaults.insert("materials".to_string(), serde_json::json!(["interlocking_brick", "concrete", "steel", "timber", "rammed_earth"]));
            defaults.insert("currency".to_string(), serde_json::json!("UGX"));
        }
        "TZ" | "TANZANIA" => {
            defaults.insert("climate".to_string(), serde_json::json!("tropical_savanna"));
            defaults.insert("seismic_zone".to_string(), serde_json::json!(2));
            defaults.insert("materials".to_string(), serde_json::json!(["cement_block", "concrete", "steel", "corrugated_iron"]));
            defaults.insert("currency".to_string(), serde_json::json!("TZS"));
        }
        "RW" | "RWANDA" => {
            defaults.insert("climate".to_string(), serde_json::json!("subtropical_highland"));
            defaults.insert("seismic_zone".to_string(), serde_json::json!(2));
            defaults.insert("materials".to_string(), serde_json::json!(["compressed_earth_block", "stone", "concrete", "steel"]));
            defaults.insert("currency".to_string(), serde_json::json!("RWF"));
        }
        _ => return None,
    }

    Some(defaults)
}

/// Standard steel profile sizes commonly available in African markets
pub const STEEL_PROFILES: &[(&str, f64, f64, f64)] = &[
    // (name, width_mm, height_mm, thickness_mm)
    ("SHS 25x25x2", 25.0, 25.0, 2.0),
    ("SHS 40x40x3", 40.0, 40.0, 3.0),
    ("SHS 50x50x3", 50.0, 50.0, 3.0),
    ("SHS 60x60x4", 60.0, 60.0, 4.0),
    ("SHS 80x80x4", 80.0, 80.0, 4.0),
    ("SHS 100x100x5", 100.0, 100.0, 5.0),
    ("RHS 40x20x2", 40.0, 20.0, 2.0),
    ("RHS 60x40x3", 60.0, 40.0, 3.0),
    ("RHS 80x40x3", 80.0, 40.0, 3.0),
    ("RHS 100x50x4", 100.0, 50.0, 4.0),
    ("Angle 25x25x3", 25.0, 25.0, 3.0),
    ("Angle 40x40x4", 40.0, 40.0, 4.0),
    ("Angle 50x50x5", 50.0, 50.0, 5.0),
    ("Pipe 25NB", 33.7, 33.7, 3.2),
    ("Pipe 32NB", 42.4, 42.4, 3.2),
    ("Pipe 40NB", 48.3, 48.3, 3.2),
    ("Pipe 50NB", 60.3, 60.3, 3.6),
    ("Pipe 80NB", 88.9, 88.9, 4.0),
];

/// Standard timber sizes commonly available in African markets (mm)
pub const TIMBER_SIZES: &[(&str, f64, f64)] = &[
    // (name, width_mm, height_mm)
    ("38x38", 38.0, 38.0),
    ("50x50", 50.0, 50.0),
    ("50x75", 50.0, 75.0),
    ("50x100", 50.0, 100.0),
    ("50x150", 50.0, 150.0),
    ("75x75", 75.0, 75.0),
    ("100x100", 100.0, 100.0),
    ("150x150", 150.0, 150.0),
];

/// Standard concrete block sizes (mm)
pub const CONCRETE_BLOCKS: &[(&str, f64, f64, f64)] = &[
    // (name, length_mm, width_mm, height_mm)
    ("4 inch block", 400.0, 100.0, 200.0),
    ("6 inch block", 400.0, 150.0, 200.0),
    ("9 inch block", 400.0, 225.0, 200.0),
];

/// Standard corrugated iron sheet sizes (mm)
pub const CORRUGATED_SHEETS: &[(&str, f64, f64, f64)] = &[
    // (name, length_mm, width_mm, thickness_mm)
    ("8 ft sheet", 2440.0, 760.0, 0.4),
    ("10 ft sheet", 3050.0, 760.0, 0.4),
    ("12 ft sheet", 3660.0, 760.0, 0.4),
];

/// Validation helper for parameters
pub fn validate_params(
    params: &serde_json::Map<String, serde_json::Value>,
    definitions: &[ParameterDef],
) -> Result<(), String> {
    for def in definitions {
        let value = params.get(&def.name);

        if def.required && value.is_none() {
            return Err(format!("Required parameter '{}' is missing", def.name));
        }

        if let Some(val) = value {
            match def.param_type {
                ParameterType::Float | ParameterType::Length | ParameterType::Angle | ParameterType::Ratio => {
                    if let Some(num) = val.as_f64() {
                        if let Some(min) = def.min {
                            if num < min {
                                return Err(format!("Parameter '{}' must be >= {}", def.name, min));
                            }
                        }
                        if let Some(max) = def.max {
                            if num > max {
                                return Err(format!("Parameter '{}' must be <= {}", def.name, max));
                            }
                        }
                    } else {
                        return Err(format!("Parameter '{}' must be a number", def.name));
                    }
                }
                ParameterType::Integer | ParameterType::Count => {
                    if let Some(num) = val.as_i64() {
                        if let Some(min) = def.min {
                            if num as f64 < min {
                                return Err(format!("Parameter '{}' must be >= {}", def.name, min));
                            }
                        }
                        if let Some(max) = def.max {
                            if num as f64 > max {
                                return Err(format!("Parameter '{}' must be <= {}", def.name, max));
                            }
                        }
                    } else {
                        return Err(format!("Parameter '{}' must be an integer", def.name));
                    }
                }
                ParameterType::Boolean => {
                    if !val.is_boolean() {
                        return Err(format!("Parameter '{}' must be a boolean", def.name));
                    }
                }
                ParameterType::String => {
                    if !val.is_string() {
                        return Err(format!("Parameter '{}' must be a string", def.name));
                    }
                }
                ParameterType::Enum => {
                    if !val.is_string() {
                        return Err(format!("Parameter '{}' must be a string", def.name));
                    }
                }
            }
        }
    }

    Ok(())
}

/// Convert parameters to JSON schema for MCP tools
pub fn params_to_json_schema(definitions: &[ParameterDef]) -> serde_json::Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for def in definitions {
        let mut schema = serde_json::Map::new();

        schema.insert("type".to_string(), match def.param_type {
            ParameterType::Float | ParameterType::Length | ParameterType::Angle | ParameterType::Ratio => serde_json::json!("number"),
            ParameterType::Integer | ParameterType::Count => serde_json::json!("integer"),
            ParameterType::Boolean => serde_json::json!("boolean"),
            ParameterType::String => serde_json::json!("string"),
            ParameterType::Enum => serde_json::json!("string"),
        }.into());

        schema.insert("description".to_string(), serde_json::json!(def.description));

        if let Some(default) = &def.default {
            schema.insert("default".to_string(), default.clone());
        }
        if let Some(min) = def.min {
            schema.insert("minimum".to_string(), serde_json::json!(min));
        }
        if let Some(max) = def.max {
            schema.insert("maximum".to_string(), serde_json::json!(max));
        }
        if let Some(unit) = &def.unit {
            schema.insert("unit".to_string(), serde_json::json!(unit));
        }

        properties.insert(def.name.clone(), serde_json::Value::Object(schema));

        if def.required {
            required.push(def.name.clone());
        }
    }

    let mut result = serde_json::Map::new();
    result.insert("type".to_string(), serde_json::json!("object"));
    result.insert("properties".to_string(), serde_json::Value::Object(properties));
    if !required.is_empty() {
        result.insert("required".to_string(), serde_json::json!(required));
    }

    serde_json::Value::Object(result)
}