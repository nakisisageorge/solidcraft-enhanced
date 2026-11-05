# AFRICAN MODELS GUIDE — solidcraft-enhanced

## Overview

The `solidcraft-africa` crate provides 30+ parametric models tailored for African manufacturing, construction, engineering, and infrastructure contexts. All models are built on SolidCraft's parametric engine and can be customized through parameters.

## Model Categories

### 🌞 Solar Energy (5 models)
| Model ID | Name | Description | Key Parameters |
|----------|------|-------------|----------------|
| `solar.panel_mount` | Solar Panel Ground Mount | Fixed-tilt ground mounting for PV panels | Panel dims, tilt, azimuth, rows, foundation |
| `solar.adjustable_tilt_frame` | Adjustable Tilt Frame | Seasonal tilt adjustment for PV panels | Min/max tilt, positions, actuation type |
| `solar.roof_mount` | Roof Mount System | Residential/commercial roof mounting | Roof type, pitch, attachment, flashings |
| `solar.ground_mount_array` | Utility-Scale Ground Mount Array | Large-scale solar farm with tracking option | Arrays, tracking, spacing, fencing, roads |
| `solar.water_heater_frame` | Solar Water Heater Frame | Thermosiphon solar water heating frame | Tank dims, collector dims, mount type |

### 💧 Water Infrastructure (6 models)
| Model ID | Name | Description | Key Parameters |
|----------|------|-------------|----------------|
| `water.elevated_tank` | Elevated Water Tank | Steel tank on braced tower | Capacity, tower height, legs, seismic |
| `water.underground_tank` | Underground Water Tank | Buried concrete/steel tank | Capacity, material, chambers, water table |
| `water.modular_tank` | Modular Bolted Panel Tank | Bolted panel tank for rapid deployment | Panel type, rings, roof, foundation |
| `water.pump_house` | Water Pump House | Building for pumping equipment | Pumps, crane beam, ventilation, security |
| `water.pipe_routing` | Water Pipe Routing System | Parametric pipe network layout | Diameters, material, terrain, valves |
| `water.rainwater_harvesting` | Rainwater Harvesting System | Complete RWH system for buildings | Roof area, tank type, first flush, usage |

### 🏠 Housing (5 models)
| Model ID | Name | Description | Key Parameters |
|----------|------|-------------|----------------|
| `housing.modular_unit` | Modular Housing Unit | Prefabricated modular housing | Construction, rooms, sanitation, verandah |
| `housing.interlocking_brick_wall` | Interlocking Brick Wall | Compressed earth block wall system | Brick type, cement ratio, ring beam, openings |
| `housing.roof_truss` | Roof Truss System | Prefabricated timber/steel trusses | Span, pitch, type, material, covering |
| `housing.foundation_layout` | Building Foundation Layout | Parametric foundation system | Type, soil, bearing capacity, seismic |
| `housing.latrine_block` | Institutional Latrine Block | Sanitation for schools/clinics | Stalls, toilet type, gender separation, MHM |

### 🌾 Agriculture (5 models)
| Model ID | Name | Description | Key Parameters |
|----------|------|-------------|----------------|
| `agriculture.drip_irrigation` | Drip Irrigation Layout | Parametric drip irrigation design | Field size, emitter spacing, water source |
| `agriculture.greenhouse` | Greenhouse Frame | Protected cultivation structure | Span, length, frame type, covering, vents |
| `agriculture.grain_silo` | Grain Storage Silo | Farm/cooperative grain storage | Capacity, type, construction, aeration |
| `agriculture.animal_shelter` | Livestock Shelter | Housing for various livestock | Animal type, system, frame, manure handling |
| `agriculture.composting_toilet` | Composting Toilet | Ecological sanitation | Type, vault volume, urine diversion, additive |

### 🏭 Manufacturing (5 models)
| Model ID | Name | Description | Key Parameters |
|----------|------|-------------|----------------|
| `manufacturing.machine_base` | Machine Base | Heavy-duty base for industrial machinery | Construction, plate thickness, ribs, vibration isolation |
| `manufacturing.conveyor_frame` | Conveyor Frame System | Modular conveyor for bulk handling | Type, belt width, frame material, idlers |
| `manufacturing.jig_fixture` | Jig & Fixture Template | Manufacturing jig/fixture templates | Type, workpiece size, clamping, modularity |
| `manufacturing.workbench` | Industrial Workbench System | Modular workshop workbench | Top material, frame, storage, power, lighting |
| `manufacturing.tooling_plate` | Precision Tooling Plate | Modular fixturing plate | Material, hole pattern, T-slots, dowels |

