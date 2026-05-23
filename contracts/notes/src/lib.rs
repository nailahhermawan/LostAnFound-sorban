#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// ============================================================
// DATA STRUCTURES
// ============================================================

/// Status laporan barang
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ItemStatus {
    Open,     // baru dilaporkan, belum ada yang klaim
    Claimed,  // ada yang ngaku / ada yang mau ngambil
    Resolved, // selesai, barang udah balik ke pemilik
}

/// Tipe laporan
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum ItemType {
    Lost,  // laporan kehilangan
    Found, // laporan penemuan
}

/// Lokasi di kampus IPB Dramaga
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum CampusLocation {
    Fasilkom,   // Fakultas Ilmu Komputer
    Faperta,    // Fakultas Pertanian
    Fateta,     // Fakultas Teknologi Pertanian
    Fapet,      // Fakultas Peternakan
    Fpik,       // Fakultas Perikanan
    GedungAnak, // Gedung Anak / Rektorat area
    Perpustakaan,
    Asrama,
    Kantin,
    Other,      // lokasi lain
}

/// Struct utama untuk tiap laporan barang
#[contracttype]
#[derive(Clone, Debug)]
pub struct LostFoundItem {
    pub id: u64,
    pub reporter: String,       // nama/NIM pelapor (karena Soroban Studio ga butuh wallet auth)
    pub item_type: ItemType,    // Lost atau Found
    pub item_name: String,      // nama barang, misal "Laptop Asus silver"
    pub description: String,    // ciri-ciri detail
    pub location: CampusLocation,
    pub status: ItemStatus,
    pub timestamp: u64,         // ledger sequence sebagai penanda waktu
    pub contact: String,        // WA / line pelapor buat dihubungi
}

// Storage key
const ITEM_DATA: Symbol = symbol_short!("ITEM_DATA");

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct LostFoundContract;

#[contractimpl]
impl LostFoundContract {

    // ── READ ─────────────────────────────────────────────────

    /// Ambil semua laporan
    pub fn get_all_items(env: Env) -> Vec<LostFoundItem> {
        env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env))
    }

    /// Ambil laporan berdasarkan ID
    pub fn get_item_by_id(env: Env, id: u64) -> Option<LostFoundItem> {
        let items: Vec<LostFoundItem> = env
            .storage()
            .instance()
            .get(&ITEM_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let item = items.get(i).unwrap();
            if item.id == id {
                return Some(item);
            }
        }
        None
    }

    // ── CREATE ────────────────────────────────────────────────

    /// Buat laporan baru (Lost atau Found)
    pub fn create_item(
        env: Env,
        reporter: String,
        item_type: ItemType,
        item_name: String,
        description: String,
        location: CampusLocation,
        contact: String,
    ) -> String {
        let mut items: Vec<LostFoundItem> = env
            .storage()
            .instance()
            .get(&ITEM_DATA)
            .unwrap_or(Vec::new(&env));

        let item = LostFoundItem {
            id: env.prng().gen::<u64>(),
            reporter,
            item_type,
            item_name,
            description,
            location,
            status: ItemStatus::Open, // default Open saat pertama dibuat
            timestamp: env.ledger().sequence() as u64,
            contact,
        };

        items.push_back(item);
        env.storage().instance().set(&ITEM_DATA, &items);

        String::from_str(&env, "Laporan berhasil dibuat")
    }

    // ── UPDATE ────────────────────────────────────────────────

    /// Update status laporan (Open → Claimed → Resolved)
    pub fn update_status(env: Env, id: u64, new_status: ItemStatus) -> String {
        let mut items: Vec<LostFoundItem> = env
            .storage()
            .instance()
            .get(&ITEM_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let mut item = items.get(i).unwrap();
            if item.id == id {
                item.status = new_status;
                items.set(i, item);
                env.storage().instance().set(&ITEM_DATA, &items);
                return String::from_str(&env, "Status berhasil diupdate");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }

    // ── DELETE ────────────────────────────────────────────────

    /// Hapus laporan berdasarkan ID (misal laporan salah / spam)
    pub fn delete_item(env: Env, id: u64) -> String {
        let mut items: Vec<LostFoundItem> = env
            .storage()
            .instance()
            .get(&ITEM_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            if items.get(i).unwrap().id == id {
                items.remove(i);
                env.storage().instance().set(&ITEM_DATA, &items);
                return String::from_str(&env, "Laporan berhasil dihapus");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }
}

mod test;