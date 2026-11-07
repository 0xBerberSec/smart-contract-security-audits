#![allow(clippy::arithmetic_side_effects)]

use std::{
    collections::HashMap,
    fmt,
    ops::{Deref, DerefMut},
    time::{Duration, Instant},
};

use crate::{
    action::decrease_position::DecreasePositionSwapType,
    clock::ClockKind,
    fixed::FixedPointOps,
    market::{
        BaseMarket, BorrowingFeeMarketMut, LiquidityMarket, LiquidityMarketMut, PerpMarket,
        PnlFactorKind, PositionImpactMarket, SwapMarket,
    },
    num::{MulDiv, Num, Unsigned, UnsignedAbs},
    params::{
        fee::{
            BorrowingFeeKinkModelParams, BorrowingFeeKinkModelParamsForOneSide, BorrowingFeeParams,
            FundingFeeParams, LiquidationFeeParams,
        },
        position::PositionImpactDistributionParams,
        FeeParams, PositionParams, PriceImpactParams,
    },
    pool::{Balance, Delta, Pool},
    position::Position,
    BaseMarketMut, BorrowingFeeMarket, PerpMarketMut, PositionImpactMarketMut, PositionMut,
    PositionState, PositionStateMut, SwapMarketMut,
};
use num_traits::{CheckedSub, Signed};

/// Test Pool.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TestPool<T> {
    long_amount: T,
    short_amount: T,
}

impl<T> Balance for TestPool<T>
where
    T: MulDiv + Num + CheckedSub,
{
    type Num = T;

    type Signed = T::Signed;

    fn long_amount(&self) -> crate::Result<Self::Num> {
        Ok(self.long_amount.clone())
    }

    fn short_amount(&self) -> crate::Result<Self::Num> {
        Ok(self.short_amount.clone())
    }
}

impl<T> Pool for TestPool<T>
where
    T: MulDiv + Num + CheckedSub,
{
    fn checked_apply_delta(&self, delta: Delta<&Self::Signed>) -> crate::Result<Self> {
        let mut ans = self.clone();
        if let Some(amount) = delta.long() {
            ans.apply_delta_to_long_amount(amount)?;
        }
        if let Some(amount) = delta.short() {
            ans.apply_delta_to_short_amount(amount)?;
        }
        Ok(ans)
    }

    fn apply_delta_to_long_amount(&mut self, delta: &Self::Signed) -> Result<(), crate::Error> {
        if delta.is_positive() {
            self.long_amount = self
                .long_amount
                .checked_add(&delta.unsigned_abs())
                .ok_or(crate::Error::Overflow)?;
        } else {
            self.long_amount = self
                .long_amount
                .checked_sub(&delta.unsigned_abs())
                .ok_or(crate::Error::Computation("decreasing long amount"))?;
        }
        Ok(())
    }

    fn apply_delta_to_short_amount(&mut self, delta: &Self::Signed) -> Result<(), crate::Error> {
        if delta.is_positive() {
            self.short_amount = self
                .short_amount
                .checked_add(&delta.unsigned_abs())
                .ok_or(crate::Error::Overflow)?;
        } else {
            self.short_amount = self
                .short_amount
                .checked_sub(&delta.unsigned_abs())
                .ok_or(crate::Error::Computation("decreasing short amount"))?;
        }
        Ok(())
    }
}

/// Max PnL Factors.
#[derive(Debug, Clone)]
pub struct MaxPnlFactors<T> {
    /// For deposit.
    pub deposit: T,
    /// For withdrawal.
    pub withdrawal: T,
    /// For trader.
    pub trader: T,
    /// For ADL.
    pub adl: T,
}

#[derive(Debug, Clone, Copy, Default)]
struct Clock {
    advanced: Duration,
}

impl Clock {
    fn now(&self) -> Instant {
        Instant::now() + self.advanced
    }

    fn move_forward(&mut self, duration: Duration) {
        self.advanced += duration;
    }
}

/// Test Market.
#[derive(Debug, Clone)]
pub struct TestMarket<T: Unsigned, const DECIMALS: u8> {
    config: TestMarketConfig<T, DECIMALS>,
    total_supply: T,
    value_to_amount_divisor: T,
    funding_amount_per_size_adjustment: T,
    primary: TestPool<T>,
    swap_impact: TestPool<T>,
    virtual_inventory_for_swaps: Option<TestPool<T>>,
    fee: TestPool<T>,
    open_interest: (TestPool<T>, TestPool<T>),
    open_interest_in_tokens: (TestPool<T>, TestPool<T>),
    position_impact: TestPool<T>,
    borrowing_factor: TestPool<T>,
    funding_factor_per_second: T::Signed,
    funding_amount_per_size: (TestPool<T>, TestPool<T>),
    claimable_funding_amount_per_size: (TestPool<T>, TestPool<T>),
    collateral_sum: (TestPool<T>, TestPool<T>),
    total_borrowing: TestPool<T>,
    clock: Clock,
    clocks: HashMap<ClockKind, Instant>,
}

impl<T: Unsigned, const DECIMALS: u8> TestMarket<T, DECIMALS> {
    /// Create a new test market.
    fn new(
        value_to_amount_divisor: T,
        funding_amount_per_size_adjustment: T,
        config: TestMarketConfig<T, DECIMALS>,
    ) -> Self
    where
        T: Default,
        T::Signed: Default,
    {
        Self {
            config,
            total_supply: Default::default(),
            value_to_amount_divisor,
            funding_amount_per_size_adjustment,
            primary: Default::default(),
            swap_impact: Default::default(),
            virtual_inventory_for_swaps: None,
            fee: Default::default(),
            open_interest: Default::default(),
            open_interest_in_tokens: Default::default(),
            position_impact: Default::default(),
            borrowing_factor: Default::default(),
            funding_factor_per_second: Default::default(),
            funding_amount_per_size: Default::default(),
            claimable_funding_amount_per_size: Default::default(),
            collateral_sum: Default::default(),
            total_borrowing: Default::default(),
            clocks: Default::default(),
            clock: Default::default(),
        }
    }

    /// Move the clock forward.
    pub fn move_clock_forward(&mut self, duration: Duration) {
        self.clock.move_forward(duration);
    }
}

