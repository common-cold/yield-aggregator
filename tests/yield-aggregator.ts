import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { YieldAggregator } from "../target/types/yield_aggregator";
import dotenv from "dotenv";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";

dotenv.config({ path: "./tests/.env" });

describe("yield-aggregator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.yieldAggregator as Program<YieldAggregator>;

  const makerKeypair = Keypair.fromSecretKey(bs58.decode(process.env.MAKER_KEYPAIR));
  const takerKeypair = Keypair.fromSecretKey(bs58.decode(process.env.TAKER_KEYPAIR));
  const mint = new PublicKey(process.env.MINT_ADDRESS);
  const decimals = new BN(10).pow(new BN(6));


  const connection = new Connection("https://api.devnet.solana.com");

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Initialize Vault", async () => {
    // Add your test here.
    const tx = await program.methods
      .initializeVault(new BN(10).mul(decimals))
      .accounts()
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
