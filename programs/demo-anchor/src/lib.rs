use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Mint}, associated_token::AssociatedToken};
use mpl_token_metadata::types::DataV2;
// use mpl_token_metadata::instructions ::{CreateMetadataAccountV3};

declare_id!("HHNX8M8WDW5PtAGtfAiTqQwaXas3DsREXk7ZDkS9JMBZ"); ///put the program id here
#[program]
pub mod anchor_spl_token {
    use anchor_lang::system_program;
    use anchor_spl::{token::{initialize_mint, InitializeMint, mint_to, MintTo, transfer, Transfer, burn, Burn, freeze_account, FreezeAccount, close_account, CloseAccount, thaw_account, ThawAccount, set_authority, SetAuthority, spl_token::instruction::AuthorityType}, associated_token, metadata::{create_metadata_accounts_v3, create_master_edition_v3}};
    // use mpl_token_metadata::instruction::CreateMasterEdition;
    

    use super::*;
    // CreateToken: is the structure defined below for create a new account and change it to mint account using 'initialize_mint' function.
    pub fn create_token(ctx: Context<CreateToken>,decimals:u8,amount:u64) -> Result<()> {
        // let amount_free_rent = Rent::get()?.minimum_balance(space as usize);
        
        system_program::create_account( // Create a new account by interacting to system program. This example new account is mint_token account.
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(), 
                system_program::CreateAccount { from: ctx.accounts.signer.to_account_info(), to: ctx.accounts.mint_token.to_account_info() }
            ), 
            10_000_000, // -- The amount of lamports to transfer to the new account. It passed 'amount_free_rent' here for free rent
            82,  // -- The amount of space in bytes to allocate new account.
            ctx.accounts.token_program.key // The program ID that will own the new account.
        )?;

