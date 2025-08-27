# Service Visualizer

A React-based visualization tool for service.json configurations. It creates interactive node graphs using ReactFlow and dagre for automatic layout, making it easier to understand complex service architectures.

Install dependencies using bun:
```bash
bun install
```

Start the development server:
```bash
bun run dev
```

The application will start on `http://localhost:5173` (or another port if 5173 is occupied).

## Usage

1. Open the application in your browser
2. Paste your `service.json` content into the text area
3. Click "Load JSON" to visualize the service configuration
4. Use the controls to:
   - Toggle text selection mode (or hold Alt key)
   - Enable/disable compact layout
   - Change layout direction
   - Auto-layout to reorganize nodes
