use anchor_lang::prelude::*;
use layerzero::prelude::*; // Import LayerZero SDK

declare_id!("CcGjSmj2ivNWcd6Q1J8Sew7fLPXNkvGQnXgKSgo5wFxz"); // Ganti dengan pubkey deploy jika perlu

#[program]
pub mod my_oapp {
    use super::*;

    pub fn lz_receive(
        ctx: Context<Receive>,
        payload: Vec<u8>,
    ) -> Result<()> {
        let msg = String::from_utf8(payload.clone()).unwrap_or("Invalid UTF-8".to_string());
        msg!("📡 Received cross-chain payload: {}", msg);

        Ok(())
    }
}

// Context untuk menerima pesan LayerZero
#[derive(Accounts)]
pub struct Receive<'info> {
    #[account(mut)]
    pub oapp: Account<'info, OApp>, // Akun program OApp
    pub owner: Signer<'info>,       // Pemilik
}
