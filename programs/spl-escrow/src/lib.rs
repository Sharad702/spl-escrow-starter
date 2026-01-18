use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod spl_escrow {
    use super::*;

    /// Create a new escrow offer
    /// TODO: Implement escrow creation
    /// - Lock seller's tokens in escrow PDA
    /// - Store escrow details (seller, amounts, mints)
    pub fn create_escrow(
        _ctx: Context<CreateEscrow>,
        _offer_amount: u64,
        _request_amount: u64,
    ) -> Result<()> {
        // TODO: Implement this function
        // 1. Transfer tokens from seller to escrow vault
        // 2. Store escrow state
        Ok(())
    }

    /// Accept an escrow offer
    /// TODO: Implement escrow acceptance
    /// - Transfer buyer's tokens to seller
    /// - Transfer escrowed tokens to buyer
    /// - Close escrow
    pub fn accept_escrow(_ctx: Context<AcceptEscrow>) -> Result<()> {
        // TODO: Implement this function
        // 1. Transfer request tokens from buyer to seller
        // 2. Transfer offer tokens from escrow to buyer
        // 3. Close escrow account
        Ok(())
    }

    /// Cancel an escrow offer
    /// TODO: Implement escrow cancellation
    /// - Refund escrowed tokens to seller
    /// - Close escrow
    pub fn cancel_escrow(_ctx: Context<CancelEscrow>) -> Result<()> {
        // TODO: Implement this function
        // 1. Transfer tokens back to seller
        // 2. Close escrow account
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    // TODO: Define accounts for creating escrow
    // Hints:
    // - Seller signs and provides tokens
    // - Escrow state PDA
    // - Escrow vault (token account owned by PDA)
    // - Token mints for validation
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    pub offer_mint: Account<'info, Mint>,
    pub request_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub seller_offer_token: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptEscrow<'info> {
    // TODO: Define accounts for accepting escrow
    // Hints:
    // - Buyer signs and provides request tokens
    // - Seller receives request tokens
    // - Buyer receives offer tokens from escrow
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    // TODO: Define accounts for canceling escrow
    // Hints:
    // - Only seller can cancel
    // - Tokens returned to seller
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

// TODO: Define the Escrow account structure
// #[account]
// pub struct Escrow {
//     pub seller: Pubkey,
//     pub offer_mint: Pubkey,
//     pub request_mint: Pubkey,
//     pub offer_amount: u64,
//     pub request_amount: u64,
//     pub bump: u8,
// }

#[error_code]
pub enum EscrowError {
    #[msg("Unauthorized: Only the seller can perform this action")]
    Unauthorized,
    #[msg("Invalid token mint")]
    InvalidMint,
    #[msg("Escrow already completed or cancelled")]
    EscrowClosed,
}
