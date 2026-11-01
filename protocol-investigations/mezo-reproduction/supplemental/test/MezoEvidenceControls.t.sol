// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

interface VME { function envString(string calldata) external returns(string memory); function createSelectFork(string calldata) external returns(uint256); }
interface PE {
 function token0() external view returns(address); function token1() external view returns(address);
 function reserve0() external view returns(uint256); function reserve1() external view returns(uint256);
 function stable() external view returns(bool); function balanceOf(address) external view returns(uint256);
 function getAmountOut(uint256,address) external view returns(uint256);
}
interface RE {
 struct Route { address from; address to; bool stable; address factory; }
 function getAmountsOut(uint256,Route[] calldata) external view returns(uint256[] memory);
 function swapExactTokensForTokens(uint256,uint256,Route[] calldata,address,uint256) external returns(uint256[] memory);
}
interface UE { function execute(bytes calldata,bytes[] calldata,uint256) external payable; }
interface TE { function balanceOf(address) external view returns(uint256); function allowance(address,address) external view returns(uint256); }

contract EvidenceControls {
 VME constant vm=VME(address(uint160(uint256(keccak256("hevm cheat code")))));
 address constant R=0xAc93A67f15342E8Ef611396B13919204c8799Cd4;
 address constant U=0xd97576d5FeD11c7C31095D4fD0af612Dc736679B;
 address constant F=0x83FE469C636C4081b87bA5b3Ae9991c6Ed104248;
 address constant BTC=0x7b7C000000000000000000000000000000000000;
 address[3] pools=[0x52e604c44417233b6CcEDDDc0d640A405Caacefb,0xEd812AEc0Fecc8fD882Ac3eccC43f3aA80A6c356,0x10906a9E9215939561597b4C8e4b98F93c02031A];
 event Coverage(address indexed pool,bool reverse,uint256 success,uint256 rejected,uint256 invalid,uint256 positive);
 event Diagnostic(string label,bool ok,bytes result,uint256 gasUsed);
 function setUp() public { vm.createSelectFork(vm.envString("MEZO_RPC_URL")); require(block.chainid==31612 && block.number==12674080,"wrong fixture"); }
 function testQuoteGridCoverage() public {
   for(uint256 i;i<3;i++) for(uint256 j;j<2;j++) {
     PE p=PE(pools[i]); bool rev=j==1;
     uint256 rin=rev?p.reserve1():p.reserve0(); uint256 rout=rev?p.reserve0():p.reserve1();
     address tin=rev?p.token1():p.token0();
     // Include dust, zero, material amounts, full input reserve and one unit beyond it.
     uint256[10] memory amounts=[uint256(0),1,2,rin/1e6,rin/1e4,rin/100,rin/10,rin/2,rin,rin+1];
     uint256 success; uint256 rejected; uint256 invalid; uint256 positive;
     for(uint256 k;k<10;k++) {
       (bool ok,bytes memory ret)=address(p).staticcall(abi.encodeCall(PE.getAmountOut,(amounts[k],tin)));
       if(!ok) { rejected++; continue; }
       if(ret.length!=32) { invalid++; continue; }
       success++; uint256 q=abi.decode(ret,(uint256));
       require(q<=rout,"reserve bound"); if(amounts[k]==0) require(q==0,"zero amount");
       if(amounts[k]>0 && q>0) positive++;
     }
     emit Coverage(address(p),rev,success,rejected,invalid,positive);
     // This fixture's six paths were preflighted: all ten inputs have valid returns.
     require(success==10 && rejected==0 && invalid==0,"grid coverage regression");
     require(positive>0,"no positive quote tested");
   }
 }
 function testFuzzMaterialQuotesAllPairs(uint256 seed) public view {
   for(uint256 i;i<3;i++) for(uint256 j;j<2;j++) {
     PE p=PE(pools[i]); uint256 rin=j==0?p.reserve0():p.reserve1();
     uint256 rout=j==0?p.reserve1():p.reserve0(); address tin=j==0?p.token0():p.token1();
     // Explicit material-input domain [ceil(reserveIn/1e6), reserveIn]. Dust is in grid test.
     uint256 lower=(rin+999999)/1e6; require(lower>0 && lower<=rin,"empty domain");
     uint256 amount=lower+seed%(rin-lower+1);
     uint256 out=p.getAmountOut(amount,tin); // Reverts fail this test, never silently skipped.
     require(out>0 && out<=rout,"material quote bounds");
   }
 }
 function testVolatileQuoteMonotonicity() public view {
   for(uint256 i;i<3;i++) { PE p=PE(pools[i]); if(p.stable()) continue;
     for(uint256 j;j<2;j++) { uint256 rin=j==0?p.reserve0():p.reserve1(); address tin=j==0?p.token0():p.token1();
       uint256 last; for(uint256 k=1;k<=20;k++) { uint256 q=p.getAmountOut(rin*k/20,tin); require(q>=last,"nonmonotone quote"); last=q; }
     }
   }
 }
 function testNativeIdentityAndMinimumLpHolder() public {
   PE p=PE(pools[0]); require(p.token0()==BTC,"native identity not inferred from revert");
   require(address(p).balance==p.reserve0(),"native reserve");
   for(uint256 i;i<3;i++) {
     PE q=PE(pools[i]); uint256 before=q.balanceOf(address(1)); require(before>=1000,"minimum holder");
     (bool ok,)=address(q).call(abi.encodeWithSignature("transferFrom(address,address,uint256)",address(1),address(this),1));
     require(!ok && q.balanceOf(address(1))==before,"unauthorized LP transfer");
   }
 }
 function expectError(address target,bytes memory data,bytes memory expected) internal {
   (bool ok,bytes memory ret)=target.call{gas:500000}(data);
   require(!ok,"expected rejection"); require(keccak256(ret)==keccak256(expected),"wrong rejection cause");
 }
 function testRouterExactDeadlineAndPath() public {
   RE.Route[] memory routes=new RE.Route[](0);
   expectError(R,abi.encodeCall(RE.swapExactTokensForTokens,(0,0,routes,address(this),0)),abi.encodeWithSignature("Expired()"));
   expectError(R,abi.encodeCall(RE.swapExactTokensForTokens,(0,0,routes,address(this),type(uint256).max)),abi.encodeWithSignature("InvalidPath()"));
 }
 function testRouterExactSlippageWithQuoteControl() public {
   PE p=PE(pools[1]); RE.Route[] memory routes=new RE.Route[](1);
   routes[0]=RE.Route(p.token0(),p.token1(),p.stable(),F);
   uint256 amount=p.reserve0()/100; uint256[] memory quote=RE(R).getAmountsOut(amount,routes);
   require(quote.length==2 && quote[1]>0,"quote control failed");
   bytes32 stateBefore=paymentState(p);
   bytes memory data=abi.encodeCall(RE.swapExactTokensForTokens,(amount,quote[1]+1,routes,address(this),type(uint256).max));
   expectError(R,data,abi.encodeWithSignature("InsufficientOutputAmount()"));
   require(paymentState(p)==stateBefore,"payment state changed on rejected swap");
 }
 function paymentState(PE p) internal view returns(bytes32) {
   return keccak256(abi.encode(TE(p.token0()).balanceOf(address(this)),TE(p.token1()).balanceOf(address(this)),TE(p.token0()).balanceOf(address(p)),TE(p.token1()).balanceOf(address(p)),TE(p.token0()).balanceOf(R),TE(p.token1()).balanceOf(R),TE(p.token0()).allowance(address(this),R)));
 }
 function testUniversalRouterPositiveEmptyExecution() public {
   bytes[] memory inputs=new bytes[](0);
   (bool ok,bytes memory ret)=U.call{gas:500000}(abi.encodeCall(UE.execute,(hex"",inputs,type(uint256).max)));
   require(ok && ret.length==0,"empty execution rejected");
 }
 function testUniversalRouterExactErrors() public {
   bytes[] memory inputs=new bytes[](0);
   expectError(U,abi.encodeCall(UE.execute,(hex"",inputs,0)),abi.encodeWithSignature("TransactionDeadlinePassed()"));
   expectError(U,abi.encodeCall(UE.execute,(hex"00",inputs,type(uint256).max)),abi.encodeWithSignature("LengthMismatch()"));
   inputs=new bytes[](1);
   expectError(U,abi.encodeCall(UE.execute,(hex"3f",inputs,type(uint256).max)),abi.encodeWithSignature("InvalidCommandType(uint256)",63));
 }
 function testUniversalRouterDiagnostic() public {
   bytes[] memory inputs=new bytes[](0); uint256 start=gasleft();
   (bool ok,bytes memory ret)=U.call{gas:500000}(abi.encodeCall(UE.execute,(hex"",inputs,type(uint256).max)));
   emit Diagnostic("empty execution",ok,ret,start-gasleft());
 }
}
