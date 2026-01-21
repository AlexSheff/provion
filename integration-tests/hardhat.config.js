require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.24",
  networks: {
    arbitrumLocal: {
      url: "http://localhost:8547",
      accounts: ["0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"]
    }
  }
};
