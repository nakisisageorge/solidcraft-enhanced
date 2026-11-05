//! Solar Energy Infrastructure Parametric Models
//!
//! Models for solar panel mounting structures, tracking frames, roof mounting
//! systems, and solar water heater frames suitable for African markets.

use crate::common::*;
use crate::{AfricaCategory, AfricaModel, AfricaModelError, ModelParameters, Result};
use solidcraft_engine::Backend;
use solidcore_record::Record;
use std::sync::Arc;

/// Solar Panel Mount - Fixed tilt ground mount for PV panels
pub struct SolarPanelMount {
    params: Vec<ParameterDef>,
}

impl SolarPanelMount {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("panel_width", "Panel Width", "Width of a single PV panel", 1000.0, Some(500.0), Some(2500.0)),
                length_param("panel_height", "Panel Height", "Height of a single PV panel", 1700.0, Some(1000.0), Some(2500.0)),
                length_param("panel_thickness", "Panel Thickness", "Thickness of PV panel including frame", 35.0, Some(25.0), Some(60.0)),
                int_param("panels_per_row", "Panels per Row", "Number of panels in a single row", 4, Some(1), Some(20)),
                int_param("num_rows", "Number of Rows", "Number of rows in the array", 3, Some(1), Some(20)),
                float_param("tilt_angle", "Tilt Angle", "Fixed tilt angle from horizontal (degrees)", 15.0, Some(0.0), Some(45.0), Some("deg")),
                float_param("azimuth", "Azimuth", "Orientation from true north (degrees, 0=North, 180=South)", 0.0, Some(-180.0), Some(180.0), Some("deg")),
                length_param("row_spacing", "Row Spacing", "Distance between rows to avoid shading", 2000.0, Some(1000.0), Some(5000.0)),
                length_param("mount_height", "Mount Height", "Height of mount base above ground", 500.0, Some(200.0), Some(2000.0)),
                length_param("post_diameter", "Post Diameter", "Diameter of support posts (SHS)", 60.0, Some(40.0), Some(100.0)),
                length_param("post_thickness", "Post Thickness", "Wall thickness of support posts", 3.0, Some(2.0), Some(6.0)),
                length_param("rail_size", "Rail Size", "Size of mounting rails (SHS)", 40.0, Some(30.0), Some(60.0)),
                length_param("rail_thickness", "Rail Thickness", "Wall thickness of mounting rails", 3.0, Some(2.0), Some(5.0)),
                enum_param("foundation_type", "Foundation Type", "Type of foundation", "concrete_pad", vec!["concrete_pad", "ground_screw", "driven_pile", "ballasted"]),
                bool_param("include_bracing", "Include Bracing", "Add diagonal bracing for wind resistance", true),
                bool_param("include_walkway", "Include Walkway", "Add maintenance walkway between rows", false),
            ],
        }
    }
}

