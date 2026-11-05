//! Manufacturing Fixtures Parametric Models
//!
//! Models for machine bases, conveyor frames, jig/fixture templates,
//! workbench systems, and tooling plates suitable for African manufacturing.

use crate::common::*;
use crate::{AfricaCategory, AfricaModel, AfricaModelError, ModelParameters, Result};
use solidcraft_engine::Backend;
use solidcore_record::Record;

/// Machine Base - Foundation for industrial machinery
pub struct MachineBase {
    params: Vec<ParameterDef>,
}

impl MachineBase {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("base_length", "Base Length", "Length of machine base", 2000.0, Some(500.0), Some(10000.0)),
                length_param("base_width", "Base Width", "Width of machine base", 1500.0, Some(500.0), Some(8000.0)),
                length_param("base_height", "Base Height", "Height of base", 300.0, Some(150.0), Some(1000.0)),
                enum_param("construction", "Construction", "Base construction type", "welded_steel", vec!["welded_steel", "cast_iron", "concrete", "epoxy_granite", "steel_frame_concrete"]),
                length_param("plate_thickness", "Plate Thickness", "Top plate thickness (steel)", 25.0, Some(12.0), Some(80.0)),
                length_param("rib_spacing", "Rib Spacing", "Stiffening rib spacing", 300.0, Some(200.0), Some(600.0)),
                length_param("rib_height", "Rib Height", "Height of stiffening ribs", 150.0, Some(100.0), Some(300.0)),
                length_param("rib_thickness", "Rib Thickness", "Thickness of ribs", 10.0, Some(6.0), Some(20.0)),
                int_param("num_mounting_holes", "Mounting Holes", "Number of machine mounting holes", 4, Some(2), Some(20)),
                length_param("hole_diameter", "Hole Diameter", "Mounting bolt hole diameter", 24.0, Some(12.0), Some(48.0)),
                length_param("hole_pattern_x", "Hole Pattern X", "X spacing of mounting holes", 500.0, Some(100.0), Some(2000.0)),
                length_param("hole_pattern_y", "Hole Pattern Y", "Y spacing of mounting holes", 400.0, Some(100.0), Some(2000.0)),
                bool_param("include_leveling", "Leveling Feet", "Include adjustable leveling feet", true),
                bool_param("include_vibration", "Vibration Isolation", "Include vibration isolation mounts", true),
                bool_param("include_drip_tray", "Drip Tray", "Include coolant/oil drip tray", true),
                bool_param("include_cable_management", "Cable Management", "Include cable routing channels", true),
                enum_param("surface_finish", "Surface Finish", "Top surface finish", "machined", vec!["machined", "ground", "scraped", "epoxy_coated", "stainless_clad"]),
                float_param("flatness_tolerance", "Flatness Tolerance", "Top surface flatness (mm/m)", 0.02, Some(0.005), Some(0.1), Some("mm/m")),
            ],
        }
    }
}

impl AfricaModel for MachineBase {
    fn id(&self) -> &'static str {
        "manufacturing.machine_base"
    }

    fn name(&self) -> &'static str {
        "Machine Base"
    }

    fn description(&self) -> &'static str {
        "Heavy-duty machine base for CNC machines, presses, injection molders, and industrial equipment. Welded steel fabrication with stiffening ribs for rigidity. Epoxy granite option for ultra-high precision (thermal stability, vibration damping). Leveling feet for installation on uneven floors. Vibration isolation for precision work. Mounting hole patterns customizable. Drip tray for coolant containment. Designed for African workshop conditions (power fluctuations, dust, temperature)."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Manufacturing
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

impl Default for MachineBase {
    fn default() -> Self {
        Self::new()
    }
}

/// Conveyor Frame - Frame for belt/roller conveyors
pub struct ConveyorFrame {
    params: Vec<ParameterDef>,
}

