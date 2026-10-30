// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import {FixedRefundEscrow} from "../src/RefundEscrow.sol";
import {TestSupport} from "./helpers/TestSupport.sol";
import {ReentrantRefundReceiver} from "./helpers/RefundReceivers.sol";
import {InvariantTargeting} from "./helpers/InvariantTargeting.sol";

/// @dev Closed actor model: no untracked depositor or arbitrary external call.
contract RefundHandler is TestSupport {
    FixedRefundEscrow public immutable escrow;
    ReentrantRefundReceiver public immutable receiver;
    uint256 public deposits;
    uint256 public refunds;
    uint256 public callbackExperiments;

    constructor(FixedRefundEscrow target) {
        escrow = target;
        receiver = new ReentrantRefundReceiver(target);
    }

    function deposit(uint96 seed, bool chooseBob) external {
        address actor = chooseBob ? BOB : ALICE;
        uint256 amount = uint256(seed) % 10 ether + 1;
        vm.deal(actor, actor.balance + amount);
        vm.prank(actor);
        escrow.deposit{value: amount}();
        ++deposits;
    }

    function refund(bool chooseBob) external {
        address actor = chooseBob ? BOB : ALICE;
        if (escrow.credits(actor) == 0) return;
        vm.prank(actor);
        escrow.refund();
        ++refunds;
    }

    function attack(uint96 seed) external {
        uint256 amount = uint256(seed) % 10 ether + 1;
        vm.deal(address(this), amount);
        uint256 before = address(receiver).balance;
        receiver.run{value: amount}(1);
        eq(address(receiver).balance - before, amount, "handler observed an extra refund");
        ++callbackExperiments;
    }

    function liabilities() external view returns (uint256) {
        return escrow.credits(ALICE) + escrow.credits(BOB) + escrow.credits(address(receiver));
    }
}

contract FixedSolvencyInvariant is TestSupport, InvariantTargeting {
    FixedRefundEscrow private escrow;
    RefundHandler private handler;

    function setUp() public {
        escrow = new FixedRefundEscrow();
        handler = new RefundHandler(escrow);
        handler.deposit(2 ether, false);
        handler.deposit(3 ether, true);
    }

    // Foundry reads this target list directly. No external invariant library is required.
    function targetContracts() public view returns (address[] memory targets) {
        targets = new address[](1);
        targets[0] = address(handler);
    }

    function invariant_balanceCoversCredits() public view {
        require(address(escrow).balance >= handler.liabilities(), "fixed pool became insolvent");
    }
}
