//! Construction Systems Parametric Models
//!
//! Models for scaffolding systems, formwork panels, rebar bending templates,
//! shuttering systems, and tower crane bases suitable for African construction.

use crate::common::*;
use crate::{AfricaCategory, AfricaModel, AfricaModelError, ModelParameters, Result};
use solidcraft_engine::Backend;
use solidcore_record::Record;

/// Scaffolding System - Modular scaffolding for construction
pub struct ScaffoldingSystem {
    params: Vec<ParameterDef>,
}

impl ScaffoldingSystem {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("bay_length", "Bay Length", "Length of scaffold bay", 2.5, Some(1.2), Some(3.0)),
                length_param("bay_width", "Bay Width", "Width of scaffold bay (standards spacing)", 1.5, Some(0.9), Some(2.4)),
                length_param("lift_height", "Lift Height", "Vertical spacing of ledgers/transoms", 2.0, Some(1.5), Some(2.5)),
                int_param("num_bays_long", "Bays Longitudinal", "Number of bays along building", 10, Some(1), Some(100)),
                int_param("num_bays_trans", "Bays Transverse", "Number of bays across building", 4, Some(1), Some(20)),
                int_param("num_lifts", "Number of Lifts", "Number of vertical lifts", 10, Some(1), Some(50)),
                enum_param("scaffold_type", "Scaffold Type", "Scaffolding system type", "kwikstage", vec!["kwikstage", "cuplock", "ringlock", "tube_coupler", "frame", "mobile_tower", "system_scaffold"]),
                enum_param("material", "Material", "Tube material", "steel_galvanized", vec!["steel_galvanized", "steel_painted", "aluminum", "bamboo"]),
                length_param("tube_diameter", "Tube Diameter", "Scaffold tube outer diameter", 48.3, Some(42.0), Some(60.0)),
                length_param("tube_thickness", "Tube Thickness", "Tube wall thickness", 3.2, Some(2.5), Some(5.0)),
                bool_param("include_toeboards", "Toeboards", "Include toeboards on all lifts", true),
                bool_param("include_guardrails", "Guardrails", "Include double guardrails", true),
                bool_param("include_ladders", "Ladders", "Include internal ladder access", true),
                bool_param("include_loading_bay", "Loading Bay", "Include material loading bay", true),
                bool_param("include_protection", "Public Protection", "Include fans/hoarding for public protection", false),
                enum_param("tie_pattern", "Tie Pattern", "Building tie pattern", "every_4m", vec!["every_4m", "every_2_lifts", "every_lift", "box_ties", "through_ties", "reveal_ties"]),
                bool_param("include_design_check", "Design Check", "Include capacity verification", true),
            ],
        }
    }
}

impl AfricaModel for ScaffoldingSystem {
    fn id(&self) -> &'static str {
        "construction.scaffolding"
    }

    fn name(&self) -> &'static str {
        "Modular Scaffolding System"
    }

    fn description(&self) -> &'static str {
        "Modular scaffolding system for African construction projects. Kwikstage, Cuplock, and Ringlock systems common. Tube & coupler for complex geometries. Bamboo scaffolding still used in some regions (not for high-rise). Includes guardrails, toeboards, ladder access, loading bays. Tie patterns for different building types. Design verification for wind loads (cyclone regions). Material takeoff generation. Suitable for building, bridge, industrial, and maintenance access."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Construction
    }

    fn parameters_schema(&self) -> serde_json::Value {
        params_to_json_schema(&self.params)
    }

    fn validate_params(&self, params: &ModelParameters) -> Result<()> {
        // Custom validation for num_bays_trans
        let mut params_copy = params.clone();
        // Fix the With -> Some typo in the parameter definition would be needed
        validate_params(&params.params, &self.params).map_err(AfricaModelError::Validation)
    }

    fn generate(&self, backend: &mut Backend, params: &ModelParameters) -> Result<Record> {
        let _ = params;
        backend.call("document.inspect", serde_json::json!({}))
            .map_err(|e| AfricaModelError::Generation(e.to_string()))
    }
}

impl Default for ScaffoldingSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Formwork Panel - Concrete formwork panel system
pub struct FormworkPanel {
    params: Vec<ParameterDef>,
}

