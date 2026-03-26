#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Address, Env};

// --- DICCIONARIO DE AMENAZAS (Threat Model) ---
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PoolError {
    NotAuthorized = 1,
    InsufficientFunds = 2,
    NegativeAmount = 3, 
    MathOverflow = 4, 
}

#[contract]
pub struct KuyfiCore;

#[contractimpl]
impl KuyfiCore {
    
    // --- FUNCIÓN DE DEPÓSITO ASEGURADA ---
    pub fn deposit(env: Env, from: Address, amount: i128) -> Result<(), PoolError> {
        from.require_auth(); 
        
        if amount <= 0 {
            return Err(PoolError::NegativeAmount);
        }

        let current_balance: i128 = env.storage().persistent().get(&from).unwrap_or(0);
        let new_balance = current_balance.checked_add(amount).ok_or(PoolError::MathOverflow)?;
        env.storage().persistent().set(&from, &new_balance);
        
        // NUEVO: Grito criptográfico hacia el exterior (Para tu monitor en Python)
        env.events().publish((symbol_short!("deposit"), from), amount);
        
        Ok(()) 
    }

    // --- FUNCIÓN DE RETIRO ASEGURADA ---
    pub fn withdraw(env: Env, to: Address, amount: i128) -> Result<(), PoolError> {
        to.require_auth();
        
        if amount <= 0 {
            return Err(PoolError::NegativeAmount);
        }
        
        let current_balance: i128 = env.storage().persistent().get(&to).unwrap_or(0);
        
        if amount > current_balance {
            return Err(PoolError::InsufficientFunds);
        }

        let new_balance = current_balance.checked_sub(amount).ok_or(PoolError::MathOverflow)?;
        env.storage().persistent().set(&to, &new_balance);

        // NUEVO: Grito criptográfico hacia el exterior (Para tu monitor en Python)
        env.events().publish((symbol_short!("withdraw"), to), amount);

        Ok(())
    }
}