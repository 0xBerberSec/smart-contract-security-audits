// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

/// @notice Minimal interface for a local, educational ETH-refund protocol.
interface IRefundEscrow {
    function deposit() external payable;
    function refund() external;
    function credits(address account) external view returns (uint256);
}

abstract contract RefundEscrowLedger is IRefundEscrow {
    mapping(address => uint256) public credits;

    error EmptyDeposit();
    error NothingToRefund();
    error TransferFailed();

    function deposit() external payable {
        if (msg.value == 0) revert EmptyDeposit();
        credits[msg.sender] += msg.value;
    }
}

/// @notice Intentionally vulnerable teaching contract. Never deploy with real funds.
contract VulnerableRefundEscrow is RefundEscrowLedger {
    function refund() external {
        uint256 amount = credits[msg.sender];
        if (amount == 0) revert NothingToRefund();

        // BUG: an external receiver can consume the same credit before this write.
        (bool sent,) = payable(msg.sender).call{value: amount}("");
        if (!sent) revert TransferFailed();
        credits[msg.sender] = 0;
    }
}

/// @notice Proposed teaching fix: consume the entitlement before the external call.
contract FixedRefundEscrow is RefundEscrowLedger {
    function refund() external {
        uint256 amount = credits[msg.sender];
        if (amount == 0) revert NothingToRefund();

        credits[msg.sender] = 0;
        (bool sent,) = payable(msg.sender).call{value: amount}("");
        // Reverting restores the credit too; a refusing recipient does not burn it.
        if (!sent) revert TransferFailed();
    }
}
