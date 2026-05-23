# 🎓 IPB Campus Lost & Found Registry

> Sistem pelaporan barang hilang dan temuan berbasis blockchain untuk kampus IPB University, dibangun menggunakan Soroban Smart Contract di jaringan Stellar.

---

## 📖 Deskripsi Projek

IPB Campus Lost & Found Registry adalah dApp (decentralized application) yang memungkinkan mahasiswa IPB melaporkan barang hilang maupun barang temuan secara transparan dan permanen di atas blockchain Stellar. Tidak ada data yang bisa dimanipulasi — setiap laporan tercatat on-chain dan bisa diverifikasi siapapun.

---

## 🔭 Visi Projek

Mewujudkan sistem Lost & Found kampus yang transparan, anti-manipulasi, dan mudah diakses seluruh civitas akademika IPB — menggantikan sistem manual berbasis grup chat atau mading yang tidak terstruktur dengan solusi Web3 yang modern dan terdesentralisasi.

---

## ✨ Fitur

| Fitur | Deskripsi |
|---|---|
| 📋 Lapor Barang Hilang | Buat laporan kehilangan barang beserta deskripsi dan lokasi di kampus IPB |
| 🔍 Lapor Barang Temuan | Mahasiswa yang menemukan barang bisa membuat laporan penemuan |
| 📍 Filter Lokasi Kampus | Lokasi spesifik IPB Dramaga (Fasilkom, Faperta, Fateta, Perpustakaan, Asrama, Kantin, dll) |
| 🔄 Update Status | Status laporan bisa diperbarui: `Open` → `Claimed` → `Resolved` |
| 🗑️ Hapus Laporan | Laporan yang sudah tidak relevan atau sudah selesai bisa dihapus |
| 🔎 Cari by ID | Ambil detail laporan spesifik berdasarkan ID unik |

---

## 🔗 Deployed Smart Contract

| | |
|---|---|
| **Network** | Stellar Testnet (Soroban) |
| **Contract ID** | `CCQ5WINSWNCPYNIEYBG06HMYS74KDXXGSXZZ57GYFCQ6FF43UN7E4Y26` |
| **Explorer** | [Lihat di Stellar Expert](https://stellar.expert/explorer/testnet/contract/CCQ5WINSWNCPYNIEYBG06HMYS74KDXXGSXZZ57GYFCQ6FF43UN7E4Y26) |

---

## 🛠️ Tech Stack

- **Smart Contract** — Rust + Soroban SDK
- **Blockchain** — Stellar Network (Testnet)
- **IDE** — [Soroban Studio](https://soroban.studio)

---

## 📂 Struktur Kontrak

```
src/
├── lib.rs       # Contract utama (CRUD Lost & Found)
└── test.rs      # Unit tests (12 test cases)
```

### Functions

| Function | Tipe | Deskripsi |
|---|---|---|
| `create_item` | Write | Buat laporan baru (Lost/Found) |
| `get_all_items` | Read | Ambil semua laporan |
| `get_item_by_id` | Read | Ambil laporan berdasarkan ID |
| `update_status` | Write | Update status laporan |
| `delete_item` | Write | Hapus laporan |

---

## 🧪 Testing

Tersedia 12 unit test yang mencakup semua operasi CRUD:

```
✅ test_create_lost_item
✅ test_create_found_item
✅ test_create_multiple_items
✅ test_get_all_items_empty
✅ test_get_item_by_id_found
✅ test_get_item_by_id_not_found
✅ test_update_status_to_claimed
✅ test_update_status_to_resolved
✅ test_update_status_item_not_found
✅ test_delete_item
✅ test_delete_item_not_found
✅ test_delete_one_of_many
```

---

## 📄 License

MITtal age by:

- **Decentralizing Data**: Moving note-taking from centralized servers to a global, distributed blockchain
- **Ensuring Ownership**: Empowering users to have complete control and ownership over their digital thoughts and information
- **Guaranteeing Immutability**: Providing a permanent, tamper-proof record of notes that cannot be altered or deleted by third parties
- **Enhancing Privacy**: Leveraging blockchain security to protect personal information from unauthorized access
- **Building Trustless Systems**: Creating a platform where data integrity is guaranteed by code, not by company promises

We envision a future where digital information is truly personal and sovereign, empowering individuals with complete autonomy over their digital assets.

## Key Features

### 1. **Simple Note Creation**

- Create notes with just one function call
- Specify title and content for each note
- Automated ID generation for unique identification
- Persistent storage on the Stellar blockchain

### 2. **Efficient Data Retrieval**

- Fetch all stored notes in a single call
- Structured data representation for easy frontend integration
- Quick access to your entire note collection
- Real-time synchronization with the blockchain state

### 3. **Secure Deletion**

- Remove specific notes using their unique IDs
- Permanent removal from the contract storage
- Clean and efficient storage management
- Immediate update of the note list after deletion

### 4. **Transparency and Security**

- View all note activities on the blockchain
- Blockchain-based verification of all storage actions
- Immutable records of note creation and deletion
- Protected against unauthorized modifications

### 5. **Stellar Network Integration**

- Leverages the high speed and low cost of Stellar
- Built using the modern Soroban Smart Contract SDK
- Scalable architecture for growing note collections
- Interoperable with other Stellar-based services

## Contract Details

- Contract Address: CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M
  (Screenshot has been removed)

## Future Scope

### Short-Term Enhancements

1. **Note Encryption**: Support for end-to-end encryption of note content for enhanced privacy
2. **Category Management**: Add tags and categories to organize notes efficiently
3. **Rich Text Support**: Extend support beyond plain text to include Markdown and formatted content
4. **Search Functionality**: Implement advanced search filters for large note collections

### Medium-Term Development

5. **Collaborative Notes**: Implement multi-signature requirements for shared or collaborative note-taking
   - Shared access for multiple addresses
   - Permission-based editing and viewing
   - Version history tracking
6. **Notification System**: Off-chain bridge to alert users of new updates or shared notes
7. **Asset Attachment**: Capability to attach digital assets or tokens to specific notes
8. **Inter-Contract Integration**: Allow other smart contracts to interact with and store data in the notes contract

### Long-Term Vision

9. **Cross-Chain Synchronization**: Extend note storage to multiple blockchain networks
10. **Decentralized UI Hosting**: Host the frontend on IPFS or similar decentralized platforms
11. **AI-Powered Summarization**: Optional integration with AI to help users summarize their notes
12. **Privacy Layers**: Implement zero-knowledge proofs for completely private note content
13. **DAO Governance**: Community-driven protocol improvements and feature prioritization
14. **Identity Management**: Integration with decentralized identity (DID) systems for user management

### Enterprise Features

15. **Corporate Documentation**: Adapt the system for secure corporate record-keeping
16. **Immutable Logging**: Create time-locked logs for audit purposes
17. **Automated Reporting**: Automatic note triggers for periodic reporting
18. **Multi-Language Support**: Expand accessibility with internationalization

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the three main functions:

- `create_note()` - Create a new note with a title and content
- `get_notes()` - Retrieve all stored notes from the contract
- `delete_note()` - Remove a specific note by its ID

---

**Stellar Notes DApp** - Securing Your Thoughts on the Blockchain