### 🏗️ Construction (5 models)
| Model ID | Name | Description | Key Parameters |
|----------|------|-------------|----------------|
| `construction.scaffolding` | Modular Scaffolding System | Kwikstage/Cuplock/Ringlock scaffolding | Bay size, lifts, type, material, ties |
| `construction.formwork_panel` | Concrete Formwork Panel System | Modular formwork for walls/slabs | Panel type, face material, ties, walers |
| `construction.rebar_bending` | Rebar Bending Schedule Template | BS 8666/SANS 282 bending schedules | Element type, bar sizes, spacing, curtailment |
| `construction.shuttering` | Concrete Shuttering System | Formwork for slabs/beams/columns | System, prop type, spacing, cycle time |
| `construction.tower_crane_base` | Tower Crane Base Foundation | Crane foundation for high-rise | Crane type, capacity, base type, soil |

---

## Common Parameters

### Geographic Context
All models accept an optional `context` parameter for geographic defaults:

```json
{
  "context": {
    "country": "KE",
    "climate": "tropical_savanna",
    "seismic_zone": 2,
    "materials": ["concrete", "steel", "corrugated_iron", "stone"],
    "building_code": "KS 02-1070"
  }
}
```

### Country Defaults (10 African Countries)
| Country | Climate Zone | Seismic | Common Materials |
|---------|--------------|---------|------------------|
| Kenya (KE) | Tropical Savanna | 2 | Concrete, Steel, Corrugated Iron, Stone |
| Nigeria (NG) | Tropical Monsoon | 1 | Concrete, Cement Block, Steel, Burnt Brick |
| South Africa (ZA) | Mediterranean | 1 | Concrete, Steel, Brick, Timber |
| Egypt (EG) | Hot Desert | 2 | Concrete, Steel, Stone, Brick |
| Ethiopia (ET) | Subtropical Highland | 3 | Stone, Concrete, CEB, Eucalyptus |
| Ghana (GH) | Tropical Monsoon | 1 | Concrete, Cement Block, Steel, Bamboo |
| Uganda (UG) | Tropical Savanna | 2 | Interlocking Brick, Concrete, Steel, Timber |
| Tanzania (TZ) | Tropical Savanna | 2 | Cement Block, Concrete, Steel, Corrugated Iron |
| Rwanda (RW) | Subtropical Highland | 2 | CEB, Stone, Concrete, Steel |

### Units
All length parameters support multiple units:
- `mm` (default)
- `cm`
- `m`
- `in`
- `ft`

```json
{
  "params": { "panel_width": 1000 },
  "units": "mm"
}
```

---

## Model Customization

### Parameter Expressions
Most dimensions accept expressions with parameters:

```json
{
  "params": {
    "panel_width": "L",
    "panel_height": "L * 1.7",
    "tilt_angle": "latitude * 0.9"
  }
}
```

### Parameter References
Models define parameters that can be referenced:
```json
{
  "params": {
    "L": 1000,           // Base length parameter
    "panel_width": "L",
    "panel_height": "L * 1.7",
    "row_spacing": "panel_height * cos(tilt_angle) * 1.5"
  }
}
```

### Conditional Parameters
Some parameters depend on others:
```json
{
  "params": {
    "include_bracing": true,
    "bracing_size": "if(include_bracing, 60, 0)",
    "bracing_thickness": "if(include_bracing, 3, 0)"
  }
}
```

---

## Usage Examples

### CLI Generation
```bash
# Generate solar panel mount via CLI
solidcraft-cli exec africa.models.generate '{"model_id": "solar.panel_mount", "params": {"panel_width": 1000, "tilt_angle": 15}}'

# Generate with geographic context
solidcraft-cli exec africa.models.generate '{"model_id": "water.elevated_tank", "params": {"capacity_liters": 50000}, "context": {"country": "UG"}}'
```

### MCP Tool Calls
```json
// List all models
{"name": "africa.models.list", "arguments": {}}

// Get schema for solar mount
{"name": "africa.models.schema", "arguments": {"model_id": "solar.panel_mount"}}

// Generate with custom params
{"name": "africa.models.generate", "arguments": {"model_id": "solar.panel_mount", "params": {"panel_width": 1000, "tilt_angle": 12.5, "foundation_type": "ground_screw"}}}

// Validate parameters
{"name": "africa.models.validate", "arguments": {"model_id": "solar.panel_mount", "params": {"panel_width": 1000, "tilt_angle": 15}}}
```

### Batch Generation
```json
// Generate multiple models in one batch
{
  "name": "batch",
  "arguments": {
    "commands": [
      {"command": "africa.models.generate", "params": {"model_id": "solar.panel_mount", "params": {"panels_per_row": 6, "num_rows": 4}}},
      {"command": "africa.models.generate", "params": {"model_id": "water.elevated_tank", "params": {"capacity_liters": 100000}}},
      {"command": "africa.models.generate", "params": {"model_id": "housing.modular_unit", "params": {"num_rooms": 3, "include_verandah": true}}}
    ],
    "coalesce_parameters": true
  }
}
```

