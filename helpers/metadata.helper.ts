import { Connection, PublicKey } from "@solana/web3.js";
import { Metadata, PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import axios from "axios";

export const getMetadataAddress = async (
  mint: PublicKey
): Promise<PublicKey> => {
  return (
    await PublicKey.findProgramAddress(
      [Buffer.from("metadata"), PROGRAM_ID.toBuffer(), mint.toBuffer()],
      PROGRAM_ID
    )
  )[0];
};

export const getMetadata = async (connection: Connection, mint: PublicKey) => {
  const pda = await getMetadataAddress(mint);
  const { data } = await Metadata.fromAccountAddress(connection, pda);

  const regex = /\x00/g;
  const name = data.name.replace(regex, "");
  const symbol = data.symbol.replace(regex, "");
  const uri = data.uri.replace(regex, "");

  const metadata = (await axios.get(uri)).data;
  const image = metadata?.image;

  return {
    name,
    symbol,
    uri,
    image,
  };
};
