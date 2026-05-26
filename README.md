# Pactora: A Soroban Freelance Escrow Contract

Pactora is a Stellar Soroban smart contract that locks project payment before freelance work begins and releases it after client approval.

## Problem

A freelance designer in Manila working with a small business client in Singapore often starts projects without guaranteed payment, causing delayed income or unpaid work when the client disappears after delivery.

## Solution

Pactora lets the Singapore client lock USDC into a Soroban escrow contract before work begins, holds the funds for the Manila freelancer, and releases payment after client approval, using Stellar's low-cost, fast settlement to make cross-border freelance payments affordable and trusted.

## Timeline

- Week 1: Build and test the Soroban escrow contract.
- Week 2: Deploy to Stellar testnet and record contract calls.
- Week 3: Add optional Freighter wallet UX and polish the demo flow.
- Week 4: Add Frontend to maximize user experience.

## Stellar Features Used

- USDC transfers through Stellar asset token contracts
- Soroban smart contracts
- Trustlines for asset support

## Vision and Purpose

Pactora helps Filipino freelancers verify that project funds are secured before they start work. The purpose is to reduce unpaid freelance labor while still protecting clients from paying before delivery.

## Prerequisites

- Rust
- Soroban CLI
- Freighter wallet for optional wallet UX

## Build

```bash
soroban contract build
```

## Test

```bash
cargo test
```

## Deploy to Stellar Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/pactora.wasm \
  --source <CLIENT_SECRET_KEY_OR_IDENTITY> \
  --network testnet
```

## Sample CLI Calls

Replace the placeholder values with your deployed contract ID, Stellar addresses, and token contract address.

### Create Escrow

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <CLIENT_IDENTITY> \
  --network testnet \
  -- \
  create_escrow \
  --client <CLIENT_ADDRESS> \
  --freelancer <FREELANCER_ADDRESS> \
  --token <USDC_TOKEN_CONTRACT_ADDRESS> \
  --amount 10000000
```

### Fund Escrow

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <CLIENT_IDENTITY> \
  --network testnet \
  -- \
  fund_escrow \
  --escrow_id 1
```

### Approve Work

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <CLIENT_IDENTITY> \
  --network testnet \
  -- \
  approve_work \
  --escrow_id 1
```

### Release Payment

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <CLIENT_IDENTITY> \
  --network testnet \
  -- \
  release_payment \
  --escrow_id 1
```

### Cancel Escrow

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <CLIENT_IDENTITY> \
  --network testnet \
  -- \
  cancel_escrow \
  --escrow_id 1
```

## License

MIT

## Contract ID
CARJ4EZ26VGAXXRXEIRPMKRD4LYKHWDVVNMDR22XZGV2R5NAW3L3TGFJ

## Contract Link
https://stellar.expert/explorer/testnet/contract/CARJ4EZ26VGAXXRXEIRPMKRD4LYKHWDVVNMDR22XZGV2R5NAW3L3TGFJ?filter=history

## Contract Screenshot
<img width="1920" height="1080" alt="Stellar-Screenshot" src="https://github.com/user-attachments/assets/02693403-36a3-4d3c-82be-b7b271a51797" />


