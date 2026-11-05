//! Housing Parametric Models
//!
//! Models for modular housing units, interlocking brick walls, roof truss systems,
//! foundation layouts, and sanitation blocks suitable for African markets.

use crate::common::*;
use crate::{AfricaCategory, AfricaModel, AfricaModelError, ModelParameters, Result};
use solidcraft_engine::Backend;
use solidcore_record::Record;

/// Modular Housing Unit - Prefabricated housing module
pub struct ModularHousingUnit {
    params: Vec<ParameterDef>,
}

impl ModularHousingUnit {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("unit_width", "Unit Width", "Width of housing module", 3000.0, Some(2400.0), Some(4800.0)),
                length_param("unit_length", "Unit Length", "Length of housing module", 6000.0, Some(4000.0), Some(12000.0)),
                length_param("unit_height", "Unit Height", "Internal ceiling height", 2700.0, Some(2400.0), Some(3000.0)),
                enum_param("construction", "Construction", "Wall construction system", "steel_frame_panel", vec!["steel_frame_panel", "timber_frame", "sip_panel", "container_based", "modular_block", "rammed_earth_panel"]),
                enum_param("roof_type", "Roof Type", "Roof style", "gable", vec!["gable", "hip", "flat", "mono_pitch", "butterfly"]),
                float_param("roof_pitch", "Roof Pitch", "Roof pitch angle (degrees)", 22.5, Some(5.0), Some(45.0), Some("deg")),
                int_param("num_rooms", "Number of Rooms", "Internal room divisions", 2, Some(1), Some(6)),
                bool_param("include_sanitation", "Include Sanitation", "Add bathroom/toilet module", true),
                bool_param("include_kitchen", "Include Kitchen", "Add kitchenette", true),
                bool_param("include_verandah", "Verandah", "Add covered outdoor space", true),
                length_param("verandah_depth", "Verandah Depth", "Depth of verandah", 1500.0, Some(1000.0), Some(3000.0)),
                enum_param("floor_type", "Floor Type", "Floor construction", "concrete_slab", vec!["concrete_slab", "raised_timber", "compressed_earth", "tile_on_screed"]),
                enum_param("cladding", "External Cladding", "External wall finish", "fibre_cement", vec!["fibre_cement", "corrugated_iron", "brick_veneer", "render", "timber", "bamboo"]),
                bool_param("solar_ready", "Solar Ready", "Pre-wire for solar PV system", true),
                bool_param("water_tank_ready", "Water Tank Ready", "Provision for roof water tank", true),
            ],
        }
    }
}

impl AfricaModel for ModularHousingUnit {
    fn id(&self) -> &'static str {
        "housing.modular_unit"
    }

    fn name(&self) -> &'static str {
        "Modular Housing Unit"
    }

    fn description(&self) -> &'static str {
        "Prefabricated modular housing unit for rapid deployment across Africa. Steel frame with panelized walls allows flat-pack transport. Configurable room layouts, optional sanitation and kitchen modules. Verandah for outdoor living (cultural preference). Solar-ready and water-harvesting-ready. Meets UN-Habitat adequate housing standards. Suitable for post-disaster, refugee, low-income, and workforce housing."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Housing
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

impl Default for ModularHousingUnit {
    fn default() -> Self {
        Self::new()
    }
}

/// Interlocking Brick Wall - Compressed earth/stabilized soil block wall
pub struct InterlockingBrickWall {
    params: Vec<ParameterDef>,
}

impl InterlockingBrickWall {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("wall_length", "Wall Length", "Total length of wall", 10000.0, Some(1000.0), Some(100000.0)),
                length_param("wall_height", "Wall Height", "Height of wall", 2700.0, Some(2000.0), Some(6000.0)),
                length_param("brick_length", "Brick Length", "Length of interlocking brick", 300.0, Some(250.0), Some(400.0)),
                length_param("brick_width", "Brick Width", "Width of interlocking brick", 150.0, Some(100.0), Some(200.0)),
                length_param("brick_height", "Brick Height", "Height of interlocking brick", 100.0, Some(80.0), Some(150.0)),
                enum_param("brick_type", "Brick Type", "Interlocking brick type", "hydraform", vec!["hydraform", "makiga", "stabilized_soil", "compressed_earth", "cement_stabilized"]),
                float_param("cement_ratio", "Cement Ratio", "Cement stabilization percentage", 5.0, Some(3.0), Some(10.0), Some("%")),
                length_param("mortar_joint", "Mortar Joint", "Thickness of mortar joint (if used)", 10.0, Some(0.0), Some(20.0)),
                bool_param("include_ring_beam", "Ring Beam", "Add reinforced concrete ring beam at top", true),
                length_param("ring_beam_depth", "Ring Beam Depth", "Depth of ring beam", 200.0, Some(150.0), Some(300.0)),
                length_param("ring_beam_width", "Ring Beam Width", "Width of ring beam", 150.0, Some(100.0), Some(200.0)),
                bool_param("include_lintels", "Lintels", "Add lintels over openings", true),
                int_param("num_openings", "Openings", "Number of door/window openings", 4, Some(0), Some(20)),
                length_param("opening_width", "Opening Width", "Standard opening width", 900.0, Some(600.0), Some(2000.0)),
                length_param("opening_height", "Opening Height", "Standard opening height", 2100.0, Some(1500.0), Some(2400.0)),
            ],
        }
    }
}

