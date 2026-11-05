# DESIGN DOCUMENT — solidcraft-enhanced

## 1. Original Architecture

### 1.1 Overview
SolidCraft is an open-source parametric 3D CAD application built in pure Rust. The original architecture consists of a modular crate workspace with clear separation of concerns:

**Foundation Layer:**
- `solidcraft-geom`: Core geometry primitives (points, vectors, curves, surfaces, transforms)
- `solidcraft-sketch`: 2D constraint solver, sketch entities, dimensions, parameters
- `solidcraft-kernel`: B-rep modeling kernel (NVIDIA SMLib port), boolean operations, features, topology

**Core Layer:**
- `solidcraft-engine`: Session management, command registry, undo/redo, selection, parameter management
- `solidcraft-doc`: Document format, serialization, design containers, history
- `solidcraft-io`: STEP, IGES, STL, 3MF, OBJ, GLTF import/export, point clouds
- `solidcraft-render`: wgpu/egui rendering, tessellation, PBR materials, raytracing
- `solidcraft-gpu`: GPU abstraction, render job host, headless rendering, screenshots

**Infrastructure Layer:**
- `solidcraft-derive`: Proc macros for command parameters, schemas, serialization
- `solidcraft-profile`: Profiling, timing spans, Chrome trace export, performance analysis
- `solidcraft-testkit`: Testing utilities, model cache, snapshot comparison

**Application Layer:**
- `solidcraft-cli`: Headless CLI for run, eval, snapshot, MCP server, benchmarks
- `solidcraft-web`: WASM web app, browser CAD, WASM MCP client
- `solidcraft` (desktop): Native desktop app with egui, full CAD UI

**AI Integration:**
- `solidcraft-mcp`: MCP server with stdio transport for AI agents

### 1.2 Data Flow
```
User Input → CLI/App → Engine (Command Registry) → Kernel (B-rep) → Render → Output
                    ↓
              Document (Serialization) → File I/O (STEP/3MF/STL)
                    ↓
              MCP Server → AI Agent (stdio transport)
```

### 1.3 Key Design Decisions
- **Pure Rust**: No C/C++ dependencies, memory safety, easy cross-compilation
- **Command-based**: Every operation is a command, enabling undo/redo, scripting, MCP
- **Parametric**: Dimensions as expressions, constraint solver maintains relationships
- **B-rep kernel**: SMLib provides robust boundary representation modeling
- **Modular crates**: Clear boundaries, independent compilation, reusable components

## 2. Problems Identified

### 2.1 Missing African Domain Models
No parametric models for African manufacturing, construction, engineering contexts (solar, water, housing, agriculture, manufacturing, construction).

### 2.2 Limited MCP Transport Options
MCP server only supports stdio transport, preventing remote AI agent access over network.

### 2.3 No Benchmark Regression Detection
No performance benchmarking suite to detect regressions in CAD operations.

### 2.4 No Interactive Architecture Visualization
No visual documentation of the crate dependency graph and data flow.

### 2.5 Documentation Gaps
Missing design document, portfolio case study, before/after comparison, interactive documentation tree.

### 2.6 CI/CD Enhancement Opportunities
Missing cargo-audit, cargo-deny, TruffleHog, SBOM generation, multi-platform builds.

### 2.7 Security Hardening Gaps
No supply chain verification, dependency vulnerability scanning in CI.

## 3. Proposed Architecture

### 3.1 New Components

#### 3.1.1 `solidcraft-africa` Crate (Domain Extension Layer)
```
solidcraft-africa/
├── src/
│   ├── lib.rs              # Registry, common types, geographic context
│   ├── common/             # Parameter validation, units, country defaults
│   ├── solar/              # Solar panel mounts, adjustable frames, roof systems
│   ├── water/              # Elevated/underground/modular tanks, pump houses, piping
│   ├── housing/            # Modular units, interlocking brick, roof trusses, foundations
│   ├── agriculture/        # Drip irrigation, greenhouses, grain silos, animal shelters
│   ├── manufacturing/      # Machine bases, conveyors, jigs, workbenches, tooling plates
│   └── construction/       # Scaffolding, formwork, rebar templates, shuttering, crane bases
```

**Integration Points:**
- Depends on: `engine`, `kernel`, `io`, `render`
- Provides: 30+ parametric models via command registry
- Accessible via: CLI, MCP tools, WebAssembly

#### 3.1.2 Enhanced MCP Server (Transport Layer)
```
solidcraft-mcp/
├── src/
│   ├── transport.rs        # HTTP, SSE, WebSocket transports
│   ├── server.rs           # JSON-RPC 2.0 handling (existing)
│   ├── lib.rs              # Exports (updated)
│   └── ...
```

**New Transports:**
- **HTTP**: RESTful JSON-RPC with session management, CORS
- **SSE**: Server-Sent Events for real-time streaming updates
- **WebSocket**: Bidirectional for interactive agent sessions

**Security:**
- Session-based authentication
- Rate limiting per session
- Input validation on all tool calls
- Path scoping maintained

