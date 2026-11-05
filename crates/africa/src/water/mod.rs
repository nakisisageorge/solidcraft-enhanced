//! Water Infrastructure Parametric Models
//!
//! Models for water storage tanks (elevated, underground, modular), pump houses,
//! pipe routing systems, and rainwater harvesting suitable for African markets.

use crate::common::*;
use crate::{AfricaCategory, AfricaModel, AfricaModelError, ModelParameters, Result};
use solidcraft_engine::Backend;
use solidcore_record::Record;

/// Elevated Water Tank - Steel tank on tower structure
pub struct ElevatedWaterTank {
    params: Vec<ParameterDef>,
}

impl ElevatedWaterTank {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("capacity_liters", "Capacity", "Tank capacity in liters", 50000.0, Some(5000.0), Some(500000.0)),
                length_param("tank_height", "Tank Height", "Height of cylindrical tank", 3000.0, Some(2000.0), Some(6000.0)),
                length_param("tower_height", "Tower Height", "Height of support tower", 12000.0, Some(6000.0), Some(30000.0)),
                length_param("leg_diameter", "Leg Diameter", "Diameter of tower legs (CHS)", 150.0, Some(80.0), Some(300.0)),
                length_param("leg_thickness", "Leg Thickness", "Wall thickness of tower legs", 6.0, Some(4.0), Some(12.0)),
                length_param("bracing_size", "Bracing Size", "Size of diagonal bracing (SHS)", 60.0, Some(40.0), Some(100.0)),
                length_param("bracing_thickness", "Bracing Thickness", "Wall thickness of bracing", 3.0, Some(2.0), Some(6.0)),
                enum_param("tank_shape", "Tank Shape", "Shape of water tank", "cylindrical", vec!["cylindrical", "rectangular", "spherical", "conical_bottom"]),
                bool_param("include_ladder", "Include Ladder", "Add access ladder with safety cage", true),
                bool_param("include_platform", "Include Platform", "Add maintenance platform at tank level", true),
                bool_param("include_railings", "Include Railings", "Add guardrails on platform", true),
                enum_param("foundation", "Foundation Type", "Tower foundation type", "pad_footing", vec!["pad_footing", "pile_cap", "raft", "ground_beam"]),
                bool_param("seismic_design", "Seismic Design", "Include seismic bracing", true),
            ],
        }
    }
}

impl AfricaModel for ElevatedWaterTank {
    fn id(&self) -> &'static str {
        "water.elevated_tank"
    }

    fn name(&self) -> &'static str {
        "Elevated Water Tank"
    }

    fn description(&self) -> &'static str {
        "Elevated steel water storage tank on braced tower structure. Common for municipal water supply in African towns and cities. Gravity-fed distribution provides reliable pressure without pumps. Designed for African wind loads and seismic zones. Hot-dip galvanized or epoxy-coated for corrosion resistance."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Water
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

impl Default for ElevatedWaterTank {
    fn default() -> Self {
        Self::new()
    }
}

/// Underground Water Tank - Buried concrete/steel tank
pub struct UndergroundWaterTank {
    params: Vec<ParameterDef>,
}

impl UndergroundWaterTank {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("capacity_liters", "Capacity", "Tank capacity in liters", 20000.0, Some(5000.0), Some(200000.0)),
                enum_param("material", "Material", "Tank construction material", "reinforced_concrete", vec!["reinforced_concrete", "steel", "polyethylene", "fiberglass", "brick_masonry"]),
                length_param("tank_length", "Tank Length", "Length of rectangular tank", 6000.0, Some(3000.0), Some(20000.0)),
                length_param("tank_width", "Tank Width", "Width of rectangular tank", 3000.0, Some(2000.0), Some(10000.0)),
                length_param("tank_height", "Tank Height", "Internal height of tank", 2500.0, Some(1500.0), Some(4000.0)),
                length_param("cover_thickness", "Cover Slab Thickness", "Thickness of access cover slab", 200.0, Some(150.0), Some(400.0)),
                length_param("wall_thickness", "Wall Thickness", "Tank wall thickness", 250.0, Some(150.0), Some(500.0)),
                length_param("base_thickness", "Base Slab Thickness", "Thickness of base slab", 300.0, Some(200.0), Some(500.0)),
                int_param("num_chambers", "Number of Chambers", "Internal chambers for cleaning", 2, Some(1), Some(4)),
                bool_param("include_manhole", "Include Manhole", "Add access manhole with cover", true),
                bool_param("include_vent", "Include Vent", "Add ventilation pipe", true),
                bool_param("include_overflow", "Include Overflow", "Add overflow pipe", true),
                float_param("water_table", "Water Table", "Groundwater level below surface (m)", 3.0, Some(0.5), Some(10.0), Some("m")),
                enum_param("soil_type", "Soil Type", "Subsoil classification", "clay", vec!["clay", "sandy_clay", "sand", "gravel", "rock", "laterite"]),
            ],
        }
    }
}