        initialize_mint( // store metadata(not include: Name, sym, or uri...) of token into this. it called Mint account. It don't store amount token
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint{mint:ctx.accounts.mint_token.to_account_info(),rent:ctx.accounts.rent.to_account_info()}
            ), 
            decimals, 
            ctx.accounts.signer.key, 
            Some(ctx.accounts.signer.key)
        )?;


        associated_token::create( // it is Token account
            CpiContext::new(
                ctx.accounts.associate_token_program.to_account_info(), 
                associated_token::Create { 
                    payer: ctx.accounts.signer.to_account_info(), 
                    associated_token: ctx.accounts.token_account.to_account_info(), 
                    authority: ctx.accounts.signer.to_account_info(), 
                    mint: ctx.accounts.mint_token.to_account_info(), 
                    system_program: ctx.accounts.system_program.to_account_info(), 
                    token_program: ctx.accounts.token_program.to_account_info() 
                }
            )
        )?;

        mint_to(
            CpiContext::new(
                ctx.accounts.token_account.to_account_info(), 
                MintTo{authority:ctx.accounts.signer.to_account_info(),mint:ctx.accounts.mint_token.to_account_info(),to:ctx.accounts.token_account.to_account_info()}
            ), 
            amount
        )?;

        Ok(())
    }
     // note: Authority is difference Own of Account.
    pub fn transer_token(ctx: Context<TransferToken>,amount:u64)->Result<()>{

        msg!("Started {:} tokens transfer from account {:} to {:}",amount,ctx.accounts.from_account.key(),ctx.accounts.to_account.key());

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                Transfer{authority:ctx.accounts.signer.to_account_info(),from:ctx.accounts.from_account.to_account_info(),to:ctx.accounts.to_account.to_account_info()}
            ), 
            amount
        )?;

        Ok(())
    }
   //it's only set Mint and FreezeToken authority on Mint Token account.
    pub fn set_authority_token(ctx: Context<SetAuthorityToken>,authority_value:u8)->Result<()>{
        let account_or_mint;
        let authority_type;
        match authority_value {
            0=> {
                authority_type = anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens;
                account_or_mint=ctx.accounts.mint_token.to_account_info();
            },
            1=> {
                authority_type = anchor_spl::token::spl_token::instruction::AuthorityType::FreezeAccount;
                account_or_mint=ctx.accounts.mint_token.to_account_info();
            },
            2 => {
                authority_type = anchor_spl::token::spl_token::instruction::AuthorityType::AccountOwner;
                account_or_mint = ctx.accounts.token_account.to_account_info();
            },
            _ => {
                authority_type = anchor_spl::token::spl_token::instruction::AuthorityType::CloseAccount;
                account_or_mint = ctx.accounts.token_account.to_account_info();
            }
        }
        set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                SetAuthority{
                    account_or_mint:account_or_mint,
                    current_authority:ctx.accounts.signer.to_account_info()
                }
            ), 
            authority_type.clone(), 
            Some(ctx.accounts.new_signer.key())
        )?;

        Ok(())
    }

    pub fn burn_token(ctx: Context<BurnToken>,amount:u64)->Result<()>{
        burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                Burn{
                    authority:ctx.accounts.signer.to_account_info(),
                    from:ctx.accounts.token_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info()
                }
            ), 
            amount
        )?;
        Ok(())
    }

    pub fn freeze_token(ctx: Context<FreezeToken>)->Result<()>{
        
        freeze_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                FreezeAccount{
                    account:ctx.accounts.token_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    authority:ctx.accounts.signer.to_account_info(),
                }
            )
        )?;


        Ok(())
    }

    pub fn un_freeze_token(ctx: Context<FreezeToken>)->Result<()>{
        
        thaw_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                ThawAccount{
                    account:ctx.accounts.token_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    authority:ctx.accounts.signer.to_account_info(),
                }
            )
        )?;


        Ok(())
    }

    pub fn close_token(ctx: Context<CloseToken>)->Result<()>{
        
        close_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                CloseAccount{
                    account:ctx.accounts.token_account.to_account_info(),
                    destination:ctx.accounts.signer.to_account_info(),
                    authority:ctx.accounts.signer.to_account_info(),
                }
            )
        )?;


        Ok(())
    }
    //use for NFTs
    pub fn set_token_metadata(ctx: Context<CreateMetadata>, data:MetadataData)->Result<()>{
        // get Metadata Account from Token Mint account.
        /* let seeds = &[
                b"metadata".as_ref(),
                token_metadata_program_id.as_ref(),
                mint_address.as_ref(),
            ];
            let (metadata_address, _) = Pubkey::find_program_address(seeds, token_metadata_program_id); */

        let (metadata_address,b1) = Pubkey::find_program_address(&[
            b"metadata", // it's default of Megadat Program.
            &ctx.accounts.metadata_program.key.to_bytes(),
            &ctx.accounts.mint_token.key().to_bytes()
            ], 
            ctx.accounts.metadata_program.key
        );

        let metadata_account = &ctx.accounts.metadata_account;
        let master_account = &ctx.accounts.master_account;

        if metadata_address != *metadata_account.key{
            return err!(ProgramErrors::PdaNotMatched)
        }
        
        let (master_address,b2) = Pubkey::find_program_address(&[
            b"metadata", 
            &ctx.accounts.metadata_program.key.to_bytes(),
            &ctx.accounts.mint_token.key().to_bytes(),
            b"edition"
            ], 
            ctx.accounts.metadata_program.key
        );

        if master_address != *master_account.key{
            return err!(ProgramErrors::PdaNotMatched)
        }

        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.metadata_program.to_account_info(), 
                anchor_spl::metadata::CreateMetadataAccountsV3{
                    metadata:metadata_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    mint_authority:ctx.accounts.signer.to_account_info(),
                    update_authority:ctx.accounts.signer.to_account_info(),
                    payer:ctx.accounts.signer.to_account_info(),
                    system_program:ctx.accounts.system_program.to_account_info(),
                    rent:ctx.accounts.rent.to_account_info(),
                }
            ), 
            DataV2 { 
                name: data.name, 
                symbol: data.symbol, 
                uri: data.uri, 
                seller_fee_basis_points: data.seller_fee_basis_points,
                creators: None, 
                collection: None, 
                uses: None 
            },
            true, 
            true, 
            None
        )?;

       
        create_master_edition_v3(
            CpiContext::new(
                ctx.accounts.metadata_program.to_account_info(), 
                anchor_spl::metadata::CreateMasterEditionV3 {
                    metadata:metadata_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    mint_authority:ctx.accounts.signer.to_account_info(),
                    update_authority:ctx.accounts.signer.to_account_info(),
                    payer:ctx.accounts.signer.to_account_info(),
                    system_program:ctx.accounts.system_program.to_account_info(),
                    rent:ctx.accounts.rent.to_account_info(),
                    edition:ctx.accounts.edition_account.to_account_info(),
                    token_program:ctx.accounts.token_program.to_account_info()
                }
            ), 
            Some(data.suply)
        )?;

        Ok(())
    }
   // add: Name, symbol, url..to token(not NFTs)
    pub fn add_metadata_token(ctx: Context<AddMetadata>, name:String, symbol:String, url:String)->Result<()>{
        // let (metadata_address,b1) = Pubkey::find_program_address(&[
        //     b"metadata", // it's default of Megadat Program.
        //     &ctx.accounts.token_metadata_program.key.to_bytes(),
        //     &ctx.accounts.mint_token.key().to_bytes()
        //     ], 
        //     ctx.accounts.token_metadata_program.key
        // );

        // let metadata_account = &ctx.accounts.metadata_account;

        // if metadata_address != *metadata_account.key{
        //     return err!(ProgramErrors::PdaNotMatched)
        // }
        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(), 
                anchor_spl::metadata::CreateMetadataAccountsV3{
                    metadata:ctx.accounts.metadata_account.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    mint_authority:ctx.accounts.signer.to_account_info(),
                    update_authority:ctx.accounts.signer.to_account_info(),
                    payer:ctx.accounts.signer.to_account_info(),
                    system_program:ctx.accounts.system_program.to_account_info(),
                    rent:ctx.accounts.rent.to_account_info(),
                }
            ), 
            DataV2 { 
                name: name.to_owned(), 
                symbol: symbol.to_owned(), 
                uri: url.to_owned(), 
                seller_fee_basis_points: 0,
                creators: None, 
                collection: None, 
                uses: None 
            },
            true, 
            true, 
            None
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddMetadata<'info> {
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    pub mint_token: Account<'info, Mint>,
    /// CHECK: This is not dangerous
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: This is not dangerous
    pub update_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous
    pub token_metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}
