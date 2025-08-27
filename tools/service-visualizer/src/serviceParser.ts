import { Node, Edge } from 'reactflow';
import { ServiceConfig, BridgeConfig } from './types';

export function parseServiceToFlow(service: ServiceConfig): { nodes: Node[]; edges: Edge[] } {
  const nodes: Node[] = [];
  const edges: Edge[] = [];
  
  const serviceNodeId = `service-${service.id}`;
  nodes.push({
    id: serviceNodeId,
    type: 'service',
    position: { x: 400, y: 50 },
    data: { 
      label: service.name,
      status: service.status,
      id: service.id
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
        label: workflow.component.source.Registry?.registry.package || 'Unknown Component',
        version: workflow.component.source.Registry?.registry.version,
        workflowId: workflowId.slice(0, 8),
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

    if (workflow.component.source.Registry) {
      const componentNodeId = `component-${workflowId}`;
      nodes.push({
        id: componentNodeId,
        type: 'component',
        position: { x: 200 + index * 400, y: yOffset + 300 },
        data: {
          label: workflow.component.source.Registry.registry.package,
          version: workflow.component.source.Registry.registry.version,
          domain: workflow.component.source.Registry.registry.domain
        }
      });

      edges.push({
        id: `${workflowNodeId}-${componentNodeId}`,
        source: workflowNodeId,
        target: componentNodeId
      });
    }

    // Handle submit aggregator contracts
    if (workflow.submit?.aggregator?.evm_contracts) {
      workflow.submit.aggregator.evm_contracts.forEach((contract, contractIndex) => {
        const aggNodeId = `aggregator-${workflowId}-${contractIndex}`;
        nodes.push({
          id: aggNodeId,
          type: 'aggregator',
          position: { x: 350 + index * 400 + contractIndex * 50, y: yOffset + 150 },
          data: {
            label: `Submit Contract`,
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