import * as anchor from "@project-serum/anchor";
import { Program, BN as BigNumber } from "@project-serum/anchor";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { SplV2 } from "../target/types/spl_v2";
import * as spl from "@solana/spl-token";
import { Account, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PROGRAM_ID as METAPLEX_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import { assert } from "chai";
import { getMetadataAddress, getMetadata } from "../helpers/metadata.helper";

describe("spl-v2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const wallet = anchor.Wallet.local().payer;

  const program = anchor.workspace.SplV2 as Program<SplV2>;

  const TOKEN_DECIMALS = 6;
  const LAMPORTS_PER_TOKEN = Math.pow(10, TOKEN_DECIMALS);
  let splV1Mint: PublicKey;
  let splV2Mint: PublicKey;

  let userSplV1Account: Account;
  let userSplV2Account: Account;

  before(async () => {
    splV1Mint = await spl.createMint(
      anchor.getProvider().connection,
      wallet,
      wallet.publicKey,
      null,
      TOKEN_DECIMALS
    );
    userSplV1Account = await spl.getOrCreateAssociatedTokenAccount(
      anchor.getProvider().connection,
      wallet,
      splV1Mint,
      wallet.publicKey
    );
    await spl.mintTo(
      anchor.getProvider().connection,
      wallet,
      splV1Mint,
      userSplV1Account.address,
      wallet.publicKey,
      200_000_000 * LAMPORTS_PER_TOKEN
    );
  });

  const splV2Config = Keypair.generate();

  it("CreateSplV2", async () => {
    const [splV2Address, splV2Bump] = await PublicKey.findProgramAddress(
      [splV2Config.publicKey.toBuffer()],
      program.programId
    );

    const name = "SPL V2 TOKEN";
    const symbol = "SPLV2";
    const uri =
      "https://arweave.net/OwXDf7SM6nCVY2cvQ4svNjtV7WBTz3plbI4obN9JNkk";

    const metadataAddress = await getMetadataAddress(splV2Address);

    const tx = await program.methods
      .createSplV2(splV2Bump, name, symbol, uri)
      .accounts({
        config: splV2Config.publicKey,
        creator: wallet.publicKey,
        fromSpl: splV1Mint,
        splV2: splV2Address,
        metadata: metadataAddress,
        tokenMetadataProgram: METAPLEX_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([splV2Config])
      .rpc();

    splV2Mint = splV2Address;
    userSplV2Account = await spl.getOrCreateAssociatedTokenAccount(
      anchor.getProvider().connection,
      wallet,
      splV2Mint,
      wallet.publicKey
    );

    console.log("Mint", splV2Mint.toBase58());
    console.log("Your transaction signature", tx);

    const metadata = await getMetadata(
      anchor.getProvider().connection,
      splV2Mint
    );
    console.log("Metadata", metadata);
  });

  it("UpdateSplMetadata", async () => {
    const name = "SPL V2 Token";
    const symbol = "SPLV2";
    const uri =
      "https://arweave.net/OwXDf7SM6nCVY2cvQ4svNjtV7WBTz3plbI4obN9JNkk";
    const metadataAccountAddress = await getMetadataAddress(splV2Mint);

    const tx = await program.methods
      .updateSplV2Metadata(name, symbol, uri)
      .accounts({
        authority: wallet.publicKey,
        metadata: metadataAccountAddress,
        tokenMetadataProgram: METAPLEX_METADATA_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const metadata = await getMetadata(
      anchor.getProvider().connection,
      splV2Mint
    );
    console.log("Metadata", metadata);
  });

  it("SwapSpl", async () => {
    const amount = new BigNumber(1_000_000 * LAMPORTS_PER_TOKEN);

    const preSplV1Balances = (
      await spl.getAccount(
        anchor.getProvider().connection,
        userSplV1Account.address
      )
    ).amount;
    const preSplV2Balances = (
      await spl.getAccount(
        anchor.getProvider().connection,
        userSplV2Account.address
      )
    ).amount;

    const tx = await program.methods
      .swapSpl(amount)
      .accounts({
        config: splV2Config.publicKey,
        user: wallet.publicKey,
        fromSpl: splV1Mint,
        splV2: splV2Mint,
        userFromSplAccount: userSplV1Account.address,
        userSplV2Account: userSplV2Account.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const postSplV1Balances = (
      await spl.getAccount(
        anchor.getProvider().connection,
        userSplV1Account.address
      )
    ).amount;
    const postSplV2Balances = (
      await spl.getAccount(
        anchor.getProvider().connection,
        userSplV2Account.address
      )
    ).amount;

    assert.equal(
      preSplV1Balances - BigInt(amount.toNumber()),
      postSplV1Balances
    );
    assert.equal(
      preSplV2Balances + BigInt(amount.toNumber()),
      postSplV2Balances
    );
  });
});
