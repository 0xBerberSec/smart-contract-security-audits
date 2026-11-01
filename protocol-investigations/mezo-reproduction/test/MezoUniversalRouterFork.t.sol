// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;
interface VmU { function createSelectFork(string calldata rpc) external returns(uint256); function envString(string calldata key) external returns(string memory); }
interface IUR { function execute(bytes calldata commands, bytes[] calldata inputs, uint256 deadline) external payable; }
contract MezoUniversalRouterFork {
 VmU constant vm=VmU(address(uint160(uint256(keccak256("hevm cheat code")))));
 address constant UR=0xd97576d5FeD11c7C31095D4fD0af612Dc736679B;
 function setUp() public { vm.createSelectFork(vm.envString("MEZO_RPC_URL")); }
 function testInvalidCommandRejected() public {
   bytes[] memory inputs=new bytes[](1); inputs[0]=hex"";
   (bool ok,)=UR.call(abi.encodeWithSelector(IUR.execute.selector,hex"ff",inputs,type(uint256).max));
   require(!ok,"invalid command accepted");
 }
 function testExpiredDeadlineRejected() public {
   bytes[] memory inputs=new bytes[](0);
   (bool ok,)=UR.call(abi.encodeWithSelector(IUR.execute.selector,hex"",inputs,0));
   require(!ok,"expired deadline accepted");
 }
}
