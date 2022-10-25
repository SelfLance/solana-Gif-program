use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("3iYPHRpMGesfBiYLS7pBWnhsXoywdYhMKh7ys5x9cLtU");

#[program]
pub mod gifportal {
    use super::*;
    // Array List of Gif is added here and thier.....

   pub fn start_stuf_off(ctx: Context<StartStuffOff>) -> ProgramResult{
    let base_account  = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
   }

   // Add Gif On Blockchain 
   pub fn add_gif(ctx:Context<AddGif>, gif_link: String) -> ProgramResult{
    let base_account  = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;
    let item = ItemStruct{
        gif_link:gif_link.to_string(),
        user_address: *user.to_account_info().key,
    };
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
   }

   // Comment on Picture and Rating for that Gif
   pub fn comment_gif(ctx: Context<AddComments>,feedback: String, rating: u64,gif_link: String) ->ProgramResult{
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;
    let feedbacks = Feedback{
        gif_comment: feedback,
        rating: rating,
        user_address: *user.to_account_info().key,
    };
    base_account.gif_feedback.push(feedbacks);
    base_account.total_comments += 1;
    Ok(())
   }

   pub fn send_tips()
}

#[derive(Accounts)]
pub struct StartStuffOff<'info>{  
    #[account(init, payer= user , space = 9000)] 
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info,System>

}

#[derive(Accounts)]
pub struct AddGif<'info>{
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user:Signer<'info>
}

#[derive(Accounts)]
pub struct AddComments<'info>{
    #[account(mut)]
    pub base_account: Account<'info, CommentAccount>,
    #[account(mut)]
    pub user:Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct{
    pub gif_link: String,
    pub user_address: Pubkey,

}

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Feedback{
    pub gif_comment: String,
    pub rating: u64,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount{
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>
}

#[account]
pub struct CommentAccount{
    pub total_comments: u64,
    pub gif_feedback: Vec<Feedback>
}


