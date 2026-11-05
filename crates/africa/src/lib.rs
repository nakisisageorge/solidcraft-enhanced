//! SolidCraft African Domain Parametric Models
//!
//! This crate provides parametric 3D models tailored for African manufacturing,
//! construction, engineering, and infrastructure contexts. All models are built
//! on SolidCraft's parametric engine and can be customized through parameters.
//!
//! # Modules
//!
//! - `solar` — Solar energy infrastructure (panel mounts, tracking frames, roof systems)
//! - `water` — Water infrastructure (tanks, pump houses, pipe routing)
//! - `housing` — Modular housing (units, interlocking bricks, roof trusses)
//! - `agriculture` — Agricultural infrastructure (irrigation, greenhouses, silos)
//! - `manufacturing` — Manufacturing fixtures (machine bases, conveyors, jigs)
//! - `construction` — Construction systems (scaffolding, formwork, rebar templates)
//! - `common` — Shared types, utilities, and African context constants

pub mod solar;
pub mod water;
pub mod housing;
pub mod agriculture;
pub mod manufacturing;
pub mod construction;
pub mod common;

use solidcraft_engine::Backend;
use solidcore_record::Record;
use thiserror::Error;

/// Error type for African domain model generation
#[derive(Error, Debug)]
pub enum AfricaModelError {
    #[error("Parameter validation failed: {0}")]
    Validation(String),
    #[error("Model generation failed: {0}")]
    Generation(String),
    #[error("Engine error: {0}")]
    Engine(#[from] solidcraft_engine::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for African domain operations
pub type Result<T> = std::result::Result<T, AfricaModelError>;

/// Trait for all African parametric models
pub trait AfricaModel: Send + Sync {
    /// Unique identifier for this model
    fn id(&self) -> &'static str;

    /// Human-readable name
    fn name(&self) -> &'static str;

    /// Description of the model and its African context
    fn description(&self) -> &'static str;

    /// Category this model belongs to
    fn category(&self) -> AfricaCategory;

    /// Generate the model with the given parameters
    fn generate(&self, backend: &mut Backend, params: &ModelParameters) -> Result<Record>;

    /// Get the parameter schema for this model
    fn parameters_schema(&self) -> serde_json::Value;

    /// Validate parameters before generation
    fn validate_params(&self, params: &ModelParameters) -> Result<()>;
}

/// Categories of African domain models
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AfricaCategory {
    Solar,
    Water,
    Housing,
    Agriculture,
    Manufacturing,
    Construction,
}

/// Parameter container for model generation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelParameters {
    /// Model-specific parameters
    pub params: serde_json::Map<String, serde_json::Value>,
    /// Unit system (mm, cm, m, in, ft)
    pub units: LengthUnit,
    /// Geographic context (affects defaults: climate, seismic, materials)
    pub context: Option<GeographicContext>,
}

/// Length units supported
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LengthUnit {
    Mm,
    Cm,
    M,
    In,
    Ft,
}

impl LengthUnit {
    /// Conversion factor to millimeters
    pub fn to_mm_factor(&self) -> f64 {
        match self {
            LengthUnit::Mm => 1.0,
            LengthUnit::Cm => 10.0,
            LengthUnit::M => 1000.0,
            LengthUnit::In => 25.4,
            LengthUnit::Ft => 304.8,
        }
    }
}

/// Geographic context for African markets
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeographicContext {
    /// Country code (ISO 3166-1 alpha-2)
    pub country: Option<String>,
    /// Climate zone
    pub climate: Option<ClimateZone>,
    /// Seismic zone (0-4)
    pub seismic_zone: Option<u8>,
    /// Available materials
    pub materials: Vec<Material>,
    /// Local building codes reference
    pub building_code: Option<String>,
}

/// African climate zones
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClimateZone {
    TropicalRainforest,
    TropicalMonsoon,
    TropicalSavanna,
    HotDesert,
    HotSemiArid,
    SubtropicalHighland,
    Mediterranean,
    TemperateOceanic,
}

/// Common construction materials in African markets
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Material {
    Concrete,
    ReinforcedConcrete,
    CementBlock,
    InterlockingBrick,
    BurntBrick,
    Stone,
    Timber,
    Bamboo,
    Steel,
    GalvanizedSteel,
    Aluminum,
    CorrugatedIron,
    Thatch,
    RammedEarth,
    CompressedEarthBlock,
}

/// Registry of all available African models
pub struct AfricaModelRegistry {
    models: Vec<Box<dyn AfricaModel>>,
}

impl AfricaModelRegistry {
    /// Create a new registry with all built-in models
    pub fn new() -> Self {
        let mut registry = Self { models: Vec::new() };
        registry.register_builtin_models();
        registry
    }

