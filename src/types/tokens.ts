export interface TokenInfo {
    name: string;
    address: string;
    symbol: string;
    decimals: number;
    totalSupply: number;
    uri: string;
    logoUri: string;
}

export interface TokenAccount {
    pubkey: string;
    mint: string;
}
