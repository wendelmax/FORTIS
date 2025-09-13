import { expect } from "chai";
import { ethers } from "hardhat";
import { FortisVoting } from "../typechain-types";

describe("FortisVoting - Final 100% Tests", function () {
  let fortisVoting: FortisVoting;
  let owner: any;
  let admin: any;
  let voter1: any;
  let voter2: any;
  let auditor: any;

  beforeEach(async function () {
    [owner, admin, voter1, voter2, auditor] = await ethers.getSigners();

    const FortisVoting = await ethers.getContractFactory("FortisVoting");
    fortisVoting = await FortisVoting.deploy();
    await fortisVoting.waitForDeployment();

    // Grant admin role
    await fortisVoting.grantRole(await fortisVoting.ADMIN_ROLE(), admin.address);
    await fortisVoting.grantRole(await fortisVoting.AUDITOR_ROLE(), auditor.address);

    // Register voters
    await fortisVoting.connect(admin).registerVoter(voter1.address);
    await fortisVoting.connect(admin).registerVoter(voter2.address);
  });

  describe("Basic Functionality", function () {
    it("Should create a new election", async function () {
      const title = "Eleição Municipal 2025";
      const description = "Eleição para prefeito e vereadores";
      const startTime = Math.floor(Date.now() / 1000) + 3600; // 1 hour from now
      const endTime = startTime + 86400; // 24 hours later

      await expect(
        fortisVoting.connect(admin).createElection(
          title,
          description,
          startTime,
          endTime
        )
      )
        .to.emit(fortisVoting, "ElectionCreated")
        .withArgs(1, title, startTime, endTime, admin.address);

      const election = await fortisVoting.getElection(1);
      expect(election.title).to.equal(title);
      expect(election.description).to.equal(description);
      expect(election.startTime).to.equal(startTime);
      expect(election.endTime).to.equal(endTime);
      expect(election.isActive).to.be.false;
      expect(election.isCompleted).to.be.false;
    });

    it("Should add a candidate to an election", async function () {
      const startTime = Math.floor(Date.now() / 1000) + 3600;
      const endTime = startTime + 86400;

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );

      await expect(
        fortisVoting.connect(admin).addCandidate(
          1,
          "João Silva",
          "PT",
          "Prefeito",
          "13",
          "https://example.com/photo.jpg",
          "Candidato a prefeito"
        )
      )
        .to.emit(fortisVoting, "CandidateAdded")
        .withArgs(1, 1, "João Silva", "PT", "13");

      const candidate = await fortisVoting.getCandidate(1);
      expect(candidate.name).to.equal("João Silva");
      expect(candidate.party).to.equal("PT");
      expect(candidate.position).to.equal("Prefeito");
      expect(candidate.number).to.equal("13");
      expect(candidate.electionId).to.equal(1);
      expect(candidate.isActive).to.be.true;
      expect(candidate.votesCount).to.equal(0);
    });

    it("Should register and remove voters", async function () {
      const [owner, admin, voter1, voter2, auditor, newVoter] = await ethers.getSigners();
      
      // Register voter
      await expect(
        fortisVoting.connect(admin).registerVoter(newVoter.address)
      )
        .to.emit(fortisVoting, "VoterRegistered")
        .withArgs(newVoter.address, admin.address);

      expect(await fortisVoting.isVoterRegistered(newVoter.address)).to.be.true;

      // Remove voter
      await expect(
        fortisVoting.connect(admin).removeVoter(newVoter.address)
      )
        .to.emit(fortisVoting, "VoterRemoved")
        .withArgs(newVoter.address);

      expect(await fortisVoting.isVoterRegistered(newVoter.address)).to.be.false;
    });

    it("Should only allow admin to create elections", async function () {
      await expect(
        fortisVoting.connect(voter1).createElection(
          "Test Election",
          "Test Description",
          Math.floor(Date.now() / 1000) + 3600,
          Math.floor(Date.now() / 1000) + 86400
        )
      ).to.be.revertedWith("Not authorized");
    });

    it("Should only allow admin to register voters", async function () {
      await expect(
        fortisVoting.connect(voter1).registerVoter(voter2.address)
      ).to.be.revertedWith("Not authorized");
    });

    it("Should only allow admin to remove voters", async function () {
      await expect(
        fortisVoting.connect(voter1).removeVoter(voter2.address)
      ).to.be.revertedWith("Not authorized");
    });

    it("Should get election count", async function () {
      expect(await fortisVoting.getElectionCount()).to.equal(0);

      const startTime = Math.floor(Date.now() / 1000) + 3600;
      const endTime = startTime + 86400;

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );

      expect(await fortisVoting.getElectionCount()).to.equal(1);
    });

    it("Should get candidate count", async function () {
      expect(await fortisVoting.getCandidateCount()).to.equal(0);

      const startTime = Math.floor(Date.now() / 1000) + 3600;
      const endTime = startTime + 86400;

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );

      await fortisVoting.connect(admin).addCandidate(
        1,
        "João Silva",
        "PT",
        "Prefeito",
        "13",
        "",
        ""
      );

      expect(await fortisVoting.getCandidateCount()).to.equal(1);
    });

    it("Should check if voter is registered", async function () {
      expect(await fortisVoting.isVoterRegistered(voter1.address)).to.be.true;
      expect(await fortisVoting.isVoterRegistered(owner.address)).to.be.false;
    });

    it("Should check if voter has voted", async function () {
      const startTime = Math.floor(Date.now() / 1000) + 3600;
      const endTime = startTime + 86400;

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );

      expect(await fortisVoting.hasVoterVoted(1, voter1.address)).to.be.false;
    });

    it("Should get election candidates", async function () {
      const startTime = Math.floor(Date.now() / 1000) + 3600;
      const endTime = startTime + 86400;

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );

      await fortisVoting.connect(admin).addCandidate(1, "João Silva", "PT", "Prefeito", "13", "", "");
      await fortisVoting.connect(admin).addCandidate(1, "Maria Santos", "PSDB", "Prefeito", "45", "", "");

      const candidates = await fortisVoting.getElectionCandidates(1);
      expect(candidates.length).to.equal(2);
      expect(candidates[0]).to.equal(1);
      expect(candidates[1]).to.equal(2);
    });

    it("Should get election results", async function () {
      const startTime = Math.floor(Date.now() / 1000) + 3600;
      const endTime = startTime + 86400;

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );

      await fortisVoting.connect(admin).addCandidate(1, "João Silva", "PT", "Prefeito", "13", "", "");
      await fortisVoting.connect(admin).addCandidate(1, "Maria Santos", "PSDB", "Prefeito", "45", "", "");

      const results = await fortisVoting.getElectionResults(1);
      expect(results.length).to.equal(2);
      expect(results[0].name).to.equal("João Silva");
      expect(results[1].name).to.equal("Maria Santos");
    });
  });

  describe("Time-based Operations", function () {
    it("Should activate election when start time is reached", async function () {
      const startTime = Math.floor(Date.now() / 1000) + 1; // 1 second from now
      const endTime = startTime + 86400;

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );

      // Fast forward time to start time
      await ethers.provider.send("evm_increaseTime", [2]);
      await ethers.provider.send("evm_mine", []);

      await expect(
        fortisVoting.connect(admin).activateElection(1)
      )
        .to.emit(fortisVoting, "ElectionUpdated")
        .withArgs(1, "Test Election", true, false);

      const election = await fortisVoting.getElection(1);
      expect(election.isActive).to.be.true;
    });

    it("Should complete election when end time is reached", async function () {
      const startTime = Math.floor(Date.now() / 1000) + 1; // 1 second from now
      const endTime = startTime + 3; // 3 seconds later

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );

      // Activate election
      await ethers.provider.send("evm_increaseTime", [2]);
      await ethers.provider.send("evm_mine", []);
      await fortisVoting.connect(admin).activateElection(1);

      // Complete election
      await ethers.provider.send("evm_increaseTime", [2]);
      await ethers.provider.send("evm_mine", []);

      const merkleRoot = "0x1234567890abcdef";
      const ipfsHash = "QmTestHash";

      await expect(
        fortisVoting.connect(admin).completeElection(1, merkleRoot, ipfsHash)
      )
        .to.emit(fortisVoting, "ElectionCompleted")
        .withArgs(1, merkleRoot, ipfsHash);

      const election = await fortisVoting.getElection(1);
      expect(election.isActive).to.be.false;
      expect(election.isCompleted).to.be.true;
      expect(election.merkleRoot).to.equal(merkleRoot);
      expect(election.ipfsHash).to.equal(ipfsHash);
    });
  });

  describe("Voting with Time Management", function () {
    it("Should cast vote in active election", async function () {
      const startTime = Math.floor(Date.now() / 1000) + 1; // 1 second from now
      const endTime = startTime + 86400;

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );

      await fortisVoting.connect(admin).addCandidate(
        1,
        "João Silva",
        "PT",
        "Prefeito",
        "13",
        "",
        ""
      );

      // Activate election
      await ethers.provider.send("evm_increaseTime", [2]);
      await ethers.provider.send("evm_mine", []);
      await fortisVoting.connect(admin).activateElection(1);

      const encryptedVote = "encrypted_vote_data";
      const zkProof = "zk_proof_data";
      const nullifier = "unique_nullifier_1";

      await expect(
        fortisVoting.connect(voter1).castVote(
          1,
          1,
          encryptedVote,
          zkProof,
          nullifier
        )
      )
        .to.emit(fortisVoting, "VoteCast")
        .withArgs(1, 1, voter1.address, encryptedVote, zkProof);

      const candidate = await fortisVoting.getCandidate(1);
      expect(candidate.votesCount).to.equal(1);

      const election = await fortisVoting.getElection(1);
      expect(election.totalVotes).to.equal(1);
    });
  });
});
