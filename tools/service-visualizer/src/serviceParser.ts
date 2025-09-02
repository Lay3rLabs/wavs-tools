import { Node, Edge } from 'reactflow';
import { ServiceConfig, BridgeConfig } from './types';

export function parseServiceToFlow(service: ServiceConfig): { nodes: Node[]; edges: Edge[] } {
  const nodes: Node[] = [];
  const edges: Edge[] = [];
  
  const serviceNodeId = `service-${service.id || 'main'}`;
  nodes.push({
    id: serviceNodeId,
    type: 'service',
    position: { x: 400, y: 50 },
    data: { 
      label: service.name,
      status: service.status,
      id: service.id || 'main'
    }
  });

  // Collect all unique destinations across all workflows
  const destinationMap = new Map<string, { chain: string; address: string; workflows: string[] }>();
  const aggregatorComponents = new Map<string, string>(); // workflowId -> aggComponentNodeId
  
  Object.entries(service.workflows).forEach(([workflowId, workflow]) => {
    if (workflow.submit?.aggregator?.component?.config) {
      Object.entries(workflow.submit.aggregator.component.config).forEach(([chainName, address]) => {
        const key = `${chainName}-${address}`;
        if (!destinationMap.has(key)) {
          destinationMap.set(key, {
            chain: chainName,
            address: address as string,
            workflows: [workflowId]
          });
        } else {
          destinationMap.get(key)!.workflows.push(workflowId);
        }
      });
    }
  });

  let yOffset = 200;
  Object.entries(service.workflows).forEach(([workflowId, workflow], index) => {
    const workflowNodeId = `workflow-${workflowId}`;
    
    nodes.push({
      id: workflowNodeId,
      type: 'workflow',
      position: { x: 200 + index * 400, y: yOffset },
      data: {
        label: workflow.component.source.Registry?.registry.package || 
                (workflow.component.source.Digest ? 'Digest Component' : 'Unknown Component'),
        digest: workflow.component.source.Digest,
        version: workflow.component.source.Registry?.registry.version,
        workflowId: workflowId,
        chain: workflow.trigger.block_interval?.chain_name || workflow.trigger.evm_contract_event?.chain_name || 'Unknown'
      }
    });

    edges.push({
      id: `${serviceNodeId}-${workflowNodeId}`,
      source: serviceNodeId,
      target: workflowNodeId,
      animated: true
    });

    // Handle different trigger types
    const trigger = workflow.trigger as any;
    if (trigger?.block_interval) {
      const triggerNodeId = `trigger-${workflowId}`;
      nodes.push({
        id: triggerNodeId,
        type: 'trigger',
        position: { x: 50 + index * 400, y: yOffset + 150 },
        data: {
          label: `Block Interval Trigger`,
          chain: trigger.block_interval.chain_name,
          blocks: trigger.block_interval.n_blocks
        }
      });

      edges.push({
        id: `${triggerNodeId}-${workflowNodeId}`,
        source: triggerNodeId,
        target: workflowNodeId,
        label: `Every ${trigger.block_interval.n_blocks} blocks`
      });
    } else if (trigger?.evm_contract_event) {
      const triggerNodeId = `trigger-${workflowId}`;
      nodes.push({
        id: triggerNodeId,
        type: 'trigger',
        position: { x: 50 + index * 400, y: yOffset + 150 },
        data: {
          label: `Event Trigger`,
          chain: trigger.evm_contract_event.chain_name,
          address: trigger.evm_contract_event.address,
          eventHash: trigger.evm_contract_event.event_hash
        }
      });

      edges.push({
        id: `${triggerNodeId}-${workflowNodeId}`,
        source: triggerNodeId,
        target: workflowNodeId,
        label: 'Contract Event'
      });
    }

    // Show component details
    const componentNodeId = `component-${workflowId}`;
    if (workflow.component.source.Registry) {
      nodes.push({
        id: componentNodeId,
        type: 'component',
        position: { x: 200 + index * 400, y: yOffset + 300 },
        data: {
          label: workflow.component.source.Registry.registry.package,
          version: workflow.component.source.Registry.registry.version,
          domain: workflow.component.source.Registry.registry.domain,
          digest: workflow.component.source.Registry.registry.digest,
          config: workflow.component.config,
          permissions: workflow.component.permissions,
          envKeys: workflow.component.env_keys
        }
      });

      edges.push({
        id: `${workflowNodeId}-${componentNodeId}`,
        source: workflowNodeId,
        target: componentNodeId
      });
    } else if (workflow.component.source.Digest) {
      nodes.push({
        id: componentNodeId,
        type: 'component',
        position: { x: 200 + index * 400, y: yOffset + 300 },
        data: {
          label: 'Digest Component',
          digest: workflow.component.source.Digest,
          config: workflow.component.config,
          permissions: workflow.component.permissions,
          envKeys: workflow.component.env_keys
        }
      });

      edges.push({
        id: `${workflowNodeId}-${componentNodeId}`,
        source: workflowNodeId,
        target: componentNodeId
      });
    }

    // Handle submit aggregator contracts and aggregator component
    if (workflow.submit?.aggregator) {
      const agg = workflow.submit.aggregator;
      
      // Create aggregator node if there's a component
      if (agg.component) {
        const aggComponentNodeId = `aggregator-component-${workflowId}`;
        nodes.push({
          id: aggComponentNodeId,
          type: 'component',
          position: { x: 200 + index * 400, y: yOffset + 450 },
          data: {
            label: 'Aggregator Component',
            digest: agg.component.source?.Digest || agg.component.source?.Registry?.registry.digest,
            config: agg.component.config,
            permissions: agg.component.permissions,
            isAggregator: true
          }
        });

        // Create edge from workflow to aggregator component
        edges.push({
          id: `${workflowNodeId}-${aggComponentNodeId}`,
          source: workflowNodeId,
          target: aggComponentNodeId,
          label: 'Aggregator',
          style: { stroke: '#9c27b0', strokeWidth: 2 },
          animated: true
        });
        
        // Store aggregator component for later destination connections
        aggregatorComponents.set(workflowId, aggComponentNodeId);
      }
      
      // Also show direct evm_contracts if present (without aggregator component)
      else if (agg.evm_contracts) {
        agg.evm_contracts.forEach((contract, contractIndex) => {
          const aggNodeId = `aggregator-${workflowId}-${contractIndex}`;
          nodes.push({
            id: aggNodeId,
            type: 'aggregator',
            position: { x: 350 + index * 400 + contractIndex * 50, y: yOffset + 150 },
            data: {
              label: `Submit: ${contract.chain_name}`,
              chain: contract.chain_name,
              address: contract.address,
              fullAddress: true
            }
          });

          edges.push({
            id: `${workflowNodeId}-${aggNodeId}`,
            source: workflowNodeId,
            target: aggNodeId,
            label: 'Submit to'
          });
        });
      }
    }
  });

  // Create unique destination nodes and connect them to their aggregator components
  let destIndex = 0;
  destinationMap.forEach((dest, key) => {
    const destNodeId = `destination-${key}`;
    nodes.push({
      id: destNodeId,
      type: 'aggregator',
      position: { x: 100 + destIndex * 400, y: yOffset + 600 },
      data: {
        label: `Service Handler`,
        chain: dest.chain,
        address: dest.address,
        fullAddress: true,
        isDestination: true,
        workflowCount: dest.workflows.length,
        workflows: dest.workflows
      }
    });

    // Create edges from each aggregator component to this destination
    dest.workflows.forEach(workflowId => {
      const aggComponentNodeId = aggregatorComponents.get(workflowId);
      if (aggComponentNodeId) {
        edges.push({
          id: `${aggComponentNodeId}-${destNodeId}`,
          source: aggComponentNodeId,
          target: destNodeId,
          label: dest.workflows.length > 1 ? `To ${dest.chain} (${dest.workflows.length} workflows)` : `To ${dest.chain}`,
          style: { stroke: '#4caf50', strokeWidth: 2 },
          animated: true
        });
      }
    });
    
    destIndex++;
  });

  return { nodes, edges };
}

