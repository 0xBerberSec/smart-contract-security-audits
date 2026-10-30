// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import {IRefundEscrow} from "../../src/RefundEscrow.sol";

/// @dev Bounded callback used only by the local teaching tests.
contract ReentrantRefundReceiver {
    IRefundEscrow public immutable escrow;
    uint256 public attempts;
    bool public lastReentrySucceeded;
    uint256 private maxAttempts;
    uint256 private refundSize;

    constructor(IRefundEscrow target) {
        escrow = target;
    }

    function run(uint256 maximumAttempts) external payable {
        attempts = 0;
        lastReentrySucceeded = false;
        maxAttempts = maximumAttempts;
        refundSize = msg.value;
        escrow.deposit{value: msg.value}();
        escrow.refund();
    }

    receive() external payable {
        if (attempts < maxAttempts && address(escrow).balance >= refundSize) {
            ++attempts;
            // Catch the fixed contract's rejection so the legitimate refund can finish.
            (bool ok,) = address(escrow).call(abi.encodeCall(IRefundEscrow.refund, ()));
            lastReentrySucceeded = ok;
        }
    }
}

contract RejectingRefundReceiver {
    IRefundEscrow public immutable escrow;

    constructor(IRefundEscrow target) {
        escrow = target;
    }

    function fund() external payable {
        escrow.deposit{value: msg.value}();
    }

    function requestRefund() external {
        escrow.refund();
    }

    receive() external payable {
        revert("RECEIVER_REFUSES_ETH");
    }
}
