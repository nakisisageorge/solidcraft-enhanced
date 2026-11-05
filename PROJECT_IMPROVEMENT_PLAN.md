# PROJECT IMPROVEMENT PLAN — solidcraft-enhanced

## Original Project
**Name:** SolidCraft  
**Repository:** https://github.com/tangency-labs/solidcraft  
**Technology:** Rust, egui, wgpu, WASM, MCP (Model Context Protocol)  
**Purpose:** Open-source parametric 3D CAD application built in pure Rust. Desktop, web, and automation through MCP.

## Problems Identified
1. **No African domain parametric models** — Missing parametric components for African manufacturing, construction, engineering (solar, water, housing, agriculture, manufacturing)
2. **MCP server limited to stdio transport** — No HTTP/SSE transports for remote AI agent access
3. **No benchmark suite** — No performance regression detection for CAD operations
4. **No interactive architecture visualization** — No visual documentation of the CAD engine architecture
5. **Documentation gaps** — Missing design document, case study, before/after comparison, interactive documentation tree
6. **CI/CD could be enhanced** — Missing cargo-audit, cargo-deny, TruffleHog, SBOM generation, multi-platform builds
7. **Security hardening opportunities** — Supply chain security, dependency scanning, WASM sandboxing

## Proposed Improvements

### 1. African Domain Parametric Models (NEW CRATE: `solidcraft-africa`)
Create a new crate with parametric models for African contexts:
- **Solar Energy**: Solar panel mounting structures, adjustable tilt frames, roof mounting systems
- **Water Infrastructure**: Water tank designs (elevated, underground, modular), pump houses, pipe routing
- **Housing**: Modular housing units, interlocking brick systems, roof truss designs
- **Agriculture**: Irrigation system components, greenhouse frames, grain silo designs
- **Manufacturing**: Machine bases, conveyor components, jig/fixture templates
- **Construction**: Scaffolding systems, formwork components, rebar bending templates

### 2. MCP Server Enhancement
- Add HTTP transport (axum-based) with JSON-RPC over HTTP
- Add SSE (Server-Sent Events) transport for real-time streaming
- Add WebSocket transport for bidirectional communication
- New MCP tools for African domain models
- Authentication/authorization for remote access
- Rate limiting and request validation

### 3. Benchmark Suite with Regression Detection
- Criterion-based benchmarks for:
  - Constraint solver performance
  - Boolean operations (union, difference, intersection)
  - STEP/IGES import/export
  - Mesh generation and tessellation
  - Parametric model regeneration
  - WASM build size and runtime performance
- Regression detection with configurable thresholds
- GitHub Actions integration for PR benchmark comparison
- Historical benchmark tracking

### 4. Interactive Architecture Visualization
- React Flow-based visualization of:
  - Crate dependency graph
  - CAD kernel architecture (sketch → constraint solver → engine → render)
  - MCP server architecture (transports → tools → engine)
  - Data flow: User input → Sketch → Constraints → 3D Model → Render
  - African domain crate integration points
- Expandable/collapsible nodes with descriptions
- Click-to-navigate to source code

### 5. Comprehensive Documentation Suite
- **DESIGN_DOCUMENT.md**: Original vs proposed architecture, technology decisions, security decisions
- **PORTFOLIO_CASE_STUDY.md**: Problem, research, weaknesses, engineering approach, improvements, lessons learned
- **BEFORE_AFTER.md**: Detailed comparison table
- **INTERACTIVE_TREE.md**: Documentation tree structure
- **MCP_GUIDE.md**: MCP server usage, transports, tools, AI agent integration
- **AFRICAN_MODELS_GUIDE.md**: Parametric model catalog, customization guide
- **ARCHITECTURE_VIS.md**: Architecture visualization guide

### 6. Enhanced CI/CD Pipeline
- **Security Jobs**: cargo-audit, cargo-deny, TruffleHog, SBOM (cyclonedx)
- **Multi-platform Builds**: Linux (x86_64, aarch64), macOS (x86_64, aarch64), Windows (x86_64)
- **WASM Build**: wasm32-unknown-unknown with wasm-bindgen
- **Benchmark Job**: Run benchmarks, compare with baseline, fail on regression
- **Documentation Build**: Build and deploy docs
- **Release Automation**: Semantic versioning, changelog generation

### 7. Security Hardening
- Dependency vulnerability scanning (cargo-audit, cargo-deny)
- Supply chain verification (cargo-deny advisories)
- Secret scanning (TruffleHog)
- SBOM generation (cyclonedx-bom)
- WASM sandboxing verification
- MCP server input validation and rate limiting

## New Features
- African parametric model library (15+ models)
- MCP HTTP/SSE/WebSocket transports
- Benchmark regression detection dashboard
- Interactive architecture explorer (web-based)
- Comprehensive documentation portal
- Multi-platform release binaries
- African manufacturing/construction showcase models

## Security Improvements
- Automated dependency vulnerability scanning
- Supply chain integrity verification
- Secret detection in CI
- SBOM for transparency
- MCP server authentication framework
- Input sanitization for all MCP tools
- Rate limiting on MCP endpoints

## Performance Improvements
- Benchmark suite for critical paths
- Regression detection with alerts
- WASM build optimization (size, startup time)
- Parallel benchmark execution
- Historical performance tracking

## Testing Improvements
- Unit tests for African domain models
- Integration tests for MCP transports
- Property-based tests for constraint solver
- Visual regression tests for rendering
- WASM runtime tests
- Benchmark CI integration

## Documentation Improvements
- Professional README with badges, screenshots, quick start
- Architecture decision records
- Interactive documentation tree
- API documentation for all public crates
- MCP server protocol documentation
- African model customization tutorials
- Contribution guidelines

## Architecture Improvements
- New `solidcraft-africa` crate with clean separation
- MCP transport abstraction layer
- Benchmark harness crate
- Documentation generation pipeline
- Plugin system for domain extensions

## Expected Result
A production-ready, enhanced SolidCraft distribution that:
- Demonstrates senior Rust engineering (workspace, WASM, MCP, benchmarking)
- Showcases AI engineering (MCP server with multiple transports for AI agents)
- Demonstrates cybersecurity (supply chain, SBOM, secret scanning, WASM sandboxing)
- Addresses African technology opportunities (manufacturing, construction, engineering parametric models)
- Provides excellent developer experience (docs, visualization, benchmarks, CI/CD)
- Serves as a portfolio centerpiece for 3D CAD/Manufacturing engineering category