#[derive(Debug,AnchorDeserialize,AnchorSerialize)]
pub struct MetadataData{
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub suply: u64
    // pub creators: Option<Vec<Creator>>,
    // pub collection: Option<Collection>,
    // pub uses: Option<Uses>,
}

#[derive(Accounts)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub signer:Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub metadata_account:AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub master_account:AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub edition_account:AccountInfo<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    /// CHECK:
    pub metadata_program:AccountInfo<'info>,
    pub rent:Sysvar<'info,Rent>
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub mint_token:Signer<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    ///CHECK:
    #[account(mut)]
    pub token_account:AccountInfo<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    pub rent:Sysvar<'info,Rent>
}

#[derive(Accounts)]
pub struct TransferToken<'info>{    
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub from_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub to_account:Account<'info,TokenAccount>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
}

#[derive(Accounts)]
pub struct SetAuthorityToken<'info> {
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub new_signer:Signer<'info>,
    #[account(mut)]
    pub token_account:Account<'info,TokenAccount>,
    pub token_program:Program<'info,Token>,
}

#[derive(Accounts)]
pub struct BurnToken<'info> {
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub token_account:Account<'info,TokenAccount>,
    pub token_program:Program<'info,Token>,
}

#[derive(Accounts)]
pub struct FreezeToken<'info> {
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub token_account:Account<'info,TokenAccount>,
    pub token_program:Program<'info,Token>,
}

#[derive(Accounts)]
pub struct CloseToken<'info> {
    #[account(mut)]
    pub mint_token:Account<'info,Mint>,
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub token_account:Account<'info,TokenAccount>,
    pub token_program:Program<'info,Token>,
}

#[error_code]
pub enum ProgramErrors {
    #[msg("PDA account not matched")]
    PdaNotMatched
}
// https://github.com/WaqasAyubShah/SplToken