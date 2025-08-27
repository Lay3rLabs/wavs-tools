import dagre from 'dagre';
import { Node, Edge, Position } from 'reactflow';

const nodeDimensions: Record<string, { width: number; height: number }> = {
  service: { width: 200, height: 80 },
  workflow: { width: 250, height: 90 },
  trigger: { width: 400, height: 100 },
  component: { width: 250, height: 90 },
  aggregator: { width: 380, height: 70 },
  chain: { width: 380, height: 100 },
  bridge: { width: 140, height: 50 },
  contract: { width: 380, height: 60 },
  default: { width: 172, height: 36 }
};

export function getLayoutedElements(
  nodes: Node[],
  edges: Edge[],
  direction: 'TB' | 'LR' = 'TB'
) {
  if (!nodes.length) return { nodes, edges };
  
  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));

  const isHorizontal = direction === 'LR';
  dagreGraph.setGraph({ 
    rankdir: direction, 
    nodesep: 150, 
    ranksep: 150,
    marginx: 50,
    marginy: 50
  });

  // Set nodes with proper dimensions
  nodes.forEach((node) => {
    const dimensions = nodeDimensions[node.type || 'default'] || nodeDimensions.default;
    dagreGraph.setNode(node.id, { 
      width: dimensions.width, 
      height: dimensions.height 
    });
  });

  // Set edges
  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  // Run the layout algorithm
  dagre.layout(dagreGraph);

  // Apply the computed positions back to the nodes
  const layoutedNodes = nodes.map((node) => {
    const nodeWithPosition = dagreGraph.node(node.id);
    const dimensions = nodeDimensions[node.type || 'default'] || nodeDimensions.default;
    
    return {
      ...node,
      targetPosition: isHorizontal ? Position.Left : Position.Top,
      sourcePosition: isHorizontal ? Position.Right : Position.Bottom,
      position: {
        x: nodeWithPosition.x - dimensions.width / 2,
        y: nodeWithPosition.y - dimensions.height / 2,
      }
    };
  });

  return { nodes: layoutedNodes, edges };
}