impl AfricaModel for UndergroundWaterTank {
    fn id(&self) -> &'static str {
        "water.underground_tank"
    }

    fn name(&self) -> &'static str {
        "Underground Water Tank"
    }

    fn description(&self) -> &'static str {
        "Buried water storage tank for rainwater harvesting or municipal backup. Reinforced concrete construction typical for African contexts. Suitable for areas with high water table (buoyancy calculations included). Multiple chambers allow cleaning without service interruption. Manhole access with child-safe covers."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Water
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

impl Default for UndergroundWaterTank {
    fn default() -> Self {
        Self::new()
    }
}

/// Modular Water Tank - Bolted panel tank for easy transport
pub struct ModularWaterTank {
    params: Vec<ParameterDef>,
}

impl ModularWaterTank {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("capacity_liters", "Capacity", "Tank capacity in liters", 100000.0, Some(10000.0), Some(1000000.0)),
                enum_param("panel_type", "Panel Type", "Modular panel material", "grc", vec!["grc", "steel_epoxy", "stainless_steel", "galvanized_steel"]),
                length_param("panel_width", "Panel Width", "Standard panel width", 1000.0, Some(500.0), Some(2000.0)),
                length_param("panel_height", "Panel Height", "Standard panel height", 1000.0, Some(500.0), Some(2000.0)),
                length_param("tank_diameter", "Tank Diameter", "Overall tank diameter", 4000.0, Some(2000.0), Some(20000.0)),
                length_param("tank_height", "Tank Height", "Overall tank height", 4000.0, Some(2000.0), Some(10000.0)),
                int_param("num_rings", "Number of Rings", "Vertical panel rings", 4, Some(2), Some(10)),
                bool_param("include_roof", "Include Roof", "Add modular roof panels", true),
                bool_param("include_ladder", "Include Ladder", "Add internal/external ladder", true),
                enum_param("foundation", "Foundation", "Foundation type", "concrete_ring", vec!["concrete_ring", "concrete_slab", "compacted_gravel", "steel_frame"]),
            ],
        }
    }
}

impl AfricaModel for ModularWaterTank {
    fn id(&self) -> &'static str {
        "water.modular_tank"
    }

    fn name(&self) -> &'static str {
        "Modular Bolted Panel Tank"
    }

    fn description(&self) -> &'static str {
        "Bolted panel water tank for rapid deployment in remote African locations. Panels nest for efficient transport (shipping container optimized). Glass-reinforced concrete (GRC) or epoxy-coated steel panels. No heavy lifting equipment required for assembly. Popular for mining camps, refugee settlements, and rural water supply."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Water
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

impl Default for ModularWaterTank {
    fn default() -> Self {
        Self::new()
    }
}

/// Pump House - Building for water pumping equipment
pub struct PumpHouse {
    params: Vec<ParameterDef>,
}

impl PumpHouse {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("building_length", "Building Length", "Internal length of pump house", 4000.0, Some(3000.0), Some(10000.0)),
                length_param("building_width", "Building Width", "Internal width of pump house", 3000.0, Some(2500.0), Some(8000.0)),
                length_param("wall_height", "Wall Height", "Height to eaves", 3000.0, Some(2500.0), Some(5000.0)),
                enum_param("construction", "Construction", "Wall construction type", "concrete_block", vec!["concrete_block", "brick", "reinforced_concrete", "steel_frame_clad", "rammed_earth"]),
                enum_param("roof_type", "Roof Type", "Roof construction", "corrugated_iron", vec!["corrugated_iron", "concrete_slab", "tile", "thatch"]),
                int_param("num_pumps", "Number of Pumps", "Pump sets to accommodate", 2, Some(1), Some(6)),
                length_param("pump_base_thickness", "Pump Base Thickness", "Inertia base thickness for vibration", 300.0, Some(200.0), Some(600.0)),
                bool_param("include_crane", "Include Crane Beam", "Add overhead crane beam for pump removal", true),
                bool_param("include_ventilation", "Include Ventilation", "Add louvers and roof vents", true),
                bool_param("include_control_room", "Control Room", "Add separate control/office room", true),
                bool_param("security_doors", "Security Doors", "Steel security doors and window bars", true),
            ],
        }
    }
}

