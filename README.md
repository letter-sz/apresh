<div align="center"><h1>🌍 Apresh - Decentralized Delivery Network</h1></div>

<br/>

Apresh is a decentralized delivery platform built on **Internet Computer Protocol (ICP)** — a next-generation distributed cloud infrastructure — that revolutionizes package delivery by connecting shippers directly with carriers in a secure, peer-to-peer marketplace. We're transforming underutilized vehicle capacity and commuter routes into efficient delivery networks, offering fair pricing without intermediaries.

<br/>

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
- **Distributed Infrastructure**: Built on **Internet Computer Protocol**, creating a transparent environment with verifiable audit trails and minimal overhead
- **Available for Everyone**: Anyone who commutes can become a carrier, turning planned trips into delivery opportunities
- **Fair Market Pricing**: Pricing controlled by market demand without hidden fees; shippers choose carriers based on price, rating, and delivery estimates
- **User Autonomy**: Shippers have freedom to make choices about their packages, supported by enhanced communication with independent carriers
- **Automated Escrow**: Programmable contracts handle secure payments and cryptographic verification for package handoff

### Market Potential

- **Global CEP Market**: Expected to reach **$632.1B** by 2033
- **European Last-Mile**: Accounts for **39%** of the total parcel market
- **Polish Market Focus**: Serviceable Obtainable Market (SOM) in Poland estimated at **~$4B**; notably, **60%** of recipients are not attached to a specific courier company
- **Gig Economy Growth**: $455B market by 2028 (Mastercard)
- **Decentralized Logistics**: Emerging market expected to reach $3.2B by 2030
- **Cost Savings**: 40-60% reduction compared to traditional services

### Revenue Model

- **Transaction Fees**: Small fee on completed shipments to support operational costs and service quality (typically less than 1% of delivery value)
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
- 🔐 **Tamper-Proof Records** - immutable transaction history on distributed infrastructure
- 🔒 **No Custody Risk** - automated escrow system, platform never holds funds
- 🌍 **Global Reach** - borderless payments and operations
- 📈 **Infinitely Scalable** - built on distributed cloud architecture for unlimited throughput
- 🔓 **Open Source** - transparent and auditable code

---

## 🏗️ Technical Architecture

### Technology Stack

#### Backend Infrastructure (Internet Computer)
- **Application Logic**: Rust-based canister handling shipment lifecycle
- **Payment System**: ICRC-1 digital ledger for payments and escrow
- **Authentication**: Internet Identity for privacy-preserving auth
- **Data Storage**: Distributed state management with transaction logging
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
│    Internet Computer (ICP) - Distributed Cloud       │
│  ┌─────────────┬─┴────────────┬──────────────────┐   │
│  │  Internet   │  Application │  Payment Ledger  │   │
│  │  Identity   │   Canister   │   Canister       │   │
│  │  (Auth)     │  (Business)  │  (Settlements)   │   │
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

#### 1. Application Canister (`/src/contract`)
**Rust-based business logic implementing:**
- Shipment lifecycle management (create → buy → in-transit → finalize)
- ICRC-1 payment transfers for escrow and settlement
- Encrypted communication channels between shipper/carrier
- Refund mechanisms and transaction tracking
- Admin controls and access management

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

- **Escrow Protection**: Funds locked in programmable escrow until delivery proof
- **Zero-Knowledge Secrets**: Shipper generates secret, carrier must provide to finalize
- **Encrypted Channels**: All communication uses X25519 + ChaCha20-Poly1305
- **No Platform Custody**: Platform never controls user funds
- **Automatic Refunds**: Guaranteed refunds on cancellation or failure
- **Privacy-First**: Internet Identity provides anonymous authentication

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

- **Transfer Fee**: Adjustable via `setTransferFee` (typically < 1% of transaction value)
- **Payment Ledger**: ICRC-1 compatible digital ledger
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

- **Decentralized Cloud Infrastructure**: No dependency on centralized cloud providers (AWS, Azure, Google Cloud)
- **Enterprise Performance**: Sub-second response times with 200ms query latency
- **Zero User Fees**: Infrastructure costs covered by developers, not users
- **Unlimited Scalability**: Distributed subnet architecture supports unlimited growth
- **Full-Stack Platform**: Host frontends, backends, and data storage on a single decentralized infrastructure

---

## 📞 Contact

**For Business Inquiries:**

- Email: contact@apresh.eu
- LinkedIn: [Company Page] (Coming Soon)

---

<p align="center">
  <strong>Built with ❤️ on the Internet Computer</strong>
</p>
