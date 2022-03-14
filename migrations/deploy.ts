// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

import {Provider} from "@project-serum/anchor";

const anchor = require("@project-serum/anchor");

  // Configure client to use the provider.
  anchor.setProvider(anchor.Provider.env());
  //// Read the generated IDL.
    const idl = JSON.parse(
        require("fs").readFileSync("./target/idl/nft_staking.json", "utf8")
    );

    //Address of the deployed program
    const programId = new anchor.web3.PublicKey("GBpQdoXPzopEfP6Nmwv6PprWUo1nKcziK6Bqjeb7pCzi");

    //Generate the program client from IDL
    const program = new anchor.Program(idl, programId);
    console.log(program)