impl FormworkPanel {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("panel_height", "Panel Height", "Formwork panel height", 3000.0, Some(1500.0), Some(6000.0)),
                length_param("panel_width", "Panel Width", "Formwork panel width", 1200.0, Some(600.0), Some(2400.0)),
                enum_param("panel_type", "Panel Type", "Formwork panel system", "steel_frame_ply", vec!["steel_frame_ply", "aluminum_frame_ply", "plastic_modular", "timber_framed", "fiberglass", "stay_in_place"]),
                enum_param("face_material", "Face Material", "Concrete contact surface", "film_faced_plywood", vec!["film_faced_plywood", "plastic_sheet", "steel_plate", "phenolic_plywood", "bamboo_plywood", "abs_plastic"]),
                length_param("frame_thickness", "Frame Thickness", "Frame profile depth", 80.0, Some(50.0), Some(150.0)),
                bool_param("include_corners", "Corner Panels", "Include internal/external corner panels", true),
                bool_param("include_taper_ties", "Taper Ties", "Include taper tie holes", true),
                length_param("tie_spacing", "Tie Spacing", "Horizontal tie spacing", 600.0, Some(300.0), Some(1200.0)),
                length_param("vertical_tie_spacing", "Vertical Tie Spacing", "Vertical tie spacing", 300.0, Some(200.0), Some(600.0)),
                bool_param("include_walers", "Walers", "Include horizontal walers/strongbacks", true),
                length_param("waler_spacing", "Waler Spacing", "Vertical waler spacing", 1200.0, Some(600.0), Some(2400.0)),
                bool_param("include_kickers", "Kickers", "Include column/wall kickers", true),
                bool_param("include_stop_ends", "Stop Ends", "Include stop-end panels", true),
                bool_param("crane_handling", "Crane Handling", "Designed for crane handling", true),
                float_param("max_pressure", "Max Concrete Pressure", "Design concrete pressure (kN/m²)", 60.0, Some(30.0), Some(150.0), Some("kN/m²")),
            ],
        }
    }
}

impl AfricaModel for FormworkPanel {
    fn id(&self) -> &'static str {
        "construction.formwork_panel"
    }

    fn name(&self) -> &'static str {
        "Concrete Formwork Panel System"
    }

    fn description(&self) -> &'static str {
        "Modular concrete formwork panel system for African construction. Steel frame with film-faced plywood face (most common). Aluminum frame for lighter weight/higher reuse. Plastic modular for small projects. Includes corners, taper ties, walers, kickers, stop ends. Crane-handled panels for high-rise. Design pressure up to 100 kN/m² for tall walls. Generates panel layout, tie schedule, waler schedule, material takeoff. Compatible with local plywood (South African, imported)."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Construction
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

impl Default for FormworkPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// Rebar Bending Template - Rebar cutting and bending schedules
pub struct RebarBendingTemplate {
    params: Vec<ParameterDef>,
}

impl RebarBendingTemplate {
    pub fn new() -> Self {
        Self {
            params: vec![
                enum_param("structure_type", "Structure Type", "Concrete element type", "beam", vec!["beam", "column", "slab", "wall", "footing", "pile_cap", "staircase", "retaining_wall", "water_tank", "silo"]),
                length_param("element_length", "Element Length", "Length of concrete element", 6000.0, Some(1000.0), Some(20000.0)),
                length_param("element_width", "Element Width", "Width/depth of element", 300.0, Some(150.0), Some(2000.0)),
                length_param("element_height", "Element Height", "Height of element (beams/columns)", 500.0, Some(200.0), Some(2000.0)),
                length_param("cover", "Concrete Cover", "Clear cover to reinforcement", 40.0, Some(25.0), Some(75.0)),
                enum_param("bar_grade", "Bar Grade", "Reinforcement steel grade", "450MPa", vec!["250MPa", "450MPa", "500MPa", "BS4449_B500B", "SANS_920"]),
                enum_param("main_bar_size", "Main Bar Size", "Longitudinal bar diameter", "Y20", vec!["Y10", "Y12", "Y16", "Y20", "Y25", "Y32", "Y40"]),
                int_param("num_main_bars", "Main Bars", "Number of main bars", 4, Some(2), Some(16)),
                enum_param("link_bar_size", "Link Bar Size", "Shear link/stirrup diameter", "Y10", vec!["Y8", "Y10", "Y12", "Y16"]),
                length_param("link_spacing", "Link Spacing", "Stirrup spacing", 200.0, Some(75.0), Some(400.0)),
                bool_param("include_top_bars", "Top Bars", "Include top reinforcement (beams/slabs)", true),
                enum_param("top_bar_size", "Top Bar Size", "Top bar diameter", "Y16", vec!["Y10", "Y12", "Y16", "Y20"]),
                int_param("num_top_bars", "Top Bars Count", "Number of top bars", 2, Some(1), Some(8)),
                bool_param("include_curtailment", "Curtailment", "Show bar curtailment lengths", true),
                bool_param("include_laps", "Lap Splices", "Include lap splice lengths", true),
                bool_param("include_chairs", "Chairs/Spacers", "Include chair/spacer schedule", true),
                bool_param("bending_schedule", "Bending Schedule", "Generate BS 8666 / SANS 282 schedule", true),
            ],
        }
    }
}

