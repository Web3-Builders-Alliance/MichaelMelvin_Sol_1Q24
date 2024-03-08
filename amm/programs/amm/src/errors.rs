use anchor_lang::error_code;

#[error_code]
pub enum AmmError {
    #[msg("The fee must be less than 100% (< 10000)")]
    FeeTooHigh,

    #[msg("The liquidity pool is locked. You cannot deposit or withdraw from it.")]
    Locked,

    #[msg("The instriction has expired.")]
    Expired,

    #[msg("One of the amount passed is invalid.")]
    InvalidAmount,

    #[msg("Somthing went wrong with the deposit. Please try again.")]
    InvalidDeposit,

    #[msg("Somthing went wrong with the withdraw. Please try again.")]
    InvalidWithdraw,

    #[msg("Slippage exceeded. Please try again.")]
    SlippageExceeded,

    #[msg("Incorrect K. Please try again.")]
    IncorrectK,
}
