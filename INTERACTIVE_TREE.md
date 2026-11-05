# INTERACTIVE DOCUMENTATION TREE — solidcraft-enhanced

## Overview

The interactive documentation tree provides a navigable, expandable/collapsible view of all project documentation, organized by architectural layer and function. It complements the architecture visualization by providing text-based navigation with search and filtering.

## Tree Structure

```
solidcraft-enhanced/
├── 📁 FOUNDATION LAYER
│   ├── 📄 solidcraft-geom — Core geometry primitives
│   ├── 📄 solidcraft-sketch — 2D constraint solver
│   └── 📄 solidcraft-kernel — B-rep modeling kernel (SMLib)
│
├── 📁 CORE LAYER
│   ├── 📄 solidcraft-engine — Session, commands, undo/redo
│   ├── 📄 solidcraft-doc — Document format & serialization
│   ├── 📄 solidcraft-io — Import/export (STEP, IGES, STL, 3MF)
│   ├── 📄 solidcraft-render — wgpu/egui rendering & tessellation
│   └── 📄 solidcraft-gpu — GPU abstraction & headless rendering
│
├── 📁 INFRASTRUCTURE LAYER
│   ├── 📄 solidcraft-derive — Proc macros for commands
│   ├── 📄 solidcraft-profile — Profiling & Chrome trace export
│   └── 📄 solidcraft-testkit — Testing utilities & snapshots
│
├── 📁 DOMAIN EXTENSION LAYER
│   ├── 📄 solidcraft-africa — 30+ African parametric models
│   ├── 📄 solidcraft-benchmark — Benchmarks & regression detection
│   └── 📄 solidcraft-mcp — MCP server (stdio/HTTP/SSE/WS)
│
├── 📁 MCP TRANSPORT LAYER
│   ├── 📄 stdio transport — Legacy JSON-RPC over stdio
│   ├── 📄 HTTP transport — RESTful JSON-RPC with sessions
│   ├── 📄 SSE transport — Server-Sent Events streaming
│   └── 📄 WebSocket transport — Bidirectional communication
│
├── 📁 APPLICATION LAYER
│   ├── 📄 solidcraft-cli — Headless CLI & MCP server
│   ├── 📄 solidcraft-web — WASM browser CAD app
│   └── 📄 solidcraft (desktop) — Native egui desktop app
│
├── 📁 EXTERNAL DEPENDENCIES
│   ├── 📄 solidcore — NVIDIA SMLib B-rep kernel
│   ├── 📄 egui — Immediate mode GUI
│   ├── 📄 wgpu — Cross-platform GPU API
│   ├── 📄 wasm-bindgen — Rust ↔ JS/WASM interop
│   └── 📄 axum — HTTP/SSE/WS server framework
│
├── 📁 DOCUMENTATION
│   ├── 📄 README.md — Project overview & quick start
│   ├── 📄 DESIGN_DOCUMENT.md — Architecture decisions
│   ├── 📄 PORTFOLIO_CASE_STUDY.md — Problem → Solution → Results
│   ├── 📄 BEFORE_AFTER.md — Detailed comparison tables
│   ├── 📄 MCP_GUIDE.md — Transport setup & AI integration
│   ├── 📄 AFRICAN_MODELS_GUIDE.md — Model catalog & customization
│   ├── 📄 ARCHITECTURE_VIZ.md — Visualization guide
│   ├── 📄 THIS_FILE.md — Interactive tree (this file)
│   ├── 📄 ARCHITECTURE/
│   │   └── 🌐 index.html — Interactive React Flow visualization
│   ├── 📄 API/
│   │   ├── 📄 COMMANDS.md — Full command reference
│   │   ├── 📄 MCP_TOOLS.md — MCP tool reference
│   │   └── 📄 AFRICAN_MODELS.md — African model API
│   └── 📄 GUIDES/
│       ├── 📄 GETTING_STARTED.md — Quick start guide
│       ├── 📄 CLI_USAGE.md — CLI command reference
│       ├── 📄 MCP_INTEGRATION.md — AI agent integration
│       ├── 📄 AFRICAN_MODELS_USAGE.md — Model customization
│       ├── 📄 BENCHMARKING.md — Benchmark runner guide
│       ├── 📄 SECURITY.md — Security configuration
│       └── 📄 CONTRIBUTING.md — Contribution guidelines
│
├── 📁 CI/CD & RELEASE
│   ├── 📄 .github/workflows/ci.yml — Main CI pipeline
│   ├── 📄 .github/workflows/release.yml — Release automation
│   ├── 📄 .github/workflows/security.yml — Security scanning
│   ├── 📄 .github/workflows/benchmark.yml — Regression detection
│   └── 📄 .github/workflows/docs.yml — Documentation build
│
├── 📁 SECURITY
│   ├── 📄 SECURITY.md — Security policy
│   ├── 📄 DEPENDENCY_POLICY.md — Dependency management
│   ├── 📄 SBOM/ — Software Bill of Materials
│   └── 📄 THREAT_MODEL.md — Threat modeling
│
├── 📁 TESTS
│   ├── 📄 UNIT/ — Per-crate unit tests
│   ├── 📄 INTEGRATION/ — CLI, MCP, benchmark tests
│   ├── 📄 KERNEL_CASES/ — 200+ SMLib regression cases
│   ├── 📄 PROPERTY/ — Property-based tests
│   ├── 📄 VISUAL/ — Snapshot comparison tests
│   ├── 📄 WASM/ — wasm-bindgen-test suite
│   └── 📄 BENCHMARK/ — Criterion benchmarks
│
├── 📁 EXAMPLES
│   ├── 📄 SOLAR_FARM/ — 100kW solar farm layout
│   ├── 📄 WATER_SUPPLY/ — Elevated tank + pipe network
│   ├── 📄 HOUSING_DEV/ — 50-unit modular housing
│   ├── 📄 AGRICULTURE/ — Drip irrigation + greenhouse
│   ├── 📄 MANUFACTURING/ — Machine base + conveyor
│   └── 📄 CONSTRUCTION/ — Scaffolding + formwork + rebar
│
├── 📁 ASSETS
│   ├── 📄 ICONS/ — App icons (PNG, ICO, SVG)
│   ├── 📄 SCREENSHOTS/ — Documentation screenshots
│   ├── 📄 DIAGRAMS/ — Architecture diagrams
│   └── 📄 LOGOS/ — SolidCraft & GICT Africa logos
│
└── 📁 CONFIGURATION
    ├── 📄 Cargo.toml — Workspace root
    ├── 📄 Cargo.lock — Locked dependencies
    ├── 📄 rustfmt.toml — Formatting config
    ├── 📄 clippy.toml — Linting config
    ├── 📄 deny.toml — cargo-deny config
    ├── 📄 .cargo/config.toml — Cargo config
    └── 📄 .editorconfig — Editor config
```