---

## Design Guidelines for African Contexts

### Solar
- **Tilt Angle**: Near equator (0-15°), higher latitudes (latitude ± 15°)
- **Tracking**: Single-axis for >50kW, fixed for <50kW
- **Foundation**: Ground screw for speed, concrete pad for permanence
- **Bracing**: Essential for wind loads (>35 m/s in cyclones)

### Water
- **Elevated Tanks**: 12-15m tower for 1-2 bar pressure
- **Underground Tanks**: Buoyancy check for high water table
- **Pipe Material**: HDPE for flexibility, uPVC for cost
- **Rainwater**: First flush diverter mandatory

### Housing
- **Interlocking Brick**: 5-7% cement, dry-stack, ring beam required
- **Roof Truss**: Fink truss up to 12m span, 22.5° pitch for corrugated iron
- **Foundation**: Strip footing for firm soil, raft for black cotton soil
- **Sanitation**: VIP latrine standard, MHM facilities for schools

### Agriculture
- **Drip Irrigation**: PC emitters for slopes, gravity-fed from 2m tank
- **Greenhouse**: Roll-up sides for passive cooling, insect net for pests
- **Grain Silo**: Aeration critical for aflatoxin prevention
- **Animal Shelter**: Ridge ventilation, deep litter for poultry

### Manufacturing
- **Machine Base**: Epoxy granite for precision, welded steel for general
- **Conveyor**: Troughed idlers, belt cleaner for sticky materials
- **Jigs**: Poka-yoke mandatory, modular for high-mix

### Construction
- **Scaffolding**: Kwikstage common, tie every 4m or 2 lifts
- **Formwork**: Steel frame + film-faced ply, crane-handled panels
- **Rebar**: 450MPa standard, lap splices per BS 8666
- **Crane Base**: Holding-down bolts, settlement < 25mm

---

## Integration with MCP

### Tool: `africa.models.list`
```json
{
  "name": "africa.models.list",
  "arguments": {}
}
```

### Tool: `africa.models.schema`
```json
{
  "name": "africa.models.schema",
  "arguments": {"model_id": "solar.panel_mount"}
}
```

### Tool: `africa.models.generate`
```json
{
  "name": "africa.models.generate",
  "arguments": {
    "model_id": "solar.panel_mount",
    "params": {
      "panel_width": 1000,
      "panel_height": 1700,
      "panels_per_row": 4,
      "num_rows": 3,
      "tilt_angle": 15,
      "foundation_type": "ground_screw"
    }
  }
}
```

### Tool: `africa.models.validate`
```json
{
  "name": "africa.models.validate",
  "arguments": {
    "model_id": "solar.panel_mount",
    "params": {
      "panel_width": 1000,
      "tilt_angle": 15
    }
  }
}
```

### Tool: `africa.models.categories`
```json
{
  "name": "africa.models.categories",
  "arguments": {}
}
```

---

## Performance Tips

1. **Batch Operations**: Use `batch` command for multiple models
2. **Parameter Coalescing**: Set `coalesce_parameters: true` for batch
3. **Validation First**: Call `validate` before `generate`
4. **Reuse Designs**: Save generated designs with `file.save`
5. **Selective Measurement**: Use `measure: false` in `inspect_design` for speed

---

## Extending with Custom Models

To add a new African model:

1. Create new file in appropriate category module
2. Implement `AfricaModel` trait
3. Define parameters in `new()` constructor
4. Implement `generate()` with SolidCraft commands
5. Register in `AfricaModelRegistry::register_builtin_models()`

```rust
pub struct MyCustomModel {
    params: Vec<ParameterDef>,
}

impl MyCustomModel {
    pub fn new() -> Self {
        Self {
            params: vec![
                length_param("dimension", "Dimension", "Description", 1000.0, Some(100.0), Some(5000.0)),
                // ...
            ],
        }
    }
}

impl AfricaModel for MyCustomModel {
    fn id(&self) -> &'static str { "category.my_custom_model" }
    fn name(&self) -> &'static str { "My Custom Model" }
    fn category(&self) -> AfricaCategory { AfricaCategory::Manufacturing }
    // ... implement other methods
}
```

Then add to registry:
```rust
self.register(Box::new(MyCustomModel::new()));
```

---

## Resources

- **GitHub**: https://github.com/nakisisageorge/solidcraft-enhanced
- **Architecture Viz**: `docs/architecture/index.html`
- **MCP Guide**: `MCP_GUIDE.md`
- **Original SolidCraft**: https://github.com/tangency-labs/solidcraft