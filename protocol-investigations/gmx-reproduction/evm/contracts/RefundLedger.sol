// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @notice Stand-alone educational model of the refund-list accounting question.
/// @dev This is not a copy of GMX ExternalHandler and is not a production patch.
contract RefundLedger {
    mapping(address => uint256) public balances;

    function deposit(uint256 amount) external {
        balances[address(this)] += amount;
    }

    function refundListed(address recipient, uint256 amount, bool listed) external {
        if (listed) {
            require(balances[address(this)] >= amount, "insufficient");
            balances[address(this)] -= amount;
            balances[recipient] += amount;
        }
    }

    function depositThenRevert(uint256 amount) external {
        balances[address(this)] += amount;
        revert("route reverted");
    }
}