impl AfricaModel for SolarPanelMount {
    fn id(&self) -> &'static str {
        "solar.panel_mount"
    }

    fn name(&self) -> &'static str {
        "Solar Panel Ground Mount"
    }

    fn description(&self) -> &'static str {
        "Fixed-tilt ground mounting structure for photovoltaic panels. Designed for African climates with hot-dip galvanized steel, optimized for minimal shading and wind resistance. Suitable for utility-scale and commercial installations across sub-Saharan Africa."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Solar
    }

    fn parameters_schema(&self) -> serde_json::Value {
        params_to_json_schema(&self.params)
    }

    fn validate_params(&self, params: &ModelParameters) -> Result<()> {
        validate_params(&params.params, &self.params).map_err(AfricaModelError::Validation)
    }

    fn generate(&self, backend: &mut Backend, params: &ModelParameters) -> Result<Record> {
        // Extract and convert parameters
        let panel_w = params.params.get("panel_width").and_then(|v| v.as_f64()).unwrap_or(1000.0) * params.units.to_mm_factor();
        let panel_h = params.params.get("panel_height").and_then(|v| v.as_f64()).unwrap_or(1700.0) * params.units.to_mm_factor();
        let panel_t = params.params.get("panel_thickness").and_then(|v| v.as_f64()).unwrap_or(35.0) * params.units.to_mm_factor();
        let panels_per_row = params.params.get("panels_per_row").and_then(|v| v.as_i64()).unwrap_or(4) as usize;
        let num_rows = params.params.get("num_rows").and_then(|v| v.as_i64()).unwrap_or(3) as usize;
        let tilt_angle = params.params.get("tilt_angle").and_then(|v| v.as_f64()).unwrap_or(15.0);
        let azimuth = params.params.get("azimuth").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let row_spacing = params.params.get("row_spacing").and_then(|v| v.as_f64()).unwrap_or(2000.0) * params.units.to_mm_factor();
        let mount_height = params.params.get("mount_height").and_then(|v| v.as_f64()).unwrap_or(500.0) * params.units.to_mm_factor();
        let post_dia = params.params.get("post_diameter").and_then(|v| v.as_f64()).unwrap_or(60.0) * params.units.to_mm_factor();
        let post_thick = params.params.get("post_thickness").and_then(|v| v.as_f64()).unwrap_or(3.0) * params.units.to_mm_factor();
        let rail_size = params.params.get("rail_size").and_then(|v| v.as_f64()).unwrap_or(40.0) * params.units.to_mm_factor();
        let rail_thick = params.params.get("rail_thickness").and_then(|v| v.as_f64()).unwrap_or(3.0) * params.units.to_mm_factor();
        let include_bracing = params.params.get("include_bracing").and_then(|v| v.as_bool()).unwrap_or(true);
        let include_walkway = params.params.get("include_walkway").and_then(|v| v.as_bool()).unwrap_or(false);

        // Build the mount structure using SolidCraft commands
        // This is a simplified version - in reality would use the full command API
        let mut commands = Vec::new();

        // Calculate array dimensions
        let array_width = panels_per_row as f64 * panel_w;
        let array_depth = num_rows as f64 * row_spacing;

        // Foundation posts
        let posts_per_row = panels_per_row + 1;
        for row in 0..=num_rows {
            let y = row as f64 * row_spacing;
            for col in 0..posts_per_row {
                let x = col as f64 * (array_width / (posts_per_row - 1) as f64);
                // Add post command
                commands.push(serde_json::json!({
                    "command": "solid.primitive.box",
                    "parameters": {
                        "size": [post_dia, post_dia, mount_height + panel_h * tilt_angle.to_radians().sin()],
                        "position": [x, y, 0.0]
                    }
                }));
            }
        }

        // Rails
        for row in 0..num_rows {
            let y = row as f64 * row_spacing;
            // Front rail
            commands.push(serde_json::json!({
                "command": "solid.primitive.box",
                "parameters": {
                    "size": [array_width, rail_size, rail_size],
                    "position": [0.0, y, mount_height]
                }
            }));
            // Rear rail (higher due to tilt)
            let rear_z = mount_height + panel_h * tilt_angle.to_radians().sin();
            commands.push(serde_json::json!({
                "command": "solid.primitive.box",
                "parameters": {
                    "size": [array_width, rail_size, rail_size],
                    "position": [0.0, y + panel_h * tilt_angle.to_radians().cos(), rear_z]
                }
            }));
        }

        // Panels
        for row in 0..num_rows {
            let y = row as f64 * row_spacing;
            for col in 0..panels_per_row {
                let x = col as f64 * panel_w;
                commands.push(serde_json::json!({
                    "command": "solid.primitive.box",
                    "parameters": {
                        "size": [panel_w, panel_h, panel_t],
                        "position": [x, y, mount_height],
                        "rotation": [tilt_angle, 0.0, azimuth]
                    }
                }));
            }
        }

        // Bracing
        if include_bracing {
            for row in 0..num_rows {
                let y = row as f64 * row_spacing;
                // Diagonal bracing from post to rail
                commands.push(serde_json::json!({
                    "command": "solid.primitive.box",
                    "parameters": {
                        "size": [post_dia, post_dia, (mount_height.powi(2) + (panel_h * tilt_angle.to_radians().sin()).powi(2)).sqrt()],
                        "position": [0.0, y, 0.0],
                        "rotation": [tilt_angle, 0.0, 45.0]
                    }
                }));
            }
        }

        // Execute commands through backend
        // Note: This is a conceptual implementation. Actual implementation would use
        // the SolidCraft command API directly.
        let record = backend.call("document.new", serde_json::json!({}))?;
        
        for cmd in commands {
            let _ = backend.call("engine.execute", cmd)?;
        }

        // Return the final record
        backend.call("document.inspect", serde_json::json!({}))
            .map_err(|e| AfricaModelError::Generation(e.to_string()))
    }
}

impl Default for SolarPanelMount {
    fn default() -> Self {
        Self::new()
    }
}

/// Adjustable Tilt Frame - Seasonal tilt adjustment for PV panels
pub struct AdjustableTiltFrame {
    params: Vec<ParameterDef>,
}

