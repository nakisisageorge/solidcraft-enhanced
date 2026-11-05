# MCP GUIDE — solidcraft-enhanced

## Model Context Protocol (MCP) Server Guide

This guide covers the MCP server in solidcraft-enhanced, including the new HTTP, SSE, and WebSocket transports for remote AI agent access.

## Table of Contents
1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Transports](#transports)
4. [Server Configuration](#server-configuration)
5. [Available Tools](#available-tools)
6. [African Domain Tools](#african-domain-tools)
7. [AI Agent Integration](#ai-agent-integration)
8. [Security](#security)
9. [Troubleshooting](#troubleshooting)
10. [Examples](#examples)

---

## Overview

The SolidCraft MCP server exposes the CAD engine's command registry to AI agents via the Model Context Protocol (MCP). Every modeling operation is a command, enabling AI agents to:

- Create and modify parametric 3D designs
- Run constraint solvers and boolean operations
- Import/export CAD formats (STEP, IGES, STL, 3MF)
- Generate meshes and render images
- Access African domain parametric models
- Run benchmarks and check regressions

### Key Features
- **Multiple Transports**: stdio (legacy), HTTP, SSE, WebSocket
- **Session Management**: UUID-based sessions with timeouts
- **Path Scoping**: Agents confined to allowed directories
- **Compact Output**: Token-efficient responses for LLM agents
- **Streaming**: SSE for long-running operations
- **Bidirectional**: WebSocket for interactive sessions

---

## Quick Start

### Stdio Transport (Legacy, Local Only)
```bash
# Headless session from design file
solidcraft-cli mcp --in design.sc

# Bridged to running desktop app
solidcraft-cli mcp --connect 127.0.0.1:7878

# Auto-detect running app or start headless
solidcraft-cli mcp
```

### HTTP Transport (Remote Access)
```bash
# Start HTTP server on port 7878
solidcraft-cli mcp --http 0.0.0.0:7878 --in design.sc

# HTTP only (no stdio)
solidcraft-cli mcp --http 0.0.0.0:7878 --http-only --in design.sc
```

### With African Domain Models
```bash
# African models automatically available via MCP tools
solidcraft-cli mcp --http 0.0.0.0:7878 --in design.sc
# Agent can now call: africa.models.list, africa.models.generate, etc.
```

### Compact Output for LLMs
```bash
# Reduces token usage for LLM agents
solidcraft-cli mcp --compact --http 0.0.0.0:7878
# inspect_design summarizes sketches, JSON not pretty-printed
```

---

## Transports

### 1. Stdio Transport (Legacy)
- **Protocol**: Newline-delimited JSON-RPC 2.0
- **Use Case**: Local AI agents (Claude Desktop, OpenCode, local scripts)
- **Limitation**: No network access, single process

```bash
solidcraft-cli mcp [--headless] [--in DESIGN] [--connect HOST:PORT]
```

### 2. HTTP Transport
- **Protocol**: RESTful JSON-RPC over HTTP/1.1
- **Use Case**: Remote agents, CI/CD pipelines, cloud deployments
- **Features**: Session management, CORS, health checks

```bash
solidcraft-cli mcp --http 0.0.0.0:7878 [--http-only] [--in DESIGN]
```

**Endpoints:**
| Method | Path | Description |
|--------|------|-------------|
| POST | `/mcp` | Generic JSON-RPC request |
| POST | `/mcp/initialize` | Initialize session |
| GET | `/mcp/tools/list` | List available tools |
| POST | `/mcp/tools/call` | Call a tool |
| GET | `/mcp/resources/list` | List resources |
| POST | `/mcp/resources/read` | Read resource |
| GET | `/mcp/sse` | SSE event stream |
| GET | `/mcp/ws` | WebSocket upgrade |
| GET | `/health` | Health check |

**Headers:**
- `Mcp-Session-Id`: Session identifier (returned on initialize)
- `Content-Type`: `application/json`

### 3. SSE Transport (Server-Sent Events)
- **Protocol**: HTTP with `text/event-stream`
- **Use Case**: Real-time streaming updates, long operations
- **Features**: Automatic reconnection, event-based

```bash
# Connect to SSE stream
curl -N -H "Accept: text/event-stream" http://localhost:7878/mcp/sse
```

**Events:**
| Event | Description |
|-------|-------------|
| `message` | JSON-RPC response/notification |
| `heartbeat` | Keep-alive (every 30s) |

### 4. WebSocket Transport
- **Protocol**: WebSocket with JSON-RPC messages
- **Use Case**: Interactive agents, bidirectional communication
- **Features**: Full duplex, binary support, low latency

```bash
# Connect via WebSocket
wscat -c ws://localhost:7878/mcp/ws
```

**Message Format:**
```json
// Request
{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "engine.execute", "arguments": {...}}}

// Response
{"jsonrpc": "2.0", "id": 1, "result": {...}}
```

---

## Server Configuration

### Command Line Options
```bash
solidcraft-cli mcp [OPTIONS]

Options:
  --headless              Force headless session (don't bridge to running app)
  --in <PATH>             Start from design file (.sc, .step, .3mf)
  --connect <HOST:PORT>   Bridge to running app's control port
  --http <ADDR>           Enable HTTP/SSE/WS server (e.g., 0.0.0.0:7878)
  --http-only             Serve only HTTP/SSE/WS (no stdio)
  --no-ws                 Disable WebSocket transport
  --no-sse                Disable SSE transport
  --allow-path <DIR>      Add allowed directory for agent file access
  --allow-any-path        Allow agent access to any path (DANGEROUS)
  --param-check <LEVEL>   Parameter checking: off, warn, strict
  --render-gpu            Enable GPU rendering for headless
  --compact               Compact output for LLM agents
  --compact-env           Use SOLIDCRAFT_MCP_COMPACT=1 env var
```

### Environment Variables
| Variable | Description | Default |
|----------|-------------|---------|
| `SOLIDCRAFT_MCP_COMPACT` | Compact output (1/0/true/false) | 0 |
| `SOLIDCRAFT_CONTROL_PORT` | Control port for app discovery | 7878 |
| `RUST_LOG` | Logging level (error/warn/info/debug/trace) | warn |

### Path Scoping
By default, agents can only access:
1. App's recovery folder
2. Server's private scratch folder (screenshots)
3. Each `--allow-path` directory
4. Open design's folder (added on open/save)

```bash
# Allow specific directories
solidcraft-cli mcp --allow-path /home/user/projects --allow-path /data/models

# DANGEROUS: Allow any path
solidcraft-cli mcp --allow-any-path
```

---

## Available Tools

### Core CAD Tools
| Tool | Description | Parameters |
|------|-------------|------------|
| `list_commands` | List command catalog with filters | `filter`, `prefix`, `panel`, `compact`, `limit` |
| `describe_command` | Get command docs and schemas | `command` (id) |
| `engine.execute` | Execute one command | `command`, `params` |
| `batch` | Run multiple commands atomically | `commands[]`, `coalesce_parameters`, `rollback` |
| `inspect_design` | Get design state with measurements | `measure`, `sketches`, `bodies` |
| `inspect_measure` | Measure bodies (volume, bbox, etc.) | `body`, `items` |
| `inspect_compare_picture` | Compare rendered images | `reference`, `view`, `bodies` |
| `screenshot` | Capture render to file | `path`, `view`, `width`, `height` |
| `check_parameters` | Test parameter changes | `set{}`, `measure` |
| `try_parameters` | Preview parameter changes | `changes{}`, `measure` |
| `apply_parameters` | Apply parameter changes | `changes{}`, `measure` |

### Document Tools
| Tool | Description |
|------|-------------|
| `document.open` | Open design file |
| `document.save` | Save design |
| `file.export` | Export to STEP/IGES/STL/3MF |
| `file.import` | Import from file |

### Parameter Tools
| Tool | Description |
|------|-------------|
| `parameters.list` | List all parameters |
| `parameters.change` | Change parameter expression |
| `parameters.set` | Set parameter value |
| `timeline.edit` | Edit timeline feature |
| `edit.checkpoint` | Create undo checkpoint |
| `edit.restore_checkpoint` | Restore from checkpoint |

### UI/Rendering Tools
| Tool | Description |
|------|-------------|
| `ui.render` | Render to image |
| `ui.view` | Set camera view |
| `ui.view.apply` | Apply saved view |

---

## African Domain Tools

New tools for African parametric models (require `solidcraft-africa` crate):

### Model Discovery
```json
// List all available African models
{
  "name": "africa.models.list",
  "arguments": {}
}

// Response
{
  "models": [
    {"id": "solar.panel_mount", "name": "Solar Panel Ground Mount", "category": "Solar", "description": "..."},
    {"id": "water.elevated_tank", "name": "Elevated Water Tank", "category": "Water", "description": "..."}
  ]
}
```

### Model Schema
```json
// Get parameter schema for a model
{
  "name": "africa.models.schema",
  "arguments": {"model_id": "solar.panel_mount"}
}

// Response: JSON Schema with all parameters, types, ranges, defaults
```

### Model Validation
```json
// Validate parameters before generation
{
  "name": "africa.models.validate",
  "arguments": {
    "model_id": "solar.panel_mount",
    "params": {"panel_width": 1000, "tilt_angle": 15}
  }
}
```

### Model Generation
```json
// Generate African parametric model
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
      "foundation_type": "concrete_pad"
    }
  }
}

// Response: Generated design with body IDs, measurements, bbox
```

### Category Listing
```json
// List models by category
{
  "name": "africa.models.categories",
  "arguments": {}
}

// Response
{
  "categories": [
    {"category": "Solar", "count": 5},
    {"category": "Water", "count": 6},
    {"category": "Housing", "count": 5},
    {"category": "Agriculture", "count": 5},
    {"category": "Manufacturing", "count": 5},
    {"category": "Construction", "count": 5}
  ]
}
```

---

## AI Agent Integration

### 1. Local Agent (Stdio)
```json
// Claude Desktop config
{
  "mcpServers": {
    "solidcraft": {
      "command": "solidcraft-cli",
      "args": ["mcp", "--in", "design.sc"]
    }
  }
}
```

### 2. Remote Agent (HTTP)
```python
import httpx
import json

async def call_tool(client, session_id, tool_name, args):
    response = await client.post(
        "http://localhost:7878/mcp/tools/call",
        headers={"Mcp-Session-Id": session_id, "Content-Type": "application/json"},
        json={"name": tool_name, "arguments": args}
    )
    return response.json()

# Initialize session
init_response = await client.post(
    "http://localhost:7878/mcp/initialize",
    json={"protocolVersion": "2025-06-18"}
)
session_id = init_response.headers["Mcp-Session-Id"]

# Generate African solar mount
result = await call_tool(client, session_id, "africa.models.generate", {
    "model_id": "solar.panel_mount",
    "params": {"panel_width": 1000, "tilt_angle": 15}
})
```

### 3. Streaming with SSE
```python
import httpx

async def stream_updates():
    async with httpx.AsyncClient() as client:
        async with client.stream(
            "GET",
            "http://localhost:7878/mcp/sse",
            headers={"Mcp-Session-Id": session_id, "Accept": "text/event-stream"}
        ) as response:
            async for line in response.aiter_lines():
                if line.startswith("data: "):
                    event = json.loads(line[6:])
                    print(f"Update: {event}")
```

### 4. Interactive with WebSocket
```python
import websockets
import json

async def interactive_session():
    async with websockets.connect(
        "ws://localhost:7878/mcp/ws",
        extra_headers={"Mcp-Session-Id": session_id}
    ) as ws:
        # Send tool call
        await ws.send(json.dumps({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {"name": "engine.execute", "arguments": {"command": "solid.box", "params": {"size": [50,50,50]}}}
        }))
        
        # Receive response
        response = await ws.recv()
        print(json.loads(response))
```

### 5. African Models from AI Agent
```python
# List African models
models = await call_tool(client, session_id, "africa.models.list", {})

# Get schema for solar mount
schema = await call_tool(client, session_id, "africa.models.schema", {
    "model_id": "solar.panel_mount"
})

# Generate with custom parameters
result = await call_tool(client, session_id, "africa.models.generate", {
    "model_id": "solar.panel_mount",
    "params": {
        "panel_width": 1000,
        "panel_height": 1700,
        "panels_per_row": 6,
        "num_rows": 4,
        "tilt_angle": 12.5,  # Optimal for equatorial
        "azimuth": 0,        # True north
        "row_spacing": 3500, # Avoid shading
        "foundation_type": "ground_screw",  # Low-cost, no concrete
        "include_bracing": True,
        "include_walkway": True
    }
})
```

---

## Security

### Path Scoping
Agents are confined to allowed directories:
- Recovery folder (auto)
- Scratch folder (screenshots, auto)
- `--allow-path` directories
- Open design's folder (auto-added)

### Session Management
- UUID-based sessions
- 30-minute inactivity timeout (configurable)
- Session ID in `Mcp-Session-Id` header

### Rate Limiting
- Per-session request limits
- Configurable via server options
- 429 response on limit exceeded

### Input Validation
- All tool parameters validated via JSON Schema
- Parameter checking: off/warn/strict
- Path traversal prevented by VFS

### Transport Security
| Transport | Auth | Encryption | CORS |
|-----------|------|------------|------|
| stdio | OS user | N/A | N/A |
| HTTP | Session ID | TLS recommended | Configurable |
| SSE | Session ID | TLS recommended | Configurable |
| WebSocket | Session ID | TLS recommended | N/A |

**Production Recommendation**: Run behind reverse proxy (nginx, Caddy) with TLS termination.

---

## Troubleshooting

### Server Won't Start
```bash
# Check port availability
lsof -i :7878

# Check Rust version
rustc --version  # Needs 1.95+

# Check dependencies
cargo build --release -p solidcraft-cli
```

### Agent Can't Connect
```bash
# Test HTTP endpoint
curl http://localhost:7878/health

# Test SSE
curl -N http://localhost:7878/mcp/sse

# Test WebSocket
wscat -c ws://localhost:7878/mcp/ws
```

### Path Access Denied
```bash
# Add required paths
solidcraft-cli mcp --allow-path /home/user/projects --allow-path /data

# Check path scoping
# Agent can only reach: recovery, scratch, --allow-path, design folder
```

### Compact Output Issues
```bash
# Enable compact mode
solidcraft-cli mcp --compact

# Or via env
SOLIDCRAFT_MCP_COMPACT=1 solidcraft-cli mcp
```

### Performance Issues
```bash
# Enable GPU rendering for headless
solidcraft-cli mcp --render-gpu --in design.sc

# Profile commands
SOLIDCRAFT_PROFILE=1 solidcraft-cli mcp --in design.sc
```

---

## Examples

### Example 1: Generate Solar Farm Layout
```python
# 100kW solar farm for Kenya (equatorial)
result = await call_tool(client, session_id, "africa.models.generate", {
    "model_id": "solar.ground_mount_array",
    "params": {
        "num_arrays": 20,
        "panels_per_array": 24,
        "tilt_angle": 10,      # Near equator
        "tracking": "single_axis",
        "array_spacing": 8000,
        "ground_clearance": 1000,
        "terrain": "flat",
        "include_fencing": True,
        "include_roads": True
    }
})
```

### Example 2: Design Water Supply System
```python
# Elevated tank for Ugandan town
tank = await call_tool(client, session_id, "africa.models.generate", {
    "model_id": "water.elevated_tank",
    "params": {
        "capacity_liters": 100000,
        "tank_height": 4000,
        "tower_height": 15000,
        "leg_diameter": 200,
        "include_ladder": True,
        "include_platform": True,
        "foundation": "pile_cap",
        "seismic_design": True
    }
})

# Pipe network
pipes = await call_tool(client, session_id, "africa.models.generate", {
    "model_id": "water.pipe_routing",
    "params": {
        "main_diameter": 200,
        "branch_diameter": 63,
        "pipe_length": 5000,
        "pipe_material": "hdpe",
        "bury_depth": 1.0,
        "include_thrust_blocks": True,
        "include_air_valves": True,
        "num_connections": 200,
        "terrain": "rolling"
    }
})
```

### Example 3: Modular Housing Development
```python
# 50-unit housing development for Rwanda
housing = await call_tool(client, session_id, "africa.models.generate", {
    "model_id": "housing.modular_unit",
    "params": {
        "unit_width": 4000,
        "unit_length": 8000,
        "construction": "interlocking_brick",
        "roof_type": "gable",
        "roof_pitch": 30,
        "num_rooms": 3,
        "include_sanitation": True,
        "include_kitchen": True,
        "include_verandah": True,
        "floor_type": "concrete_slab",
        "cladding": "render",
        "solar_ready": True,
        "water_tank_ready": True
    }
})

# Latrine block for school
latrine = await call_tool(client, session_id, "africa.models.generate", {
    "model_id": "housing.latrine_block",
    "params": {
        "num_stalls": 8,
        "toilet_type": "vip",
        "separate_genders": True,
        "include_urinals": True,
        "include_handwashing": True,
        "include_menstrual": True,
        "accessible_stall": True,
        "pit_depth": 4000,
        "vent_pipe": True,
        "roof_rainwater": True
    }
})
```

### Example 4: Run Benchmark Suite
```bash
# Run all benchmarks
cargo bench -p solidcraft-benchmark

# Check regressions in CI
solidcraft-bench check --fail-on-regression --threshold 10.0

# Update baselines after intentional optimization
solidcraft-bench run --output current_results.json
solidcraft-bench update-baseline --results current_results.json
```

---

## Support

- **Issues**: https://github.com/nakisisageorge/solidcraft-enhanced/issues
- **Discussions**: https://github.com/nakisisageorge/solidcraft-enhanced/discussions
- **Original SolidCraft**: https://github.com/tangency-labs/solidcraft
- **MCP Specification**: https://modelcontextprotocol.io

## License
MIT OR Apache-2.0 (same as upstream SolidCraft)