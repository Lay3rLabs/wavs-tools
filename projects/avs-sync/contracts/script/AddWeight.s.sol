// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script, console} from "forge-std/Script.sol";
import {IStrategy} from "eigenlayer-contracts/src/contracts/interfaces/IStrategy.sol";
import {IStrategyManager} from "eigenlayer-contracts/src/contracts/interfaces/IStrategyManager.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title AddWeight
 * @notice Simple script to add weight to a single operator by staking more tokens
 *
 * Usage:
 * forge script script/AddWeight.s.sol:AddWeight --rpc-url $RPC_URL --broadcast
 *
 * Environment variables:
 * - OPERATOR_ADDRESS: The operator to add weight to
 * - LST_CONTRACT_ADDRESS: The LST token contract
 * - LST_STRATEGY_ADDRESS: The strategy contract for the LST
 * - STRATEGY_MANAGER_ADDRESS: EigenLayer strategy manager
 * - AMOUNT: Amount to add (in wei, e.g., 1000000000000000000 for 1 ETH)
 */
contract AddWeight is Script {
    function run() public {
        address operatorAddress = vm.envAddress("OPERATOR_ADDRESS");
        address lstContractAddress = vm.envAddress("LST_CONTRACT_ADDRESS");
        address lstStrategyAddress = vm.envAddress("LST_STRATEGY_ADDRESS");
        address strategyManagerAddress = vm.envAddress("STRATEGY_MANAGER_ADDRESS");
        uint256 amount = vm.envUint("AMOUNT");

        console.log("Adding weight to operator:", operatorAddress);
        console.log("Amount to add:", amount);

        IERC20 lstToken = IERC20(lstContractAddress);
        IStrategy lstStrategy = IStrategy(lstStrategyAddress);
        IStrategyManager strategyManager = IStrategyManager(strategyManagerAddress);

        // Check current balance
        uint256 currentBalance = lstToken.balanceOf(operatorAddress);
        console.log("Current LST balance:", currentBalance);

        // Try to mint if not enough LST (only works for LSTs with submit(address))
        if (currentBalance < amount) {
            console.log("Minting LST tokens...");

            vm.broadcast(operatorAddress);
            (bool success,) =
                lstContractAddress.call{value: amount}(abi.encodeWithSignature("submit(address)", address(0)));

            if (!success) {
                revert("Cannot mint LST tokens");
            }

            console.log("Successfully minted LST tokens");
        }

        // Broadcast approve from operator
        vm.broadcast(operatorAddress);
        lstToken.approve(strategyManagerAddress, amount);

        // Get shares before deposit
        uint256 sharesBefore = strategyManager.stakerDepositShares(operatorAddress, lstStrategy);
        console.log("Shares before deposit:", sharesBefore);

        // Broadcast deposit from operator
        vm.broadcast(operatorAddress);
        strategyManager.depositIntoStrategy(lstStrategy, lstToken, amount);

        // Get shares after deposit
        uint256 sharesAfter = strategyManager.stakerDepositShares(operatorAddress, lstStrategy);
        console.log("Shares after deposit:", sharesAfter);
        console.log("New shares added:", sharesAfter - sharesBefore);
    }
}