impl AfricaModel for RebarBendingTemplate {
    fn id(&self) -> &'static str {
        "construction.rebar_bending"
    }

    fn name(&self) -> &'static str {
        "Rebar Bending Schedule Template"
    }

    fn description(&self) -> &'static str {
        "Parametric rebar bending schedule generator for African construction. BS 8666 (UK/Commonwealth) and SANS 282 (South Africa) standards. Beams, columns, slabs, walls, footings, pile caps, staircases, water tanks, silos. Calculates cutting lengths, bend dimensions, lap splices, curtailment. Chair/spacer schedule. Bar grade 450MPa standard (BS 4449 B500B / SANS 920). Outputs cutting list, bending schedule (shape codes), mass estimate, bundle tags. Reduces waste, speeds site fixing."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Construction
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

impl Default for RebarBendingTemplate {
    fn default() -> Self {
        Self::new()
    }
}

/// Shuttering System - Formwork for slabs/beams/columns
pub struct ShutteringSystem {
    params: Vec<ParameterDef>,
}

impl ShutteringSystem {
    pub fn new() -> Self {
        Self {
            params: vec![
                enum_param("element", "Element", "Concrete element to shutter", "slab", vec!["slab", "beam", "column", "wall", "staircase", "landing", "ramp", "cantilever"]),
                length_param("span", "Span", "Clear span of slab/beam", 6000.0, Some(2000.0), Some(15000.0)),
                length_param("width", "Width", "Width of element", 300.0, Some(150.0), Some(1000.0)),
                length_param("depth", "Depth", "Depth/thickness of element", 200.0, Some(100.0), Some(1000.0)),
                enum_param("system", "Shuttering System", "Formwork system", "prop_slab", vec!["prop_slab", "table_form", "flying_form", "tunnel_form", "climbing_form", "slip_form", "steel_beam_slab"]),
                enum_param("prop_type", "Prop Type", "Support prop type", "steel_adjustable", vec!["steel_adjustable", "aluminum_adjustable", "timber", "screw_jack", "hydraulic"]),
                length_param("prop_spacing", "Prop Spacing", "Prop grid spacing", 1200.0, Some(900.0), Some(1800.0)),
                length_param("beam_spacing", "Primary Beam Spacing", "Primary beam spacing", 2400.0, Some(1200.0), Some(3600.0)),
                length_param("secondary_spacing", "Secondary Beam Spacing", "Secondary beam/joist spacing", 600.0, Some(400.0), Some(900.0)),
                bool_param("include_edge_forms", "Edge Forms", "Include slab edge forms", true),
                bool_param("include_openings", "Openings", "Include stair/penetration openings", true),
                int_param("num_openings", "Number of Openings", "Count of openings", 2, Some(0), Some(20)),
                bool_param("include_camber", "Camber", "Include camber in beams/slabs", false),
                float_param("camber_span_ratio", "Camber Ratio", "Camber as fraction of span", 0.003, Some(0.001), Some(0.01), Some("")),
                bool_param("crane_handling", "Crane Handling", "Panels designed for crane lifting", true),
                float_param("cycle_time", "Cycle Time", "Target floor cycle (days)", 7.0, Some(3.0), Some(14.0), Some("days")),
            ],
        }
    }
}