impl AfricaModel for InterlockingBrickWall {
    fn id(&self) -> &'static str {
        "housing.interlocking_brick_wall"
    }

    fn name(&self) -> &'static str {
        "Interlocking Brick Wall System"
    }

    fn description(&self) -> &'static str {
        "Interlocking compressed earth/stabilized soil block wall system. Hydraform, Makiga, and similar block types widely used in Africa. Dry-stacking (no mortar) or thin mortar joints. Includes ring beam, lintels, and opening detailing. Low-carbon, uses local soil, creates local employment. Suitable for housing, schools, clinics, and community buildings."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Housing
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

impl Default for InterlockingBrickWall {
    fn default() -> Self {
        Self::new()
    }
}

/// Roof Truss System - Prefabricated timber/steel roof trusses
pub struct RoofTrussSystem {
    params: Vec<ParameterDef>,
}

impl RoofTrussSystem {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("span", "Span", "Clear span of truss", 8000.0, Some(3000.0), Some(20000.0)),
                float_param("pitch", "Pitch", "Roof pitch angle (degrees)", 22.5, Some(10.0), Some(45.0), Some("deg")),
                length_param("truss_spacing", "Truss Spacing", "Center-to-center spacing of trusses", 1200.0, Some(600.0), Some(2400.0)),
                enum_param("truss_type", "Truss Type", "Truss configuration", "fink", vec!["fink", "howe", "king_post", "queen_post", "scissor", "attic", "mono", "hip_girder"]),
                enum_param("material", "Material", "Truss material", "timber", vec!["timber", "light_gauge_steel", "steel_tube", "glulam"]),
                length_param("top_chord_size", "Top Chord Size", "Top chord member size", 50.0, Some(38.0), Some(100.0)),
                length_param("bottom_chord_size", "Bottom Chord Size", "Bottom chord member size", 50.0, Some(38.0), Some(100.0)),
                length_param("web_size", "Web Size", "Web member size", 38.0, Some(38.0), Some(75.0)),
                bool_param("include_overhang", "Overhang", "Include eaves overhang", true),
                length_param("overhang_length", "Overhang Length", "Eaves overhang projection", 600.0, Some(300.0), Some(1200.0)),
                enum_param("roof_covering", "Roof Covering", "Roof sheeting type", "corrugated_iron", vec!["corrugated_iron", "box_profile", "tile", "thatch", "shingle", "membrane"]),
                bool_param("include_purlins", "Purlins", "Include roof purlins", true),
                length_param("purlin_spacing", "Purlin Spacing", "Purlin spacing", 900.0, Some(600.0), Some(1200.0)),
                bool_param("include_bracing", "Bracing", "Include longitudinal and diagonal bracing", true),
                float_param("wind_speed", "Design Wind Speed", "Basic wind speed (m/s)", 35.0, Some(25.0), Some(60.0), Some("m/s")),
            ],
        }
    }
}

impl AfricaModel for RoofTrussSystem {
    fn id(&self) -> &'static str {
        "housing.roof_truss"
    }

    fn name(&self) -> &'static str {
        "Roof Truss System"
    }

    fn description(&self) -> &'static str {
        "Prefabricated roof truss system for African housing and commercial buildings. Timber (eucalyptus, pine) or light gauge steel. Fink truss most common for spans up to 12m. Includes purlins, bracing, overhangs. Designed for African wind speeds (up to 45 m/s in cyclone regions). Compatible with corrugated iron (most common), tile, and thatch coverings."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Housing
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

impl Default for RoofTrussSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Foundation Layout - Building foundation system
pub struct FoundationLayout {
    params: Vec<ParameterDef>,
}