impl TestMarket<u64, 9> {
    /// Create a new [`TestMarket`] with config.
    pub fn with_config(config: TestMarketConfig<u64, 9>) -> Self {
        Self::new(1, 10_000, config)
    }
}

#[cfg(feature = "u128")]
impl TestMarket<u128, 20> {
    /// Create a new [`TestMarket`] with config.
    pub fn with_config(config: TestMarketConfig<u128, 20>) -> Self {
        Self::new(10u128.pow(20 - 9), 10u128.pow(10), config)
    }
}

impl Default for TestMarket<u64, 9> {
    fn default() -> Self {
        Self::with_config(Default::default())
    }
}

#[cfg(feature = "u128")]
impl Default for TestMarket<u128, 20> {
    fn default() -> Self {
        Self::with_config(Default::default())
    }
}

const SECONDS_PER_YEAR: u64 = 365 * 24 * 3600;

/// Test Market Config.
#[derive(Debug, Clone)]
pub struct TestMarketConfig<T, const DECIMALS: u8> {
    /// Swap impact params.
    pub swap_impact_params: PriceImpactParams<T>,
    /// Swap fee params.
    pub swap_fee_params: FeeParams<T>,
    /// Position params.
    pub position_params: PositionParams<T>,
    /// Position impact params.
    pub position_impact_params: PriceImpactParams<T>,
    /// Order fee params.
    pub order_fee_params: FeeParams<T>,
    /// Position impact distribution params.
    pub position_impact_distribution_params: PositionImpactDistributionParams<T>,
    /// Borrowing fee params.
    pub borrowing_fee_params: BorrowingFeeParams<T>,
    /// Borrowing fee kink model params.
    pub borrowing_fee_kink_model_params: BorrowingFeeKinkModelParamsForOneSide<T>,
    /// Funding fee params.
    pub funding_fee_params: FundingFeeParams<T>,
    /// Reserve factor.
    pub reserve_factor: T,
    /// Open interest reserve factor.
    pub open_interest_reserve_factor: T,
    /// Max PnL factors.
    pub max_pnl_factors: MaxPnlFactors<T>,
    /// Min PnL factors after ADL.
    pub min_pnl_factor_after_adl: T,
    /// Max pool amount.
    pub max_pool_amount: T,
    /// Max pool value for deposit.
    pub max_pool_value_for_deposit: T,
    /// Max open interest.
    pub max_open_interest: T,
    /// Min collateral factor for OI.
    pub min_collateral_factor_for_oi: T,
    /// Ignore open interest for usage factor.
    pub ignore_open_interest_for_usage_factor: bool,
    /// Liquidation fee params.
    pub liquidation_fee_params: LiquidationFeeParams<T>,
}

impl Default for TestMarketConfig<u64, 9> {
    fn default() -> Self {
        Self {
            swap_impact_params: PriceImpactParams::builder()
                .exponent(2_000_000_000)
                .positive_factor(4)
                .negative_factor(8)
                .build(),
            swap_fee_params: FeeParams::builder()
                .fee_receiver_factor(370_000_000)
                .positive_impact_fee_factor(500_000)
                .negative_impact_fee_factor(700_000)
                .build(),
            position_params: PositionParams::new(
                1_000_000_000,
                1_000_000_000,
                10_000_000,
                5_000_000,
                5_000_000,
                2_500_000,
            ),
            position_impact_params: PriceImpactParams::builder()
                .exponent(2_000_000_000)
                .positive_factor(1)
                .negative_factor(2)
                .build(),
            order_fee_params: FeeParams::builder()
                .fee_receiver_factor(370_000_000)
                .positive_impact_fee_factor(500_000)
                .negative_impact_fee_factor(700_000)
                .build(),
            position_impact_distribution_params: PositionImpactDistributionParams::builder()
                .distribute_factor(1_000_000_000)
                .min_position_impact_pool_amount(1_000_000_000)
                .build(),
            borrowing_fee_params: BorrowingFeeParams::builder()
                .receiver_factor(370_000_000)
                .factor_for_long(28)
                .factor_for_short(28)
                .exponent_for_long(1_000_000_000)
                .exponent_for_short(1_000_000_000)
                .build(),
            borrowing_fee_kink_model_params: BorrowingFeeKinkModelParamsForOneSide::builder()
                .optimal_usage_factor(750_000_000)
                .base_borrowing_factor(600_000_000 / SECONDS_PER_YEAR)
                .above_optimal_usage_borrowing_factor(1_500_000_000 / SECONDS_PER_YEAR)
                .build(),
            funding_fee_params: FundingFeeParams::builder()
                .exponent(1_000_000_000)
                .funding_factor(20)
                .max_factor_per_second(10)
                .min_factor_per_second(1)
                .increase_factor_per_second(10)
                .decrease_factor_per_second(0)
                .threshold_for_stable_funding(50_000_000)
                .threshold_for_decrease_funding(0)
                .build(),
            reserve_factor: 1_000_000_000,
            max_pnl_factors: MaxPnlFactors {
                deposit: 600_000_000,
                withdrawal: 300_000_000,
                trader: 500_000_000,
                adl: 500_000_000,
            },
            min_pnl_factor_after_adl: 0,
            open_interest_reserve_factor: 1_000_000_000,
            max_pool_amount: 1_000_000_000 * 1_000_000_000,
            max_pool_value_for_deposit: u64::MAX,
            max_open_interest: u64::MAX,
            // min collateral factor of 0.005 when open interest is $83,000,000
            min_collateral_factor_for_oi: 5 * 10u64.pow(6) / 83_000_000,
            ignore_open_interest_for_usage_factor: false,
            liquidation_fee_params: LiquidationFeeParams::builder()
                .factor(2_000_000)
                .receiver_factor(370_000_000)
                .build(),
        }
    }
}

