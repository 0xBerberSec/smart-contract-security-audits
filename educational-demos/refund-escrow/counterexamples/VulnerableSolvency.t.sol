// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import {VulnerableRefundEscrow} from "../src/RefundEscrow.sol";
import {TestSupport} from "../test/helpers/TestSupport.sol";
import {ReentrantRefundReceiver} from "../test/helpers/RefundReceivers.sol";

/// @notice This safety-property test MUST fail on the deliberately vulnerable version.
contract VulnerableSolvencyCounterexample is TestSupport {
    function test_invariantBalanceCoversCreditsAfterRefund() public {
        VulnerableRefundEscrow escrow = new VulnerableRefundEscrow();
        vm.deal(ALICE, 4 ether);
        vm.prank(ALICE);
        escrow.deposit{value: 4 ether}();
        vm.deal(address(this), 1 ether);
        ReentrantRefundReceiver receiver = new ReentrantRefundReceiver(escrow);
        receiver.run{value: 1 ether}(1);

        require(address(escrow).balance >= escrow.credits(ALICE), "INVARIANT_BROKEN: underbacked credits");
    }
}
