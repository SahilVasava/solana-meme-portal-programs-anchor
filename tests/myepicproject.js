const anchor = require('@project-serum/anchor');
const BN = require('bn.js');

const { SystemProgram } = anchor.web3;

const main = async() => {
    console.log("Starting test..")

    const provider = anchor.Provider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Myepicproject;

    const baseAccount = anchor.web3.Keypair.generate();

    const tx = await program.rpc.startStuffOff({
        accounts: {
            baseAccount: baseAccount.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount],
    });

    console.log("Your transaction signature", tx);

    let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('GIF Count', account.totalGifs.toString())

    await program.rpc.addGif('https://media.giphy.com/media/MaXYVi4y8xqcFeotvt/giphy.gif',{
        accounts: {
            baseAccount: baseAccount.publicKey,
            user: provider.wallet.publicKey,
        },
    });

    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('GIF Count', account.totalGifs.toString())

    console.log('GIF list', account.gifList)

    await program.rpc.upvoteGif(new BN('0'),{
        accounts: {
            baseAccount: baseAccount.publicKey,
            user: provider.wallet.publicKey,
        },
    });

    account = await program.account.baseAccount.fetch(baseAccount.publicKey);

    console.log('GIF list', account.gifList)
    console.log('Upvoter ', account.gifList[0].upvoters[0].toString())

    await program.rpc.downvoteGif(new BN('0'),{
        accounts: {
            baseAccount: baseAccount.publicKey,
            user: provider.wallet.publicKey,
        },
    });

    account = await program.account.baseAccount.fetch(baseAccount.publicKey);

    console.log('GIF list', account.gifList)
    console.log('Downvoter ', account.gifList[0].downvoters[0].toString())
    
}

const runMain = async () => {
    try {
        await main();
        process.exit(0);
    } catch (error) {
        console.error(error);
        process.exit(1);
    }
};

runMain();
