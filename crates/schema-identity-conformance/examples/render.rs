//! Render the deterministic conformance cohort.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let report = schema_identity_conformance::evaluate_fixture()?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
