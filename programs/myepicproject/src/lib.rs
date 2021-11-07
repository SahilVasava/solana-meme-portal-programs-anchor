use anchor_lang::prelude::*;
use std::convert::TryInto;

declare_id!("8ASSD5uBFtyzxbnuF9pt3SUiEBju9nBXhzKaoZsxcLJC");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            upvotes: 0,
            downvotes: 0,
            upvoters: Vec::new(),
            downvoters: Vec::new(),
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn upvote_gif(ctx: Context<UpdateGif>, item_index: u64) -> ProgramResult {
        let item_index_usize: usize = item_index.try_into().unwrap();
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        if !base_account.gif_list[item_index_usize].upvoters.iter().any(|&i| i==*user.to_account_info().key) {
            base_account.gif_list[item_index_usize].upvotes += 1;
            base_account.gif_list[item_index_usize].upvoters.push(*user.to_account_info().key);
            if base_account.gif_list[item_index_usize].downvoters.iter().any(|&i| i==*user.to_account_info().key) {
                base_account.gif_list[item_index_usize].downvotes -= 1;
                let index = base_account.gif_list[item_index_usize].downvoters.iter().position(|x| *x == *user.to_account_info().key).unwrap();
                base_account.gif_list[item_index_usize].downvoters.remove(index);
            }

        } else {
                base_account.gif_list[item_index_usize].upvotes -= 1;
                let index = base_account.gif_list[item_index_usize].upvoters.iter().position(|x| *x == *user.to_account_info().key).unwrap();
                base_account.gif_list[item_index_usize].upvoters.remove(index);
        }
        Ok(())

    }

    pub fn downvote_gif(ctx: Context<UpdateGif>, item_index: u64) -> ProgramResult {
        let item_index_usize: usize = item_index.try_into().unwrap();
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        if !base_account.gif_list[item_index_usize].downvoters.iter().any(|&i| i==*user.to_account_info().key) {
            base_account.gif_list[item_index_usize].downvotes += 1;
            base_account.gif_list[item_index_usize].downvoters.push(*user.to_account_info().key);
            if base_account.gif_list[item_index_usize].upvoters.iter().any(|&i| i==*user.to_account_info().key) {
                base_account.gif_list[item_index_usize].upvotes -= 1;
                let index = base_account.gif_list[item_index_usize].upvoters.iter().position(|x| *x == *user.to_account_info().key).unwrap();
                base_account.gif_list[item_index_usize].upvoters.remove(index);
            }
        } else {
                base_account.gif_list[item_index_usize].downvotes -= 1;
                let index = base_account.gif_list[item_index_usize].downvoters.iter().position(|x| *x == *user.to_account_info().key).unwrap();
                base_account.gif_list[item_index_usize].downvoters.remove(index);
        }

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
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub upvotes: u64,
    pub downvotes: u64,
    pub upvoters: Vec<Pubkey>,
    pub downvoters: Vec<Pubkey>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
