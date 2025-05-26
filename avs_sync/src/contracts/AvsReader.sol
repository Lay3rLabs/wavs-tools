interface IRegistryCoordinator {
    function QuorumCount() external view returns (uint8);
    function GetOperatorId(address operator) external view returns (bytes32);
    function GetOperatorFromId(bytes32 operatorId) external view returns (address);
    function GetOperatorStatus(address operator) external view returns (uint8); // 0 = never, 1 = reg, 2 = de-reg
    function GetCurrentQuorumBitmap(bytes32 operatorId) external view returns (uint256);
}

// IStakeRegistry.sol
interface IStakeRegistry {
    function GetCurrentStake(bytes32 operatorId, uint8 quorumNumber) external view returns (uint256);
    function GetLatestStakeUpdate(bytes32 operatorId, uint8 quorumNumber) external view returns (StakeUpdate memory);
}

// IBLSApkRegistry.sol
interface IBLSApkRegistry {
    function OperatorToPubkeyHash(address operator) external view returns (bytes32);
    function PubkeyHashToOperator(bytes32 pubkeyHash) external view returns (address);
    function OperatorToPubkey(address operator) external view returns (G1Point memory);
}

struct G1Point {
    uint256 X;
    uint256 Y;
}

struct StakeUpdate {
    uint256 blockNumber;
    uint256 stake;
}

function getQuorumCount(IRegistryCoordinator registry)
    public
    view
    returns (uint8)
{
    return registry.QuorumCount();
}