    /// Register all built-in models
    fn register_builtin_models(&mut self) {
        // Solar models
        self.register(Box::new(solar::SolarPanelMount::new()));
        self.register(Box::new(solar::AdjustableTiltFrame::new()));
        self.register(Box::new(solar::RoofMountSystem::new()));
        self.register(Box::new(solar::GroundMountArray::new()));
        self.register(Box::new(solar::SolarWaterHeaterFrame::new()));

        // Water models
        self.register(Box::new(water::ElevatedWaterTank::new()));
        self.register(Box::new(water::UndergroundWaterTank::new()));
        self.register(Box::new(water::ModularWaterTank::new()));
        self.register(Box::new(water::PumpHouse::new()));
        self.register(Box::new(water::PipeRoutingSystem::new()));
        self.register(Box::new(water::RainwaterHarvesting::new()));

        // Housing models
        self.register(Box::new(housing::ModularHousingUnit::new()));
        self.register(Box::new(housing::InterlockingBrickWall::new()));
        self.register(Box::new(housing::RoofTrussSystem::new()));
        self.register(Box::new(housing::FoundationLayout::new()));
        self.register(Box::new(housing::LatrineBlock::new()));

        // Agriculture models
        self.register(Box::new(agriculture::DripIrrigationLayout::new()));
        self.register(Box::new(agriculture::GreenhouseFrame::new()));
        self.register(Box::new(agriculture::GrainSilo::new()));
        self.register(Box::new(agriculture::AnimalShelter::new()));
        self.register(Box::new(agriculture::CompostingToilet::new()));

        // Manufacturing models
        self.register(Box::new(manufacturing::MachineBase::new()));
        self.register(Box::new(manufacturing::ConveyorFrame::new()));
        self.register(Box::new(manufacturing::JigFixtureTemplate::new()));
        self.register(Box::new(manufacturing::WorkbenchSystem::new()));
        self.register(Box::new(manufacturing::ToolingPlate::new()));

        // Construction models
        self.register(Box::new(construction::ScaffoldingSystem::new()));
        self.register(Box::new(construction::FormworkPanel::new()));
        self.register(Box::new(construction::RebarBendingTemplate::new()));
        self.register(Box::new(construction::ShutteringSystem::new()));
        self.register(Box::new(construction::TowerCraneBase::new()));
    }

    /// Register a new model
    pub fn register(&mut self, model: Box<dyn AfricaModel>) {
        self.models.push(model);
    }

    /// Get all registered models
    pub fn models(&self) -> &[Box<dyn AfricaModel>] {
        &self.models
    }

    /// Get a model by ID
    pub fn get(&self, id: &str) -> Option<&dyn AfricaModel> {
        self.models.iter().find(|m| m.id() == id).map(|m| m.as_ref())
    }

    /// Get models by category
    pub fn by_category(&self, category: AfricaCategory) -> Vec<&dyn AfricaModel> {
        self.models
            .iter()
            .filter(|m| m.category() == category)
            .map(|m| m.as_ref())
            .collect()
    }

    /// Get all categories with model counts
    pub fn categories(&self) -> Vec<(AfricaCategory, usize)> {
        let mut counts = std::collections::HashMap::new();
        for model in &self.models {
            *counts.entry(model.category()).or_insert(0) += 1;
        }
        counts.into_iter().collect()
    }
}

impl Default for AfricaModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to generate a model by ID
pub fn generate_model(
    backend: &mut Backend,
    model_id: &str,
    params: &ModelParameters,
) -> Result<Record> {
    let registry = AfricaModelRegistry::new();
    let model = registry
        .get(model_id)
        .ok_or_else(|| AfricaModelError::Validation(format!("Unknown model: {}", model_id)))?;
    model.validate_params(params)?;
    model.generate(backend, params)
}

/// List all available models as JSON schema
pub fn list_models_schema() -> serde_json::Value {
    let registry = AfricaModelRegistry::new();
    let mut models = Vec::new();

    for model in registry.models() {
        models.push(serde_json::json!({
            "id": model.id(),
            "name": model.name(),
            "description": model.description(),
            "category": model.category(),
            "parameters": model.parameters_schema(),
        }));
    }

    serde_json::json!({ "models": models })
}