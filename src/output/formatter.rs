use crate::vcs::Vcs;
use anyhow::Result;

pub fn display_message(message: &str) {
    println!("\n{}", "=".repeat(60));
    println!("Generated commit message:");
    println!("{}", "=".repeat(60));
    println!("{}", message.trim());
    println!("{}", "=".repeat(60));
}

pub fn handle_output(
    message: &str,
    dry_run: bool,
    apply: bool,
    vcs: &dyn Vcs,
) -> Result<()> {
    display_message(message);

    if dry_run {
        println!("\n[Dry run] No commit was made.");
        return Ok(());
    }

    if apply {
        println!("\nApplying commit...");
        vcs.commit(message.trim())?;
        println!("Commit applied successfully.");
        return Ok(());
    }

    println!("\nRun with --apply to commit, or --dry-run to preview only.");
    Ok(())
}
