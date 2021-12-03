import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import {createKeypairFromFile, getPayer, getRpcUrl} from "./utils";
import fs from "mz/fs";
import path from "path";
import * as borsh from 'borsh';



let connection: Connection;
let programId: PublicKey;
let payer: Keypair;

const PROGRAM_PATH = path.resolve(__dirname, '../dist/ovn');
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'ovn-keypair.json');
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'ovn.so');
let greetedPubkey: PublicKey;


class GreetingAccount {
    counter = 0;
    constructor(fields: {counter: number} | undefined = undefined) {
        if (fields) {
            this.counter = fields.counter;
        }
    }
}


const GreetingSchema = new Map([
    [GreetingAccount, {kind: 'struct', fields: [['counter', 'u32']]}],
]);

/**
 * The expected size of each greeting account.
 */
const GREETING_SIZE = borsh.serialize(
    GreetingSchema,
    new GreetingAccount(),
).length;

enum Method {
    MINT
}

class ProgramData {
    method: Method = Method.MINT
    args: number = 0
    // data: string = '';

    constructor(fields: {method: Method, args: number} | undefined = undefined) {
        if(fields) {
            this.method = fields.method;
            this.args = fields.args;
        }
    }
}

const DataSchema = new Map([
    [ProgramData, {kind: 'struct', fields: [['method', 'u8'], ['args', 'u128']]}],
]);

// Read program id from keypair file
async function checkProgram() {

    try {
        const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
        programId = programKeypair.publicKey;
    } catch (err) {
        const errMsg = (err as Error).message;
        throw new Error(
            `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``,
        );
    }

// Check if the program has been deployed
    const programInfo = await connection.getAccountInfo(programId);
    if (programInfo === null) {
        if (fs.existsSync(PROGRAM_SO_PATH)) {
            throw new Error(
                'Program needs to be deployed with `solana program deploy dist/program/helloworld.so`',
            );
        } else {
            throw new Error('Program needs to be built and deployed');
        }
    } else if (!programInfo.executable) {
        throw new Error(`Program is not executable`);
    }
    console.log(`Using program ${programId.toBase58()}`)

    const GREETING_SEED = 'hello';
    greetedPubkey = await PublicKey.createWithSeed(
        payer.publicKey,
        GREETING_SEED,
        programId,
    );

    // Check if the greeting account has already been created
    const greetedAccount = await connection.getAccountInfo(greetedPubkey);
    if (greetedAccount === null) {
        console.log(
            'Creating account',
            greetedPubkey.toBase58(),
            'to say hello to',
        );
        const lamports = await connection.getMinimumBalanceForRentExemption(
            GREETING_SIZE,
        );

        const transaction = new Transaction().add(
            SystemProgram.createAccountWithSeed({
                fromPubkey: payer.publicKey,
                basePubkey: payer.publicKey,
                seed: GREETING_SEED,
                newAccountPubkey: greetedPubkey,
                lamports,
                space: GREETING_SIZE,
                programId,
            }),
        );
        await sendAndConfirmTransaction(connection, transaction, [payer]);
    }
}

export async function establishPayer(): Promise<void> {
    let fees = 0;
    if (!payer) {
        const {feeCalculator} = await connection.getRecentBlockhash();

        // Calculate the cost to fund the greeter account
        // fees += await connection.getMinimumBalanceForRentExemption(GREETING_SIZE);

        // Calculate the cost of sending transactions
        fees += feeCalculator.lamportsPerSignature * 100; // wag

        payer = await getPayer();
    }

    let lamports = await connection.getBalance(payer.publicKey);
    if (lamports < fees) {
        // If current balance is not enough to pay for fees, request an airdrop
        const sig = await connection.requestAirdrop(
            payer.publicKey,
            fees - lamports,
        );
        await connection.confirmTransaction(sig);
        lamports = await connection.getBalance(payer.publicKey);
    }

    console.log(
        'Using account',
        payer.publicKey.toBase58(),
        'containing',
        lamports / LAMPORTS_PER_SOL,
        'SOL to pay for fees',
    );
}

export async function establishConnection(): Promise<void> {
    const rpcUrl = await getRpcUrl();
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
}


export async function executeProgram(): Promise<void> {
    // console.log('Saying hello to', greetedPubkey.toBase58());
    // const data: Buffer = Buffer.from("dsklgfdklgjdfg");
    const data = borsh.serialize(DataSchema, new ProgramData({method: 0, args: 123}))
    const instruction = new TransactionInstruction({
        keys: [{pubkey: greetedPubkey, isSigner: false, isWritable: true}],
        programId,
        data: Buffer.from(data)
        // data: Buffer.alloc(0), // All instructions are hellos
    });
    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer],
    );
}

async function main() {
    await establishConnection();
    await establishPayer();
    await checkProgram();
    await executeProgram();
}

main().then(
    () => process.exit(),
    err => {
        console.log(err);
        process.exit(-1);
    }
)