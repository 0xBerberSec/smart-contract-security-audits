// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

interface Vm {
    function createSelectFork(string calldata rpcUrl) external returns (uint256);
    function envString(string calldata key) external returns (string memory);
}
interface IERC20Minimal { function balanceOf(address) external view returns (uint256); }
interface IPool {
    function token0() external view returns(address);
    function token1() external view returns(address);
    function factory() external view returns(address);
    function stable() external view returns(bool);
    function reserve0() external view returns(uint256);
    function reserve1() external view returns(uint256);
    function getReserves() external view returns(uint256,uint256,uint256);
    function totalSupply() external view returns(uint256);
    function getAmountOut(uint256,address) external view returns(uint256);
    function quote(address,uint256,uint256) external view returns(uint256);
}
interface IPoolFactory {
    function allPoolsLength() external view returns(uint256);
    function allPools(uint256) external view returns(address);
    function isPool(address) external view returns(bool);
    function implementation() external view returns(address);
    function getPool(address,address,bool) external view returns(address);
}
abstract contract MezoPoolForkBase {
    Vm internal constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
    address internal constant FACTORY = 0x83FE469C636C4081b87bA5b3Ae9991c6Ed104248;
    address[] internal pools;
    function setUp() public virtual {
        string memory rpc = vm.envString("MEZO_RPC_URL");
        // When forge is already running against the persisted local Anvil fork,
        // do not create a nested fork (which would re-query the upstream RPC).
        if (keccak256(bytes(rpc)) != keccak256(bytes("http://127.0.0.1:8546")) &&
            keccak256(bytes(rpc)) != keccak256(bytes("http://localhost:8546"))) {
            vm.createSelectFork(rpc);
        }
        pools.push(0x52e604c44417233b6CcEDDDc0d640A405Caacefb);
        pools.push(0xEd812AEc0Fecc8fD882Ac3eccC43f3aA80A6c356);
        pools.push(0x10906a9E9215939561597b4C8e4b98F93c02031A);
    }
    function _assert(bool ok, string memory why) internal pure { require(ok, why); }
    function _balance(address token, address who) internal view returns(uint256 b, bool ok) {
        (bool success, bytes memory ret) = token.staticcall(abi.encodeWithSelector(IERC20Minimal.balanceOf.selector, who));
        if (success && ret.length >= 32) return (abi.decode(ret,(uint256)), true);
        return (0,false);
    }
}
