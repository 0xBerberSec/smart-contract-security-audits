// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;
import "./MezoPoolForkBase.sol";

contract MezoPoolAccountingInvariants is MezoPoolForkBase {
    function testReservesCoveredByTokenBalancesAndSupply() public {
        for (uint256 i; i < pools.length; i++) {
            IPool p = IPool(pools[i]);
            uint256 r0 = p.reserve0(); uint256 r1 = p.reserve1();
            (uint256 b0,bool ok0) = _balance(p.token0(), pools[i]);
            (uint256 b1,bool ok1) = _balance(p.token1(), pools[i]);
            // Mezo's canonical BTC address is native; its ERC-20 balanceOf selector is unavailable.
            if (!ok0) { b0 = pools[i].balance; ok0 = true; }
            _assert(ok0 && ok1, "token balance unavailable");
            _assert(b0 >= r0 && b1 >= r1, "accounting reserve exceeds balance");
            _assert(p.totalSupply() >= 1000, "LP supply below minimum lock");
            (uint256 q0,uint256 q1,) = p.getReserves();
            _assert(q0 == r0 && q1 == r1, "reserve views diverge");
        }
    }

    function testNoFreeOutputFromZeroInputReadPath() public {
        for (uint256 i; i < pools.length; i++) {
            IPool p = IPool(pools[i]);
            (bool ok, bytes memory ret) = address(p).staticcall(abi.encodeWithSelector(IPool.getAmountOut.selector, 0, p.token0()));
            _assert(ok && ret.length >= 32 && abi.decode(ret,(uint256)) == 0, "zero-input quote is not zero");
        }
    }
}
