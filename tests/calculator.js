const assert = require('assert');
const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

describe('calculator', () => {

  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.Provider.env());
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Calculator;

  it('Creates a calculator', async () => {
    console.log("🚀 Starting test...");

    const tx = await program.rpc.Initialize({
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.systemProgram.programId,
      },
      signers: [program],
    });

    console.log("Transaction: ", tx);
  });

  it("Adds two numbers", async function() {

  });

  it('Multiplies two numbers', async function() {

  })

  it('Subtracts two numbers', async function() {

  });

  it('Divides two numbers', async function() {

  });
});