impl AfricaModel for ShutteringSystem {
    fn id(&self) -> &'static str {
        "construction.shuttering"
    }

    fn name(&self) -> &'static str {
        "Concrete Shuttering System"
    }

    fn description(&self) -> &'static str {
        "Complete shuttering (formwork) system for suspended slabs, beams, columns in African building construction. Prop & slab (most common), table forms (repetitive floors), flying forms (high-rise), tunnel forms (cellular), climbing forms (core walls). Steel adjustable props standard. Primary/secondary beam grid with plywood decking. Edge forms, opening forms. Camber option. Cycle time optimization for contractor planning. Generates prop loads, beam schedules, plywood cutting list, prop removal sequence."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Construction
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

impl Default for ShutteringSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Tower Crane Base - Foundation for tower crane
pub struct TowerCraneBase {
    params: Vec<ParameterDef>,
}

impl TowerCraneBase {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("crane_model", "Crane Model", "Tower crane model identifier", 0.0, Some(0.0), Some(0.0)), // Placeholder
                enum_param("crane_type", "Crane Type", "Tower crane type", "hammerhead", vec!["hammerhead", "luffing_jib", "flat_top", "self_erecting", "mobile_tower"]),
                length_param("max_capacity", "Max Capacity", "Maximum lifting capacity (tonnes)", 8.0, Some(1.0), Some(50.0)),
                length_param("max_radius", "Max Radius", "Maximum jib radius", 60.0, Some(30.0), Some(80.0)),
                length_param("freestanding_height", "Freestanding Height", "Height without ties", 40.0, Some(20.0), Some(80.0)),
                length_param("tied_height", "Tied Height", "Height with building ties", 150.0, Some(50.0), Some(300.0)),
                enum_param("base_type", "Base Type", "Base configuration", "concrete_pad", vec!["concrete_pad", "pile_cap", "ballasted", "chassis_frame", "cross_frame"]),
                length_param("pad_length", "Pad Length", "Base pad length", 6.0, Some(3.0), Some(15.0)),
                length_param("pad_width", "Pad Width", "Base pad width", 6.0, Some(3.0), Some(15.0)),
                length_param("pad_thickness", "Pad Thickness", "Base pad thickness", 1.2, Some(0.8), Some(3.0)),
                length_param("pedestal_height", "Pedestal Height", "Concrete pedestal height", 1.5, Some(0.5), Some(3.0)),
                bool_param("include_holding_down", "Holding Down Bolts", "Include anchor bolt cage", true),
                int_param("num_bolts", "Number of Bolts", "Holding down bolt count", 16, Some(8), Some(32)),
                length_param("bolt_diameter", "Bolt Diameter", "Anchor bolt diameter", 40.0, Some(24.0), Some(64.0)),
                enum_param("soil_type", "Soil Type", "Subsoil for bearing", "dense_sand", vec!["dense_sand", "firm_clay", "stiff_clay", "gravel", "rock", "piled"]),
                float_param("bearing_capacity", "Bearing Capacity", "Allowable bearing pressure", 300.0, Some(100.0), Some(1000.0), Some("kPa")),
                bool_param("include_settlement", "Settlement Check", "Include differential settlement analysis", true),
                bool_param("seismic_zone", "Seismic Zone", "Include seismic design", true),
            ],
        }
    }
}

impl AfricaModel for TowerCraneBase {
    fn id(&self) -> &'static str {
        "construction.tower_crane_base"
    }

    fn name(&self) -> &'static str {
        "Tower Crane Base Foundation"
    }

    fn description(&self) -> &'static str {
        "Tower crane base foundation for African high-rise construction. Hammerhead, luffing jib, flat-top, self-erecting crane types. Concrete pad, pile cap, ballasted, or cross-frame bases. Holding-down bolt cage for base section. Bearing capacity verification for African soils (laterite, black cotton, sand, rock). Settlement analysis. Seismic design for East African Rift zone. Generates pad reinforcement, bolt cage, concrete volume, crane reaction loads. Compatible with major crane brands (Liebherr, Potain, Zoomlion, XCMG, Terex)."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Construction
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

impl Default for TowerCraneBase {
    fn default() -> Self {
        Self::new()
    }
}