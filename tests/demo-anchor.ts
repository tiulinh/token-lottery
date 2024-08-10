
import * as anchor from "@coral-xyz/anchor";  
import { Program } from "@coral-xyz/anchor";
import { AnchorSplToken } from "../target/types/anchor_spl_token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { createAccount, getAccount } from "@solana/spl-token";


describe("create-tokens", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorSplToken as Program<AnchorSplToken>;

  const mintToken = anchor.web3.Keypair.generate()
  
  const associateTokenProgram = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")

  const tokenAccount =anchor.utils.token.associatedAddress({mint:mintToken.publicKey,owner:provider.publicKey})
  const mintToken_test= new anchor.web3.PublicKey("8FmMYcyegEKvq885UF4YHz1ywSQHjZtQ8QcVJVE2BeVu");
  const tokenAccount_test = new anchor.web3.PublicKey("Bti22dPiVSXDXrmcqYHTZH3UuryGcg1N3QG6agbSVuAM");
  const TOKEN_METADATA_PROGRAM = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
  console.log("program adress: ",program.programId.toJSON());
  console.log("Mint Token: ",mintToken.publicKey.toBase58());
  console.log("Token Account: ",tokenAccount.toBase58());
  console.log("Metadata Program: ",TOKEN_METADATA_PROGRAM.toBase58());
  // const ta = anchor.web3.PublicKey.findProgramAddressSync(
  //   [provider.publicKey.toBuffer(),TOKEN_PROGRAM_ID.toBuffer(),mintToken.publicKey.toBuffer()],
  //   associateTokenProgram
  // )[0]

  // let tokenAccountKeyPair = anchor.web3.Keypair.generate();



  it.skip("Create token!", async () => {

    console.log(mintToken.publicKey.toBase58())
    console.log(tokenAccount.toBase58())

    try {
      const tx = await program.methods.createToken(9,new anchor.BN(10**9*100))
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
        newSigner:new_signer.publicKey,
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
          mintToken:mintToken.publicKey,
          tokenAccount:tokenAccount,
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
    //  const tx = await program.methods.setTokenMetadata
  })
  
});

// 8AJwJssCTy8xHwYgPPxyainXLQtdk9Km2HSC1PtC7jKY on devnet
// token1: 8FmMYcyegEKvq885UF4YHz1ywSQHjZtQ8QcVJVE2BeVu