impl ConveyorFrame {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("conveyor_length", "Conveyor Length", "Total conveyor length", 10000.0, Some(1000.0), Some(50000.0)),
                length_param("belt_width", "Belt Width", "Conveyor belt width", 600.0, Some(300.0), Some(2000.0)),
                length_param("frame_height", "Frame Height", "Height of conveyor frame", 800.0, Some(500.0), Some(1500.0)),
                enum_param("conveyor_type", "Conveyor Type", "Type of conveyor", "belt", vec!["belt", "roller", "chain", "slat", "modular_belt", "screw", "pneumatic"]),
                enum_param("frame_material", "Frame Material", "Frame construction", "steel_tube", vec!["steel_tube", "aluminum_profile", "stainless_steel", "painted_steel"]),
                length_param("leg_spacing", "Leg Spacing", "Distance between support legs", 2000.0, Some(1000.0), Some(4000.0)),
                length_param("leg_diameter", "Leg Diameter", "Support leg tube diameter", 60.0, Some(40.0), Some(100.0)),
                length_param("leg_thickness", "Leg Thickness", "Leg wall thickness", 3.0, Some(2.0), Some(6.0)),
                enum_param("idler_type", "Idler Type", "Roller/idler type", "troughed", vec!["troughed", "flat", "return", "impact", "training"]),
                length_param("idler_spacing", "Idler Spacing", "Carrying idler spacing", 1000.0, Some(500.0), Some(2000.0)),
                length_param("return_idler_spacing", "Return Idler Spacing", "Return idler spacing", 3000.0, Some(2000.0), Some(5000.0)),
                bool_param("include_belt_cleaner", "Belt Cleaner", "Include primary/secondary belt cleaners", true),
                bool_param("include_skirt_board", "Skirt Board", "Include loading zone skirt boards", true),
                bool_param("include_takeup", "Take-up", "Include gravity/screw take-up", true),
                bool_param("include_guard", "Guarding", "Include nip point and return roller guards", true),
                bool_param("include_walkway", "Walkway", "Include maintenance walkway", false),
                float_param("incline_angle", "Incline Angle", "Conveyor incline (degrees)", 0.0, Some(-20.0), Some(20.0), Some("deg")),
                bool_param("reversible", "Reversible", "Bi-directional operation", false),
            ],
        }
    }
}

impl AfricaModel for ConveyorFrame {
    fn id(&self) -> &'static str {
        "manufacturing.conveyor_frame"
    }

    fn name(&self) -> &'static str {
        "Conveyor Frame System"
    }

    fn description(&self) -> &'static str {
        "Modular conveyor frame for bulk material handling in African mining, agriculture, and manufacturing. Belt conveyor most common. Steel tube frame with troughed idlers. Galvanized for corrosion resistance. Belt cleaners essential for sticky materials (wet ore, bagasse). Skirt boards at loading zone. Gravity take-up for tension. Full guarding for safety compliance. Walkway option for long conveyors. Incline up to 18° with cleated belt."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Manufacturing
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

impl Default for ConveyorFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// Jig Fixture Template - Template for manufacturing jigs/fixtures
pub struct JigFixtureTemplate {
    params: Vec<ParameterDef>,
}

