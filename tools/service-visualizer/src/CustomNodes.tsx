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
    }}
>
      <Handle type="target" position={Position.Top} />
      <div className="selectable-text">{data.label}</div>
      <div className="selectable-text" style={{ fontSize: '10px', marginTop: '5px' }}>
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
      minWidth: '180px'
    }}>
      <Handle type="target" position={Position.Top} />
      <div style={{ fontWeight: 'bold' }}>{data.label}</div>
      <div style={{ fontSize: '10px', marginTop: '5px', wordBreak: 'break-all' }}>
        {data.address}
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
      minWidth: '250px'
    }}
>
      <Handle type="target" position={Position.Top} />
      <div className="selectable-text" style={{ fontWeight: 'bold' }}>{data.label}</div>
      {data.version && (
        <div className="selectable-text" style={{ fontSize: '10px', marginTop: '3px' }}>
          Version: {data.version}
        </div>
      )}
      <div className="selectable-text" style={{ fontSize: '10px', marginTop: '5px' }}>
        Chain: {data.chain}
      </div>
      {data.digest && (
        <div className="selectable-text" style={{ fontSize: '9px', marginTop: '3px', wordBreak: 'break-all' }}>
          Digest: <span style={{ userSelect: 'all' }}>{data.digest}</span>
        </div>
      )}
      <div className="selectable-text" style={{ fontSize: '9px', marginTop: '3px', opacity: 0.8, wordBreak: 'break-all' }}>
        ID: {data.workflowId}
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
      fontSize: '12px',
      minWidth: '380px'
    }}
>
      <div className="selectable-text" style={{ fontWeight: 'bold' }}>{data.label}</div>
      {data.chain && (
        <div className="selectable-text" style={{ fontSize: '10px', marginTop: '5px' }}>
          Chain: {data.chain}
        </div>
      )}
      {data.blocks && (
        <div className="selectable-text" style={{ fontSize: '10px', marginTop: '3px' }}>
          Every {data.blocks} blocks
        </div>
      )}
      {data.address && (
        <div className="selectable-text" style={{ fontSize: '9px', marginTop: '5px', wordBreak: 'break-all' }}>
          Contract: <span style={{ userSelect: 'all' }}>{data.address}</span>
        </div>
      )}
      {data.eventHash && (
        <div className="selectable-text" style={{ fontSize: '9px', marginTop: '3px', wordBreak: 'break-all', opacity: 0.8 }}>
          Event: <span style={{ userSelect: 'all' }}>{data.eventHash}</span>
        </div>
      )}
      <Handle type="source" position={Position.Right} />
    </div>
  );
}

export function ComponentNode({ data }: any) {
  return (
    <div style={{
      background: data.isAggregator ? '#9c27b0' : '#4caf50',
      color: 'white',
      padding: '10px',
      borderRadius: '6px',
      minWidth: '250px'
    }}
>
      <Handle type="target" position={Position.Top} />
      <div className="selectable-text" style={{ fontWeight: 'bold' }}>Component</div>
      <div className="selectable-text" style={{ fontSize: '11px', marginTop: '5px' }}>
        {data.label}
      </div>
      {data.version && (
        <div className="selectable-text" style={{ fontSize: '10px', marginTop: '3px' }}>
          Version: {data.version}
        </div>
      )}
      {data.domain && (
        <div className="selectable-text" style={{ fontSize: '9px', marginTop: '3px', opacity: 0.8 }}>
          Registry: {data.domain}
        </div>
      )}
      {data.digest && (
        <div className="selectable-text" style={{ fontSize: '9px', marginTop: '3px', wordBreak: 'break-all' }}>
          Digest: <span style={{ userSelect: 'all' }}>{data.digest}</span>
        </div>
      )}
      {data.config && Object.keys(data.config).length > 0 && (
        <div style={{ marginTop: '5px', borderTop: '1px solid rgba(255,255,255,0.3)', paddingTop: '5px' }}>
          <div className="selectable-text" style={{ fontSize: '9px', fontWeight: 'bold' }}>Config:</div>
          {Object.entries(data.config).map(([key, value]) => (
            <div key={key} className="selectable-text" style={{ fontSize: '8px', marginTop: '2px', wordBreak: 'break-all' }}>
              {key}: <span style={{ userSelect: 'all' }}>{String(value)}</span>
            </div>
          ))}
        </div>
      )}
      {data.permissions && (
        <div style={{ marginTop: '5px', borderTop: '1px solid rgba(255,255,255,0.3)', paddingTop: '5px' }}>
          <div className="selectable-text" style={{ fontSize: '9px', fontWeight: 'bold' }}>Permissions:</div>
          <div className="selectable-text" style={{ fontSize: '8px', marginTop: '2px' }}>
            HTTP: {data.permissions.allowed_http_hosts || 'none'}
          </div>
          <div className="selectable-text" style={{ fontSize: '8px', marginTop: '2px' }}>
            FileSystem: {data.permissions.file_system ? 'Yes' : 'No'}
          </div>
        </div>
      )}
      {data.envKeys && data.envKeys.length > 0 && (
        <div style={{ marginTop: '5px', borderTop: '1px solid rgba(255,255,255,0.3)', paddingTop: '5px' }}>
          <div className="selectable-text" style={{ fontSize: '9px', fontWeight: 'bold' }}>Env Keys:</div>
          {data.envKeys.map((key: string) => (
            <div key={key} className="selectable-text" style={{ fontSize: '8px', marginTop: '2px' }}>
              {key}
            </div>
          ))}
        </div>
      )}
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}

export function AggregatorNode({ data }: any) {
  return (
    <div style={{
      background: data.isDestination ? '#2e7d32' : '#f44336',
      color: 'white',
      padding: '8px',
      borderRadius: '4px',
      minWidth: '200px',
      border: data.isDestination ? '2px solid #4caf50' : 'none'
    }}
>
      <Handle type="target" position={Position.Top} />
      <div className="selectable-text" style={{ fontWeight: 'bold' }}>{data.label}</div>
      {data.isDestination && data.workflowCount > 1 && (
        <div className="selectable-text" style={{ fontSize: '9px', marginTop: '3px' }}>
          ({data.workflowCount} workflows)
        </div>
      )}
      <div className="selectable-text" style={{ fontSize: '10px', marginTop: '5px' }}>
        Chain: {data.chain}
      </div>
      <div className="selectable-text" style={{ fontSize: '9px', marginTop: '3px' }}>
        Handler Contract:
      </div>
      <div className="selectable-text" style={{ fontSize: '9px', wordBreak: 'break-all' }}>
        <span style={{ userSelect: 'all' }}>{data.address}</span>
      </div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}