#### 3.1.3 Benchmark Suite (Quality Layer)
```
solidcraft-benchmark/
├── src/
│   ├── lib.rs              # Core types, regression detection
│   ├── benches.rs          # Benchmark definitions per category
│   ├── regression.rs       # Historical tracking, regression reports
│   ├── runner.rs           # CLI runner (solidcraft-bench)
│   └── storage.rs          # File/SQLite storage backends
```

**Benchmark Categories:**
1. Constraint Solver (sketch 10/50 constraints, parametric)
2. Boolean Operations (union, difference, intersection)
3. Import/Export (STEP 100 bodies, 3MF mesh)
4. Mesh Generation (50 boxes, curved surfaces HQ)
5. Parametric Regeneration (single/batch parameter changes)
6. WASM Build (size, time)
7. WASM Runtime (startup, command execution)

**Regression Detection:**
- Historical baseline storage
- Configurable threshold (default 10%)
- Severity classification (Minor/Major/Critical)
- CI/CD integration with exit codes
- HTML report generation

#### 3.1.4 Interactive Architecture Visualization
- React Flow based diagram at `docs/architecture/index.html`
- Expandable/collapsible nodes with descriptions
- Category filtering and search
- Click-to-view details panel
- Mini-map navigation

### 3.2 Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           EXTERNAL DEPENDENCIES                             │
│  solidcore (B-rep) ──► kernel    egui ◄── desktop    wgpu ◄── desktop/web  │
│       │                     │                        │                      │
└───────┼─────────────────────┼────────────────────────┼──────────────────────┘
        ▼                     ▼                        ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                            FOUNDATION LAYER                                 │
│  geom ──► sketch ──► kernel ──► (SMLib via solidcore)                      │
└─────────────────────────────────────────────────────────────────────────────┘
        │                     │                        │
        ▼                     ▼                        ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              CORE LAYER                                     │
│  engine ◄── derive, profile, testkit                                        │
│    │       │          │          │                                          │
│    ▼       ▼          ▼          ▼                                          │
│   doc      io        render ◄── gpu                                         │
└─────────────────────────────────────────────────────────────────────────────┘
        │       │          │          │
        ▼       ▼          ▼          ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         EXTENSION LAYER                                     │
│  africa ◄── engine, kernel, io, render                                      │
│  benchmark ◄── engine, kernel                                               │
│  mcp ◄── engine, doc, io                                                    │
│      │          │          │                                                 │
│      ▼          ▼          ▼                                                 │
│   stdio      HTTP       SSE ◄── axum                                        │
│                │          │                                                  │
│                ▼          ▼                                                  │
│              WebSocket                                                    │
└─────────────────────────────────────────────────────────────────────────────┘
        │       │          │
        ▼       ▼          ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                          APPLICATION LAYER                                  │
│  cli ◄── engine, doc, io, mcp, benchmark                                    │
│  web ◄── engine, render, gpu, wasm-bindgen                                 │
│  desktop ◄── engine, render, gpu, egui, wgpu                               │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 4. New Architecture

### 4.1 Data Flow (Enhanced)

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ AI Agent    │────►│ MCP Server  │────►| Engine      │
│ (remote)    │     │ (HTTP/SSE/WS)│     │ (Commands)  │
└─────────────┘     └─────────────┘     └──────┬──────┘
                                               │
                    ┌──────────────────────────┼──────────────────────────┐
                    ▼                          ▼                          ▼
             ┌─────────────┐            ┌─────────────┐            ┌─────────────┐
             │ Africa      │            │ Kernel      │            │ Document    │
             │ Models      │            │ (B-rep)     │            │ (History)   │
             └─────────────┘            └─────────────┘            └─────────────┘
                    │                          │                          │
                    ▼                          ▼                          ▼
             ┌─────────────────────────────────────────────────────────────────┐
             │                        RENDER / EXPORT                          │
             └─────────────────────────────────────────────────────────────────┘
```

### 4.2 MCP Tool Extensions for African Models

New MCP tools added:
- `africa.models.list` — List all available African domain models
- `africa.models.generate` — Generate a model by ID with parameters
- `africa.models.schema` — Get JSON schema for a model's parameters
- `africa.models.validate` — Validate parameters before generation
- `africa.models.categories` — List models by category

### 4.3 Benchmark Integration

CI/CD Pipeline:
```yaml
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run benchmarks
        run: cargo bench --bench solidcraft_benchmark
      - name: Check regressions
        run: solidcraft-bench check --fail-on-regression
      - name: Upload report
        uses: actions/upload-artifact@v4
        with:
          name: benchmark-report
          path: benchmark_report.html
