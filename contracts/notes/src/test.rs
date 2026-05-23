#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Ledger, Env};

/// Helper: buat env fresh + deploy contract
fn setup() -> (Env, LostFoundContractClient<'static>) {
    let env = Env::default();
    let contract_id = env.register_contract(None, LostFoundContract);
    let client = LostFoundContractClient::new(&env, &contract_id);
    (env, client)
}

/// Helper: bikin String soroban dari &str
fn s(env: &Env, val: &str) -> soroban_sdk::String {
    soroban_sdk::String::from_str(env, val)
}

// ============================================================
// CREATE TESTS
// ============================================================

#[test]
fn test_create_lost_item() {
    let (env, client) = setup();

    let result = client.create_item(
        &s(&env, "Budi / J0101"),
        &ItemType::Lost,
        &s(&env, "Laptop Asus VivoBook silver"),
        &s(&env, "Stiker IPB di cover belakang, charger putih"),
        &CampusLocation::Fasilkom,
        &s(&env, "08123456789"),
    );

    assert_eq!(result, s(&env, "Laporan berhasil dibuat"));
}

#[test]
fn test_create_found_item() {
    let (env, client) = setup();

    let result = client.create_item(
        &s(&env, "Siti / G0202"),
        &ItemType::Found,
        &s(&env, "Dompet coklat"),
        &s(&env, "Berisi KTM dan uang cash, nemu di depan perpus"),
        &CampusLocation::Perpustakaan,
        &s(&env, "08987654321"),
    );

    assert_eq!(result, s(&env, "Laporan berhasil dibuat"));
}

#[test]
fn test_create_multiple_items() {
    let (env, client) = setup();

    client.create_item(
        &s(&env, "Andi"),
        &ItemType::Lost,
        &s(&env, "Kunci motor Honda"),
        &s(&env, "Gantungan kunci warna merah"),
        &CampusLocation::Kantin,
        &s(&env, "081111"),
    );

    client.create_item(
        &s(&env, "Rina"),
        &ItemType::Found,
        &s(&env, "Tas ransel hitam"),
        &s(&env, "Merk Eiger, ada inisial 'RD' di dalam"),
        &CampusLocation::Asrama,
        &s(&env, "082222"),
    );

    let items = client.get_all_items();
    assert_eq!(items.len(), 2);
}

// ============================================================
// READ TESTS
// ============================================================

#[test]
fn test_get_all_items_empty() {
    let (env, client) = setup();
    let items = client.get_all_items();
    assert_eq!(items.len(), 0);
}

#[test]
fn test_get_item_by_id_found() {
    let (env, client) = setup();

    // create dulu
    client.create_item(
        &s(&env, "Doni"),
        &ItemType::Lost,
        &s(&env, "Powerbank Xiaomi"),
        &s(&env, "Warna hitam 20000mAh"),
        &CampusLocation::Fateta,
        &s(&env, "083333"),
    );

    // ambil semua, grab id dari item pertama
    let items = client.get_all_items();
    let id = items.get(0).unwrap().id;

    // get by id
    let result = client.get_item_by_id(&id);
    assert!(result.is_some());
    assert_eq!(result.unwrap().item_name, s(&env, "Powerbank Xiaomi"));
}

#[test]
fn test_get_item_by_id_not_found() {
    let (env, client) = setup();

    // id random yang ga ada
    let result = client.get_item_by_id(&999999u64);
    assert!(result.is_none());
}

// ============================================================
// UPDATE TESTS
// ============================================================

#[test]
fn test_update_status_to_claimed() {
    let (env, client) = setup();

    client.create_item(
        &s(&env, "Fara"),
        &ItemType::Lost,
        &s(&env, "Earphone JBL"),
        &s(&env, "Warna putih, case hitam"),
        &CampusLocation::GedungAnak,
        &s(&env, "084444"),
    );

    let items = client.get_all_items();
    let id = items.get(0).unwrap().id;

    // update Open → Claimed
    let result = client.update_status(&id, &ItemStatus::Claimed);
    assert_eq!(result, s(&env, "Status berhasil diupdate"));

    // verify status berubah
    let updated = client.get_item_by_id(&id).unwrap();
    assert_eq!(updated.status, ItemStatus::Claimed);
}

