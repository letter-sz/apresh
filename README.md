# Apresh

**Turning Empty Space into an Opportunity**

Apresh is a decentralized delivery platform built on the **ICP blockchain** that revolutionizes package delivery by connecting shippers directly with carriers in a trustless, blockchain-powered marketplace. We're transforming underutilized vehicle capacity and commuter routes into efficient delivery networks, offering fair pricing without intermediaries.

---

## 🚀 The Business Opportunity

### Problem We Solve

The current logistics and delivery market faces critical inefficiencies:
- **High Costs for Small Businesses**: Platform fees can absorb up to **30%** of delivery revenue
- **Underutilized Assets**: **20-24%** of EU road freight kilometers are traveled by empty trucks; private urban car occupancy averages only **1.2-1.9** persons per trip
- **Last-Mile Inefficiency**: Last-mile transportation accounts for **40%** of global logistics spending
- **Operational Friction**: Repeat deliveries affect **20-30%** of recipients, increasing costs
- **Lack of Control**: **48%** of consumers prefer specific delivery time slots, but services only offer vague windows

### Our Solution

Apresh provides a decentralized alternative to traditional courier services:
- **Decentralized Network**: Built on **ICP blockchain**, creating a trustless environment with clear audit trails and reduced overhead
- **Available for Everyone**: Anyone who commutes can become a carrier, turning planned trips into delivery opportunities
- **Fair Market Pricing**: Pricing controlled by market demand without hidden fees; shippers choose carriers based on price, rating, and delivery estimates
- **User Autonomy**: Shippers have freedom to make choices about their packages, supported by enhanced communication with independent carriers
- **Smart Contracts**: Handle escrow, ensuring secure payments and cryptographic verification for package handoff

### Market Potential

- **Global CEP Market**: Expected to reach **$632.1B** by 2033
- **European Last-Mile**: Accounts for **39%** of the total parcel market
- **Polish Market Focus**: Serviceable Obtainable Market (SOM) in Poland estimated at **~$4B**; notably, **60%** of recipients are not attached to a specific courier company
- **Gig Economy Growth**: $455B market by 2028 (Mastercard)
- **Blockchain Logistics**: Expected to reach $3.2B by 2030
- **Cost Savings**: 40-60% reduction compared to traditional services

### Revenue Model

- **Transaction Fees**: Base fee on completed shipments to support operational costs and service quality (configurable, currently 0.001 ICP tokens)
- **Value-Added Services**: Custom features and integrations for professional businesses via bounty system
- **Premium Services**: Access to advanced routing algorithms, real-time data analytics, and statistical dashboards
- **B2B Solutions**: Enterprise API access for bulk shipping and e-commerce integration
- **Network Effects**: Platform value increases with each new user

---

## 💼 Key Features

### For Shippers
- 📦 **Create Shipments** with custom parameters (size, value, destination)
- 💰 **Set Your Price** - competitive market-driven pricing
- 🔐 **Secure Escrow** - payment held in smart contract until delivery
- 🔑 **Proof of Delivery** - cryptographic secret verification
- 💬 **Encrypted Communication** - secure messaging with carriers
- 📱 **QR Code Verification** - easy package confirmation

### For Carriers
- 🗺️ **Browse Shipments** - filter by location, size, and compensation
- 🚗 **Optimize Routes** - earn more by matching existing trips
- ⚡ **Instant Payments** - automatic settlement via blockchain
- 📊 **Reputation System** - build trust through completed deliveries
- 💼 **Flexible Work** - choose what to deliver and when

### Platform Benefits
- ⛓️ **Blockchain Security** - immutable records on Internet Computer
- 🔒 **No Custody Risk** - smart contracts handle all funds
- 🌍 **Global Reach** - borderless payments and operations
- 📈 **Scalable** - built on ICP for unlimited throughput
- 🔓 **Open Source** - transparent and auditable code

---

## 🏗️ Technical Architecture

### Technology Stack

#### Blockchain Layer (Internet Computer)
- **Smart Contract**: Rust-based canister handling shipment lifecycle
- **Token Standard**: ICRC-1 ledger for payments and escrow
- **Authentication**: Internet Identity for privacy-preserving auth
- **Storage**: On-chain state management with refund logging
- **Testing**: PocketIC for local integration testing

