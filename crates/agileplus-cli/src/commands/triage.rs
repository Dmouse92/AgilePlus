//! `agileplus triage` command implementation.
//!
//! Pulls the next ticket from the triage queue and records the operator outcome
//! through the hexagonal `TriagePort`.

use anyhow::{Context, Result};
use clap::Args;

use agileplus_domain::ports::{TriageOutcome, TriagePort, TriageTicket};

/// Arguments for the `triage` subcommand.
#[derive(Debug, Args)]
pub struct TriageArgs {
    /// Output format: table (default) or json.
    #[arg(long, default_value = "table")]
    pub output: String,

    /// Outcome to record after pulling the next ticket.
    #[arg(long, default_value = "accepted", value_name = "OUTCOME")]
    pub outcome: String,

    /// Preview the next ticket without recording an outcome.
    #[arg(long)]
    pub peek: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriageRunResult {
    pub ticket: TriageTicket,
    pub recorded_outcome: Option<TriageOutcome>,
}

/// Execute one triage step through the provided port.
pub async fn run<S: TriagePort>(args: &TriageArgs, triage: &S) -> Result<TriageRunResult> {
    let ticket = triage.next_ticket().await.context("fetching next triage ticket")?;
    let recorded_outcome = if args.peek {
        None
    } else {
        let outcome = parse_outcome(&args.outcome)?;
        triage
            .record_outcome(&ticket.id, outcome)
            .await
            .with_context(|| format!("recording triage outcome for ticket {}", ticket.id))?;
        Some(outcome)
    };

    Ok(TriageRunResult {
        ticket,
        recorded_outcome,
    })
}

/// Run the `triage` command and emit the selected output format.
pub async fn run_triage<S: TriagePort>(args: &TriageArgs, triage: &S) -> Result<()> {
    let result = run(args, triage).await?;

    if args.output == "json" {
        println!("{}", serde_json::to_string_pretty(&result.ticket)?);
    } else {
        println!("Ticket:      {}", result.ticket.id);
        println!("Title:       {}", result.ticket.title);
        println!("Intent:      {}", result.ticket.intent);
        println!("Priority:    {}", result.ticket.priority);
        println!("Status:      {}", result.ticket.status);
        println!("Source:      {}", result.ticket.source);
        if let Some(feature_slug) = &result.ticket.feature_slug {
            println!("Feature:     {feature_slug}");
        }
        if !result.ticket.tags.is_empty() {
            println!("Tags:        {}", result.ticket.tags.join(", "));
        }
    }

    match result.recorded_outcome {
        Some(outcome) => println!("\nRecorded outcome: {}", outcome_label(outcome)),
        None => println!("\n(peek mode — no outcome recorded)"),
    }

    Ok(())
}

fn parse_outcome(s: &str) -> Result<TriageOutcome> {
    match s.to_ascii_lowercase().as_str() {
        "accepted" | "accept" => Ok(TriageOutcome::Accepted),
        "dismissed" | "dismiss" => Ok(TriageOutcome::Dismissed),
        other => anyhow::bail!("Unknown outcome '{other}'. Must be: accepted, dismissed"),
    }
}

fn outcome_label(outcome: TriageOutcome) -> &'static str {
    match outcome {
        TriageOutcome::Accepted => "accepted",
        TriageOutcome::Dismissed => "dismissed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn parse_outcome_valid() {
        assert_eq!(parse_outcome("accepted").unwrap(), TriageOutcome::Accepted);
        assert_eq!(parse_outcome("Dismiss").unwrap(), TriageOutcome::Dismissed);
    }

    #[test]
    fn parse_outcome_invalid() {
        assert!(parse_outcome("unknown").is_err());
    }

    proptest! {
        #[test]
        fn parse_intent_accepts_any_letter_case(selector in 0usize..4, upper_flags in proptest::collection::vec(any::<bool>(), 3..=7)) {
            let canonical = ["bug", "feature", "idea", "task"][selector];
            let mixed: String = canonical
                .chars()
                .zip(upper_flags.into_iter().cycle())
                .map(|(ch, upper)| if upper { ch.to_ascii_uppercase() } else { ch })
                .collect();

            prop_assert_eq!(parse_intent(&mixed).unwrap(), parse_intent(canonical).unwrap());
        }

        #[test]
        fn classifier_override_preserves_input_and_forces_confidence(
            input in any::<String>(),
            selector in 0usize..4,
        ) {
            let classifier = TriageClassifier::new();
            let intent = [Intent::Bug, Intent::Feature, Intent::Idea, Intent::Task][selector];
            let result = classifier.classify_with_override(&input, intent);

            prop_assert_eq!(result.intent, intent);
            prop_assert_eq!(result.confidence, 1.0);
            prop_assert_eq!(result.raw_input, input);
            prop_assert_eq!(result.matched_keywords, vec!["user-override".to_string()]);
        }

        #[test]
        fn classifier_output_stays_within_expected_bounds(input in any::<String>()) {
            let classifier = TriageClassifier::new();
            let result = classifier.classify(&input);

            prop_assert!(matches!(
                result.intent,
                Intent::Bug | Intent::Feature | Intent::Idea | Intent::Task
            ));
            prop_assert!((0.0..=1.0).contains(&result.confidence));
            prop_assert_eq!(result.raw_input, input);
        }
    }
}
