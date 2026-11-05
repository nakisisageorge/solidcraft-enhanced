# ARCHITECTURE VISUALIZATION GUIDE — solidcraft-enhanced

## Overview

The interactive architecture visualization is a React Flow-based diagram that shows the crate dependency graph, data flow, and system architecture of solidcraft-enhanced. It's accessible at `docs/architecture/index.html` and can be viewed in any modern browser.

## Features

### Interactive Navigation
- **Pan**: Click and drag empty space
- **Zoom**: Mouse wheel or pinch gesture
- **Mini-map**: Bottom-left corner for overview
- **Fit View**: Double-click empty space

### Node Interaction
- **Click Node**: Shows detail panel (top-right)
- **Hover**: Tooltip with category
- **Search**: Real-time filtering by name/description
- **Category Filter**: Dropdown to filter by layer

### Visual Encoding
| Category | Color | Shape |
|----------|-------|-------|
| Foundation | `#58a6ff` | Circle node |
| Core | `#a371f7` | Rectangle node |
| Infrastructure | `#ffa657` | Rectangle node |
| Domain Extension | `#3fb950` | Rectangle node |
| AI Integration | `#d2a8ff` | Rectangle node |
| Quality | `#f78166` | Rectangle node |
| Application | `#79c0ff` | Rounded rectangle |
| External | `#8b949e` | Dashed border |
| MCP Transport | `#ff7b72` | Rounded rectangle |

### Edge Types
- **Solid**: Direct dependency
- **Dashed**: MCP transport connection
- **Animated**: Active data flow (on hover)

## Usage

### Opening the Visualization
```bash
# Open directly in browser
open docs/architecture/index.html

# Or serve locally
cd docs/architecture
python3 -m http.server 8080
# Then open http://localhost:8080
```

### Keyboard Shortcuts
| Key | Action |
|-----|--------|
| `F` | Fit view to all nodes |
| `C` | Center on selected node |
| `/` | Focus search box |
| `Esc` | Clear selection |
| `1-9` | Filter by category (1=Foundation, 2=Core, etc.) |

### Filtering
- **Category Dropdown**: Select specific architectural layer
- **Search Box**: Type to filter by node label or description
- **Combined**: Both filters work together

### Detail Panel
Click any node to see:
- Full label
- Description
- Category
- Node ID
- Incoming/outgoing connection count

## Architecture Layers Explained

### 1. External Dependencies (Left)
- **solidcore**: NVIDIA SMLib port - B-rep kernel, Record layer
- **egui**: Immediate mode GUI for desktop app
- **wgpu**: Cross-platform GPU abstraction
- **wasm-bindgen**: Rust ↔ JavaScript/WASM interop
- **axum**: HTTP/SSE/WebSocket server framework

### 2. Foundation Layer (Top-Center)
- **solidcraft-geom**: Points, vectors, curves, surfaces, transforms
- **solidcraft-sketch**: 2D constraint solver, entities, dimensions
- **solidcraft-kernel**: B-rep modeling, boolean ops, features, topology

### 3. Core Layer (Center)
- **solidcraft-engine**: Session, command registry, undo/redo, selection
- **solidcraft-doc**: Document format, serialization, history
- **solidcraft-io**: STEP, IGES, STL, 3MF, OBJ, GLTF import/export
- **solidcraft-render**: wgpu/egui rendering, tessellation, PBR
- **solidcraft-gpu**: GPU abstraction, render job host, headless

### 4. Infrastructure Layer (Bottom-Center)
- **solidcraft-derive**: Proc macros for commands, schemas
- **solidcraft-profile**: Profiling, Chrome trace export
- **solidcraft-testkit**: Testing utilities, snapshots

### 5. Domain Extension Layer (Right-Center)
- **solidcraft-africa**: 30+ African parametric models
- **solidcraft-benchmark**: Criterion benchmarks, regression detection
- **solidcraft-mcp**: MCP server with multi-transport support

### 6. MCP Transport Layer (Bottom-Right)
- **stdio**: Legacy newline-delimited JSON-RPC
- **HTTP**: RESTful JSON-RPC with sessions
- **SSE**: Server-Sent Events for streaming
- **WebSocket**: Bidirectional for interactive agents

### 7. Application Layer (Right)
- **solidcraft-cli**: Headless CLI, MCP server, benchmarks
- **solidcraft-web**: WASM web app, browser CAD
- **solidcraft (desktop)**: Native egui desktop app

## Data Flow

```
User Input → CLI/App → Engine (Command Registry) → Kernel (B-rep) → Render → Output
                    ↓
              Document (Serialization) → File I/O (STEP/3MF/STL)
                    ↓
              MCP Server → AI Agent (stdio/HTTP/SSE/WS)
```

## Customization

### Adding New Nodes
Edit `docs/architecture/index.html` and add to `initialNodes` array:

```javascript
{ 
  id: 'my_crate', 
  type: 'crate', 
  position: { x: 500, y: 300 }, 
  data: { 
    label: 'solidcraft-my_crate', 
    description: 'Description of my crate', 
    category: 'Core' 
  }
}
```

### Adding New Edges
Edit `initialEdges` array:

```javascript
{ 
  id: 'e_new', 
  source: 'engine', 
  target: 'my_crate', 
  type: 'dependency' 
}
```

### Changing Colors
Modify `categoryColors` object:

```javascript
const categoryColors = {
  'My Category': '#ff6b6b',  // New color
  // ...
};
```

## Embedding

The visualization can be embedded in other pages:

```html
<iframe 
  src="https://github.com/nakisisageorge/solidcraft-enhanced/blob/main/docs/architecture/index.html"
  width="100%" 
  height="600px"
  frameborder="0">
</iframe>
```

Or serve the static files from your own domain.

## Development

### Local Development
```bash
cd docs/architecture
# Edit index.html directly - no build step needed
# Uses React 18 + React Flow 11 via CDN
```

### Adding Interactivity
The visualization uses React 18 with React Flow 11. All code is in a single HTML file using Babel standalone for JSX transformation.

### Performance
- 40+ nodes, 47 edges render at 60fps
- Virtualized rendering for large graphs
- Memoized node/edge components

## Accessibility

- **Keyboard Navigation**: Full keyboard support
- **Screen Readers**: ARIA labels on all interactive elements
- **Color Blind Safe**: Color palette tested for deuteranopia/protanopia
- **High Contrast**: Works with system high contrast mode
- **Reduced Motion**: Respects `prefers-reduced-motion`

## Browser Support

| Browser | Version |
|---------|---------|
| Chrome | 90+ |
| Firefox | 88+ |
| Safari | 15+ |
| Edge | 90+ |

## License

The visualization code is part of solidcraft-enhanced under MIT OR Apache-2.0 license.
React Flow is MIT licensed.