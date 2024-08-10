
import * as anchor from "@coral-xyz/anchor";  
import { Program } from "@coral-xyz/anchor";
import { AnchorSplToken } from "../target/types/anchor_spl_token";
import { createAccount, getAccount,TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import fs from "fs";
import {PROGRAM_ID as METADATA_PROGRAM_ID,createUpdateMetadataAccountV2Instruction,DataV2} from "@metaplex-foundation/mpl-token-metadata";

describe("create-tokens", () => {
  // Configure the client to use the local cluster.
  const path="/Volumes/Backup/Tai Lieu Hoc Blockchain/Rust/id.json"
  const secret = JSON.parse(fs.readFileSync(path).toString()) as number[];
  const secretKey = Uint8Array.from(secret);
  const payer= Keypair.fromSecretKey(secretKey);
  console.log("pay Keypair:",payer.publicKey.toBase58());

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
 
  const program = anchor.workspace.AnchorSplToken as Program<AnchorSplToken>;

  const mintToken = anchor.web3.Keypair.generate()
  
  const associateTokenProgram = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")

  const tokenAccount =anchor.utils.token.associatedAddress({mint:mintToken.publicKey,owner:provider.publicKey})

  const mintToken_test= new anchor.web3.PublicKey("5bFx8W1d7SRpBBf1cM4nXS4nnjawejHrs9RHT2Bt5xoa");
  const tokenAccount_test = new anchor.web3.PublicKey("AV6LaZibjc2SrrtPtH2kZjm4Gajz81kHwk8sWjLHiQHK");
  // const TOKEN_METADATA_PROGRAM = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")

  console.log("Program ID: ",program.programId.toJSON());
  console.log("Token program: ",TOKEN_PROGRAM_ID.toBase58());
  console.log("Mint Token: ",mintToken_test.toBase58());  
  console.log("Token Account: ",tokenAccount_test.toBase58());
  console.log("Metadata Program: ",METADATA_PROGRAM_ID.toBase58());
  // const ta = anchor.web3.PublicKey.findProgramAddressSync(
  //   [provider.publicKey.toBuffer(),TOKEN_PROGRAM_ID.toBuffer(),mintToken.publicKey.toBuffer()],
  //   associateTokenProgram
  // )[0]

  // let tokenAccountKeyPair = anchor.web3.Keypair.generate();



  it.skip("Create token!", async () => {

    console.log("Mint account: ",mintToken.publicKey.toBase58())
    console.log("Token account: ",tokenAccount.toBase58())

    try {
      const tx = await program.methods.createToken(9,new anchor.BN(10**9*1000))
        .accounts({
          mintToken:mintToken.publicKey,
          tokenAccount:tokenAccount,
          // associateTokenProgram,
        })
        .signers([mintToken])
        .rpc();
        console.log("Your transaction signature", tx);
      } catch (error) {
        console.log(error)
      }
  });


  it.skip("Token transfer", async () =>{

    let reciever = anchor.web3.Keypair.generate()

    const signature = await provider.connection.requestAirdrop(reciever.publicKey,anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction(signature)

    let recieverTokenAccountKeypair = anchor.web3.Keypair.generate()
    await createAccount(provider.connection,reciever,mintToken.publicKey,reciever.publicKey,recieverTokenAccountKeypair);

    try {
      const tx = await program.methods.transerToken(new anchor.BN(10**9*90))
      .accounts({
        mintToken:mintToken.publicKey,
        fromAccount:tokenAccount,
        toAccount:recieverTokenAccountKeypair.publicKey,
        // associateTokenProgram
      })
      .signers([])
      .rpc()

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error)
    }


  })

  

  it.skip("Set Authority token!", async () => {

    let new_signer = anchor.web3.Keypair.generate()
    try {
      const tx = await program.methods.setAuthorityToken(0)
      .accounts({
        mintToken:mintToken.publicKey,
        tokenAccount:tokenAccount,
        newSigner:payer.publicKey,
      })
      .signers([new_signer])
      .rpc();
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.log(e)
    }
  });

  it.skip("Freeze token!", async () => {

    
    const tx = await program.methods.freezeToken()
    .accounts({
      mintToken:mintToken.publicKey,
      tokenAccount,
    })
    .signers([])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it.skip("Unfreeze token!", async () => {

    
    const tx = await program.methods.unFreezeToken()
    .accounts({
      mintToken:mintToken.publicKey,
      tokenAccount,
    })
    .signers([])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it.skip("Burn token!", async () => {

    try {
      const tx = await program.methods.burnToken(new anchor.BN(10**9*10))
        .accounts({
          mintToken:mintToken_test,
          tokenAccount:tokenAccount_test,
        })
        .signers([])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error)
    }
    
  });

  it.skip("Close token!", async () => {

    
    const tx = await program.methods.closeToken()
    .accounts({
      mintToken:mintToken.publicKey,
      tokenAccount,
    })
    .signers([])
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it.skip("set metadata", async()=>{
    let metadataAddress: PublicKey;
    // const payer = Keypair.generate();
    // await provider.connection.requestAirdrop(payer.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    // console.log("payer: ",payer.publicKey.toBase58());
    const tokenMetadata = {
      name: "Linh Token",
      symbol: "LINHDEMO",
      uri: "https://example.com/my-token-metadata.json",
    };
 
    //get metadata account from Mint Token account.
    [metadataAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mintToken_test.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    );
    console.log(" Metadata account: ",metadataAddress.toBase58())
 
    try{
      const tx = await program.methods.addMetadataToken(tokenMetadata.name,tokenMetadata.symbol,tokenMetadata.uri)
      .accounts({
       signer: payer.publicKey,
       mintToken:mintToken_test,
       metadataAccount:metadataAddress,
       tokenMetadataProgram:METADATA_PROGRAM_ID,
 
      })
      .signers([])
      .rpc();
      console.log("Your transaction signature", tx);
    }catch(error){
      console.log("-------------------------");
      console.log(error);
    }
  
  })

});
