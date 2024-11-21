export type Strategy = {
    id: number;
    strategyType: string;
    status: string;
    nextTimeExecute: number;
    intervalTime: number;
    accountPrivateKey: string;
    tokenAddress: string;
    price: number;
    amount: number;
    prioritizationFee: number;
    slippage: number;
    txHash: string | null;
};

export type StrategyWithFullInformation = {
    id: number;
    strategyType: string;
    status: string;
    nextTimeExecute: number;
    intervalTime: number;
    accountPrivateKey: string;
    tokenAddress: string;
    price: number;
    amount: number;
    prioritizationFee: number;
    slippage: number;
    txHash: string | null;
    createdAt: string;
    logoUri: string;
    decimals: number;
    tokenName: string;
    accountName: string;
    accountPublicKey: string;
};
