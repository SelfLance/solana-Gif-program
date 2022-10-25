const anchor = require("@project-serum/anchor");

const main = async()=>{
  // console.log("Testing started......,", anchor.getProvider());

  
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

   
  // anchor.setProvider(anchor.Provider.env())
  const program = anchor.workspace.Gifportal;
 let base_account = await anchor.web3.Keypair.generate();
 let base_account1 = await anchor.web3.Keypair.generate();

 console.log("Accounts Detail: ", anchor.web3.SystemProgram.programId)
  const tx = await program.rpc.startStufOff({
    accounts:{
      baseAccount: base_account.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    },
    signers:[base_account]
  });  

  console.log("Transaction is **************: ", tx);
  const account = await program.account.baseAccount.fetch(base_account.publicKey)
  console.log("Gifts Counts:  ", account.totalGifs.toString())


  const tx1 = await program.rpc.addGif("https://media0.giphy.com/media/3o6vXTpomeZEyxufGU/giphy.gif",
    {
      accounts:{
        baseAccount: base_account.publicKey,
        user: provider.wallet.publicKey,
  }})
  console.log("Transaction Done count: ", tx1);
  const account1 = await program.account.baseAccount.fetch(base_account.publicKey);
  console.log("Gif Count : ",account1.totalGifs.toString())
  console.log("Gif Count : ",account1.gifList)


  // const tx2 = await program.rpc.addGif(
  //   {
  //     accounts:{
  //       baseAccount: base_account1.publicKey
  // }})
  // console.log("Transaction Done count: ", tx2);
  // const account2 = await program.account.baseAccount.fetch(base_account1.publicKey);
  // console.log("Gif Count : ",account2.totalGifs.toString())
}


const runMain = async ()=>{
  try{
    await main();
    process.exit()

  }catch(error){
    console.error(error);
    process.exit();
  }
}
runMain();


// const anchor = require("@project-serum/anchor");


// describe("gifportal", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const program = anchor.workspace.Gifportal;
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });
