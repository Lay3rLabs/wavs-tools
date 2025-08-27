import React, { useState, useCallback, useMemo, useEffect } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  Connection,
  Edge,
  BackgroundVariant
} from 'reactflow';
import 'reactflow/dist/style.css';

import { parseServiceToFlow, parseBridgeToFlow } from './serviceParser';
import { getLayoutedElements } from './layoutManager';
import { 
  ServiceNode, 
  ChainNode, 
  BridgeNode, 
  ContractNode,
  WorkflowNode,
  TriggerNode,
  ComponentNode,
  AggregatorNode
} from './CustomNodes';
import { ServiceConfig, BridgeConfig } from './types';

const nodeTypes = {
  service: ServiceNode,
  chain: ChainNode,
  bridge: BridgeNode,
  contract: ContractNode,
  workflow: WorkflowNode,
  trigger: TriggerNode,
  component: ComponentNode,
  aggregator: AggregatorNode
};

function App() {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [jsonInput, setJsonInput] = useState('');
  const [layoutDirection, setLayoutDirection] = useState<'TB' | 'LR'>('TB');
  const [selectionMode, setSelectionMode] = useState(false);

  // Handle Alt key for temporary selection mode
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.altKey) {
        setSelectionMode(true);
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      if (!e.altKey) {
        setSelectionMode(false);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    };
  }, []);

  const onConnect = useCallback(
    (params: Connection | Edge) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  const handleLoadJson = useCallback(() => {
    try {
      const config = JSON.parse(jsonInput);
      
      // Auto-detect the type based on JSON structure
      if ('workflows' in config) {
        // It's a service config
        const { nodes: parsedNodes, edges: parsedEdges } = parseServiceToFlow(config as ServiceConfig);
        const layouted = getLayoutedElements(parsedNodes, parsedEdges, layoutDirection);
        setNodes(layouted.nodes);
        setEdges(layouted.edges);
      } else if ('chains' in config && 'bridges' in config) {
        // It's a bridge config
        const { nodes: parsedNodes, edges: parsedEdges } = parseBridgeToFlow(config as BridgeConfig);
        setNodes(parsedNodes);
        setEdges(parsedEdges);
      } else {
        alert('Unrecognized JSON format. Expected either a service config with workflows or a bridge config with chains and bridges.');
      }
    } catch (error) {
      console.error('Error parsing JSON:', error);
      alert('Invalid JSON format. Please check your input.');
    }
  }, [jsonInput, layoutDirection, setNodes, setEdges]);


  const handleAutoLayout = useCallback(() => {
    const layouted = getLayoutedElements(nodes, edges, layoutDirection);
    setNodes(layouted.nodes);
    setEdges(layouted.edges);
  }, [nodes, edges, layoutDirection, setNodes, setEdges]);

  return (
    <div className="app">
      <div className="controls">
        <div style={{ marginBottom: '10px' }}>
          <label style={{ marginRight: '20px' }}>
            <input 
              type="checkbox" 
              checked={selectionMode}
              onChange={(e) => setSelectionMode(e.target.checked)}
              style={{ marginRight: '5px' }}
            />
            Text Selection Mode (or hold Alt key)
          </label>
          <label>
            Layout Direction:
            <select 
              value={layoutDirection} 
              onChange={(e) => setLayoutDirection(e.target.value as 'TB' | 'LR')}
              style={{ marginLeft: '10px' }}
            >
              <option value="TB">Top to Bottom</option>
              <option value="LR">Left to Right</option>
            </select>
          </label>
        </div>
        <div>
          <textarea
            value={jsonInput}
            onChange={(e) => setJsonInput(e.target.value)}
            placeholder="Paste your service.json or bridge configuration here..."
            style={{ width: '80%', height: '100px', marginRight: '10px' }}
          />
          <button onClick={handleLoadJson} style={{ marginRight: '10px' }}>
            Load JSON
          </button>
          <button onClick={handleAutoLayout}>
            Auto Layout
          </button>
        </div>
      </div>
      <div className="flow-container">
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          nodeTypes={nodeTypes}
          fitView
          nodesDraggable={!selectionMode}
          elementsSelectable={true}
          panOnDrag={selectionMode ? false : true}
        >
          <Controls />
          <MiniMap />
          <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
        </ReactFlow>
      </div>
    </div>
  );
}

export default App;