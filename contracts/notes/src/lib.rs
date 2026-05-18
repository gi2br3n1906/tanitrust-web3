#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, token, Symbol};

// 1. Struktur Data yang lebih lengkap
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarvestBatch {
    pub farmer: Address,
    pub logistics: Address, 
    pub available_sacks: u32,
    pub price_per_sack: i128,
    pub token_address: Address, 
    pub is_active: bool,
}

#[contract]
pub struct TaniTrustContract;

#[contractimpl]
impl TaniTrustContract {
    
    // 2. Tambahkan Panen + Tentukan Kurirnya
    pub fn add_harvest(
        env: Env, 
        harvest_id: u64, 
        farmer: Address, 
        logistics: Address,
        total_weight_kg: u32, 
        price_per_sack: i128,
        token_address: Address
    ) {
        farmer.require_auth();
        let available_sacks = total_weight_kg / 50;
        
        let batch = HarvestBatch {
            farmer: farmer.clone(),
            logistics,
            available_sacks,
            price_per_sack,
            token_address,
            is_active: true,
        };
        
        env.storage().persistent().set(&harvest_id, &batch);

        // EVENT: Kasih tahu dunia ada panen baru
        env.events().publish((symbol_short!("harvest"), harvest_id), farmer);
    }

    pub fn buy_sacks(env: Env, buyer: Address, harvest_id: u64, amount_of_sacks: u32) {
        buyer.require_auth();
        let mut batch: HarvestBatch = env.storage().persistent().get(&harvest_id).expect("Not Found");
        
        assert!(batch.is_active, "Closed");
        assert!(batch.available_sacks >= amount_of_sacks, "No Stock");

        let total_price = batch.price_per_sack * (amount_of_sacks as i128);

        // LOCK FUNDS: Uang pindah dari Pembeli ke Contract (Escrow)
        let token_client = token::Client::new(&env, &batch.token_address);
        token_client.transfer(&buyer, &env.current_contract_address(), &total_price);

        batch.available_sacks -= amount_of_sacks;
        if batch.available_sacks == 0 { batch.is_active = false; }

        env.storage().persistent().set(&harvest_id, &batch);

        // EVENT: Pembelian berhasil
        env.events().publish((symbol_short!("buy"), harvest_id), buyer);
    }

    // 3. FITUR JAGOAN: Konfirmasi & Bagi Duit Otomatis
    pub fn confirm_delivery(env: Env, buyer: Address, harvest_id: u64, amount_bought: u32) {
        buyer.require_auth();
        let batch: HarvestBatch = env.storage().persistent().get(&harvest_id).expect("Not Found");

        let total_held = batch.price_per_sack * (amount_bought as i128);
        
        // Kalkulasi Bagi Hasil: 90% Petani, 10% Logistik
        let farmer_share = (total_held * 90) / 100;
        let logistics_share = total_held - farmer_share;

        let token_client = token::Client::new(&env, &batch.token_address);
        
        // Transfer dari Contract ke Petani
        token_client.transfer(&env.current_contract_address(), &batch.farmer, &farmer_share);
        
        // Transfer dari Contract ke Logistik
        token_client.transfer(&env.current_contract_address(), &batch.logistics, &logistics_share);

        // EVENT: Transaksi Selesai Total
        env.events().publish((symbol_short!("release"), harvest_id), (batch.farmer, batch.logistics));
    }
    
    pub fn get_harvest(env: Env, harvest_id: u64) -> HarvestBatch {
        env.storage().persistent().get(&harvest_id).expect("Not Found")
    }
}