import { SimulateCosmWasmClient } from "@oraichain/cw-simulate";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate"
import { coins, GasPrice } from '@cosmjs/stargate';
import { readFileSync } from 'fs';

import { Cw20BaseClient } from "./build/Cw20Base.client"
import { InstantiateMsg, ExecuteMsg } from "./build/Cw20Base.types"

async function main() {
    const senderAddress = "orai12zyu8w93h0q2lcnt50g3fn0w3yqnhy4fvawaqz";
    const player1Address = "orai14vcw5qk0tdvknpa38wz46js5g7vrvut8lk0lk6";
    const player2Address = "orai14vcw5qk0tdvknpa38wz46js5g7vrvut8lk0l02";
    const player3Address = "orai14vcw5qk0tdvknpa38wz46js5g7vrvut8lk0l03";

    const wasmBytecode = readFileSync('cw20-base.wasm');

    // create an instance
    const client = new SimulateCosmWasmClient({
        chainId: "Oraichain",
        bech32Prefix: "orai",
        metering: true
    });

    // set account balance
    client.app.bank.setBalance(senderAddress, coins('10000000', 'orai'));
    client.app.bank.setBalance(player1Address, coins('50000', 'orai'));
    client.app.bank.setBalance(player2Address, coins('50000', 'orai'));
    client.app.bank.setBalance(player3Address, coins('50000', 'orai'));

    // console.log("[-] Sender balance: ", await client.getBalance(senderAddress, 'orai'));
    // console.log("[-] Players balance: ", await client.getBalance(player1Address, 'orai'));
    // console.log("[-] Player2 balance: ", await client.getBalance(player3Address, 'orai'));

    // get codeId
    const wasmCode = new Uint8Array(wasmBytecode.buffer);
    const { codeId } = await client.upload(senderAddress, wasmCode, "auto");
    //   console.log("[-] Code id: ", codeId);

    // deploy contract
    const { contractAddress } = await client.instantiate(
        senderAddress,
        codeId,
        {
            decimals: 10,
            initial_balances: [{ amount: '100000', address: senderAddress }],
            name: "fake token",
            symbol: 'FTC'
        } as InstantiateMsg,
        "cw20",
        "auto"
    );
    console.log("[+] Contract address: ", contractAddress);

    const cw20BaseClient = new Cw20BaseClient(client, senderAddress, contractAddress);
    // console.log(lotteryClient);

    // test query contract
    //   let lotteryContractBalance = await lotteryClient.contractBalance();
    //   console.log(lotteryContractBalance);

    //   let lotteryMinimalDonation = await lotteryClient.minimalDonation();
    //   console.log(lotteryMinimalDonation);

    // test execute contract
    await client.execute(senderAddress, contractAddress, { transfer: { amount: '1000', recipient: player1Address } } as ExecuteMsg, "auto");
    // await cw20BaseClient.transfer({amount: '1000', recipient: player1Address}, "auto");
    let balance = await cw20BaseClient.balance({ address: player1Address });
    console.log(balance);
    //   await client.execute(player2Address, contractAddress, { donate: {} }, "auto", "none", [{ amount: "1000", denom: "orai" }]);
    //   await client.execute(player3Address, contractAddress, { donate: {} }, "auto", "none", [{ amount: "1000", denom: "orai" }]);

    //   await lotteryClient.pickWinner('auto', "none", [{ amount: "0", denom: "orai" }]);
    //   console.log("[+] After pick winner:");
    //   console.log("[+] Sender balance: ", await client.getBalance(senderAddress, 'orai'));
    //   console.log("[+] Player1 balance: ", await client.getBalance(player1Address, 'orai'));
    //   console.log("[+] Player2 balance: ", await client.getBalance(player2Address, 'orai'));
    //   console.log("[+] Player3 balance: ", await client.getBalance(player3Address, 'orai'));
}

main();