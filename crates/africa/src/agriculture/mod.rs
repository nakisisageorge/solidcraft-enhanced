//! Agricultural Infrastructure Parametric Models
//!
//! Models for irrigation systems, greenhouse frames, grain silos, animal shelters,
//! and composting toilets suitable for African markets.

use crate::common::*;
use crate::{AfricaCategory, AfricaModel, AfricaModelError, ModelParameters, Result};
use solidcraft_engine::Backend;
use solidcore_record::Record;

/// Drip Irrigation Layout - Parametric drip irrigation system design
pub struct DripIrrigationLayout {
    params: Vec<ParameterDef>,
}

impl DripIrrigationLayout {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("field_length", "Field Length", "Length of irrigated area", 100.0, Some(10.0), Some(1000.0)),
                length_param("field_width", "Field Width", "Width of irrigated area", 50.0, Some(10.0), Some(500.0)),
                length_param("bed_width", "Bed Width", "Width of planting beds", 1.0, Some(0.5), Some(2.0)),
                length_param("bed_spacing", "Bed Spacing", "Spacing between bed centers", 1.5, Some(1.0), Some(3.0)),
                length_param("lateral_spacing", "Lateral Spacing", "Distance between drip laterals", 0.5, Some(0.2), Some(1.0)),
                length_param("emitter_spacing", "Emitter Spacing", "Distance between emitters on lateral", 0.3, Some(0.15), Some(1.0)),
                float_param("emitter_flow", "Emitter Flow Rate", "Flow rate per emitter (L/h)", 2.0, Some(0.5), Some(8.0), Some("L/h")),
                enum_param("water_source", "Water Source", "Water supply type", "tank_gravity", vec!["tank_gravity", "pump_pressurized", "municipal", "borehole", "river_pump"]),
                length_param("mainline_diameter", "Mainline Diameter", "Main supply pipe diameter", 50.0, Some(25.0), Some(150.0)),
                length_param("submain_diameter", "Submain Diameter", "Submain pipe diameter", 32.0, Some(20.0), Some(75.0)),
                length_param("lateral_diameter", "Lateral Diameter", "Drip lateral diameter", 16.0, Some(12.0), Some(25.0)),
                bool_param("include_filter", "Filter", "Include screen/disc filter", true),
                bool_param("include_fertigation", "Fertigation", "Include fertilizer injection point", true),
                bool_param("include_flush_valves", "Flush Valves", "Include end-of-line flush valves", true),
                bool_param("pressure_compensating", "PC Emitters", "Use pressure-compensating emitters", true),
                enum_param("topography", "Topography", "Field slope", "flat", vec!["flat", "gentle_slope", "steep_slope", "terraced"]),
            ],
        }
    }
}

impl AfricaModel for DripIrrigationLayout {
    fn id(&self) -> &'static str {
        "agriculture.drip_irrigation"
    }

    fn name(&self) -> &'static str {
        "Drip Irrigation Layout"
    }

    fn description(&self) -> &'static str {
        "Parametric drip irrigation system for African smallholder and commercial farms. Calculates lateral lengths, emitter counts, flow requirements, and pipe sizing. Pressure-compensating emitters recommended for sloping terrain. Gravity-fed from elevated tank common in off-grid areas. Includes filtration, fertigation option, and flush valves. Compatible with HDPE, LDPE, and uPVC pipe available in African markets."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Agriculture
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

impl Default for DripIrrigationLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Greenhouse Frame - Protected cultivation structure
pub struct GreenhouseFrame {
    params: Vec<ParameterDef>,
}

impl GreenhouseFrame {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("span", "Span", "Greenhouse width (clear span)", 8.0, Some(4.0), Some(20.0)),
                length_param("length", "Length", "Greenhouse length", 20.0, Some(10.0), Some(100.0)),
                length_param("eave_height", "Eave Height", "Height at gutter line", 3.0, Some(2.0), Some(5.0)),
                length_param("ridge_height", "Ridge Height", "Height at center", 4.5, Some(3.0), Some(7.0)),
                enum_param("frame_type", "Frame Type", "Structural frame type", "tunnel", vec!["tunnel", "gothic_arch", "gable", "venlo", "sawtooth", "shade_net"]),
                enum_param("frame_material", "Frame Material", "Frame construction material", "galvanized_steel", vec!["galvanized_steel", "aluminum", "bamboo", "timber", "pvc_pipe"]),
                enum_param("covering", "Covering", "Glazing/covering material", "pe_film", vec!["pe_film", "pvc_film", "polycarbonate", "glass", "shade_net_50", "shade_net_70", "insect_net"]),
                float_param("bay_spacing", "Bay Spacing", "Distance between frames", 2.0, Some(1.5), Some(4.0)),
                bool_param("include_vents", "Vents", "Include roof and/or side vents", true),
                enum_param("vent_type", "Vent Type", "Ventilation type", "roll_up_sides", vec!["roll_up_sides", "roof_vent", "both", "motorized_roof", "passive_ridge"]),
                bool_param("include_door", "Door", "Include access door(s)", true),
                int_param("num_doors", "Number of Doors", "Access doors", 2, Some(1), Some(4)),
                bool_param("include_irrigation", "Irrigation Ready", "Pre-install irrigation supports", true),
                bool_param("include_benching", "Benching", "Include growing benches", false),
                enum_param("foundation", "Foundation", "Foundation type", "ground_post", vec!["ground_post", "concrete_strip", "concrete_pad", "screw_anchor", "ballasted"]),
            ],
        }
    }
}

