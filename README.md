# Secure SPL Token Escrow Challenge

Build a secure escrow program for SPL tokens on Solana.

## 🎯 Objective

Create a Solana program that allows:
1. Seller to create an escrow offer (lock tokens)
2. Buyer to accept the offer (exchange tokens)
3. Seller to cancel the offer (refund)

## 📋 Requirements

### Core Features
- [ ] Create escrow with SPL tokens
- [ ] Accept escrow (token swap)
- [ ] Cancel escrow (refund to seller)
- [ ] Proper token account handling

### Security Requirements
- [ ] Validate token mints
- [ ] Verify token account ownership
- [ ] Prevent double-spending
- [ ] Handle token decimals correctly

## 🏗️ Project Structure

```
spl-escrow-starter/
├── programs/
│   └── spl-escrow/
│       └── src/
│           └── lib.rs          # Your program logic
├── tests/
│   └── spl-escrow.ts          # Test file
├── Anchor.toml
├── Cargo.toml
└── package.json
```

## 🚀 Getting Started

### Prerequisites
- Rust & Cargo
- Solana CLI
- Anchor Framework

### Installation

```bash
npm install
anchor build
anchor test
```

## 📝 Evaluation Criteria

| Criteria | Weight |
|----------|--------|
| **Correctness** | Pass/Fail |
| **Token Handling** | 30% |
| **Security** | 40% |
| **Code Quality** | 30% |

### What We Check
- **Token Handling:** Correct SPL token transfers, mint validation
- **Security:** Ownership checks, reentrancy prevention
- **Code Quality:** Clean Rust code, proper error handling

## 📚 Resources

- [SPL Token Program](https://spl.solana.com/token)
- [Anchor SPL Guide](https://www.anchor-lang.com/docs/tokens)
- [Solana Cookbook - Token Transfers](https://solanacookbook.com/references/token.html)

## 🔗 Submission

1. Fork this repository
2. Implement the escrow program
3. Ensure all tests pass
4. Submit your repo URL on the platform

Good luck! 🎉
# spl-escrow-starter