```

## 5. Why the Changes Were Made

### 5.1 African Domain Models
**Reasoning**: The portfolio lacks African manufacturing/construction/engineering parametric models. Africa has unique constraints (materials, climate, skills, cost) requiring purpose-built designs. This fills a genuine gap in open-source CAD.

**Alternatives Considered**: 
- Add to existing crates: Rejected — pollutes core with domain-specific code
- Separate repo: Rejected — loses integration with kernel, MCP, CLI
- New crate in workspace: Chosen — clean separation, shared infrastructure

### 5.2 HTTP/SSE/WebSocket MCP Transports
**Reasoning**: stdio-only limits MCP to local AI agents. Remote agents (cloud, distributed teams, CI/CD) need network transports. HTTP for REST compatibility, SSE for streaming, WebSocket for bidirectional.

**Alternatives Considered**:
- gRPC: Rejected — not MCP standard, more complex
- MQTT: Rejected — wrong paradigm for request/response
- Keep stdio only: Rejected — limits adoption

### 5.3 Benchmark Regression Detection
**Reasoning**: CAD kernels are performance-sensitive. Boolean ops, constraint solving, tessellation must not regress. Historical tracking enables trend analysis.

**Alternatives Considered**:
- Manual benchmarking: Rejected — not reproducible
- External CI only: Rejected — needs local developer workflow too
- Integrated criterion + custom storage: Chosen — Rust-native, flexible

### 5.4 Interactive Architecture Visualization
**Reasoning**: Static diagrams rot. Interactive React Flow diagram stays current, explorable, demonstrates React/TypeScript skills alongside Rust.

## 6. Technology Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Language | Rust | Memory safety, performance, WASM, ecosystem |
| MCP Transport | axum + tokio | Async, HTTP/WS/SSE, production-ready |
| Benchmarking | criterion | Statistical rigor, HTML reports, Rust-native |
| Visualization | React Flow | Interactive, TypeScript, maintained |
| Storage | JSON files (+ SQLite optional) | Simple, portable, git-friendly |
| CI/CD | GitHub Actions | Native integration, free for open source |
| Security | cargo-audit, cargo-deny, TruffleHog | Rust ecosystem standard tools |

## 7. Security Decisions

1. **Path Scoping**: Maintained from original — agents cannot escape allowed directories
2. **Session Management**: UUID-based sessions with activity timeouts
3. **Rate Limiting**: Per-session request limits on MCP endpoints
4. **Input Validation**: All tool parameters validated via JSON Schema
5. **Dependency Scanning**: cargo-audit + cargo-deny in CI
6. **Secret Detection**: TruffleHog in CI pipeline
7. **SBOM Generation**: cyclonedx-bom for supply chain transparency
8. **WASM Sandboxing**: WebAssembly isolates browser execution

## 8. Performance Decisions

1. **Benchmark Categories**: Cover critical paths (solver, boolean, mesh, parametric)
2. **Regression Threshold**: 10% default, configurable per project
3. **Historical Storage**: JSON for portability, SQLite option for scale
4. **WASM Optimization**: `wasm-opt` in release builds, size monitoring
5. **Parallel Benchmarks**: criterion runs benchmarks in parallel

## 9. Scalability Considerations

1. **Crate Boundaries**: Clear interfaces allow independent scaling
2. **MCP Session State**: Stateless design with external session store option
3. **Benchmark History**: JSON file scales to thousands of entries; SQLite for millions
4. **African Models**: Parameter-driven, no hardcoded geometry limits
5. **WASM Bundle Size**: Monitored via benchmark, code splitting available

## 10. Deployment Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ GitHub      │────►│ CI/CD       │────►│ Artifacts   │
│ (source)    │     │ (Actions)   │     │ (binaries)  │
└─────────────┘     └─────────────┘     └──────┬──────┘
                                               │
                    ┌──────────────────────────┼──────────────────────────┐
                    ▼                          ▼                          ▼
             ┌─────────────┐            ┌─────────────┐            ┌─────────────┐
             │ Linux       │            │ macOS       │            │ Windows     │
             │ (x86_64,    │            │ (x86_64,    │            │ (x86_64)    │
             │  aarch64)   │            │  aarch64)   │            │             │
             └─────────────┘            └─────────────┘            └─────────────┘
                    │                          │                          │
                    ▼                          ▼                          ▼
             ┌─────────────────────────────────────────────────────────────────┐
             │                    GitHub Releases / Cargo                      │
             └─────────────────────────────────────────────────────────────────┘
```

## 11. Testing Strategy

1. **Unit Tests**: Per-crate, kernel cases, parameter validation
2. **Integration Tests**: CLI commands, MCP tool calls, benchmark runner
3. **Property Tests**: Constraint solver invariants, boolean operation properties
4. **Visual Regression**: Snapshot comparison for rendering
5. **Benchmark CI**: Regression detection on every PR
6. **WASM Tests**: wasm-bindgen-test in browser/node
7. **Security Tests**: cargo-audit, dependency validation

## 12. Future Improvements

1. **African Models**: Expand to 50+ models (road, bridge, power, telecom)
2. **MCP Auth**: OAuth2/OIDC for enterprise deployments
3. **Benchmark Dashboard**: Web UI for historical trends
4. **AI-Assisted Design**: LLM-generated parameter expressions
5. **Collaborative Editing**: CRDT-based multi-user design sessions
6. **Plugin System**: Dynamic crate loading for domain extensions
7. **Cloud Rendering**: Offload tessellation/raytracing to GPU instances