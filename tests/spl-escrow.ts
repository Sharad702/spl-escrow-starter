import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplEscrow } from "../target/types/spl_escrow";
import { createMint, createAccount, mintTo } from "@solana/spl-token";
import { expect } from "chai";

describe("spl-escrow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SplEscrow as Program<SplEscrow>;

  it("Creates an escrow", async () => {
    // TODO: Test escrow creation
    // 1. Create token mints
    // 2. Create token accounts
    // 3. Mint tokens to seller
    // 4. Create escrow
    // 5. Verify tokens are locked
  });

  it("Accepts an escrow", async () => {
    // TODO: Test escrow acceptance
    // 1. Setup escrow (or use from previous test)
    // 2. Buyer accepts escrow
    // 3. Verify token swaps occurred
  });

  it("Cancels an escrow", async () => {
    // TODO: Test escrow cancellation
    // 1. Create new escrow
    // 2. Seller cancels
    // 3. Verify tokens returned to seller
  });

  it("Prevents unauthorized cancellation", async () => {
    // TODO: Test that non-sellers cannot cancel
    // 1. Create escrow
    // 2. Different user tries to cancel
    // 3. Expect error
  });
});
