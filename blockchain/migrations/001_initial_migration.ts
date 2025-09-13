import { ethers } from "hardhat";

/**
 * Migração inicial para deploy dos contratos FORTIS
 * Esta migração deploya todos os contratos na ordem correta
 */
async function main() {
  console.log("🚀 Iniciando migração inicial FORTIS...");

  // Get the deployer account
  const [deployer] = await ethers.getSigners();
  console.log("📝 Migrating with account:", deployer.address);
  console.log("💰 Account balance:", (await deployer.getBalance()).toString());

  // Step 1: Deploy FortisToken
  console.log("\n🪙 Step 1: Deploying FortisToken...");
  const FortisToken = await ethers.getContractFactory("FortisToken");
  const fortisToken = await FortisToken.deploy();
  await fortisToken.deployed();
  console.log("✅ FortisToken deployed to:", fortisToken.address);

  // Step 2: Deploy TimelockController
  console.log("\n⏰ Step 2: Deploying TimelockController...");
  const TimelockController = await ethers.getContractFactory("TimelockController");
  const timelock = await TimelockController.deploy(
    3600, // minDelay: 1 hour
    [deployer.address], // proposers
    [deployer.address], // executors
    deployer.address // admin
  );
  await timelock.deployed();
  console.log("✅ TimelockController deployed to:", timelock.address);

  // Step 3: Deploy FortisGovernance
  console.log("\n🏛️ Step 3: Deploying FortisGovernance...");
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

  // Step 4: Deploy FortisVoting
  console.log("\n🗳️ Step 4: Deploying FortisVoting...");
  const FortisVoting = await ethers.getContractFactory("FortisVoting");
  const fortisVoting = await FortisVoting.deploy();
  await fortisVoting.deployed();
  console.log("✅ FortisVoting deployed to:", fortisVoting.address);

  // Step 5: Deploy FortisAudit
  console.log("\n📊 Step 5: Deploying FortisAudit...");
  const FortisAudit = await ethers.getContractFactory("FortisAudit");
  const fortisAudit = await FortisAudit.deploy();
  await fortisAudit.deployed();
  console.log("✅ FortisAudit deployed to:", fortisAudit.address);

  // Step 6: Deploy FortisIdentity
  console.log("\n🆔 Step 6: Deploying FortisIdentity...");
  const FortisIdentity = await ethers.getContractFactory("FortisIdentity");
  const fortisIdentity = await FortisIdentity.deploy();
  await fortisIdentity.deployed();
  console.log("✅ FortisIdentity deployed to:", fortisIdentity.address);

  // Step 7: Setup roles and permissions
  console.log("\n🔐 Step 7: Setting up roles and permissions...");
  
  // Setup TimelockController roles
  await timelock.grantRole(await timelock.PROPOSER_ROLE(), fortisGovernance.address);
  await timelock.grantRole(await timelock.EXECUTOR_ROLE(), fortisGovernance.address);
  await timelock.revokeRole(await timelock.TIMELOCK_ADMIN_ROLE(), deployer.address);
  console.log("✅ TimelockController roles configured");

  // Setup FortisVoting roles
  await fortisVoting.grantRole(await fortisVoting.ADMIN_ROLE(), deployer.address);
  await fortisVoting.grantRole(await fortisVoting.MINISTER_ROLE(), deployer.address);
  await fortisVoting.grantRole(await fortisVoting.AUDITOR_ROLE(), deployer.address);
  await fortisVoting.grantRole(await fortisVoting.NODE_ROLE(), deployer.address);
  console.log("✅ FortisVoting roles configured");

  // Setup FortisAudit roles
  await fortisAudit.grantRole(await fortisAudit.ADMIN_ROLE(), deployer.address);
  await fortisAudit.grantRole(await fortisAudit.MINISTER_ROLE(), deployer.address);
  await fortisAudit.grantRole(await fortisAudit.AUDITOR_ROLE(), deployer.address);
  console.log("✅ FortisAudit roles configured");

  // Setup FortisIdentity roles
  await fortisIdentity.grantRole(await fortisIdentity.ADMIN_ROLE(), deployer.address);
  console.log("✅ FortisIdentity roles configured");

  // Step 8: Mint initial tokens
  console.log("\n💰 Step 8: Minting initial tokens...");
  const mintAmount = ethers.utils.parseEther("1000000"); // 1M tokens
  await fortisToken.mint(deployer.address, mintAmount);
  await fortisToken.delegate(deployer.address);
  console.log("✅ Initial tokens minted and delegated");

  // Step 9: Save deployment info
  console.log("\n💾 Step 9: Saving deployment information...");
  
  const deploymentInfo = {
    network: await ethers.provider.getNetwork(),
    deployer: deployer.address,
    deploymentDate: new Date().toISOString(),
    blockNumber: await ethers.provider.getBlockNumber(),
    contracts: {
      FortisToken: {
        address: fortisToken.address,
        transactionHash: fortisToken.deployTransaction.hash
      },
      TimelockController: {
        address: timelock.address,
        transactionHash: timelock.deployTransaction.hash
      },
      FortisGovernance: {
        address: fortisGovernance.address,
        transactionHash: fortisGovernance.deployTransaction.hash
      },
      FortisVoting: {
        address: fortisVoting.address,
        transactionHash: fortisVoting.deployTransaction.hash
      },
      FortisAudit: {
        address: fortisAudit.address,
        transactionHash: fortisAudit.deployTransaction.hash
      },
      FortisIdentity: {
        address: fortisIdentity.address,
        transactionHash: fortisIdentity.deployTransaction.hash
      }
    }
  };

  console.log("✅ Deployment information saved");
  console.log("\n🎉 Migração inicial FORTIS concluída com sucesso!");
  
  console.log("\n📋 Resumo dos contratos deployados:");
  console.log("🪙 FortisToken:", fortisToken.address);
  console.log("⏰ TimelockController:", timelock.address);
  console.log("🏛️ FortisGovernance:", fortisGovernance.address);
  console.log("🗳️ FortisVoting:", fortisVoting.address);
  console.log("📊 FortisAudit:", fortisAudit.address);
  console.log("🆔 FortisIdentity:", fortisIdentity.address);

  console.log("\n📝 Próximos passos:");
  console.log("1. Verificar contratos no Polygonscan");
  console.log("2. Configurar frontend com os endereços");
  console.log("3. Testar funcionalidades básicas");
  console.log("4. Deploy em testnet para testes");
  console.log("5. Integração com backend");

  return deploymentInfo;
}

main()
  .then((deploymentInfo) => {
    console.log("\n💾 Deployment info:", JSON.stringify(deploymentInfo, null, 2));
    process.exit(0);
  })
  .catch((error) => {
    console.error("❌ Erro durante a migração:", error);
    process.exit(1);
  });