#[test]
fn test_update_status_to_resolved() {
    let (env, client) = setup();

    client.create_item(
        &s(&env, "Hendra"),
        &ItemType::Found,
        &s(&env, "KTM IPB"),
        &s(&env, "Atas nama mahasiswa Fapet"),
        &CampusLocation::Fapet,
        &s(&env, "085555"),
    );

    let items = client.get_all_items();
    let id = items.get(0).unwrap().id;

    // langsung Resolved
    client.update_status(&id, &ItemStatus::Resolved);
    let updated = client.get_item_by_id(&id).unwrap();
    assert_eq!(updated.status, ItemStatus::Resolved);
}

#[test]
fn test_update_status_item_not_found() {
    let (env, client) = setup();

    let result = client.update_status(&999999u64, &ItemStatus::Claimed);
    assert_eq!(result, s(&env, "Item tidak ditemukan"));
}

// ============================================================
// DELETE TESTS
// ============================================================

#[test]
fn test_delete_item() {
    let (env, client) = setup();

    client.create_item(
        &s(&env, "Reza"),
        &ItemType::Lost,
        &s(&env, "Jaket Hoodie abu"),
        &s(&env, "Ada tulisan 'IPB' di dada kiri"),
        &CampusLocation::Fpik,
        &s(&env, "086666"),
    );

    let items = client.get_all_items();
    let id = items.get(0).unwrap().id;

    let result = client.delete_item(&id);
    assert_eq!(result, s(&env, "Laporan berhasil dihapus"));

    // pastiin udah bener-bener ke-delete
    let items_after = client.get_all_items();
    assert_eq!(items_after.len(), 0);
}

#[test]
fn test_delete_item_not_found() {
    let (env, client) = setup();

    let result = client.delete_item(&999999u64);
    assert_eq!(result, s(&env, "Item tidak ditemukan"));
}

#[test]
fn test_delete_one_of_many() {
    let (env, client) = setup();

    // tambah 3 item
    client.create_item(&s(&env, "A"), &ItemType::Lost, &s(&env, "Item 1"),
        &s(&env, "-"), &CampusLocation::Other, &s(&env, "081"));
    client.create_item(&s(&env, "B"), &ItemType::Found, &s(&env, "Item 2"),
        &s(&env, "-"), &CampusLocation::Other, &s(&env, "082"));
    client.create_item(&s(&env, "C"), &ItemType::Lost, &s(&env, "Item 3"),
        &s(&env, "-"), &CampusLocation::Other, &s(&env, "083"));

    // hapus item ke-2
    let id_to_delete = client.get_all_items().get(1).unwrap().id;
    client.delete_item(&id_to_delete);

    // sisa harus 2
    assert_eq!(client.get_all_items().len(), 2);
}

// ============================================================
// FULL FLOW TEST
// ============================================================

#[test]
fn test_full_flow_lost_and_found() {
    let (env, client) = setup();

    // 1. Budi lapor kehilangan laptop
    client.create_item(
        &s(&env, "Budi / J0101"),
        &ItemType::Lost,
        &s(&env, "Laptop Lenovo ThinkPad"),
        &s(&env, "Warna hitam, stiker IPB, charger kotak"),
        &CampusLocation::Fasilkom,
        &s(&env, "08123456789"),
    );

    // 2. Siti nemu laptop, bikin laporan Found
    client.create_item(
        &s(&env, "Siti / H0303"),
        &ItemType::Found,
        &s(&env, "Laptop Lenovo ThinkPad"),
        &s(&env, "Warna hitam, ada stiker IPB, nemu di koridor Fasilkom"),
        &CampusLocation::Fasilkom,
        &s(&env, "08987654321"),
    );

    assert_eq!(client.get_all_items().len(), 2);

    // 3. Budi klaim laporan Siti
    let found_id = client.get_all_items().get(1).unwrap().id;
    client.update_status(&found_id, &ItemStatus::Claimed);

    // 4. Barang udah dikembaliin → Resolved
    client.update_status(&found_id, &ItemStatus::Resolved);

    let final_item = client.get_item_by_id(&found_id).unwrap();
    assert_eq!(final_item.status, ItemStatus::Resolved);

    // 5. Laporan Budi (Lost) bisa dihapus karena udah ketemu
    let lost_id = client.get_all_items().get(0).unwrap().id;
    client.delete_item(&lost_id);

    assert_eq!(client.get_all_items().len(), 1);
}