import { expect } from "chai";
import { ethers } from "hardhat";
import { FortisVoting } from "../typechain-types";

describe("FortisVoting - Simple Tests", function () {
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

    it("Should register voters", async function () {
      const [owner, admin, voter1, voter2, auditor, newVoter] = await ethers.getSigners();
      
      // Register voter
      await expect(
        fortisVoting.connect(admin).registerVoter(newVoter.address)
      )
        .to.emit(fortisVoting, "VoterRegistered")
        .withArgs(newVoter.address, admin.address);

      expect(await fortisVoting.isVoterRegistered(newVoter.address)).to.be.true;
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

  });
});
