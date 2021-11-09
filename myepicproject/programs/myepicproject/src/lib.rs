use anchor_lang::prelude::*;

declare_id!("71NEC9VPM8jYGW8k1JEgDgKbLcL6w9e6uquDs7R18b4P");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    // Get a reference to the account and increment total_gifs.
    let base_account = &mut ctx.accounts.base_account;

    // Build the struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *base_account.to_account_info().key,
      votes: 0,
    };
		
	// Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())

  }


  pub fn update_item(ctx: Context<UpdateItem>, gif_link: String) -> ProgramResult {
      let base_account = &mut ctx.accounts.base_account;
      let item = base_account.gif_list.iter().find(|s| s.gif_link == gif_link);
      println!("{}",gif_link);
      println!("{}", item);
      item.votes +=1;
      Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct UpdateItem<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: u64,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}