#[cfg(feature = "u128")]
impl Default for TestMarketConfig<u128, 20> {
    fn default() -> Self {
        Self {
            swap_impact_params: PriceImpactParams::builder()
                .exponent(200_000_000_000_000_000_000)
                .positive_factor(400_000_000_000)
                .negative_factor(800_000_000_000)
                .build(),
            swap_fee_params: FeeParams::builder()
                .fee_receiver_factor(37_000_000_000_000_000_000)
                .positive_impact_fee_factor(50_000_000_000_000_000)
                .negative_impact_fee_factor(70_000_000_000_000_000)
                .build(),
            position_params: PositionParams::new(
                100_000_000_000_000_000_000,
                100_000_000_000_000_000_000,
                1_000_000_000_000_000_000,
                500_000_000_000_000_000,
                500_000_000_000_000_000,
                250_000_000_000_000_000,
            ),
            position_impact_params: PriceImpactParams::builder()
                .exponent(200_000_000_000_000_000_000)
                .positive_factor(100_000_000_000)
                .negative_factor(200_000_000_000)
                .build(),
            order_fee_params: FeeParams::builder()
                .fee_receiver_factor(37_000_000_000_000_000_000)
                .positive_impact_fee_factor(50_000_000_000_000_000)
                .negative_impact_fee_factor(70_000_000_000_000_000)
                .build(),
            position_impact_distribution_params: PositionImpactDistributionParams::builder()
                .distribute_factor(100_000_000_000_000_000_000)
                .min_position_impact_pool_amount(1_000_000_000)
                .build(),
            borrowing_fee_params: BorrowingFeeParams::builder()
                .receiver_factor(37_000_000_000_000_000_000)
                .factor_for_long(2_820_000_000_000)
                .factor_for_short(2_820_000_000_000)
                .exponent_for_long(100_000_000_000_000_000_000)
                .exponent_for_short(100_000_000_000_000_000_000)
                .build(),
            borrowing_fee_kink_model_params: BorrowingFeeKinkModelParamsForOneSide::builder()
                .optimal_usage_factor(75_000_000_000_000_000_000)
                .base_borrowing_factor(60_000_000_000_000_000_000 / u128::from(SECONDS_PER_YEAR))
                .above_optimal_usage_borrowing_factor(
                    150_000_000_000_000_000_000 / u128::from(SECONDS_PER_YEAR),
                )
                .build(),
            funding_fee_params: FundingFeeParams::builder()
                .exponent(100_000_000_000_000_000_000)
                .funding_factor(2_000_000_000_000)
                .max_factor_per_second(1_000_000_000_000)
                .min_factor_per_second(30_000_000_000)
                .increase_factor_per_second(790_000_000)
                .decrease_factor_per_second(0)
                .threshold_for_stable_funding(5_000_000_000_000_000_000)
                .threshold_for_decrease_funding(0)
                .build(),
            reserve_factor: 10u128.pow(20),
            open_interest_reserve_factor: 10u128.pow(20),
            max_pnl_factors: MaxPnlFactors {
                deposit: 60_000_000_000_000_000_000,
                withdrawal: 30_000_000_000_000_000_000,
                trader: 50_000_000_000_000_000_000,
                adl: 50_000_000_000_000_000_000,
            },
            min_pnl_factor_after_adl: 0,
            max_pool_amount: 1_000_000_000 * 10u128.pow(20),
            max_pool_value_for_deposit: 1_000_000_000_000_000 * 10u128.pow(20),
            max_open_interest: 1_000_000_000 * 10u128.pow(20),
            // min collateral factor of 0.005 when open interest is $83,000,000
            min_collateral_factor_for_oi: 5 * 10u128.pow(17) / 83_000_000,
            ignore_open_interest_for_usage_factor: false,
            liquidation_fee_params: LiquidationFeeParams::builder()
                .factor(200_000_000_000_000_000)
                .receiver_factor(37_000_000_000_000_000_000)
                .build(),
        }
    }
}

impl<T, const DECIMALS: u8> TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn now(&self) -> Instant {
        self.clock.now()
    }

    fn just_passed_in_seconds(&mut self, clock: ClockKind) -> crate::Result<u64> {
        let now = self.now();
        let clock = self.clocks.entry(clock).or_insert(now);
        let duration = now.saturating_duration_since(*clock);
        *clock = now;
        Ok(duration.as_secs())
    }

    fn passed_in_seconds(&self, clock: ClockKind) -> crate::Result<u64> {
        let now = self.now();
        let clock = self.clocks.get(&clock).unwrap_or(&now);
        let duration = now.saturating_duration_since(*clock);
        Ok(duration.as_secs())
    }
}

impl<T, const DECIMALS: u8> BaseMarket<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    type Num = T;

    type Signed = T::Signed;

    type Pool = TestPool<T>;

    fn liquidity_pool(&self) -> crate::Result<&Self::Pool> {
        Ok(&self.primary)
    }

    fn claimable_fee_pool(&self) -> crate::Result<&Self::Pool> {
        Ok(&self.fee)
    }

    fn swap_impact_pool(&self) -> crate::Result<&Self::Pool> {
        Ok(&self.swap_impact)
    }

    fn open_interest_pool(&self, is_long: bool) -> crate::Result<&Self::Pool> {
        if is_long {
            Ok(&self.open_interest.0)
        } else {
            Ok(&self.open_interest.1)
        }
    }

    fn open_interest_in_tokens_pool(&self, is_long: bool) -> crate::Result<&Self::Pool> {
        if is_long {
            Ok(&self.open_interest_in_tokens.0)
        } else {
            Ok(&self.open_interest_in_tokens.1)
        }
    }

    fn collateral_sum_pool(&self, is_long: bool) -> crate::Result<&Self::Pool> {
        if is_long {
            Ok(&self.collateral_sum.0)
        } else {
            Ok(&self.collateral_sum.1)
        }
    }

    fn virtual_inventory_for_swaps_pool(
        &self,
    ) -> crate::Result<Option<impl Deref<Target = Self::Pool>>> {
        Ok(self.virtual_inventory_for_swaps.as_ref())
    }

    fn virtual_inventory_for_positions_pool(
        &self,
    ) -> crate::Result<Option<impl Deref<Target = Self::Pool>>> {
        Ok(None::<&Self::Pool>)
    }

    fn usd_to_amount_divisor(&self) -> Self::Num {
        self.value_to_amount_divisor.clone()
    }

    fn max_pool_amount(&self, _is_long_token: bool) -> crate::Result<Self::Num> {
        Ok(self.config.max_pool_amount.clone())
    }

    fn pnl_factor_config(&self, kind: PnlFactorKind, _is_long: bool) -> crate::Result<Self::Num> {
        let factor = match kind {
            PnlFactorKind::MaxAfterDeposit => self.config.max_pnl_factors.deposit.clone(),
            PnlFactorKind::MaxAfterWithdrawal => self.config.max_pnl_factors.withdrawal.clone(),
            PnlFactorKind::MaxForTrader => self.config.max_pnl_factors.trader.clone(),
            PnlFactorKind::ForAdl => self.config.max_pnl_factors.adl.clone(),
            PnlFactorKind::MinAfterAdl => self.config.min_pnl_factor_after_adl.clone(),
        };
        Ok(factor)
    }

    fn reserve_factor(&self) -> crate::Result<Self::Num> {
        Ok(self.config.reserve_factor.clone())
    }

    fn open_interest_reserve_factor(&self) -> crate::Result<Self::Num> {
        Ok(self.config.open_interest_reserve_factor.clone())
    }

    fn max_open_interest(&self, _is_long: bool) -> crate::Result<Self::Num> {
        Ok(self.config.max_open_interest.clone())
    }

    fn ignore_open_interest_for_usage_factor(&self) -> crate::Result<bool> {
        Ok(self.config.ignore_open_interest_for_usage_factor)
    }
}

