import { test, expect } from "bun:test";
import * as borsh from "borsh"
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from '@solana/web3.js'
import { COUNTER_SIZE, schema } from "./types";

// test('check-test', function () {
//     expect(2).toBe(2)
// }
// )

const user_account = Keypair.generate();
const data_account = Keypair.generate();

test('Account Initialised', async function () {
    const connection = new Connection("http://127.0.0.1:8899", "confirmed");
    const txn = await connection.requestAirdrop(user_account.publicKey, 2 * LAMPORTS_PER_SOL)
    await connection.confirmTransaction(txn);

    // console.log(await connection.getAccountInfo(user_account.publicKey));

    const programId = new PublicKey("FU5H6noQsB7YFhwJ2Rsj8tE4tM6sa75orB8CbTPcEmex");
    //min lamports needed to store data on the user_account
    const lamports = await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);

    const createDataAccount = SystemProgram.createAccount({
        fromPubkey: user_account.publicKey,
        lamports,
        newAccountPubkey: data_account.publicKey,
        programId: programId,
        space: COUNTER_SIZE
    })

    const createDataAccounttx = new Transaction();
    createDataAccounttx.add(createDataAccount);

    const signature = await connection.sendTransaction(createDataAccounttx,[user_account,data_account]);
    await connection.confirmTransaction(signature)

    const data_account_info = await connection.getAccountInfo(data_account.publicKey);
    console.log(data_account_info?.data);
    const counter = borsh.deserialize(schema,data_account_info?.data);
    console.log(counter.count);
    expect(counter.count).toBe(0);
})
