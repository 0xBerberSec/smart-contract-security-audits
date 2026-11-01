// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;
import "./MezoPoolForkBase.sol";

contract MezoPoolBoundedStatefulFuzz is MezoPoolForkBase {
    function testFuzzQuoteBounded(uint256 rawAmount, uint256 poolIndex, bool reverse) public {
        IPool p = IPool(pools[poolIndex % pools.length]);
        uint256 reserveIn = reverse ? p.reserve1() : p.reserve0();
        uint256 reserveOut = reverse ? p.reserve0() : p.reserve1();
        address tokenIn = reverse ? p.token1() : p.token0();
        uint256 amount = reserveIn == type(uint256).max ? rawAmount : rawAmount % (reserveIn + 1);
        (bool ok, bytes memory ret) = address(p).staticcall(abi.encodeWithSelector(IPool.getAmountOut.selector, amount, tokenIn));
        if (ok) {
            _assert(ret.length >= 32, "short quote");
            uint256 out = abi.decode(ret,(uint256));
            _assert(out <= reserveOut, "quote exceeds reserve");
            if (amount == 0) _assert(out == 0, "zero quote nonzero");
        }
    }
}