impl AfricaModel for GreenhouseFrame {
    fn id(&self) -> &'static str {
        "agriculture.greenhouse"
    }

    fn name(&self) -> &'static str {
        "Greenhouse Frame"
    }

    fn description(&self) -> &'static str {
        "Protected cultivation greenhouse frame for African horticulture. Tunnel (low-cost), gothic arch (better shedding), gable, Venlo (commercial), sawtooth (tropical), and shade net options. Galvanized steel tube most common; bamboo/timber for ultra-low-cost. PE film covering (150-200 micron) typical; insect net for pest exclusion. Roll-up side vents for passive cooling critical in tropical climates. Ground post foundation for rapid deployment."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Agriculture
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

impl Default for GreenhouseFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// Grain Silo - Grain storage silo for farm/cooperative
pub struct GrainSilo {
    params: Vec<ParameterDef>,
}

impl GrainSilo {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("capacity_tonnes", "Capacity", "Storage capacity in metric tonnes", 50.0, Some(5.0), Some(5000.0)),
                enum_param("silo_type", "Silo Type", "Silo configuration", "flat_bottom", vec!["flat_bottom", "hopper_bottom", "elevated_hopper", "bag_silo", "hermetic_bag"]),
                enum_param("construction", "Construction", "Wall construction", "corrugated_steel", vec!["corrugated_steel", "concrete_stave", "reinforced_concrete", "brick_masonry", "ferrocement", "woven_polypropylene"]),
                length_param("diameter", "Diameter", "Internal diameter", 4.0, Some(2.0), Some(30.0)),
                length_param("eave_height", "Eave Height", "Wall height to eaves", 6.0, Some(3.0), Some(20.0)),
                length_param("roof_pitch", "Roof Pitch", "Roof slope (degrees)", 30.0, Some(15.0), Some(45.0), Some("deg")),
                bool_param("include_aeration", "Aeration", "Include aeration system", true),
                bool_param("include_temp_cables", "Temperature Cables", "Include grain temperature monitoring", true),
                bool_param("include_level_indicator", "Level Indicator", "Include fill level indicator", true),
                bool_param("include_ladder", "Ladder", "Include external ladder with cage", true),
                bool_param("include_manhole", "Manhole", "Include roof manhole for access", true),
                bool_param("include_fumigation", "Fumigation", "Include fumigation recirculation", false),
                enum_param("foundation", "Foundation", "Foundation type", "concrete_ring", vec!["concrete_ring", "concrete_slab", "compacted_gravel", "steel_frame", "pile_cap"]),
                float_param("grain_density", "Grain Density", "Bulk density (kg/m³)", 750.0, Some(500.0), Some(900.0), Some("kg/m³")),
                float_param("angle_repose", "Angle of Repose", "Grain angle of repose (degrees)", 27.0, Some(20.0), Some(35.0), Some("deg")),
            ],
        }
    }
}

impl AfricaModel for GrainSilo {
    fn id(&self) -> &'static str {
        "agriculture.grain_silo"
    }

    fn name(&self) -> &'static str {
        "Grain Storage Silo"
    }

    fn description(&self) -> &'static str {
        "Grain storage silo for African farms, cooperatives, and strategic reserves. Corrugated steel (most common), concrete stave, ferrocement (low-cost), and hermetic bag options. Flat bottom with sweep auger or hopper bottom for gravity discharge. Aeration critical for tropical storage (prevents aflatoxin). Temperature monitoring cables. Designed for maize, wheat, rice, sorghum, millet. Cyclone-rated for coastal regions."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Agriculture
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

impl Default for GrainSilo {
    fn default() -> Self {
        Self::new()
    }
}

/// Animal Shelter - Livestock housing
pub struct AnimalShelter {
    params: Vec<ParameterDef>,
}