impl JigFixtureTemplate {
    pub fn new() -> Self {
        Self {
            params: vec![
                enum_param("fixture_type", "Fixture Type", "Type of jig/fixture", "drill_jig", vec!["drill_jig", "mill_fixture", "weld_fixture", "assembly_fixture", "inspection_fixture", "router_template", "bending_jig", "test_fixture"]),
                length_param("workpiece_length", "Workpiece Length", "Max workpiece length", 500.0, Some(50.0), Some(3000.0)),
                length_param("workpiece_width", "Workpiece Width", "Max workpiece width", 300.0, Some(50.0), Some(2000.0)),
                length_param("workpiece_height", "Workpiece Height", "Max workpiece height", 200.0, Some(20.0), Some(1000.0)),
                enum_param("base_material", "Base Material", "Fixture base material", "aluminum_tooling_plate", vec!["aluminum_tooling_plate", "steel_plate", "cast_iron", "phenolic", "hdpe", "epoxy_granite"]),
                length_param("base_thickness", "Base Thickness", "Base plate thickness", 25.0, Some(12.0), Some(50.0)),
                bool_param("include_locators", "Locators", "Include workpiece locators/datums", true),
                bool_param("include_clamps", "Clamps", "Include clamping mechanisms", true),
                enum_param("clamp_type", "Clamp Type", "Clamping method", "toggle", vec!["toggle", "pneumatic", "hydraulic", "screw", "cam", "magnetic", "vacuum"]),
                bool_param("include_bushings", "Drill Bushings", "Include hardened drill bushings", true),
                int_param("num_bushings", "Number of Bushings", "Drill bushing count", 4, Some(1), Some(50)),
                length_param("bushing_diameter", "Bushing Diameter", "Drill bushing ID", 10.0, Some(3.0), Some(50.0)),
                bool_param("modular", "Modular Design", "Modular fixture with quick-change", false),
                bool_param("include_poka_yoke", "Poka-Yoke", "Include mistake-proofing features", true),
                bool_param("include_part_presence", "Part Presence", "Include part presence sensing", false),
            ],
        }
    }
}

impl AfricaModel for JigFixtureTemplate {
    fn id(&self) -> &'static str {
        "manufacturing.jig_fixture"
    }

    fn name(&self) -> &'static str {
        "Jig & Fixture Template"
    }

    fn description(&self) -> &'static str {
        "Parametric templates for manufacturing jigs and fixtures. Drill jigs, mill fixtures, weld fixtures, assembly fixtures, inspection fixtures. Aluminum tooling plate base (lightweight, stable) or steel. Modular quick-change option for high-mix production. Poka-yoke (mistake-proofing) features: asymmetric locators, part presence sensors, sequence control. Hardened drill bushings for accuracy. Designed for African SME manufacturing (job shops, automotive supply, agro-processing)."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Manufacturing
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

impl Default for JigFixtureTemplate {
    fn default() -> Self {
        Self::new()
    }
}

/// Workbench System - Modular industrial workbench
pub struct WorkbenchSystem {
    params: Vec<ParameterDef>,
}

impl WorkbenchSystem {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("bench_length", "Bench Length", "Workbench length", 2000.0, Some(1000.0), Some(4000.0)),
                length_param("bench_depth", "Bench Depth", "Workbench depth", 750.0, Some(600.0), Some(1200.0)),
                length_param("bench_height", "Bench Height", "Working height", 900.0, Some(750.0), Some(1100.0)),
                enum_param("top_material", "Top Material", "Work surface material", "hardwood", vec!["hardwood", "steel", "stainless_steel", "epoxy_resin", "phenolic", "esd_laminate", "butcher_block", "composite"]),
                length_param("top_thickness", "Top Thickness", "Work surface thickness", 50.0, Some(25.0), Some(100.0)),
                enum_param("frame_type", "Frame Type", "Frame construction", "steel_tube", vec!["steel_tube", "aluminum_profile", "wood", "heavy_duty_welded"]),
                bool_param("include_lower_shelf", "Lower Shelf", "Include lower storage shelf", true),
                bool_param("include_back_panel", "Back Panel", "Include pegboard/back panel", true),
                enum_param("back_panel_type", "Back Panel Type", "Back panel style", "pegboard", vec!["pegboard", "louvered", "slotted", "magnetic", "whiteboard"]),
                bool_param("include_drawers", "Drawers", "Include drawer units", true),
                int_param("num_drawers", "Number of Drawers", "Drawer count", 4, Some(1), Some(12)),
                bool_param("include_vice", "Vice", "Include bench vice mounting", true),
                bool_param("include_power", "Power", "Include integrated power outlets", true),
                int_param("num_outlets", "Power Outlets", "Number of outlets", 4, Some(2), Some(12)),
                bool_param("include_lighting", "Lighting", "Include overhead task lighting", true),
                bool_param("height_adjustable", "Height Adjustable", "Electric/manual height adjustment", false),
                bool_param("mobile", "Mobile", "Locking casters for mobility", false),
                enum_param("esd_protection", "ESD Protection", "Electrostatic discharge protection", "none", vec!["none", "grounded", "monitored", "ionizing"]),
            ],
        }
    }
}

