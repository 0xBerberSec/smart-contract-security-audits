// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

/// @dev Explicit empty optional selectors for Foundry's invariant configuration ABI.
///      No forge-std dependency; targetContracts() is supplied by the test itself.
abstract contract InvariantTargeting {
    struct SelectorSet {
        address addr;
        bytes4[] selectors;
    }

    struct ArtifactSelectorSet {
        string artifact;
        bytes4[] selectors;
    }

    struct InterfaceSet {
        address addr;
        string[] artifacts;
    }

    function targetArtifactSelectors() external pure returns (ArtifactSelectorSet[] memory) {
        return new ArtifactSelectorSet[](0);
    }

    function targetArtifacts() external pure returns (string[] memory) {
        return new string[](0);
    }

    function excludeArtifacts() external pure returns (string[] memory) {
        return new string[](0);
    }

    function targetSenders() external pure returns (address[] memory) {
        return new address[](0);
    }

    function excludeSenders() external pure returns (address[] memory) {
        return new address[](0);
    }

    function excludeContracts() external pure returns (address[] memory) {
        return new address[](0);
    }

    function targetInterfaces() external pure returns (InterfaceSet[] memory) {
        return new InterfaceSet[](0);
    }

    function targetSelectors() external pure returns (SelectorSet[] memory) {
        return new SelectorSet[](0);
    }

    function excludeSelectors() external pure returns (SelectorSet[] memory) {
        return new SelectorSet[](0);
    }
}