export function parseBridgeToFlow(config: BridgeConfig): { nodes: Node[]; edges: Edge[] } {
  const nodes: Node[] = [];
  const edges: Edge[] = [];
  const chainPositions = new Map<string, { x: number; y: number }>();

  const chainCount = config.chains.length;
  const radius = 300;
  const centerX = 500;
  const centerY = 400;

  config.chains.forEach((chain, index) => {
    const angle = (2 * Math.PI * index) / chainCount;
    const x = centerX + radius * Math.cos(angle);
    const y = centerY + radius * Math.sin(angle);
    
    chainPositions.set(chain.name, { x, y });

    nodes.push({
      id: `chain-${chain.name}`,
      type: 'chain',
      position: { x, y },
      data: { 
        label: chain.name,
        contracts: chain.contracts
      },
      style: {
        background: '#e3f2fd',
        border: '2px solid #1976d2',
        borderRadius: '8px',
        padding: '10px'
      }
    });

    chain.contracts.forEach((contract, contractIndex) => {
      const contractId = `${chain.name}-${contract.type}-${contract.address}`;
      nodes.push({
        id: contractId,
        type: 'contract',
        position: { 
          x: x + (contractIndex - chain.contracts.length/2) * 120,
          y: y + 80
        },
        data: {
          label: contract.type,
          address: contract.address,
          chain: chain.name
        },
        parentNode: `chain-${chain.name}`,
        style: {
          background: contract.type === 'DepositToken' ? '#fff3e0' : '#f3e5f5',
          border: '1px solid #666',
          borderRadius: '4px',
          fontSize: '12px',
          padding: '5px'
        }
      });
    });
  });

  config.bridges.forEach((bridge, index) => {
    const bridgeId = `bridge-${index}`;
    const fromPos = chainPositions.get(bridge.from_chain);
    const toPos = chainPositions.get(bridge.to_chain);
    
    if (fromPos && toPos) {
      const midX = (fromPos.x + toPos.x) / 2;
      const midY = (fromPos.y + toPos.y) / 2;
      
      nodes.push({
        id: bridgeId,
        type: 'bridge',
        position: { x: midX - 50, y: midY - 20 },
        data: {
          label: 'Bridge',
          component: bridge.component,
          from: bridge.from_chain,
          to: bridge.to_chain
        },
        style: {
          background: '#c8e6c9',
          border: '2px solid #4caf50',
          borderRadius: '20px',
          padding: '8px 16px'
        }
      });

      edges.push({
        id: `${bridge.from_contract}-${bridgeId}`,
        source: `${bridge.from_chain}-DepositToken-${bridge.from_contract}`,
        target: bridgeId,
        animated: true,
        style: { stroke: '#4caf50', strokeWidth: 2 },
        label: bridge.from_chain,
        labelStyle: { fontSize: 12 }
      });

      edges.push({
        id: `${bridgeId}-${bridge.to_contract}`,
        source: bridgeId,
        target: `${bridge.to_chain}-Receiver-${bridge.to_contract}`,
        animated: true,
        style: { stroke: '#4caf50', strokeWidth: 2 },
        label: bridge.to_chain,
        labelStyle: { fontSize: 12 }
      });
    }
  });

  return { nodes, edges };
}