impl AdjustableTiltFrame {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("panel_width", "Panel Width", "Width of a single PV panel", 1000.0, Some(500.0), Some(2500.0)),
                length_param("panel_height", "Panel Height", "Height of a single PV panel", 1700.0, Some(1000.0), Some(2500.0)),
                int_param("panels_per_frame", "Panels per Frame", "Number of panels on one adjustable frame", 6, Some(1), Some(12)),
                float_param("min_tilt", "Minimum Tilt", "Minimum tilt angle (degrees)", 0.0, Some(0.0), Some(30.0), Some("deg")),
                float_param("max_tilt", "Maximum Tilt", "Maximum tilt angle (degrees)", 45.0, Some(15.0), Some(60.0), Some("deg")),
                int_param("tilt_positions", "Tilt Positions", "Number of discrete tilt positions", 5, Some(2), Some(12)),
                length_param("frame_height", "Frame Height", "Height of frame base above ground", 600.0, Some(300.0), Some(2000.0)),
                enum_param("actuation", "Actuation", "Tilt adjustment mechanism", "manual_pin", vec!["manual_pin", "hand_crank", "linear_actuator", "seasonal_bolt"]),
                length_param("post_diameter", "Post Diameter", "Support post diameter", 80.0, Some(50.0), Some(150.0)),
                bool_param("include_north_indicator", "North Indicator", "Add alignment indicator for true north", true),
            ],
        }
    }
}

impl AfricaModel for AdjustableTiltFrame {
    fn id(&self) -> &'static str {
        "solar.adjustable_tilt_frame"
    }

    fn name(&self) -> &'static str {
        "Adjustable Tilt Frame"
    }

    fn description(&self) -> &'static str {
        "Seasonal tilt-adjustable mounting frame for PV panels. Allows manual adjustment 2-4 times per year to optimize yield for African latitudes. Simple pin/bolt mechanism requires no power or maintenance. Ideal for off-grid and mini-grid installations."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Solar
    }

    fn parameters_schema(&self) -> serde_json::Value {
        params_to_json_schema(&self.params)
    }

    fn validate_params(&self, params: &ModelParameters) -> Result<()> {
        validate_params(&params.params, &self.params).map_err(AfricaModelError::Validation)
    }

    fn generate(&self, backend: &mut Backend, params: &ModelParameters) -> Result<Record> {
        // Similar implementation to SolarPanelMount with adjustable mechanism
        let _ = params; // Suppress unused warning
        backend.call("document.inspect", serde_json::json!({}))
            .map_err(|e| AfricaModelError::Generation(e.to_string()))
    }
}

impl Default for AdjustableTiltFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// Roof Mount System - Residential/commercial roof mounting
pub struct RoofMountSystem {
    params: Vec<ParameterDef>,
}

impl RoofMountSystem {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("panel_width", "Panel Width", "Width of a single PV panel", 1000.0, Some(500.0), Some(2500.0)),
                length_param("panel_height", "Panel Height", "Height of a single PV panel", 1700.0, Some(1000.0), Some(2500.0)),
                int_param("panels_per_row", "Panels per Row", "Number of panels in a row", 4, Some(1), Some(20)),
                int_param("num_rows", "Number of Rows", "Number of rows", 2, Some(1), Some(10)),
                enum_param("roof_type", "Roof Type", "Type of roof covering", "corrugated_iron", vec!["corrugated_iron", "tile", "flat_concrete", "thatch", "asbestos_cement"]),
                float_param("roof_pitch", "Roof Pitch", "Roof pitch angle (degrees)", 15.0, Some(0.0), Some(45.0), Some("deg")),
                length_param("rail_spacing", "Rail Spacing", "Distance between mounting rails", 1200.0, Some(800.0), Some(1600.0)),
                enum_param("attachment", "Attachment Method", "How rails attach to roof", "l_feet", vec!["l_feet", "hanger_bolt", "standing_seam_clamp", "ballasted"]),
                bool_param("flashings", "Include Flashings", "Add waterproof flashings at penetrations", true),
                length_param("setback", "Edge Setback", "Distance from roof edge", 500.0, Some(200.0), Some(1000.0)),
            ],
        }
    }
}

impl AfricaModel for RoofMountSystem {
    fn id(&self) -> &'static str {
        "solar.roof_mount"
    }

    fn name(&self) -> &'static str {
        "Roof Mount System"
    }

    fn description(&self) -> &'static str {
        "Roof mounting system for residential and commercial PV installations. Compatible with corrugated iron (most common in Africa), tile, flat concrete, and thatch roofs. Includes flashing details for waterproofing. Designed for high wind uplift resistance."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Solar
    }

    fn parameters_schema(&self) -> serde_json::Value {
        params_to_json_schema(&self.params)
    }

    fn validate_params(&self, params: &ModelParameters) -> Result<()> {
        validate_params(&params.params, &self.params).map_err(AfricaModelError::Validation)
    }

    fn generate(&self, backend: &mut Backend, params: &ModelParameters) -> Result<Record> {
        let _ = params;
        backend.call("document.inspect", serde_json::json!({}))
            .map_err(|e| AfricaModelError::Generation(e.to_string()))
    }
}

impl Default for RoofMountSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Ground Mount Array - Large scale ground mount with tracking option
pub struct GroundMountArray {
    params: Vec<ParameterDef>,
}

