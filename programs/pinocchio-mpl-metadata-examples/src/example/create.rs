use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_mpl_metadata::instructions::create::Create;

pub fn create(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // let accounts = [
    //     AccountMeta::writable(self.metadata_account.key()),
    //     AccountMeta::readonly(self.mint.key()),
    //     AccountMeta::readonly_signer(self.mint_authority.key()),
    //     AccountMeta::readonly_signer(self.payer.key()),
    //     AccountMeta::readonly(self.update_authority.key()),
    // ];
    let [metadata_account, mint, mint_authority, payer, update_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // let mut data = vec![0u8];
    // data.extend(self.name.as_bytes());
    // data.extend(self.symbol.as_bytes());
    // data.extend(self.uri.as_bytes());
    // data.extend(&self.seller_fee_basis_points.to_le_bytes());

    let name = unsafe { *(data.as_ptr() as *const u64) };
    let symbol = unsafe { *(data.as_ptr().add(8) as *const u64) };
    let uri = unsafe { *(data.as_ptr().add(16) as *const u64) };
    let seller_fee_basis_points = unsafe { *(data.as_ptr().add(24) as *const u16) };

    let signers = &[];

    Create {
        metadata_account,
        mint,
        mint_authority,
        payer,
        update_authority,
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
        seller_fee_basis_points,
    }
    .invoke_signed(signers)?;

    Ok(())
}
