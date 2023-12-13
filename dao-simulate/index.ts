import { SimulateCosmWasmClient } from "@oraichain/cw-simulate";
import { SigningCosmWasmClient, toBinary} from "@cosmjs/cosmwasm-stargate";
import { coins, GasPrice } from "@cosmjs/stargate";
import { readFileSync } from "fs";
import { InstantiateMsg as VotingInitMsg } from "./build/DaoVotingCw20Staked.types";
import { InstantiateMsg as ProposalInitMsg } from "./build/DaoProposalSingle.types";
import {InstantiateMsg as DaoInitMsg} from './build/DaoDaoCore.types'

async function main() {
  const senderAddress = "orai12zyu8w93h0q2lcnt50g3fn0w3yqnhy4fvawaqz";
  const player1Address = "orai14vcw5qk0tdvknpa38wz46js5g7vrvut8lk0lk6";
  const player2Address = "orai14vcw5qk0tdvknpa38wz46js5g7vrvut8lk0l02";
  const player3Address = "orai14vcw5qk0tdvknpa38wz46js5g7vrvut8lk0l03";

  const wasmBytecode = readFileSync("cw20-base.wasm");

  // create an instance
  const client = new SimulateCosmWasmClient({
    chainId: "Oraichain",
    bech32Prefix: "orai",
    metering: true,
  });

  // set account balance
  client.app.bank.setBalance(senderAddress, coins("10000000", "orai"));
  client.app.bank.setBalance(player1Address, coins("50000", "orai"));
  client.app.bank.setBalance(player2Address, coins("50000", "orai"));
  client.app.bank.setBalance(player3Address, coins("50000", "orai"));

  // console.log("[-] Sender balance: ", await client.getBalance(senderAddress, 'orai'));
  // console.log("[-] Players balance: ", await client.getBalance(player1Address, 'orai'));
  // console.log("[-] Player2 balance: ", await client.getBalance(player3Address, 'orai'));

  // get codeId
  //   const wasmCode = new Uint8Array(wasmBytecode.buffer);
  //   const { codeId } = await client.upload(senderAddress, wasmCode, "auto");
  const { codeId: cw20CodeId } = await client.upload(
    senderAddress,
    new Uint8Array(readFileSync("./contracts/cw20-base.wasm").buffer),
    "auto"
  );
  const { codeId: stakingCodeId } = await client.upload(
    senderAddress,
    new Uint8Array(readFileSync("./contracts/cw20-stake.wasm").buffer),
    "auto"
  );
  const { codeId: daoCodeId } = await client.upload(
    senderAddress,
    new Uint8Array(readFileSync("./contracts/dao-dao-core.wasm").buffer),
    "auto"
  );
  const { codeId: proposalCodeId } = await client.upload(
    senderAddress,
    new Uint8Array(readFileSync("./contracts/dao-proposal-single.wasm").buffer),
    "auto"
  );
  const { codeId: votingCodeId } = await client.upload(
    senderAddress,
    new Uint8Array(
      readFileSync("./contracts/dao-voting-cw20-staked.wasm").buffer
    ),
    "auto"
  );
  //   console.log("[-] Code id: ", codeId);

  const votingInitMsg: VotingInitMsg = {
    token_info: {
      new: {
        code_id: cw20CodeId,
        label: "DAO DAO governance token",
        name: "DAO",
        symbol: "DAO",
        decimals: 6,
        initial_balances: [{ amount: "100000000", address: senderAddress }],
        staking_code_id: stakingCodeId,
      },
    },
    active_threshold: {
      absolute_count: {
        count: "100",
      },
    },
  };

  const proposalInitMsg: ProposalInitMsg = {
    threshold: {
      threshold_quorum: {
        quorum: { percent: "0.015" },
        threshold: { majority: {} },
      },
    },
    max_voting_period: { time: 604800 }, // One week.
    only_members_execute: true,
    allow_revoting: false,
    pre_propose_info: {
      anyone_may_propose: {},
    },
    close_proposal_on_execution_failure: true,
  };

  const daoInitMsg: DaoInitMsg = {
    name: 'DAO DAO',
        description: 'A DAO that builds DAOs.',
        automatically_add_cw20s: true,
        automatically_add_cw721s: true,
        voting_module_instantiate_info: {
          code_id: votingCodeId,
          msg: toBinary(votingInitMsg),
          admin: { core_module: {} },
          label: 'voting module'
        },
        proposal_modules_instantiate_info: [
          {
            code_id: proposalCodeId,
            msg: toBinary(proposalInitMsg),
            admin: { core_module: {} },
            label: 'governance module'
          }
        ]
  }

  // deploy contract
  const { contractAddress } = await client.instantiate(
    senderAddress,
    daoCodeId,
    daoInitMsg,
    "dao",
    "auto"
  );
  console.log("[+] Contract address: ", contractAddress);

//   const cw20BaseClient = new Cw20BaseClient(
//     client,
//     senderAddress,
//     contractAddress
//   );
  // console.log(lotteryClient);

  // test query contract
  //   let lotteryContractBalance = await lotteryClient.contractBalance();
  //   console.log(lotteryContractBalance);

  //   let lotteryMinimalDonation = await lotteryClient.minimalDonation();
  //   console.log(lotteryMinimalDonation);

  // test execute contract
//   await client.execute(
//     senderAddress,
//     contractAddress,
//     { transfer: { amount: "1000", recipient: player1Address } } as ExecuteMsg,
//     "auto"
//   );
//   // await cw20BaseClient.transfer({amount: '1000', recipient: player1Address}, "auto");
//   let balance = await cw20BaseClient.balance({ address: player1Address });
//   console.log(balance);
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
