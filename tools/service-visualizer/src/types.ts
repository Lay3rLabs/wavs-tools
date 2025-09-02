export interface ServiceConfig {
  id?: string;
  name: string;
  workflows: Record<string, Workflow>;
  status: string;
  manager?: {
    evm?: {
      chain_name: string;
      address: string;
    };
  };
}

export interface Workflow {
  trigger: {
    block_interval?: {
      chain_name: string;
      n_blocks: number;
      start_block?: number | null;
      end_block?: number | null;
    };
    evm_contract_event?: {
      address: string;
      chain_name: string;
      event_hash: string;
    };
  } | any;
  component: {
    source: {
      Registry?: {
        registry: {
          digest: string;
          domain: string;
          version: string;
          package: string;
        };
      };
      Digest?: string;
    };
    permissions?: {
      allowed_http_hosts?: string;
      file_system?: boolean;
    };
    fuel_limit?: number | null;
    time_limit_seconds?: number | null;
    config?: Record<string, string>;
    env_keys?: string[];
  };
  submit?: {
    aggregator?: {
      url: string;
      component?: any;
      evm_contracts?: Array<{
        chain_name: string;
        address: string;
        max_gas?: number;
      }>;
      cosmos_contracts?: any;
    };
  };
  aggregators?: Array<{
    evm?: {
      chain_name: string;
      address: string;
      max_gas?: number | null;
    };
  }>;
}

export interface BridgeConfig {
  chains: Chain[];
  bridges: Bridge[];
}

export interface Chain {
  name: string;
  contracts: Contract[];
}

export interface Contract {
  type: 'DepositToken' | 'Receiver';
  address: string;
}

export interface Bridge {
  from_chain: string;
  to_chain: string;
  from_contract: string;
  to_contract: string;
  component: string;
}