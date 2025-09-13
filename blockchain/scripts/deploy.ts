import { ethers } from "hardhat";

async function main() {
  console.log("🚀 Iniciando deploy dos contratos FORTIS...");

  // Get the deployer account
  const [deployer] = await ethers.getSigners();
  console.log("📝 Deploying contracts with account:", deployer.address);
  console.log("💰 Account balance:", (await deployer.getBalance()).toString());

  // Deploy FortisToken
  console.log("\n🪙 Deploying FortisToken...");
  const FortisToken = await ethers.getContractFactory("FortisToken");
  const fortisToken = await FortisToken.deploy();
  await fortisToken.deployed();
  console.log("✅ FortisToken deployed to:", fortisToken.address);

  // Deploy TimelockController
  console.log("\n⏰ Deploying TimelockController...");
  const TimelockController = await ethers.getContractFactory("TimelockController");
  const timelock = await TimelockController.deploy(
    3600, // minDelay: 1 hour
    [deployer.address], // proposers
    [deployer.address], // executors
    deployer.address // admin
  );
  await timelock.deployed();
  console.log("✅ TimelockController deployed to:", timelock.address);

  // Deploy FortisGovernance
  console.log("\n🏛️ Deploying FortisGovernance...");
  const FortisGovernance = await ethers.getContractFactory("FortisGovernance");
  const fortisGovernance = await FortisGovernance.deploy(
    fortisToken.address, // token
    timelock.address, // timelock
    4, // quorumPercentage: 4%
    1, // votingDelay: 1 block
    17280, // votingPeriod: 3 days (17280 blocks)
    ethers.utils.parseEther("1000") // proposalThreshold: 1000 tokens
  );
  await fortisGovernance.deployed();
  console.log("✅ FortisGovernance deployed to:", fortisGovernance.address);

  // Deploy FortisVoting
  console.log("\n🗳️ Deploying FortisVoting...");
  const FortisVoting = await ethers.getContractFactory("FortisVoting");
  const fortisVoting = await FortisVoting.deploy();
  await fortisVoting.deployed();
  console.log("✅ FortisVoting deployed to:", fortisVoting.address);

  // Deploy FortisIdentity
  console.log("\n🆔 Deploying FortisIdentity...");
  const FortisIdentity = await ethers.getContractFactory("FortisIdentity");
  const fortisIdentity = await FortisIdentity.deploy();
  await fortisIdentity.deployed();
  console.log("✅ FortisIdentity deployed to:", fortisIdentity.address);

  // Deploy FortisAudit
  console.log("\n🛡️ Deploying FortisAudit...");
  const FortisAudit = await ethers.getContractFactory("FortisAudit");
  const fortisAudit = await FortisAudit.deploy();
  await fortisAudit.deployed();
  console.log("✅ FortisAudit deployed to:", fortisAudit.address);

  // Setup roles and permissions
  console.log("\n🔐 Setting up roles and permissions...");
  
  // Grant roles to governance contract
  await fortisVoting.grantRole(await fortisVoting.ADMIN_ROLE(), fortisGovernance.address);
  await fortisIdentity.grantRole(await fortisIdentity.ADMIN_ROLE(), fortisGovernance.address);
  await fortisAudit.grantRole(await fortisAudit.ADMIN_ROLE(), fortisGovernance.address);

  // Grant TSE role to deployer
  await fortisIdentity.grantRole(await fortisIdentity.TSE_ROLE(), deployer.address);
  await fortisVoting.grantRole(await fortisVoting.ADMIN_ROLE(), deployer.address);

  console.log("✅ Roles and permissions configured");

  // Save deployment info
  const deploymentInfo = {
    network: await ethers.provider.getNetwork(),
    deployer: deployer.address,
    contracts: {
      FortisToken: fortisToken.address,
      TimelockController: timelock.address,
      FortisGovernance: fortisGovernance.address,
      FortisVoting: fortisVoting.address,
      FortisIdentity: fortisIdentity.address,
      FortisAudit: fortisAudit.address
    },
    timestamp: new Date().toISOString()
  };

  console.log("\n📋 Deployment Summary:");
  console.log("====================");
  console.log(`Network: ${deploymentInfo.network.name} (${deploymentInfo.network.chainId})`);
  console.log(`Deployer: ${deploymentInfo.deployer}`);
  console.log("\nContracts:");
  Object.entries(deploymentInfo.contracts).forEach(([name, address]) => {
    console.log(`  ${name}: ${address}`);
  });

  // Verify contracts on Polygon if on mainnet
  if (deploymentInfo.network.chainId === 137) {
    console.log("\n🔍 Verifying contracts on Polygon...");
    try {
      await hre.run("verify:verify", {
        address: fortisToken.address,
        constructorArguments: []
      });
      console.log("✅ FortisToken verified");
    } catch (error) {
      console.log("❌ FortisToken verification failed:", error);
    }
  }

  console.log("\n🎉 Deploy completed successfully!");
  console.log("\n📝 Next steps:");
  console.log("1. Update frontend with contract addresses");
  console.log("2. Configure backend to use contract addresses");
  console.log("3. Test contract interactions");
  console.log("4. Deploy to production network");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("❌ Deploy failed:", error);
    process.exit(1);
  });