#### Frontend
- **Framework**: SvelteKit 5 with TypeScript
- **Styling**: Tailwind CSS + Sass
- **Maps**: MapLibre GL for geolocation
- **i18n**: Multi-language support via svelte-i18n
- **Component Library**: Shadcn-like components with Bits UI
- **Development**: Vite, Storybook for component documentation

#### Cryptography & Security
- **End-to-End Encryption**: ChaCha20-Poly1305 for messaging
- **Key Exchange**: X25519 (Curve25519) for secure channels
- **QR Codes**: Custom gradient QR generation with error correction
- **Secret Verification**: SHA-256 hashed delivery secrets
- **WASM Crypto**: Browser-side cryptography via WebAssembly

### System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Frontend Layer                        │
│  ┌──────────────┐                         ┌───────────┐ │
│  │ Web (Svelte) │                         │  Storybook│ │
│  └──────┬───────┘                         └───────────┘ │
└─────────┼───────────────────────────────────────────────┘
          │                  
          └────────┐
                   │ HTTPS / Agent
┌──────────────────┼───────────────────────────────────┐
│    Internet Computer (ICP) Blockchain Layer          │
│  ┌─────────────┬─┴────────────┬──────────────────┐   │
│  │  Internet   │   Contract   │  ICRC-1 Ledger   │   │
│  │  Identity   │   Canister   │   Canister       │   │
│  │  (Auth)     │  (Business)  │  (Payments)      │   │
│  └─────────────┴──────┬───────┴──────────────────┘   │
│                       │                              │
│              ┌────────┴────────┐                     │
│              │  State Machine  │                     │
│              │  - Shipments    │                     │
│              │  - Actors       │                     │
│              │  - Channels     │                     │
│              │  - Refund Log   │                     │
│              └─────────────────┘                     │
└──────────────────────────────────────────────────────┘
```

### Core Components

#### 1. Smart Contract (`/src/contract`)
**Rust canister implementing:**
- Shipment lifecycle management (create → buy → in-transit → finalize)
- ICRC-1 token transfers for escrow and settlement
- Encrypted communication channels between shipper/carrier
- Refund mechanisms and dead token tracking
- Admin controls and whitelist management

**Key Operations:**
```rust
createShipment   // Shipper creates listing with escrow
buyShipment      // Carrier accepts job
add_message      // Encrypted P2P messaging
finalizeShipment // Delivery confirmation & payment release
cancel_shipment  // Cancellation with refund
```

#### 2. Cryptography Crates
- **apresh-crypto**: X25519 key exchange, ChaCha20-Poly1305 encryption
- **apresh-qr**: Custom QR code generation with gradients
- **apresh-wasm**: WebAssembly bindings for browser crypto
- **apresh-derive**: Procedural macros for serialization

#### 3. Engine (`/src/crates/engine`)
**Business logic layer:**
- State machine for shipment status transitions
- Actor system (Shipper/Carrier roles)
- Operation validation and execution
- Channel-based encrypted messaging

#### 4. Frontend Application (`/src/app`)
**Features:**
- Wallet integration with balance management
- Shipment creation wizard with maps
- Marketplace browser with filtering
- QR code scanner for verification
- Real-time encrypted chat
- Multi-language support

### Shipment Flow

```
1. CREATE
   Shipper → Create listing → Approve tokens → Generate secret
   
2. MARKETPLACE
   Carrier → Browse listings → Filter by location/price
   
3. ACCEPT
   Carrier → Buy shipment → Generate channel keys
   
4. PICKUP
   Carrier → Updates status → Encrypted communication begins
   
5. DELIVERY
   Carrier → Scans QR / enters secret → Payment auto-released
   
6. SETTLEMENT
   Smart contract → Transfers escrowed funds → Updates state
