use soroban_sdk::{contracttype, Address, Vec};

/// Represents the lifecycle status of an NFT lending listing.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListingStatus {
    /// The listing is open and available for borrowers to take.
    Open,
    /// The listing has been filled by a borrower.
    Filled,
    /// The listing was cancelled by the lender.
    Cancelled,
}

/// Represents the status of an active borrowing position.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PositionStatus {
    /// The loan is active and ongoing.
    Active,
    /// The borrower repaid the loan and retrieved their collateral.
    Returned,
    /// Collateral was liquidated due to price drop or expiration.
    Liquidated,
    /// The loan expired without repayment.
    Expired,
}

/// Core listing structure for NFT collateralized lending.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Listing {
    /// Unique identifier for the listing.
    pub id: u64,
    /// Address of the lender creating the listing.
    pub lender: Address,
    /// Address of the NFT contract being listed.
    pub nft_contract: Address,
    /// Token ID of the NFT.
    pub token_id: u128,
    /// Declared price in USD (fixed-point, 7 decimals).
    pub declared_price_usd: i128,
    /// Tiered interest rates in basis points per time step.
    pub interest_schedule_bps: Vec<u32>,
    /// Maximum allowed duration for the loan in days.
    pub max_duration_days: u32,
    /// Minimum buffer required above valuation (basis points).
    pub min_collateral_buffer_bps: u32,
    /// Threshold at which position can be liquidated (basis points).
    pub liquidation_threshold_bps: u32,
    /// Current status of the listing.
    pub status: ListingStatus,
    /// Unix timestamp when listing was created.
    pub created_at: u64,
}

/// Core borrowing position structure.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    /// Unique identifier for the position.
    pub id: u64,
    /// Reference identifier to the originating listing.
    pub listing_id: u64,
    /// Address of the lender.
    pub lender: Address,
    /// Address of the borrower.
    pub borrower: Address,
    /// Address of the NFT contract.
    pub nft_contract: Address,
    /// Token ID of the NFT.
    pub token_id: u128,
    /// Declared price in USD.
    pub declared_price_usd: i128,
    /// Currency used for collateral deposit.
    pub collateral_currency: Address,
    /// Amount of collateral deposited.
    pub collateral_amount: i128,
    /// Interest schedule in basis points.
    pub interest_schedule_bps: Vec<u32>,
    /// Liquidation threshold in basis points.
    pub liquidation_threshold_bps: u32,
    /// Unix timestamp when loan started.
    pub start_time: u64,
    /// Maximum duration allowed in seconds.
    pub max_duration_secs: u64,
    /// Current position status.
    pub status: PositionStatus,
}

/// Global configuration parameters for the platform.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformConfig {
    /// Administrator address for contract governance.
    pub admin: Address,
    /// Address collecting platform protocol fees.
    pub fee_receiver: Address,
    /// Fee cut taken by platform in basis points.
    pub platform_fee_bps: u32,
    /// Incentive fee awarded to liquidators in basis points.
    pub liquidator_fee_bps: u32,
    /// Minimum allowed buffer in basis points.
    pub min_buffer_bps: u32,
    /// Maximum allowed buffer in basis points.
    pub max_buffer_bps: u32,
    /// Minimum liquidation threshold in basis points.
    pub min_liq_threshold_bps: u32,
    /// Maximum liquidation threshold in basis points.
    pub max_liq_threshold_bps: u32,
    /// Address of the oracle feed contract.
    pub oracle_address: Address,
    /// Maximum allowed age of price feed in seconds.
    pub max_price_staleness_secs: u64,
}
