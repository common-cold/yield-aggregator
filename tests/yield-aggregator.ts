import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { YieldAggregator } from "../target/types/yield_aggregator";
import dotenv from "dotenv";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
// import {
//   getDepositContext, getWithdrawContext
// } from "@jup-ag/lend/earn";

dotenv.config({ path: "./tests/.env" });

describe("yield-aggregator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.yieldAggregator as Program<YieldAggregator>;

  const ownerKeypair = Keypair.fromSecretKey(bs58.decode(process.env.MAKER_KEYPAIR!));
  const usdcMint = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
  // const takerKeypair = Keypair.fromSecretKey(bs58.decode(process.env.TAKER_KEYPAIR));
  // const mint = new PublicKey(process.env.MINT_ADDRESS);
  const decimals = new BN(3).pow(new BN(6));


  const connection = new Connection("https://api.devnet.solana.com");

  console.log(program.programId);

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Initialize Vault", async () => {
    console.log("INSIDE FUN = " + program.programId);
    const ownerUsdcAta = getAssociatedTokenAddressSync(usdcMint, ownerKeypair.publicKey, false, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
    
    const vaultAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), ownerKeypair.publicKey.toBuffer()],
      program.programId
    )[0];

    const vaultUsdcta = getAssociatedTokenAddressSync(usdcMint, vaultAccount, true, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);

    const tx = await program.methods
      .initializeVault(new BN(10).mul(decimals))
      .accounts({
        owner: ownerKeypair.publicKey,
        usdcMint: usdcMint,
        ownerUsdcAta: ownerUsdcAta,
        vaultAccount: vaultAccount,
        vaultUsdcAta: vaultUsdcta,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([ownerKeypair])
    .rpc();
    console.log("Your transaction signature", tx);
  });
  // it("test", async() => {
  //   const { getDepositContext } = await import('@jup-ag/lend/earn');
  //     const depositContext = await getDepositContext({
  //         asset: usdcMint, // asset mint address
  //         signer: ownerKeypair.publicKey, // signer public key
  //         connection,
  //     });  
  //     console.log(JSON.stringify(depositContext));

  // });
});