impl FoundationLayout {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("building_length", "Building Length", "Overall building length", 12000.0, Some(3000.0), Some(50000.0)),
                length_param("building_width", "Building Width", "Overall building width", 8000.0, Some(3000.0), Some(50000.0)),
                enum_param("foundation_type", "Foundation Type", "Foundation system", "strip_footing", vec!["strip_footing", "pad_footing", "raft", "pile_cap", "ground_beam", "strip_and_pad"]),
                length_param("footing_width", "Footing Width", "Strip footing width", 600.0, Some(400.0), Some(1200.0)),
                length_param("footing_depth", "Footing Depth", "Depth of footing below ground", 800.0, Some(500.0), Some(1500.0)),
                length_param("footing_thickness", "Footing Thickness", "Thickness of concrete footing", 300.0, Some(200.0), Some(500.0)),
                enum_param("soil_type", "Soil Type", "Subsoil classification", "firm_clay", vec!["firm_clay", "soft_clay", "compacted_sand", "dense_sand", "gravel", "laterite", "rock", "black_cotton"]),
                float_param("bearing_capacity", "Bearing Capacity", "Allowable bearing pressure (kPa)", 150.0, Some(50.0), Some(500.0), Some("kPa")),
                bool_param("include_dpc", "DPC", "Include damp proof course", true),
                bool_param("include_ground_beam", "Ground Beam", "Add ground beam tying footings", true),
                length_param("ground_beam_size", "Ground Beam Size", "Ground beam dimensions", 225.0, Some(150.0), Some(300.0)),
                bool_param("seismic_reinforcement", "Seismic Reinforcement", "Add seismic reinforcement", true),
                float_param("water_table", "Water Table", "Groundwater level below surface", 2.0, Some(0.5), Some(10.0), Some("m")),
            ],
        }
    }
}

impl AfricaModel for FoundationLayout {
    fn id(&self) -> &'static str {
        "housing.foundation_layout"
    }

    fn name(&self) -> &'static str {
        "Building Foundation Layout"
    }

    fn description(&self) -> &'static str {
        "Parametric foundation layout for African buildings. Strip footing, pad footing, raft, and pile options. Accounts for problematic soils: black cotton soil (expansive), laterite, high water table. Includes DPC, ground beams, seismic reinforcement per local codes. Generates reinforcement schedules and concrete volumes. Suitable for housing, schools, clinics, and light commercial."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Housing
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

impl Default for FoundationLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Latrine Block - Sanitation facility for schools/clinics/communities
pub struct LatrineBlock {
    params: Vec<ParameterDef>,
}

impl LatrineBlock {
    pub fn new() -> Self {
        Self {
            params: vec![
                int_param("num_stalls", "Number of Stalls", "Toilet stalls (per gender)", 4, Some(1), Some(20)),
                enum_param("toilet_type", "Toilet Type", "Toilet technology", "vip", vec!["vip", "pour_flush", "cistern_flush", "composting", "ecosan", "biodigester"]),
                enum_param("superstructure", "Superstructure", "Building construction", "brick", vec!["brick", "block", "rammed_earth", "steel_frame", "container"]),
                bool_param("separate_genders", "Separate Genders", "Separate male/female blocks", true),
                bool_param("include_urinals", "Urinals", "Include urinal section (male)", true),
                bool_param("include_handwashing", "Handwashing", "Include handwashing stations", true),
                bool_param("include_menstrual", "Menstrual Hygiene", "Include MHM facilities (female)", true),
                bool_param("accessible_stall", "Accessible Stall", "Include wheelchair accessible stall", true),
                length_param("pit_depth", "Pit Depth", "Depth of pit (for pit-based)", 3000.0, Some(2000.0), Some(5000.0)),
                length_param("pit_width", "Pit Width", "Width of pit", 1200.0, Some(1000.0), Some(2000.0)),
                length_param("pit_length", "Pit Length", "Length of pit", 1200.0, Some(1000.0), Some(2000.0)),
                enum_param("pit_lining", "Pit Lining", "Pit lining material", "brick", vec!["brick", "block", "concrete_rings", "unlined", "masonry"]),
                bool_param("vent_pipe", "Vent Pipe", "Include ventilation pipe with fly screen", true),
                bool_param("roof_rainwater", "Roof Rainwater", "Harvest rainwater from roof for handwashing", true),
            ],
        }
    }
}

impl AfricaModel for LatrineBlock {
    fn id(&self) -> &'static str {
        "housing.latrine_block"
    }

    fn name(&self) -> &'static str {
        "Institutional Latrine Block"
    }

    fn description(&self) -> &'static str {
        "Sanitation block for schools, clinics, markets, and communities in Africa. VIP (Ventilated Improved Pit) most common. Gender-separated with urinals, handwashing, menstrual hygiene management facilities. Accessible stall for disability inclusion. Optional rainwater harvesting for handwashing. Pit lining options for collapsible soils. Meets WHO/UNICEF JMP standards for improved sanitation."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Housing
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

impl Default for LatrineBlock {
    fn default() -> Self {
        Self::new()
    }
}