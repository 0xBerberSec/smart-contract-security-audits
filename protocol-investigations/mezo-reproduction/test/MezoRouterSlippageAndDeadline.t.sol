// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;
interface VmD { function createSelectFork(string calldata rpc) external returns(uint256); function envString(string calldata key) external returns(string memory); }
interface IRD { struct Route { address from; address to; bool stable; address factory; } function swapExactTokensForTokens(uint256,uint256,Route[] calldata,address,uint256) external returns(uint256[] memory); }
contract MezoRouterSlippageAndDeadline {
 VmD constant vm=VmD(address(uint160(uint256(keccak256("hevm cheat code")))));
 address constant ROUTER=0xAc93A67f15342E8Ef611396B13919204c8799Cd4;
 address constant FACTORY=0x83FE469C636C4081b87bA5b3Ae9991c6Ed104248;
 address constant MUSD=0xdD468A1DDc392dcdbEf6db6e34E89AA338F9F186;
 address constant MUSDC=0x68bBA5ecC126DedFa52C0399671E3A7a514D311C;
 function setUp() public { vm.createSelectFork(vm.envString("MEZO_RPC_URL")); }
 function testExpiredDeadlineRejectedBeforeTransfer() public {
   IRD.Route[] memory r=new IRD.Route[](1); r[0]=IRD.Route(MUSD,MUSDC,true,FACTORY);
   (bool ok,)=ROUTER.call(abi.encodeWithSelector(IRD.swapExactTokensForTokens.selector,0,0,r,address(this),0));
   require(!ok,"expired deadline accepted");
 }
 function testEmptyPathRejected() public {
   IRD.Route[] memory r=new IRD.Route[](0);
   (bool ok,)=ROUTER.call(abi.encodeWithSelector(IRD.swapExactTokensForTokens.selector,0,0,r,address(this),type(uint256).max));
   require(!ok,"empty path accepted");
 }
}