## Navigation Features

### Expand/Collapse
- Click folder icons (📁) to expand/collapse
- Click file icons (📄) to view file content
- Keyboard: `←`/`→` to collapse/expand

### Search
- Type in search box to filter tree
- Matches file/folder names and descriptions
- Highlights matches in results

### Filtering
- **Layer Filter**: Show only specific architectural layer
- **Type Filter**: Show only files, folders, or both
- **Status Filter**: Show new/modified/deprecated items

### Breadcrumbs
- Shows current path at top
- Click any level to navigate up

## File Content Viewer

Click any file (📄) to view:
- **Markdown**: Rendered with syntax highlighting
- **Code**: Syntax highlighted with line numbers
- **HTML**: Rendered preview (architecture viz)
- **Images**: Full-size view with zoom
- **TOML/YAML**: Formatted with syntax highlighting

## Integration with Architecture Visualization

The tree and visualization are linked:
- Click node in visualization → Highlights corresponding tree item
- Click tree item → Highlights corresponding node in visualization
- Sync button to toggle synchronization

## Search Index

The tree includes a full-text search index covering:
- File/folder names
- Descriptions
- Code symbols (functions, structs, traits)
- Configuration keys
- Command names
- Tool names

## Accessibility

- **Keyboard Navigation**: Full tree navigable via keyboard
- **Screen Readers**: ARIA tree role with proper labels
- **High Contrast**: Supports system high contrast mode
- **Font Scaling**: Respects browser zoom/settings
- **Reduced Motion**: Respects `prefers-reduced-motion`

## Implementation

The interactive tree is implemented as a React component using:
- **React 18** for rendering
- **React Arborist** for tree components
- **Fuse.js** for fuzzy search
- **React Markdown** for content rendering
- **Prism.js** for syntax highlighting

## Customization

### Adding New Sections
Edit the tree data structure in the component:

```javascript
const treeData = [
  {
    id: 'new_section',
    name: 'NEW SECTION',
    type: 'folder',
    children: [
      { id: 'new_file', name: 'new_file.md', type: 'file', description: '...' }
    ]
  }
];
```

### Custom Icons
Add custom icons per file type:

```javascript
const fileIcons = {
  '.md': '📄',
  '.rs': '🦀',
  '.toml': '⚙️',
  '.yml': '📋',
  '.html': '🌐',
  '.png': '🖼️',
  '.svg': '🎨',
};
```

### Custom Colors
Per-layer color coding:

```javascript
const layerColors = {
  'FOUNDATION LAYER': '#58a6ff',
  'CORE LAYER': '#a371f7',
  // ...
};
```

## Access

The interactive tree is available at:
- **Local**: `docs/architecture/tree.html` (when served)
- **Embedded**: In architecture visualization sidebar
- **Standalone**: `docs/documentation-tree.html`

## License

Part of solidcraft-enhanced under MIT OR Apache-2.0 license.