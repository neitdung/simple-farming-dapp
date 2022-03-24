const contractName = "contract.simple-farming.testnet";
module.exports = function getConfig() {
  return  {
    networkId: "testnet",
    nodeUrl: "https://rpc.testnet.near.org",
    // walletUrl: 'http://localhost:1234',
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    contractId: contractName
  };
};