impl<T, const DECIMALS: u8> BaseMarketMut<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn liquidity_pool_mut(&mut self) -> crate::Result<&mut Self::Pool> {
        Ok(&mut self.primary)
    }

    fn claimable_fee_pool_mut(&mut self) -> crate::Result<&mut Self::Pool> {
        Ok(&mut self.fee)
    }

    fn virtual_inventory_for_swaps_pool_mut(
        &mut self,
    ) -> crate::Result<Option<impl DerefMut<Target = Self::Pool>>> {
        Ok(self.virtual_inventory_for_swaps.as_mut())
    }
}

impl<T, const DECIMALS: u8> SwapMarket<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn swap_impact_params(&self) -> crate::Result<PriceImpactParams<Self::Num>> {
        Ok(self.config.swap_impact_params.clone())
    }

    fn swap_fee_params(&self) -> crate::Result<FeeParams<Self::Num>> {
        Ok(self.config.swap_fee_params.clone())
    }
}

impl<T, const DECIMALS: u8> SwapMarketMut<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn swap_impact_pool_mut(&mut self) -> crate::Result<&mut Self::Pool> {
        Ok(&mut self.swap_impact)
    }
}

impl<T, const DECIMALS: u8> LiquidityMarket<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn total_supply(&self) -> Self::Num {
        self.total_supply.clone()
    }

    fn max_pool_value_for_deposit(&self, _is_long_token: bool) -> crate::Result<Self::Num> {
        Ok(self.config.max_pool_value_for_deposit.clone())
    }
}

impl<T, const DECIMALS: u8> LiquidityMarketMut<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn mint(&mut self, amount: &Self::Num) -> Result<(), crate::Error> {
        self.total_supply = self
            .total_supply
            .checked_add(amount)
            .ok_or(crate::Error::Overflow)?;
        Ok(())
    }

    fn burn(&mut self, amount: &Self::Num) -> crate::Result<()> {
        self.total_supply = self
            .total_supply
            .checked_sub(amount)
            .ok_or(crate::Error::Computation("burning market tokens"))?;
        Ok(())
    }
}

impl<T, const DECIMALS: u8> PositionImpactMarket<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn position_impact_pool(&self) -> crate::Result<&Self::Pool> {
        Ok(&self.position_impact)
    }

    fn position_impact_params(&self) -> crate::Result<PriceImpactParams<Self::Num>> {
        Ok(self.config.position_impact_params.clone())
    }

    fn position_impact_distribution_params(
        &self,
    ) -> crate::Result<PositionImpactDistributionParams<Self::Num>> {
        Ok(self.config.position_impact_distribution_params.clone())
    }

    fn passed_in_seconds_for_position_impact_distribution(&self) -> crate::Result<u64> {
        self.passed_in_seconds(ClockKind::PriceImpactDistribution)
    }
}

impl<T, const DECIMALS: u8> PositionImpactMarketMut<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn position_impact_pool_mut(&mut self) -> crate::Result<&mut Self::Pool> {
        Ok(&mut self.position_impact)
    }

    fn just_passed_in_seconds_for_position_impact_distribution(&mut self) -> crate::Result<u64> {
        self.just_passed_in_seconds(ClockKind::PriceImpactDistribution)
    }
}

impl<T, const DECIMALS: u8> BorrowingFeeMarket<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn borrowing_fee_params(&self) -> crate::Result<BorrowingFeeParams<Self::Num>> {
        Ok(self.config.borrowing_fee_params.clone())
    }

    fn borrowing_factor_pool(&self) -> crate::Result<&Self::Pool> {
        Ok(&self.borrowing_factor)
    }

    fn total_borrowing_pool(&self) -> crate::Result<&Self::Pool> {
        Ok(&self.total_borrowing)
    }

    fn passed_in_seconds_for_borrowing(&self) -> crate::Result<u64> {
        self.passed_in_seconds(ClockKind::Borrowing)
    }

    fn borrowing_fee_kink_model_params(
        &self,
    ) -> crate::Result<BorrowingFeeKinkModelParams<Self::Num>> {
        let for_one_side = self.config.borrowing_fee_kink_model_params.clone();
        Ok(BorrowingFeeKinkModelParams::builder()
            .long(for_one_side.clone())
            .short(for_one_side)
            .build())
    }
}

impl<T, const DECIMALS: u8> BorrowingFeeMarketMut<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn borrowing_factor_pool_mut(&mut self) -> crate::Result<&mut Self::Pool> {
        Ok(&mut self.borrowing_factor)
    }

    fn just_passed_in_seconds_for_borrowing(&mut self) -> crate::Result<u64> {
        self.just_passed_in_seconds(ClockKind::Borrowing)
    }
}

impl<T, const DECIMALS: u8> PerpMarket<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn funding_factor_per_second(&self) -> &Self::Signed {
        &self.funding_factor_per_second
    }

    fn funding_amount_per_size_adjustment(&self) -> Self::Num {
        self.funding_amount_per_size_adjustment.clone()
    }

    fn funding_fee_params(&self) -> crate::Result<FundingFeeParams<Self::Num>> {
        Ok(self.config.funding_fee_params.clone())
    }

    fn funding_amount_per_size_pool(&self, is_long: bool) -> crate::Result<&Self::Pool> {
        if is_long {
            Ok(&self.funding_amount_per_size.0)
        } else {
            Ok(&self.funding_amount_per_size.1)
        }
    }

    fn claimable_funding_amount_per_size_pool(&self, is_long: bool) -> crate::Result<&Self::Pool> {
        if is_long {
            Ok(&self.claimable_funding_amount_per_size.0)
        } else {
            Ok(&self.claimable_funding_amount_per_size.1)
        }
    }

    fn position_params(&self) -> crate::Result<PositionParams<Self::Num>> {
        Ok(self.config.position_params.clone())
    }

    fn order_fee_params(&self) -> crate::Result<FeeParams<Self::Num>> {
        Ok(self.config.order_fee_params.clone())
    }

    fn min_collateral_factor_for_open_interest_multiplier(
        &self,
        _is_long: bool,
    ) -> crate::Result<Self::Num> {
        Ok(self.config.min_collateral_factor_for_oi.clone())
    }

    fn liquidation_fee_params(&self) -> crate::Result<LiquidationFeeParams<Self::Num>> {
        Ok(self.config.liquidation_fee_params.clone())
    }
}

impl<T, const DECIMALS: u8> PerpMarketMut<DECIMALS> for TestMarket<T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn funding_factor_per_second_mut(&mut self) -> &mut Self::Signed {
        &mut self.funding_factor_per_second
    }

    fn open_interest_pool_mut(&mut self, is_long: bool) -> crate::Result<&mut Self::Pool> {
        if is_long {
            Ok(&mut self.open_interest.0)
        } else {
            Ok(&mut self.open_interest.1)
        }
    }

    fn open_interest_in_tokens_pool_mut(
        &mut self,
        is_long: bool,
    ) -> crate::Result<&mut Self::Pool> {
        if is_long {
            Ok(&mut self.open_interest_in_tokens.0)
        } else {
            Ok(&mut self.open_interest_in_tokens.1)
        }
    }

    fn funding_amount_per_size_pool_mut(
        &mut self,
        is_long: bool,
    ) -> crate::Result<&mut Self::Pool> {
        if is_long {
            Ok(&mut self.funding_amount_per_size.0)
        } else {
            Ok(&mut self.funding_amount_per_size.1)
        }
    }

    fn claimable_funding_amount_per_size_pool_mut(
        &mut self,
        is_long: bool,
    ) -> crate::Result<&mut Self::Pool> {
        if is_long {
            Ok(&mut self.claimable_funding_amount_per_size.0)
        } else {
            Ok(&mut self.claimable_funding_amount_per_size.1)
        }
    }

    fn collateral_sum_pool_mut(&mut self, is_long: bool) -> crate::Result<&mut Self::Pool> {
        if is_long {
            Ok(&mut self.collateral_sum.0)
        } else {
            Ok(&mut self.collateral_sum.1)
        }
    }

    fn total_borrowing_pool_mut(&mut self) -> crate::Result<&mut Self::Pool> {
        Ok(&mut self.total_borrowing)
    }

    fn virtual_inventory_for_positions_pool_mut(
        &mut self,
    ) -> crate::Result<Option<impl DerefMut<Target = Self::Pool>>> {
        Ok(None::<&mut Self::Pool>)
    }

    fn just_passed_in_seconds_for_funding(&mut self) -> crate::Result<u64> {
        self.just_passed_in_seconds(ClockKind::Funding)
    }
}

#[cfg(test)]
mod shift_roundtrip_tests {
    use super::{TestMarket, TestPool};
    use crate::{
        action::MarketAction,
        market::{LiquidityMarketExt, LiquidityMarketMutExt},
        price::Prices,
        utils,
        PnlFactorKind,
    };

    fn market(
        long_amount: u64,
        short_amount: u64,
        swap_impact_long: u64,
        swap_impact_short: u64,
        total_supply: u64,
    ) -> TestMarket<u64, 9> {
        let mut market = TestMarket::<u64, 9>::default();
        market.primary = TestPool {
            long_amount,
            short_amount,
        };
        market.swap_impact = TestPool {
            long_amount: swap_impact_long,
            short_amount: swap_impact_short,
        };
        market.total_supply = total_supply;
        market
    }

    fn market_with_virtual_inventory(
        long_amount: u64,
        short_amount: u64,
        swap_impact_long: u64,
        swap_impact_short: u64,
        total_supply: u64,
        virtual_long_amount: u64,
        virtual_short_amount: u64,
    ) -> TestMarket<u64, 9> {
        let mut market = market(
            long_amount,
            short_amount,
            swap_impact_long,
            swap_impact_short,
            total_supply,
        );
        market.virtual_inventory_for_swaps = Some(TestPool {
            long_amount: virtual_long_amount,
            short_amount: virtual_short_amount,
        });
        market
    }

    fn with_perp_state(
        mut market: TestMarket<u64, 9>,
        long_oi: u64,
        short_oi: u64,
        long_oi_tokens: u64,
        short_oi_tokens: u64,
        position_impact_amount: u64,
        total_borrowing_long: u64,
        total_borrowing_short: u64,
    ) -> TestMarket<u64, 9> {
        market.open_interest = (
            TestPool {
                long_amount: long_oi,
                short_amount: 0,
            },
            TestPool {
                long_amount: 0,
                short_amount: short_oi,
            },
        );
        market.open_interest_in_tokens = (
            TestPool {
                long_amount: long_oi_tokens,
                short_amount: 0,
            },
            TestPool {
                long_amount: 0,
                short_amount: short_oi_tokens,
            },
        );
        market.position_impact = TestPool {
            long_amount: position_impact_amount,
            short_amount: 0,
        };
        market.total_borrowing = TestPool {
            long_amount: total_borrowing_long,
            short_amount: total_borrowing_short,
        };
        market
    }

    fn market_token_value(
        market: &TestMarket<u64, 9>,
        amount: u64,
        prices: Prices<u64>,
    ) -> crate::Result<u64> {
        let pool_value = market.pool_value(&prices, PnlFactorKind::MaxAfterWithdrawal, false)?;
        utils::market_token_amount_to_usd(
            &amount,
            &pool_value.unsigned_abs(),
            &market.total_supply,
        )
        .ok_or(crate::Error::Computation("market token value"))
    }

    fn shift_once(
        from: &mut TestMarket<u64, 9>,
        to: &mut TestMarket<u64, 9>,
        market_token_amount: u64,
        prices: Prices<u64>,
        include_virtual_inventory_impact: bool,
    ) -> crate::Result<u64> {
        let withdrawal = from.withdraw(market_token_amount, prices)?.execute()?;
        if *withdrawal.long_token_output() == 0 && *withdrawal.short_token_output() == 0 {
            return Ok(0);
        }
        let deposit = to
            .deposit(
                *withdrawal.long_token_output(),
                *withdrawal.short_token_output(),
                prices,
            )?
            .with_virtual_inventory_impact(include_virtual_inventory_impact)
            .execute()?;
        Ok(*deposit.minted())
    }

    fn assert_roundtrip_not_profitable(
        mut a: TestMarket<u64, 9>,
        mut b: TestMarket<u64, 9>,
        amount: u64,
        prices: Prices<u64>,
        include_virtual_inventory_impact: bool,
    ) -> crate::Result<()> {
        let initial_value = match market_token_value(&a, amount, prices) {
            Ok(value) => value,
            Err(_) => return Ok(()),
        };

        let b_amount = shift_once(
            &mut a,
            &mut b,
            amount,
            prices,
            include_virtual_inventory_impact,
        )?;
        if b_amount == 0 {
            return Ok(());
        }
        let final_amount = shift_once(
            &mut b,
            &mut a,
            b_amount,
            prices,
            include_virtual_inventory_impact,
        )?;

        let final_value = market_token_value(&a, final_amount, prices)?;
        assert!(
            final_value <= initial_value,
            "profitable shift roundtrip: initial_value={initial_value}, final_value={final_value}, initial_amount={amount}, final_amount={final_amount}, b_amount={b_amount}, a={a:#?}, b={b:#?}"
        );
        Ok(())
    }

    fn try_assert_triangular_cycle_not_profitable(
        mut a: TestMarket<u64, 9>,
        mut b: TestMarket<u64, 9>,
        mut c: TestMarket<u64, 9>,
        amount: u64,
        prices: Prices<u64>,
        include_virtual_inventory_impact: bool,
        repetitions: usize,
    ) -> crate::Result<()> {
        let initial_value = match market_token_value(&a, amount, prices) {
            Ok(value) => value,
            Err(_) => return Ok(()),
        };
        let mut current_amount = amount;

        for _ in 0..repetitions {
            let b_amount = match shift_once(
                &mut a,
                &mut b,
                current_amount,
                prices,
                include_virtual_inventory_impact,
            ) {
                Ok(amount) => amount,
                Err(_) => return Ok(()),
            };
            if b_amount == 0 {
                return Ok(());
            }

            let c_amount = match shift_once(
                &mut b,
                &mut c,
                b_amount,
                prices,
                include_virtual_inventory_impact,
            ) {
                Ok(amount) => amount,
                Err(_) => return Ok(()),
            };
            if c_amount == 0 {
                return Ok(());
            }

            current_amount = match shift_once(
                &mut c,
                &mut a,
                c_amount,
                prices,
                include_virtual_inventory_impact,
            ) {
                Ok(amount) => amount,
                Err(_) => return Ok(()),
            };
            if current_amount == 0 {
                return Ok(());
            }
        }

        let final_value = match market_token_value(&a, current_amount, prices) {
            Ok(value) => value,
            Err(_) => return Ok(()),
        };
        assert!(
            final_value <= initial_value,
            "profitable triangular shift cycle: initial_value={initial_value}, final_value={final_value}, initial_amount={amount}, final_amount={current_amount}, repetitions={repetitions}, a={a:#?}, b={b:#?}, c={c:#?}"
        );
        Ok(())
    }

    #[test]
    fn shift_roundtrip_constant_prices_two_markets() -> crate::Result<()> {
        let prices = Prices::new_for_test(120, 120, 1);

        let cases = [
            (
                market(1_000_000_000, 1_000_000_000, 0, 0, 121_000_000_000),
                market(1_000_000_000, 1_000_000_000, 0, 0, 121_000_000_000),
                1_000_000_000,
            ),
            (
                market(2_000_000_000, 400_000_000, 0, 0, 240_400_000_000),
                market(400_000_000, 2_000_000_000, 0, 0, 50_000_000_000),
                1_000_000_000,
            ),
            (
                market(
                    5_000_000_000,
                    5_000_000_000,
                    10_000_000,
                    10_000_000,
                    605_000_000_000,
                ),
                market(
                    1_000_000_000,
                    8_000_000_000,
                    30_000_000,
                    5_000_000,
                    128_000_000_000,
                ),
                2_500_000_000,
            ),
            (
                market(50_000_000, 900_000_000, 1_000_000, 8_000_000, 6_900_000_000),
                market(900_000_000, 50_000_000, 8_000_000, 1_000_000, 108_050_000_000),
                500_000_000,
            ),
        ];

        for (a, b, amount) in cases {
            assert_roundtrip_not_profitable(a, b, amount, prices, true)?;
        }
        Ok(())
    }

    #[test]
    fn shift_roundtrip_constant_prices_sweep() -> crate::Result<()> {
        let prices = Prices::new_for_test(120, 120, 1);
        let amounts = [1, 10_000, 1_000_000, 100_000_000, 1_000_000_000];
        let states = [
            (100_000_000, 100_000_000, 0, 0, 12_100_000_000),
            (1_000_000_000, 50_000_000, 0, 0, 120_050_000_000),
            (50_000_000, 1_000_000_000, 0, 0, 7_000_000_000),
            (
                3_000_000_000,
                1_000_000_000,
                25_000_000,
                2_000_000,
                361_000_000_000,
            ),
            (
                1_000_000_000,
                3_000_000_000,
                2_000_000,
                25_000_000,
                123_000_000_000,
            ),
        ];

        for (a_idx, a_state) in states.iter().enumerate() {
            for (b_idx, b_state) in states.iter().enumerate() {
                if a_idx == b_idx {
                    continue;
                }
                for amount in amounts {
                    if amount >= a_state.4 {
                        continue;
                    }

                    let a = market(a_state.0, a_state.1, a_state.2, a_state.3, a_state.4);
                    let b = market(b_state.0, b_state.1, b_state.2, b_state.3, b_state.4);
                    assert_roundtrip_not_profitable(a, b, amount, prices, false)?;

                    let a = market_with_virtual_inventory(
                        a_state.0,
                        a_state.1,
                        a_state.2,
                        a_state.3,
                        a_state.4,
                        a_state.0.saturating_mul(2),
                        a_state.1.saturating_mul(2),
                    );
                    let b = market_with_virtual_inventory(
                        b_state.0,
                        b_state.1,
                        b_state.2,
                        b_state.3,
                        b_state.4,
                        b_state.0.saturating_mul(2),
                        b_state.1.saturating_mul(2),
                    );
                    assert_roundtrip_not_profitable(a, b, amount, prices, true)?;
                }
            }
        }
        Ok(())
    }

    #[test]
    fn shift_triangular_cycle_constant_prices_sweep() -> crate::Result<()> {
        let prices = Prices::new_for_test(120, 120, 1);
        let base_states = [
            market(100_000_000, 100_000_000, 0, 0, 12_100_000_000),
            market(1_000_000_000, 50_000_000, 0, 0, 120_050_000_000),
            market(50_000_000, 1_000_000_000, 0, 0, 7_000_000_000),
            market(
                3_000_000_000,
                1_000_000_000,
                25_000_000,
                2_000_000,
                361_000_000_000,
            ),
            market(
                1_000_000_000,
                3_000_000_000,
                2_000_000,
                25_000_000,
                123_000_000_000,
            ),
            with_perp_state(
                market(1_000_000_000, 1_000_000_000, 50_000_000, 5_000_000, 90_000_000_000),
                90_000_000_000,
                0,
                1_000_000_000,
                0,
                250_000_000,
                2_000_000,
                0,
            ),
            with_perp_state(
                market(1_000_000_000, 1_000_000_000, 5_000_000, 50_000_000, 115_000_000_000),
                0,
                135_000_000_000,
                0,
                1_000_000_000,
                100_000_000,
                0,
                2_000_000,
            ),
            with_perp_state(
                market_with_virtual_inventory(
                    200_000_000,
                    900_000_000,
                    80_000_000,
                    80_000_000,
                    20_000_000_000,
                    5_000_000_000,
                    200_000_000,
                ),
                16_000_000_000,
                25_000_000_000,
                150_000_000,
                150_000_000,
                500_000_000,
                1_000_000,
                1_000_000,
            ),
        ];
        let amounts = [1, 10_000, 1_000_000, 100_000_000, 1_000_000_000];

        for a_idx in 0..base_states.len() {
            for b_idx in 0..base_states.len() {
                for c_idx in 0..base_states.len() {
                    if a_idx == b_idx || b_idx == c_idx || a_idx == c_idx {
                        continue;
                    }
                    for amount in amounts {
                        if amount >= base_states[a_idx].total_supply {
                            continue;
                        }
                        for include_virtual_inventory_impact in [false, true] {
                            for repetitions in [1, 2, 5] {
                                try_assert_triangular_cycle_not_profitable(
                                    base_states[a_idx].clone(),
                                    base_states[b_idx].clone(),
                                    base_states[c_idx].clone(),
                                    amount,
                                    prices,
                                    include_virtual_inventory_impact,
                                    repetitions,
                                )?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod rwa_timing_tests {
    use std::time::Duration;

    use super::{TestMarket, TestMarketConfig, TestPool};
    use crate::{
        action::MarketAction,
        market::{BorrowingFeeMarketExt, BorrowingFeeMarketMutExt},
        params::fee::BorrowingFeeKinkModelParamsForOneSide,
        price::Prices,
    };

    fn market_with_borrowing_factor(base_factor: u64) -> TestMarket<u64, 9> {
        let mut config = TestMarketConfig::<u64, 9>::default();
        config.borrowing_fee_kink_model_params = BorrowingFeeKinkModelParamsForOneSide::builder()
            .optimal_usage_factor(750_000_000)
            .base_borrowing_factor(base_factor)
            .above_optimal_usage_borrowing_factor(base_factor)
            .build();
        config.max_open_interest = u64::MAX;

        let mut market = TestMarket::<u64, 9>::with_config(config);
        market.primary = TestPool {
            long_amount: 1_000_000_000,
            short_amount: 1_000_000_000,
        };
        market.open_interest = (
            TestPool {
                long_amount: 60_000_000_000,
                short_amount: 0,
            },
            TestPool {
                long_amount: 0,
                short_amount: 0,
            },
        );
        market.open_interest_in_tokens = (
            TestPool {
                long_amount: 500_000_000,
                short_amount: 0,
            },
            TestPool {
                long_amount: 0,
                short_amount: 0,
            },
        );
        market
    }

    #[test]
    fn stale_regular_borrowing_update_consumes_closed_duration() -> crate::Result<()> {
        let prices = Prices::new_for_test(120, 120, 1);
        let regular_factor = 10;
        let closed_factor = 1_000;
        let closed_duration = Duration::from_secs(6 * 60 * 60);

        let mut stale_regular_market = market_with_borrowing_factor(regular_factor);
        let mut correct_closed_market = market_with_borrowing_factor(closed_factor);

        stale_regular_market
            .update_borrowing(&prices)?
            .execute()?;
        correct_closed_market
            .update_borrowing(&prices)?
            .execute()?;

        stale_regular_market.move_clock_forward(closed_duration);
        correct_closed_market.move_clock_forward(closed_duration);

        stale_regular_market
            .update_borrowing(&prices)?
            .execute()?;
        correct_closed_market
            .update_borrowing(&prices)?
            .execute()?;

        let stale_factor = stale_regular_market.cumulative_borrowing_factor(true)?;
        let correct_factor = correct_closed_market.cumulative_borrowing_factor(true)?;
        let missed_factor = correct_factor - stale_factor;
        let size_in_usd = 60_000_000_000_u128;
        let missed_fee_value = size_in_usd * u128::from(missed_factor) / 1_000_000_000_u128;

        println!(
            "rwa stale-open borrowing: regular_factor={regular_factor}, closed_factor={closed_factor}, duration_secs={}, stale_factor={stale_factor}, correct_closed_factor={correct_factor}, missed_factor={missed_factor}, missed_fee_value={missed_fee_value}",
            closed_duration.as_secs()
        );

        assert!(
            stale_factor < correct_factor,
            "stale regular borrowing update should under-accrue: stale={stale_factor}, correct={correct_factor}"
        );

        stale_regular_market.move_clock_forward(closed_duration);
        stale_regular_market
            .update_borrowing(&prices)?
            .execute()?;

        let stale_after_second_update = stale_regular_market.cumulative_borrowing_factor(true)?;
        println!(
            "after later update: stale_after_second_update={stale_after_second_update}, correct_closed_factor={correct_factor}"
        );
        assert!(
            stale_after_second_update < correct_factor,
            "once the clock is consumed with regular params, a later update cannot recover the missed closed borrowing: stale_after_second_update={stale_after_second_update}, correct={correct_factor}"
        );

        Ok(())
    }
}

/// Test Position
#[derive(Debug, Clone, Copy, Default)]
pub struct TestPosition<T, const DECIMALS: u8> {
    is_long: bool,
    is_collateral_token_long: bool,
    collateral_token_amount: T,
    size_in_usd: T,
    size_in_tokens: T,
    borrowing_factor: T,
    funding_fee_amount_per_size: T,
    claimable_funding_fee_amount_per_size: (T, T),
}

impl<T: Unsigned, const DECIMALS: u8> TestPosition<T, DECIMALS>
where
    T::Signed: fmt::Debug,
{
    /// Create a [`TestPositionOps`] for ops.
    pub fn ops<'a>(
        &'a mut self,
        market: &'a mut TestMarket<T, DECIMALS>,
    ) -> TestPositionOps<'a, T, DECIMALS> {
        TestPositionOps {
            market,
            position: self,
        }
    }

    /// Create an empty long position.
    pub fn long(long_token_as_collateral: bool) -> Self
    where
        T: Default,
    {
        Self {
            is_long: true,
            is_collateral_token_long: long_token_as_collateral,
            ..Default::default()
        }
    }

    /// Create an empty short position.
    pub fn short(long_token_as_collateral: bool) -> Self
    where
        T: Default,
    {
        Self {
            is_long: false,
            is_collateral_token_long: long_token_as_collateral,
            ..Default::default()
        }
    }
}

/// Test Position.
#[derive(Debug)]
pub struct TestPositionOps<'a, T: Unsigned, const DECIMALS: u8>
where
    T::Signed: fmt::Debug,
{
    market: &'a mut TestMarket<T, DECIMALS>,
    position: &'a mut TestPosition<T, DECIMALS>,
}

impl<T, const DECIMALS: u8> PositionState<DECIMALS> for TestPositionOps<'_, T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    type Num = T;

    type Signed = T::Signed;

    fn collateral_amount(&self) -> &Self::Num {
        &self.position.collateral_token_amount
    }

    fn size_in_usd(&self) -> &Self::Num {
        &self.position.size_in_usd
    }

    fn size_in_tokens(&self) -> &Self::Num {
        &self.position.size_in_tokens
    }

    fn borrowing_factor(&self) -> &Self::Num {
        &self.position.borrowing_factor
    }

    fn funding_fee_amount_per_size(&self) -> &Self::Num {
        &self.position.funding_fee_amount_per_size
    }

    fn claimable_funding_fee_amount_per_size(&self, is_long_collateral: bool) -> &Self::Num {
        if is_long_collateral {
            &self.position.claimable_funding_fee_amount_per_size.0
        } else {
            &self.position.claimable_funding_fee_amount_per_size.1
        }
    }
}

impl<T, const DECIMALS: u8> Position<DECIMALS> for TestPositionOps<'_, T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    type Market = TestMarket<T, DECIMALS>;

    fn market(&self) -> &Self::Market {
        self.market
    }

    fn is_long(&self) -> bool {
        self.position.is_long
    }

    fn is_collateral_token_long(&self) -> bool {
        self.position.is_collateral_token_long
    }

    fn are_pnl_and_collateral_tokens_the_same(&self) -> bool {
        self.position.is_long == self.position.is_collateral_token_long
    }

    fn on_validate(&self) -> crate::Result<()> {
        Ok(())
    }
}

impl<T, const DECIMALS: u8> PositionMut<DECIMALS> for TestPositionOps<'_, T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn market_mut(&mut self) -> &mut Self::Market {
        self.market
    }

    fn on_increased(&mut self) -> crate::Result<()> {
        Ok(())
    }

    fn on_decreased(&mut self) -> crate::Result<()> {
        Ok(())
    }

    fn on_swapped(
        &mut self,
        ty: DecreasePositionSwapType,
        report: &crate::action::swap::SwapReport<Self::Num, <Self::Num as Unsigned>::Signed>,
    ) -> crate::Result<()> {
        println!("swapped: ty={ty:?}, report={report:?}");
        Ok(())
    }

    fn on_swap_error(
        &mut self,
        ty: DecreasePositionSwapType,
        error: crate::Error,
    ) -> crate::Result<()> {
        eprintln!("swap error: ty={ty:?}, err={error}");
        Ok(())
    }
}

impl<T, const DECIMALS: u8> PositionStateMut<DECIMALS> for TestPositionOps<'_, T, DECIMALS>
where
    T: CheckedSub + fmt::Display + FixedPointOps<DECIMALS>,
    T::Signed: Num + std::fmt::Debug,
{
    fn collateral_amount_mut(&mut self) -> &mut Self::Num {
        &mut self.position.collateral_token_amount
    }

    fn size_in_usd_mut(&mut self) -> &mut Self::Num {
        &mut self.position.size_in_usd
    }

    fn size_in_tokens_mut(&mut self) -> &mut Self::Num {
        &mut self.position.size_in_tokens
    }

    fn borrowing_factor_mut(&mut self) -> &mut Self::Num {
        &mut self.position.borrowing_factor
    }

    fn funding_fee_amount_per_size_mut(&mut self) -> &mut Self::Num {
        &mut self.position.funding_fee_amount_per_size
    }

    fn claimable_funding_fee_amount_per_size_mut(
        &mut self,
        is_long_collateral: bool,
    ) -> &mut Self::Num {
        if is_long_collateral {
            &mut self.position.claimable_funding_fee_amount_per_size.0
        } else {
            &mut self.position.claimable_funding_fee_amount_per_size.1
        }
    }
}
