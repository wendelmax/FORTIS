use anyhow::Result;

pub struct Prover;

impl Prover {
    pub fn prove_vote(voter_id: &str, candidate_id: &str) -> Result<String> {
        Ok("proof".to_string())
    }
}