```

### Security Model

- **Escrow Protection**: Funds locked in smart contract until delivery proof
- **Zero-Knowledge Secrets**: Shipper generates secret, carrier must provide to finalize
- **Encrypted Channels**: All communication uses X25519 + ChaCha20-Poly1305
- **No Platform Custody**: Platform never controls user funds
- **Refund Guarantees**: Automatic refunds on cancellation or failure
- **Identity Privacy**: Internet Identity provides anonymous authentication

---

## 🛠️ Development

### Prerequisites
- Node.js ≥16.0.0
- Rust (with `wasm32-unknown-unknown` target)
- dfx (Internet Computer SDK)
- Bun (optional, for faster package management)

### Local Development

```bash
# Clone repository
git clone <repository-url>
cd apresh

# Install dependencies
npm install

# Start local IC replica
dfx start --clean

# Deploy canisters (run twice if first fails)
dfx deploy
dfx deploy

# Start development server
npm run dev
```

Frontend: `http://localhost:3000`

### Project Structure

```
apresh/
├── src/
│   ├── contract/          # Rust smart contract
│   │   ├── src/
│   │   │   ├── lib.rs     # Main canister logic
│   │   │   ├── transfer/  # Token operations
│   │   │   └── utils.rs   # Helpers
│   │   └── tests/         # Integration tests
│   │
│   ├── app/               # SvelteKit frontend
│   │   ├── src/
│   │   │   ├── routes/    # Page components
│   │   │   ├── components/# Reusable UI
│   │   │   └── lib/       # Business logic
│   │   └── static/        # Public assets
│   │
│   ├── crates/            # Rust libraries
│   │   ├── apresh-crypto/ # Encryption
│   │   ├── apresh-qr/     # QR generation
│   │   ├── apresh-wasm/   # WASM bindings
│   │   └── engine/        # Core logic
│   │
│   └── declarations/      # Generated IC interfaces
│
├── scripts/               # Deployment helpers
├── dfx.json              # IC configuration
└── package.json          # Dependencies
```

### Available Scripts

```bash
npm run dev         # Start dev server
npm run build       # Build production bundle
npm run test        # Run tests
npm run wasm        # Build WASM packages
npm run storybook   # Component documentation
npm run initial     # Full deployment setup
```

### Testing

```bash
# Contract tests
cd src/contract
cargo test

# WASM tests
npm run test:wasm

# Frontend tests
cd src/app
npm test
```

---

## 🌐 Deployment

### Internet Computer Mainnet

1. **Create production identity**
```bash
dfx identity new production
dfx identity use production
```

2. **Add cycles** (ICP's gas tokens)
```bash
dfx wallet --network ic balance
```

3. **Deploy with mainnet feature**
```bash
./build.sh contract --release --features mainnet
dfx deploy --network ic
```

### Configuration

- **Transfer Fee**: Adjustable via `setTransferFee` (default: 0.0001 ICP)
- **Ledger Canister**: ICRC-1 compatible token
- **Internet Identity**: Privacy-preserving authentication

---

## 🎯 Roadmap

### Phase 1: Kraków Launch (Current)
- ✅ Core shipment lifecycle
- ✅ Escrow and payments
- ✅ Encrypted messaging
- ✅ QR verification
- 🎯 **Target**: 2,000 deliveries
- 🔲 Partnerships with local businesses
- 🔲 Safe incentive program

### Phase 2: Major Polish Cities
- 🎯 **Target**: 15,000 deliveries
- 🔲 Mobile app launch (iOS/Android)
- 🔲 B2B partnerships
- 🔲 E-commerce integration
- 🔲 Route optimization algorithms
- 🔲 Reputation system
- 🔲 Multi-package batching

---

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines.

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing`)
5. Open Pull Request

---

## 📄 License

[Specify your license here]

---

## 💡 Why Internet Computer?

- **True Decentralization**: No cloud providers, fully on-chain
- **Web Speed**: Sub-second finality, 200ms query responses
- **Reverse Gas Model**: Users don't pay transaction fees
- **Infinite Scalability**: Subnet architecture supports unlimited growth
- **Web3 Native**: Host frontends and backends entirely on blockchain

---

## 📞 Contact

**For Business Inquiries:**
- Email: contact@apresh.eu
- LinkedIn: [Company Page] (Coming Soon)

---

<p align="center">
  <strong>Built with ❤️ on the Internet Computer</strong>
</p>
