import React from 'react';
import { Handle, Position } from 'reactflow';

export function ServiceNode({ data }: any) {
  return (
    <div style={{
      background: '#2196f3',
      color: 'white',
      padding: '10px 15px',
      borderRadius: '8px',
      fontWeight: 'bold',
      minWidth: '150px',
      textAlign: 'center'
    }}>
      <Handle type="target" position={Position.Top} />
      <div>{data.label}</div>
      <div style={{ fontSize: '10px', marginTop: '5px' }}>
        Status: {data.status}
      </div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}

export function ChainNode({ data }: any) {
  return (
    <div style={{
      background: '#e3f2fd',
      border: '2px solid #1976d2',
      borderRadius: '8px',
      padding: '15px',
      minWidth: '200px'
    }}>
      <Handle type="target" position={Position.Top} />
      <div style={{ fontWeight: 'bold', marginBottom: '10px' }}>{data.label}</div>
      {data.contracts?.map((contract: any, index: number) => (
        <div key={index} style={{ fontSize: '12px', marginTop: '5px' }}>
          {contract.type}: {contract.address.slice(0, 10)}...
        </div>
      ))}
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}

export function BridgeNode({ data }: any) {
  return (
    <div style={{
      background: '#c8e6c9',
      border: '2px solid #4caf50',
      borderRadius: '20px',
      padding: '8px 16px',
      textAlign: 'center'
    }}>
      <Handle type="target" position={Position.Left} />
      <div style={{ fontWeight: 'bold' }}>Bridge</div>
      <div style={{ fontSize: '10px', marginTop: '5px' }}>
        {data.component}
      </div>
      <div style={{ fontSize: '9px', marginTop: '3px' }}>
        {data.from} → {data.to}
      </div>
      <Handle type="source" position={Position.Right} />
    </div>
  );
}

export function ContractNode({ data }: any) {
  return (
    <div style={{
      background: data.label === 'DepositToken' ? '#fff3e0' : '#f3e5f5',
      border: '1px solid #666',
      borderRadius: '4px',
      padding: '8px',
      fontSize: '12px',
      minWidth: '120px'
    }}>
      <Handle type="target" position={Position.Top} />
      <div style={{ fontWeight: 'bold' }}>{data.label}</div>
      <div style={{ fontSize: '10px', marginTop: '5px' }}>
        {data.address.slice(0, 8)}...
      </div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}

export function WorkflowNode({ data }: any) {
  return (
    <div style={{
      background: '#673ab7',
      color: 'white',
      padding: '10px',
      borderRadius: '6px',
      minWidth: '150px'
    }}>
      <Handle type="target" position={Position.Top} />
      <div>{data.label}</div>
      <div style={{ fontSize: '10px', marginTop: '5px' }}>
        Chain: {data.trigger}
      </div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}

export function TriggerNode({ data }: any) {
  return (
    <div style={{
      background: '#ff9800',
      color: 'white',
      padding: '8px',
      borderRadius: '4px',
      fontSize: '12px'
    }}>
      <div>{data.label}</div>
      <div style={{ fontSize: '10px', marginTop: '5px' }}>
        {data.blocks} blocks
      </div>
      <Handle type="source" position={Position.Right} />
    </div>
  );
}

export function ComponentNode({ data }: any) {
  return (
    <div style={{
      background: '#4caf50',
      color: 'white',
      padding: '10px',
      borderRadius: '6px',
      minWidth: '180px'
    }}>
      <Handle type="target" position={Position.Top} />
      <div style={{ fontWeight: 'bold' }}>{data.label}</div>
      <div style={{ fontSize: '10px', marginTop: '5px' }}>
        v{data.version}
      </div>
      <div style={{ fontSize: '9px', marginTop: '3px' }}>
        {data.domain}
      </div>
    </div>
  );
}

export function AggregatorNode({ data }: any) {
  return (
    <div style={{
      background: '#f44336',
      color: 'white',
      padding: '8px',
      borderRadius: '4px',
      minWidth: '140px'
    }}>
      <Handle type="target" position={Position.Left} />
      <div>{data.label}</div>
      <div style={{ fontSize: '10px', marginTop: '5px' }}>
        {data.chain}
      </div>
      <div style={{ fontSize: '9px', marginTop: '3px' }}>
        {data.address.slice(0, 10)}...
      </div>
    </div>
  );
}