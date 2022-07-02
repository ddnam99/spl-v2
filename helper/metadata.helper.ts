import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";

export const getMetadata = async (mint: PublicKey): Promise<PublicKey> => {
  return (
    await PublicKey.findProgramAddress(
      [Buffer.from("metadata"), PROGRAM_ID.toBuffer(), mint.toBuffer()],
      PROGRAM_ID
    )
  )[0];
};
