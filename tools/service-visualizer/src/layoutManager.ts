import dagre from 'dagre';
import { Node, Edge, Position } from 'reactflow';

const nodeDimensions: Record<string, { width: number; height: number }> = {
  service: { width: 180, height: 60 },
  workflow: { width: 380, height: 120 },
  trigger: { width: 350, height: 90 },
  component: { width: 400, height: 200 },
  aggregator: { width: 350, height: 60 },
  chain: { width: 350, height: 90 },
  bridge: { width: 120, height: 40 },
  contract: { width: 350, height: 50 },
  default: { width: 150, height: 30 }
};

export function getLayoutedElements(
  nodes: Node[],
  edges: Edge[],
  direction: 'TB' | 'LR' = 'TB',
  compact: boolean = true
) {
  if (!nodes.length) return { nodes, edges };
  
  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));

  const isHorizontal = direction === 'LR';
  dagreGraph.setGraph({ 
    rankdir: direction, 
    nodesep: compact ? 10 : 100,  // horizontal spacing between nodes (reduced from 30)
    ranksep: compact ? 20 : 120,  // vertical spacing between ranks (reduced from 50)
    marginx: compact ? 5 : 40,    // margin around the whole graph (reduced from 10)
    marginy: compact ? 5 : 40,    // reduced from 10
    edgesep: compact ? 5 : 20,    // spacing between edges
    acyclicer: 'greedy',
    ranker: 'tight-tree'          // use tighter ranking algorithm
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