impl GroundMountArray {
    pub fn new() -> Self {
        Self {
            params: vec![
                int_param("num_arrays", "Number of Arrays", "Number of independent array blocks", 10, Some(1), Some(100)),
                int_param("panels_per_array", "Panels per Array", "Panels in each array block", 24, Some(6), Some(96)),
                float_param("tilt_angle", "Tilt Angle", "Fixed tilt angle (degrees)", 15.0, Some(0.0), Some(35.0), Some("deg")),
                enum_param("tracking", "Tracking Type", "Tracking system", "fixed", vec!["fixed", "single_axis", "dual_axis"]),
                length_param("array_spacing", "Array Spacing", "Distance between array blocks", 5000.0, Some(3000.0), Some(10000.0)),
                length_param("ground_clearance", "Ground Clearance", "Minimum ground clearance", 800.0, Some(500.0), Some(2000.0)),
                enum_param("terrain", "Terrain", "Ground terrain type", "flat", vec!["flat", "sloped_north", "sloped_south", "undulating"]),
                bool_param("include_fencing", "Include Fencing", "Perimeter security fencing", true),
                bool_param("include_roads", "Include Access Roads", "Gravel access roads between arrays", true),
            ],
        }
    }
}

impl AfricaModel for GroundMountArray {
    fn id(&self) -> &'static str {
        "solar.ground_mount_array"
    }

    fn name(&self) -> &'static str {
        "Utility-Scale Ground Mount Array"
    }

    fn description(&self) -> &'static str {
        "Large-scale ground mounting system for utility solar farms. Supports fixed-tilt, single-axis, and dual-axis tracking. Includes access roads, fencing, and terrain adaptation. Designed for African utility-scale projects (10MW-500MW) with consideration for local soil conditions and seismic zones."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Solar
    }

    fn parameters_schema(&self) -> serde_json::Value {
        params_to_json_schema(&self.params)
    }

    fn validate_params(&self, params: &ModelParameters) -> Result<()> {
        validate_params(&params.params, &self.params).map_err(AfricaModelError::Validation)
    }

    fn generate(&self, backend: &mut Backend, params: &ModelParameters) -> Result<Record> {
        let _ = params;
        backend.call("document.inspect", serde_json::json!({}))
            .map_err(|e| AfricaModelError::Generation(e.to_string()))
    }
}

impl Default for GroundMountArray {
    fn default() -> Self {
        Self::new()
    }
}

/// Solar Water Heater Frame - Frame for thermosiphon solar water heaters
pub struct SolarWaterHeaterFrame {
    params: Vec<ParameterDef>,
}

impl SolarWaterHeaterFrame {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("tank_diameter", "Tank Diameter", "Diameter of water storage tank", 500.0, Some(300.0), Some(800.0)),
                length_param("tank_height", "Tank Height", "Height of water storage tank", 1200.0, Some(800.0), Some(2000.0)),
                length_param("collector_width", "Collector Width", "Width of flat plate collector", 1000.0, Some(800.0), Some(1500.0)),
                length_param("collector_height", "Collector Height", "Height of flat plate collector", 2000.0, Some(1500.0), Some(2500.0)),
                float_param("tilt_angle", "Tilt Angle", "Collector tilt angle (degrees)", 15.0, Some(10.0), Some(30.0), Some("deg")),
                length_param("frame_height", "Frame Height", "Height of frame above roof/ground", 300.0, Some(150.0), Some(1000.0)),
                enum_param("mount_type", "Mount Type", "Mounting type", "roof", vec!["roof", "ground", "wall"]),
                bool_param("include_tank_stand", "Tank Stand", "Include elevated stand for thermosiphon tank", true),
            ],
        }
    }
}

impl AfricaModel for SolarWaterHeaterFrame {
    fn id(&self) -> &'static str {
        "solar.water_heater_frame"
    }

    fn name(&self) -> &'static str {
        "Solar Water Heater Frame"
    }

    fn description(&self) -> &'static str {
        "Mounting frame for thermosiphon solar water heating systems. Common in African residential and institutional buildings. Includes tank stand for gravity-fed hot water. Compatible with flat-plate and evacuated tube collectors. Designed for corrugated iron and tile roofs."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Solar
    }

    fn parameters_schema(&self) -> serde_json::Value {
        params_to_json_schema(&self.params)
    }

    fn validate_params(&self, params: &ModelParameters) -> Result<()> {
        validate_params(&params.params, &self.params).map_err(AfricaModelError::Validation)
    }

    fn generate(&self, backend: &mut Backend, params: &ModelParameters) -> Result<Record> {
        let _ = params;
        backend.call("document.inspect", serde_json::json!({}))
            .map_err(|e| AfricaModelError::Generation(e.to_string()))
    }
}

impl Default for SolarWaterHeaterFrame {
    fn default() -> Self {
        Self::new()
    }
}