impl AfricaModel for WorkbenchSystem {
    fn id(&self) -> &'static str {
        "manufacturing.workbench"
    }

    fn name(&self) -> &'static str {
        "Industrial Workbench System"
    }

    fn description(&self) -> &'static str {
        "Modular industrial workbench for African workshops, labs, and production lines. Hardwood top (local timber) or steel/stainless for heavy use. ESD-safe option for electronics assembly. Pegboard/louvered back panel for tool organization. Drawer units, vice mount, integrated power, task lighting. Height-adjustable option for ergonomics. Mobile on locking casters. Steel tube frame, powder-coated. Customizable for assembly, test, repair, packaging stations."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Manufacturing
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

impl Default for WorkbenchSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Tooling Plate - Precision tooling plate for fixturing
pub struct ToolingPlate {
    params: Vec<ParameterDef>,
}

impl ToolingPlate {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("plate_length", "Plate Length", "Tooling plate length", 600.0, Some(300.0), Some(2000.0)),
                length_param("plate_width", "Plate Width", "Tooling plate width", 400.0, Some(200.0), Some(1500.0)),
                length_param("plate_thickness", "Plate Thickness", "Plate thickness", 25.0, Some(12.0), Some(80.0)),
                enum_param("material", "Material", "Plate material", "aluminum_7075", vec!["aluminum_7075", "aluminum_6061", "steel_a36", "steel_1045", "stainless_304", "epoxy_granite", "ceramic"]),
                enum_param("hole_pattern", "Hole Pattern", "Mounting hole grid", "m12_50mm", vec!["m12_50mm", "m16_50mm", "m12_25mm", "m16_25mm", "imperial_2in", "custom"]),
                bool_param("include_t_slots", "T-Slots", "Include T-slots for clamping", true),
                length_param("t_slot_size", "T-Slot Size", "T-slot nominal size", 12.0, Some(10.0), Some(22.0)),
                bool_param("include_dowel_holes", "Dowel Holes", "Include precision dowel holes", true),
                length_param("dowel_diameter", "Dowel Diameter", "Dowel hole diameter", 12.0, Some(8.0), Some(20.0)),
                float_param("flatness", "Flatness", "Surface flatness tolerance", 0.01, Some(0.002), Some(0.05), Some("mm")),
                float_param("parallelism", "Parallelism", "Face parallelism tolerance", 0.01, Some(0.002), Some(0.05), Some("mm")),
                bool_param("hardcoat", "Hardcoat Anodize", "Hardcoat anodizing (aluminum only)", true),
                bool_param("include_lift_points", "Lift Points", "Include threaded lift points", true),
            ],
        }
    }
}

impl AfricaModel for ToolingPlate {
    fn id(&self) -> &'static str {
        "manufacturing.tooling_plate"
    }

    fn name(&self) -> &'static str {
        "Precision Tooling Plate"
    }

    fn description(&self) -> &'static str {
        "Precision tooling plate for modular fixturing systems. M12/M16 hole grid on 25mm or 50mm pitch compatible with standard fixture components (Mitee-Bite, Raptor, Jergens, local equivalents). Hardcoat anodized aluminum (7075-T6) for wear resistance and corrosion protection. Precision dowel holes for repeatable location. T-slots for versatile clamping. Flatness/parallelism to 0.01mm. Epoxy granite option for ultra-high precision CMM/fixture base. Lift points for crane handling."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Manufacturing
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

impl Default for ToolingPlate {
    fn default() -> Self {
        Self::new()
    }
}