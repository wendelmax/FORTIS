import { expect } from "chai";
import { ethers } from "hardhat";
import { FortisVoting } from "../typechain-types";

describe("FortisVoting", function () {
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

  describe("Election Management", function () {
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

    it("Should activate an election", async function () {
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

    it("Should complete an election", async function () {
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

  describe("Candidate Management", function () {
    beforeEach(async function () {
      const startTime = Math.floor(Date.now() / 1000) + 3600;
      const endTime = startTime + 86400;

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );
    });

    it("Should add a candidate to an election", async function () {
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

    it("Should not add candidate to completed election", async function () {
      // Complete the election first
      await ethers.provider.send("evm_increaseTime", [3601]);
      await ethers.provider.send("evm_mine", []);
      await fortisVoting.connect(admin).activateElection(1);
      
      await ethers.provider.send("evm_increaseTime", [86401]);
      await ethers.provider.send("evm_mine", []);
      await fortisVoting.connect(admin).completeElection(1, "0x0", "QmHash");

      await expect(
        fortisVoting.connect(admin).addCandidate(
          1,
          "João Silva",
          "PT",
          "Prefeito",
          "13",
          "",
          ""
        )
      ).to.be.revertedWith("Cannot add candidates to completed election");
    });
  });

  describe("Voting", function () {
    beforeEach(async function () {
      const startTime = Math.floor(Date.now() / 1000) + 7200; // 2 hours from now
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
      await ethers.provider.send("evm_increaseTime", [7201]);
      await ethers.provider.send("evm_mine", []);
      await fortisVoting.connect(admin).activateElection(1);
    });

    it("Should cast a vote", async function () {
      const candidateId = 1;
      const encryptedVote = "encrypted_vote_data";
      const zkProof = "zk_proof_data";
      const nullifier = "unique_nullifier_1";

      await expect(
        fortisVoting.connect(voter1).castVote(
          1,
          candidateId,
          encryptedVote,
          zkProof,
          nullifier
        )
      )
        .to.emit(fortisVoting, "VoteCast")
        .withArgs(1, candidateId, voter1.address, encryptedVote, zkProof);

      const candidate = await fortisVoting.getCandidate(candidateId);
      expect(candidate.votesCount).to.equal(1);

      const election = await fortisVoting.getElection(1);
      expect(election.totalVotes).to.equal(1);
    });

    it("Should not allow unregistered voter to vote", async function () {
      const unregisteredVoter = await ethers.getSigner();
      const candidateId = 1;

      await expect(
        fortisVoting.connect(unregisteredVoter).castVote(
          1,
          candidateId,
          "encrypted_vote_data",
          "zk_proof_data",
          "unique_nullifier_1"
        )
      ).to.be.revertedWith("Voter not registered");
    });
  });

  describe("Access Control", function () {
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
  });

  describe("Election Results", function () {
    beforeEach(async function () {
      const startTime = Math.floor(Date.now() / 1000) + 10800; // 3 hours from now
      const endTime = startTime + 86400;

      await fortisVoting.connect(admin).createElection(
        "Test Election",
        "Test Description",
        startTime,
        endTime
      );

      // Add candidates
      await fortisVoting.connect(admin).addCandidate(1, "João Silva", "PT", "Prefeito", "13", "", "");
      await fortisVoting.connect(admin).addCandidate(1, "Maria Santos", "PSDB", "Prefeito", "45", "", "");

      // Activate election
      await ethers.provider.send("evm_increaseTime", [10801]);
      await ethers.provider.send("evm_mine", []);
      await fortisVoting.connect(admin).activateElection(1);
    });

    it("Should return correct election results", async function () {
      // Cast votes
      await fortisVoting.connect(voter1).castVote(1, 1, "encrypted_vote_1", "zk_proof_1", "nullifier_1");
      await fortisVoting.connect(voter2).castVote(1, 2, "encrypted_vote_2", "zk_proof_2", "nullifier_2");

      const results = await fortisVoting.getElectionResults(1);
      expect(results.length).to.equal(2);
      expect(results[0].votesCount).to.equal(1);
      expect(results[1].votesCount).to.equal(1);
    });
  });
});