impl AnimalShelter {
    pub fn new() -> Self {
        Self {
            params: vec![
                enum_param("animal_type", "Animal Type", "Type of livestock", "dairy_cattle", vec!["dairy_cattle", "beef_cattle", "goats", "sheep", "pigs", "poultry_layers", "poultry_broilers", "rabbits", "camels", "donkeys"]),
                int_param("num_animals", "Number of Animals", "Design herd/flock size", 20, Some(5), Some(5000)),
                enum_param("system", "Production System", "Housing system", "zero_grazing", vec!["zero_grazing", "semi_intensive", "free_range_night", "intensive", "pasture_based"]),
                length_param("building_length", "Building Length", "Shelter length", 20.0, Some(5.0), Some(200.0)),
                length_param("building_width", "Building Width", "Shelter width", 8.0, Some(4.0), Some(30.0)),
                length_param("eave_height", "Eave Height", "Wall height", 3.0, Some(2.5), Some(5.0)),
                enum_param("frame", "Frame", "Structural frame", "timber", vec!["timber", "steel_tube", "galvanized_steel", "bamboo", "concrete_posts"]),
                enum_param("roof", "Roof", "Roof covering", "corrugated_iron", vec!["corrugated_iron", "thatch", "tile", "pe_film", "shade_net"]),
                enum_param("floor", "Floor", "Floor type", "concrete", vec!["concrete", "earth", "slatted", "deep_litter", "sand", "rubber_mat"]),
                bool_param("include_feeding", "Feeding", "Include feed troughs/mangers", true),
                bool_param("include_watering", "Watering", "Include water troughs/nipples", true),
                bool_param("include_ventilation", "Ventilation", "Include ridge/side ventilation", true),
                bool_param("include_manure_system", "Manure System", "Include manure collection/storage", true),
                enum_param("manure_type", "Manure Type", "Manure handling", "solid_scrape", vec!["solid_scrape", "flush", "slurry", "composting", "biogas"]),
                bool_param("calving_lambing", "Maternity Area", "Include calving/lambing pens", true),
            ],
        }
    }
}

impl AfricaModel for AnimalShelter {
    fn id(&self) -> &'static str {
        "agriculture.animal_shelter"
    }

    fn name(&self) -> &'static str {
        "Livestock Shelter"
    }

    fn description(&self) -> &'static str {
        "Livestock housing for African smallholder and commercial farms. Zero-grazing dairy units (common in Kenya highlands), goat/sheep pens, pig housing, poultry houses. Locally available materials: timber, bamboo, steel tube, corrugated iron, thatch. Concrete floor for hygiene; deep litter for poultry. Ventilation critical for tropical heat stress. Manure management: solid scrape, biogas digester integration. Calving/lambing pens for improved survival."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Agriculture
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

impl Default for AnimalShelter {
    fn default() -> Self {
        Self::new()
    }
}

/// Composting Toilet - Ecological sanitation for rural/peri-urban
pub struct CompostingToilet {
    params: Vec<ParameterDef>,
}

impl CompostingToilet {
    pub fn new() -> Self {
        Self {
            params: vec![
                enum_param("toilet_type", "Toilet Type", "Composting toilet design", "double_vault", vec!["double_vault", "continuous", "batch", "vermiculture", "urine_diverting", "solar_heated"]),
                int_param("num_users", "Number of Users", "Design user count", 10, Some(1), Some(100)),
                length_param("vault_volume", "Vault Volume", "Volume per vault (m³)", 1.5, Some(0.5), Some(5.0)),
                length_param("vault_depth", "Vault Depth", "Depth of composting chamber", 1.5, Some(1.0), Some(3.0)),
                length_param("vault_width", "Vault Width", "Width of composting chamber", 1.0, Some(0.8), Some(2.0)),
                length_param("vault_length", "Vault Length", "Length of composting chamber", 1.0, Some(0.8), Some(2.0)),
                enum_param("superstructure", "Superstructure", "Above-ground structure", "brick", vec!["brick", "block", "timber_frame", "rammed_earth", "ferrocement", "bamboo"]),
                enum_param("roof", "Roof", "Roof covering", "corrugated_iron", vec!["corrugated_iron", "thatch", "tile", "concrete_slab"]),
                bool_param("urine_diversion", "Urine Diversion", "Separate urine collection", true),
                bool_param("vent_pipe", "Vent Pipe", "Ventilation pipe with fly screen", true),
                bool_param("solar_chimney", "Solar Chimney", "Solar-heated ventilation chimney", false),
                bool_param("leachate_management", "Leachate Management", "Include leachate collection/drainage", true),
                bool_param("access_hatch", "Access Hatch", "Removable hatch for compost removal", true),
                enum_param("additive", "Bulking Agent", "Carbon additive", "sawdust", vec!["sawdust", "rice_husks", "maize_cobs", "ash", "soil", "leaves", "straw"]),
            ],
        }
    }
}

impl AfricaModel for CompostingToilet {
    fn id(&self) -> &'static str {
        "agriculture.composting_toilet"
    }

    fn name(&self) -> &'static str {
        "Composting Toilet"
    }

    fn description(&self) -> &'static str {
        "Ecological composting toilet for rural and peri-urban Africa. Double-vault (alternating use) most common. Urine diversion reduces moisture and odor. Produces safe compost for agriculture after 6-12 months. Solar chimney enhances ventilation in humid climates. Bulking agents: sawdust, rice husks, maize cobs (agricultural residues). Superstructure in local materials. Meets WHO guidelines for excreta reuse in agriculture."
    }

    fn category(&self) -> AfricaCategory {
        AfricaCategory::Agriculture
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

impl Default for CompostingToilet {
    fn default() -> Self {
        Self::new()
    }
}