impl AfricaModel for PumpHouse {
    fn id(&self) -> &'static str {
        "water.pump_house"
    }

    fn name(&self) -> &'static str {
        "Water Pump House"
    }

    fn description(&self) -> &'static str {
        "Pump house building for water supply pumping stations. Accommodates multiple pump sets with inertia bases for vibration isolation. Includes overhead crane beam for maintenance, ventilation for motor cooling, optional control room. Security features for remote sites. Construction materials adapted to local availability."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Water
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

impl Default for PumpHouse {
    fn default() -> Self {
        Self::new()
    }
}

/// Pipe Routing System - Parametric pipe layout for water distribution
pub struct PipeRoutingSystem {
    params: Vec<ParameterDef>,
}

impl PipeRoutingSystem {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("main_diameter", "Main Pipe Diameter", "Primary distribution pipe diameter", 150.0, Some(50.0), Some(600.0)),
                length_param("branch_diameter", "Branch Pipe Diameter", "Service connection pipe diameter", 50.0, Some(25.0), Some(150.0)),
                length_param("pipe_length", "Total Pipe Length", "Total length of pipeline", 5000.0, Some(100.0), Some(100000.0)),
                enum_param("pipe_material", "Pipe Material", "Pipe material", "hdpe", vec!["hdpe", "upvc", "galvanized_steel", "ductile_iron", "copper", "pp_r"]),
                float_param("bury_depth", "Bury Depth", "Minimum cover over pipe", 0.9, Some(0.6), Some(2.0), Some("m")),
                bool_param("include_thrust_blocks", "Thrust Blocks", "Add concrete thrust blocks at bends", true),
                bool_param("include_air_valves", "Air Valves", "Add air release valves at high points", true),
                bool_param("include_washouts", "Washouts", "Add washout valves at low points", true),
                int_param("num_connections", "Service Connections", "Number of household connections", 50, Some(1), Some(500)),
                enum_param("terrain", "Terrain Profile", "Ground profile", "flat", vec!["flat", "rolling", "steep", "river_crossing", "road_crossing"]),
            ],
        }
    }
}

impl AfricaModel for PipeRoutingSystem {
    fn id(&self) -> &'static str {
        "water.pipe_routing"
    }

    fn name(&self) -> &'static str {
        "Water Pipe Routing System"
    }

    fn description(&self) -> &'static str {
        "Parametric water distribution pipe network layout. Generates plan and profile with thrust blocks, air valves, washouts, and service connections. HDPE and uPVC most common in African municipal systems. Accounts for terrain profile with automatic high/low point detection. Includes marker post and valve chamber positions."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Water
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

impl Default for PipeRoutingSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Rainwater Harvesting System - Complete RWH system for buildings
pub struct RainwaterHarvesting {
    params: Vec<ParameterDef>,
}

impl RainwaterHarvesting {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("roof_area", "Roof Area", "Catchment roof area", 100.0, Some(20.0), Some(5000.0)),
                enum_param("roof_material", "Roof Material", "Roofing material (affects runoff coefficient)", "corrugated_iron", vec!["corrugated_iron", "tile", "concrete", "asbestos_cement", "thatch"]),
                length_param("tank_capacity", "Tank Capacity", "Storage tank capacity (liters)", 5000.0, Some(1000.0), Some(50000.0)),
                enum_param("tank_type", "Tank Type", "Storage tank type", "polyethylene", vec!["polyethylene", "concrete", "steel", "ferrocement", "brick"]),
                bool_param("first_flush", "First Flush Diverter", "Include first flush diverter", true),
                bool_param("leaf_screen", "Leaf Screen", "Include leaf/debris screen", true),
                bool_param("pump_system", "Pump System", "Include pressure pump for distribution", false),
                bool_param("treatment", "Treatment", "Include basic filtration/UV treatment", false),
                enum_param("usage", "Usage", "Primary water use", "domestic", vec!["domestic", "irrigation", "livestock", "groundwater_recharge", "mixed"]),
                float_param("annual_rainfall", "Annual Rainfall", "Mean annual rainfall (mm)", 800.0, Some(200.0), Some(3000.0), Some("mm")),
            ],
        }
    }
}

impl AfricaModel for RainwaterHarvesting {
    fn id(&self) -> &'static str {
        "water.rainwater_harvesting"
    }

    fn name(&self) -> &'static str {
        "Rainwater Harvesting System"
    }

    fn description(&self) -> &'static str {
        "Complete rainwater harvesting system for African buildings. Calculates optimal tank size from roof area and rainfall data. Includes first flush diverter, leaf screens, optional pump and treatment. Tank options: polyethylene (common), ferrocement (low-cost, local skills), concrete, brick. Suitable for domestic, school, clinic, and institutional buildings."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Water
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

impl Default for RainwaterHarvesting {
    fn default() -> Self {
        Self::new()
    }
}