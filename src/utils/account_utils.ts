import { PublicKey, Keypair } from "@solana/web3.js";
import bs58 from "bs58";

export async function privateKeyToPublicKey(privateKey: string): Promise<PublicKey | null> {
    try {
        const keypair = Keypair.fromSecretKey(bs58.decode(privateKey));
        return keypair.publicKey;
    } catch (error) {
        return null;
    }
}