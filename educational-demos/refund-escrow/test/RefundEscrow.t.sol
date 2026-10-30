// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import {
    IRefundEscrow,
    RefundEscrowLedger,
    VulnerableRefundEscrow,
    FixedRefundEscrow
} from "../src/RefundEscrow.sol";
import {TestSupport} from "./helpers/TestSupport.sol";
import {ReentrantRefundReceiver, RejectingRefundReceiver} from "./helpers/RefundReceivers.sol";

contract RefundEscrowTest is TestSupport {
    VulnerableRefundEscrow private vulnerable;
    FixedRefundEscrow private fixedEscrow;

    function setUp() public {
        vulnerable = new VulnerableRefundEscrow();
        fixedEscrow = new FixedRefundEscrow();
        vm.deal(address(this), 1_000 ether);
    }

    function fundVictim(IRefundEscrow pool, uint256 amount) private {
        vm.deal(ALICE, amount);
        vm.prank(ALICE);
        pool.deposit{value: amount}();
    }

    function test_vulnerableCallbackConsumesSameCreditTwice() public {
        fundVictim(vulnerable, 4 ether);
        ReentrantRefundReceiver receiver = new ReentrantRefundReceiver(vulnerable);

        receiver.run{value: 1 ether}(1);

        eq(address(receiver).balance, 2 ether, "receiver got two refunds for one deposit");
        eq(vulnerable.credits(address(receiver)), 0, "attacker credit is consumed");
        eq(vulnerable.credits(ALICE), 4 ether, "victim still owns four credits");
        eq(address(vulnerable).balance, 3 ether, "one victim ETH funded the extra refund");
        require(receiver.lastReentrySucceeded(), "nested refund must execute");
        require(address(vulnerable).balance < vulnerable.credits(ALICE), "insolvency witnessed");
    }

    function test_counter_noCallbackKeepsVulnerablePoolSolvent() public {
        fundVictim(vulnerable, 4 ether);
        ReentrantRefundReceiver receiver = new ReentrantRefundReceiver(vulnerable);
        receiver.run{value: 1 ether}(0);
        eq(address(receiver).balance, 1 ether, "no extra payment");
        eq(address(vulnerable).balance, vulnerable.credits(ALICE), "liability remains covered");
    }

    function test_counter_noOtherDepositorMeansNoExtractableSurplus() public {
        ReentrantRefundReceiver receiver = new ReentrantRefundReceiver(vulnerable);
        receiver.run{value: 1 ether}(1);
        eq(address(receiver).balance, 1 ether, "own deposit returned, no net gain");
        eq(receiver.attempts(), 0, "receiver bounds the callback by available ETH");
        eq(address(vulnerable).balance, 0, "empty pool, zero liability");
    }

    function test_counter_normalEOARefundKeepsVulnerablePoolSolvent() public {
        fundVictim(vulnerable, 4 ether);
        vm.prank(ALICE);
        vulnerable.refund();
        eq(ALICE.balance, 4 ether, "honest refund works");
        eq(vulnerable.credits(ALICE), 0, "credit consumed");
        eq(address(vulnerable).balance, 0, "no residual liability");
    }

    function test_counter_uncreditedCallerCannotRefund() public {
        fundVictim(vulnerable, 4 ether);
        vm.prank(BOB);
        vm.expectRevert(RefundEscrowLedger.NothingToRefund.selector);
        vulnerable.refund();
        eq(address(vulnerable).balance, 4 ether, "stranger did not withdraw");
    }

    function test_fixedRejectsNestedRefundButPaysOriginalCredit() public {
        fundVictim(fixedEscrow, 4 ether);
        ReentrantRefundReceiver receiver = new ReentrantRefundReceiver(fixedEscrow);
        receiver.run{value: 1 ether}(1);
        eq(receiver.attempts(), 1, "callback was actually attempted");
        require(!receiver.lastReentrySucceeded(), "nested call rejected");
        eq(address(receiver).balance, 1 ether, "no net gain before gas");
        eq(address(fixedEscrow).balance, 4 ether, "victim funds retained");
        eq(fixedEscrow.credits(ALICE), 4 ether, "victim claim is fully covered");
        eq(fixedEscrow.credits(address(receiver)), 0, "attacker credit consumed once");
    }

    function test_fixedRejectingReceiverRestoresCreditOnRevert() public {
        RejectingRefundReceiver receiver = new RejectingRefundReceiver(fixedEscrow);
        receiver.fund{value: 2 ether}();
        vm.expectRevert(RefundEscrowLedger.TransferFailed.selector);
        receiver.requestRefund();
        eq(fixedEscrow.credits(address(receiver)), 2 ether, "revert restores credit");
        eq(address(fixedEscrow).balance, 2 ether, "revert preserves backing");
    }

    function test_fixedCannotConsumeCreditTwice() public {
        fundVictim(fixedEscrow, 4 ether);
        vm.prank(ALICE);
        fixedEscrow.refund();
        vm.prank(ALICE);
        vm.expectRevert(RefundEscrowLedger.NothingToRefund.selector);
        fixedEscrow.refund();
        eq(ALICE.balance, 4 ether, "no second payout");
    }

    function test_fixedRejectsEmptyDeposit() public {
        vm.expectRevert(RefundEscrowLedger.EmptyDeposit.selector);
        fixedEscrow.deposit();
    }

    function test_fixedVictimCanStillExitAfterAttemptedAttack() public {
        fundVictim(fixedEscrow, 4 ether);
        ReentrantRefundReceiver receiver = new ReentrantRefundReceiver(fixedEscrow);
        receiver.run{value: 1 ether}(1);
        vm.prank(ALICE);
        fixedEscrow.refund();
        eq(ALICE.balance, 4 ether, "victim can redeem the original liability");
        eq(address(fixedEscrow).balance, 0, "pool settles all tracked claims");
    }

    function testFuzz_fixedPreservesBacking(uint96 victimSeed, uint96 attackerSeed) public {
        uint256 victimDeposit = uint256(victimSeed) % 100 ether + 1;
        uint256 attackerDeposit = uint256(attackerSeed) % 100 ether + 1;
        fundVictim(fixedEscrow, victimDeposit);
        ReentrantRefundReceiver receiver = new ReentrantRefundReceiver(fixedEscrow);
        receiver.run{value: attackerDeposit}(1);
        eq(address(receiver).balance, attackerDeposit, "no attacker net gain");
        eq(address(fixedEscrow).balance, fixedEscrow.credits(ALICE), "backing equals claims");
    }

    function testFuzz_vulnerableCanBreakBacking(uint96 seed) public {
        uint256 stake = uint256(seed) % 100 ether + 1;
        fundVictim(vulnerable, stake);
        ReentrantRefundReceiver receiver = new ReentrantRefundReceiver(vulnerable);
        receiver.run{value: stake}(1);
        eq(address(receiver).balance, 2 * stake, "profit funded by the other depositor");
        require(address(vulnerable).balance < vulnerable.credits(ALICE), "backing violated");
    }
}
