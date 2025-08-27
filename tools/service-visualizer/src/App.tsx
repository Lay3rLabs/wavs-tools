import React, { useState, useCallback, useMemo } from 'react';
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

const exampleBridgeConfig: BridgeConfig = {
  chains: [
    {
      name: "Ethereum",
      contracts: [
        { type: "DepositToken", address: "0x1234...abcd" },
        { type: "Receiver", address: "0xabcd...4321" }
      ]
    },
    {
      name: "Base",
      contracts: [
        { type: "DepositToken", address: "0x5678...edef" },
        { type: "Receiver", address: "0xef56...3210" }
      ]
    },
    {
      name: "Arbitrum",
      contracts: [
        { type: "DepositToken", address: "0x9abc...8888" },
        { type: "Receiver", address: "0xaaaa...9999" }
      ]
    },
    {
      name: "Optimism",
      contracts: [
        { type: "DepositToken", address: "0xdef0...4321" },
        { type: "Receiver", address: "0x1234...aaaa" }
      ]
    }
  ],
  bridges: [
    {
      from_chain: "Ethereum",
      to_chain: "Base",
      from_contract: "0x1234...abcd",
      to_contract: "0xef56...3210",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Base",
      to_chain: "Ethereum",
      from_contract: "0x5678...edef",
      to_contract: "0xabcd...4321",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Ethereum",
      to_chain: "Arbitrum",
      from_contract: "0x1234...abcd",
      to_contract: "0xaaaa...9999",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Arbitrum",
      to_chain: "Ethereum",
      from_contract: "0x9abc...8888",
      to_contract: "0xabcd...4321",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Ethereum",
      to_chain: "Optimism",
      from_contract: "0x1234...abcd",
      to_contract: "0x1234...aaaa",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Optimism",
      to_chain: "Ethereum",
      from_contract: "0xdef0...4321",
      to_contract: "0xabcd...4321",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Base",
      to_chain: "Arbitrum",
      from_contract: "0x5678...edef",
      to_contract: "0xaaaa...9999",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Arbitrum",
      to_chain: "Base",
      from_contract: "0x9abc...8888",
      to_contract: "0xef56...3210",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Base",
      to_chain: "Optimism",
      from_contract: "0x5678...edef",
      to_contract: "0x1234...aaaa",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Optimism",
      to_chain: "Base",
      from_contract: "0xdef0...4321",
      to_contract: "0xef56...3210",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Arbitrum",
      to_chain: "Optimism",
      from_contract: "0x9abc...8888",
      to_contract: "0x1234...aaaa",
      component: "wa.dev/wavs:bridge@0.2.0"
    },
    {
      from_chain: "Optimism",
      to_chain: "Arbitrum",
      from_contract: "0xdef0...4321",
      to_contract: "0xaaaa...9999",
      component: "wa.dev/wavs:bridge@0.2.0"
    }
  ]
};

function App() {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [jsonInput, setJsonInput] = useState('');
  const [viewMode, setViewMode] = useState<'service' | 'bridge'>('bridge');
  const [layoutDirection, setLayoutDirection] = useState<'TB' | 'LR'>('TB');

  const onConnect = useCallback(
    (params: Connection | Edge) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  const handleLoadJson = useCallback(() => {
    try {
      if (viewMode === 'service') {
        const config: ServiceConfig = JSON.parse(jsonInput);
        const { nodes: parsedNodes, edges: parsedEdges } = parseServiceToFlow(config);
        const layouted = getLayoutedElements(parsedNodes, parsedEdges, layoutDirection);
        setNodes(layouted.nodes);
        setEdges(layouted.edges);
      } else {
        const config: BridgeConfig = jsonInput ? JSON.parse(jsonInput) : exampleBridgeConfig;
        const { nodes: parsedNodes, edges: parsedEdges } = parseBridgeToFlow(config);
        setNodes(parsedNodes);
        setEdges(parsedEdges);
      }
    } catch (error) {
      console.error('Error parsing JSON:', error);
      alert('Invalid JSON format. Please check your input.');
    }
  }, [jsonInput, viewMode, layoutDirection, setNodes, setEdges]);

  const handleLoadExample = useCallback(() => {
    if (viewMode === 'bridge') {
      const { nodes: parsedNodes, edges: parsedEdges } = parseBridgeToFlow(exampleBridgeConfig);
      setNodes(parsedNodes);
      setEdges(parsedEdges);
    }
  }, [viewMode, setNodes, setEdges]);

  const handleAutoLayout = useCallback(() => {
    const layouted = getLayoutedElements(nodes, edges, layoutDirection);
    setNodes(layouted.nodes);
    setEdges(layouted.edges);
  }, [nodes, edges, layoutDirection, setNodes, setEdges]);

  return (
    <div className="app">
      <div className="controls">
        <div style={{ marginBottom: '10px' }}>
          <label>
            View Mode:
            <select 
              value={viewMode} 
              onChange={(e) => setViewMode(e.target.value as 'service' | 'bridge')}
              style={{ marginLeft: '10px', marginRight: '20px' }}
            >
              <option value="service">Service View</option>
              <option value="bridge">Bridge View</option>
            </select>
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
            placeholder={viewMode === 'service' ? 
              "Paste your service.json content here..." : 
              "Paste your bridge configuration JSON here..."}
            style={{ width: '80%', height: '100px', marginRight: '10px' }}
          />
          <button onClick={handleLoadJson} style={{ marginRight: '10px' }}>
            Load JSON
          </button>
          {viewMode === 'bridge' && (
            <button onClick={handleLoadExample} style={{ marginRight: '10px' }}>
              Load Example
            </button>
          )}
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