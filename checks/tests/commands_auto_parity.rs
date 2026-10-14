fn refuse_list_step0_stops_in_auto_naming_the_command_and_why() {
    // Arrange
    let mut failures = Vec::new();

    for spec in rows(Auto::Refuses) {
        let file = load(spec.name);
        let Some(step0) = file.step0() else {
            failures.push(format!("{}: no `Step 0` section", spec.name));
            continue;
        };
        let command = format!("/playbook:{}", spec.name);
        let blocks = file.paragraphs(&step0);

        // Act
        let read = blocks.iter().find(|(_, b)| reads_mode(b));
        let refusal = blocks.iter().find(|(_, b)| {
            let b = b.to_lowercase();
            b.contains("auto")
                && b.contains("stop")
                && b.contains(&command)
                && b.contains("because")
        });

        // Assert
        let Some((refusal_line, _)) = refusal else {
            failures.push(format!(
                "{}: Step 0 has no refusal that mentions auto, stop, `{command}` and because",
                spec.name
            ));
            continue;
        };
        match read {
            None => failures.push(format!("{}: Step 0 never reads the mode", spec.name)),
            Some((read_line, _)) if read_line > refusal_line => failures.push(format!(
                "{}: the refusal (line {}) precedes the mode read (line {})",
                spec.name,
                refusal_line + 1,
                read_line + 1
            )),
            Some(_) => {}
        }
        if let Some((line, text)) = file.first_ask_outside(None) {
            if line < *refusal_line {
                failures.push(format!(
                    "{}: ask on line {} precedes the refusal (line {}): {}",
                    spec.name,
                    line + 1,
                    refusal_line + 1,
                    text.trim()
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "refusal problems:\n{}",
        failures.join("\n")
    );
}

// ---------------------------------------------------------------------------
// quick-review and deep-review: auto never posts to GitHub
// ---------------------------------------------------------------------------

/// The section that creates and submits the GitHub review.
fn posting_section(file: &CommandFile) -> Section {
    require_section(
        file,
        file.section(|t| t.to_lowercase().contains("orchestrate posting")),
        "`Orchestrate posting`",
    )
}

/// A shell line that calls the GitHub API to create or submit a review.
fn is_post_call(line: &str) -> bool {
    static POST: OnceLock<Regex> = OnceLock::new();
    POST.get_or_init(|| Regex::new(r"gh api[^\n]*(-X POST|/reviews)").expect("post pattern"))
        .is_match(line)
}

#[test]
fn review_step0_says_auto_does_not_skip_the_post_question() {
    // Arrange
    let rule = Regex::new(r"(?is)\bauto\b[^.]*\bdoes not change posting\b[^.]*\.[^.]*--self")
        .expect("rule pattern");
    let old = Regex::new(r"(?i)auto[^.]*implies[^.]*--self").expect("old pattern");
    let mut failures = Vec::new();

    for spec in rows(Auto::ReportsOnly) {
        let file = load(spec.name);

        // Act
        let text = file.text_of(&file.step0().expect("Step 0"));

        // Assert
        if !rule.is_match(&text) {
            failures.push(format!("{}: Step 0 lacks the auto posting rule", spec.name));
        }
        if old.is_match(&text) {
            failures.push(format!(
                "{}: Step 0 still says auto implies --self",
                spec.name
            ));
        }
    }

    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn review_posting_section_is_gated_on_not_self_and_holds_every_post_call() {
    // Arrange
    let mut failures = Vec::new();

    for spec in rows(Auto::ReportsOnly) {
        let file = load(spec.name);
        let posting = posting_section(&file);

        // Act
        let gate = file
            .paragraphs(&posting)
            .into_iter()
            .find(|(_, b)| b.contains("SELF_MODE") && b.to_lowercase().contains("stop"));
        let outside: Vec<usize> = (file.body_start..file.lines.len())
            .filter(|i| !(posting.start..posting.end).contains(i))
            .filter(|&i| is_post_call(&file.lines[i]))
            .collect();

        // Assert
        match gate {
            None => failures.push(format!(
                "{}: `{}` has no `SELF_MODE` ... stop gate",
                spec.name, posting.title
            )),
            Some((gate_line, _)) => {
                let reached_first = (posting.start..gate_line)
                    .find(|&i| is_post_call(&file.lines[i]) || is_ask(&file.lines[i]));
                if let Some(i) = reached_first {
                    failures.push(format!(
                        "{}: line {} posts or asks before the `SELF_MODE` gate (line {})",
                        spec.name,
                        i + 1,
                        gate_line + 1
                    ));
                }
            }
        }
        if !outside.is_empty() {
            failures.push(format!(
                "{}: review post calls outside `{}` on lines {:?}",
                spec.name,
                posting.title,
                outside.iter().map(|i| i + 1).collect::<Vec<_>>()
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "posting gate problems:\n{}",
        failures.join("\n")
    );
}

#[test]
fn review_auto_path_text_never_posts_a_review() {
    // Arrange
    let mut failures = Vec::new();

    for spec in rows(Auto::ReportsOnly) {
        let file = load(spec.name);
        let posting = posting_section(&file);
        let Some(step0) = file.step0() else {
            failures.push(format!(
                "{}: no `Step 0` section, so no auto path to inspect",
                spec.name
            ));
            continue;
        };
        let auto_text: Vec<Section> = [Some(step0), file.auto_path()]
            .into_iter()
            .flatten()
            .collect();

        // Act
        for section in &auto_text {
            let overlaps = section.start < posting.end && posting.start < section.end;
            let posts = (section.start..section.end).any(|i| is_post_call(&file.lines[i]));

            // Assert
            if overlaps {
                failures.push(format!(
                    "{}: `{}` overlaps the posting section",
                    spec.name, section.title
                ));
            }
            if posts {
                failures.push(format!(
                    "{}: `{}` contains a call that posts a review",
                    spec.name, section.title
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "auto path problems:\n{}",
        failures.join("\n")
    );
}

// ---------------------------------------------------------------------------
// commit-and-push
// ---------------------------------------------------------------------------

#[test]
fn commit_and_push_auto_path_never_forces_and_parks_when_a_force_is_needed() {
    // Arrange
    let file = load("commit-and-push");
    let auto_path = require_section(&file, file.auto_path(), "`Auto path`");

    // Act
    let text = file.text_of(&auto_path);

    // Assert
    assert!(
        !text.contains("--force"),
        "the auto path must not mention --force or --force-with-lease"
    );
    let stops = Regex::new(r"(?is)\bforce\b[^.]*\bstops and reports\b").expect("stop pattern");
    assert!(
        stops.is_match(&text),
        "the auto path must say a push that would need a force stops and reports"
    );
}

#[test]
fn commit_and_push_auto_path_pushes_plainly_when_the_remote_branch_is_absent() {
    // Arrange
    let file = load("commit-and-push");
    let auto_path = require_section(&file, file.auto_path(), "`Auto path`");

    // Act
    let text = file.text_of(&auto_path);

    // Assert
    assert!(
        text.contains("git ls-remote"),
        "the auto path must check the remote branch with `git ls-remote`"
    );
    let plain = Regex::new(
        r"(?is)\b(absent|missing)\b[^.]*\bplain(ly)?\b|\bplain(ly)?\b[^.]*\b(absent|missing)\b",
    )
    .expect("plain push pattern");
    assert!(
        plain.is_match(&text),
        "the auto path must say it pushes plainly when the remote branch is absent"
    );
}

// ---------------------------------------------------------------------------
// learn-project
// ---------------------------------------------------------------------------

#[test]
fn learn_project_step0_stages_in_auto_and_skips_the_write_question() {
    // Arrange
    let file = load("learn-project");
    let step0 = require_section(&file, file.step0(), "`Step 0`");
    let stages = Regex::new(r"(?is)\bauto\b[^.]*--stage").expect("stage pattern");
    let skips = Regex::new(
        r"(?is)\bskips?\b[^.]*(write question|Write these to memory)|(write question|Write these to memory)[^.]*\bskipped\b",
    )
    .expect("skip pattern");

    // Act
    let text = file.text_of(&step0);

    // Assert
    assert!(
        stages.is_match(&text),
        "Step 0 must say auto behaves as `--stage`"
    );
    assert!(
        skips.is_match(&text),
        "Step 0 must say the write question is skipped in that path"
    );
}

// ---------------------------------------------------------------------------
// create-pull-request
// ---------------------------------------------------------------------------

#[test]
fn create_pull_request_step0_skips_the_self_review_in_auto() {
    // Arrange
    let file = load("create-pull-request");
    let step0 = require_section(&file, file.step0(), "`Step 0`");
    let skips = Regex::new(r"(?is)\bauto\b[^.]*\bskips?\b[^.]*self-review").expect("skip pattern");
    let why = Regex::new(r"(?is)\bimplement\b[^.]*\bopened\b[^.]*\breview\b[^.]*\bcovered\b")
        .expect("why pattern");
    let reports =
        Regex::new(r"(?is)final report[^.]*\bno self-review ran\b").expect("report pattern");

    // Act
    let text = file.text_of(&step0);

    // Assert
    assert!(
        skips.is_match(&text),
        "Step 0 must say that in auto the self-review is skipped"
    );
    assert!(
        why.is_match(&text),
        "Step 0 must say that implement's review covered a PR that implement opened"
    );
    assert!(
        reports.is_match(&text),
        "Step 0 must say the final report states that no self-review ran for other callers"
    );
}

// ---------------------------------------------------------------------------
// repo-audit, doctor, session-start: no mode block at all
// ---------------------------------------------------------------------------

#[test]
fn commands_that_never_ask_carry_no_mode_block() {
    // Arrange
    let mut failures = Vec::new();

    for spec in rows(Auto::Unchanged) {
        let file = load(spec.name);

        // Act
        if let Some((line, text)) = file.first_ask_outside(None) {
            failures.push(format!(
                "{}: expected no asks, but line {} matches: {}",
                spec.name,
                line + 1,
                text.trim()
            ));
        }
        if file.step0().is_some() {
            failures.push(format!(
                "{}: has a `Step 0`, but behaves the same in either mode",
                spec.name
            ));
        }
    }

    // Assert
    assert!(
        failures.is_empty(),
        "no-mode-block problems:\n{}",
        failures.join("\n")
    );
}

// ---------------------------------------------------------------------------
// fix
// ---------------------------------------------------------------------------

/// The `escalat` section of `fix.md`, where the hand-off rules live.
fn escalation_section(file: &CommandFile) -> Section {
    require_section(
        file,
        file.section(|t| t.to_lowercase().contains("escalat")),
        "escalation",
    )
}

#[test]
fn fix_command_file_exists() {
    // Arrange
    let path = commands_dir().join("fix.md");

    // Act
    let exists = path.is_file();

    // Assert
    assert!(exists, "{} does not exist", path.display());
}

#[test]
fn fix_frontmatter_has_description_tools_and_the_argument_hint() {
    // Arrange
    let file = load("fix");

    // Act
    let description = file.frontmatter_value("description");
    let hint = file.frontmatter_value("argument-hint");
    let tools = file.allowed_tools();

    // Assert
    assert!(
        description.is_some_and(|d| !d.is_empty()),
        "fix needs a non-empty description"
    );
    assert!(
        hint.as_deref()
            .is_some_and(|h| h.contains("[#issue | description] [--auto] [--ask]")),
        "fix argument-hint {hint:?} must contain `[#issue | description] [--auto] [--ask]`"
    );
    for needed in ["Bash", "Read", "Skill"] {
        assert!(
            tools.iter().any(|t| t == needed),
            "fix allowed-tools {tools:?} lacks {needed}"
        );
    }
}

#[test]
fn fix_escalation_names_all_five_rules() {
    // Arrange
    let file = load("fix");
    let section = escalation_section(&file);
    let text = file.text_of(&section);
    let rules = [
        (
            "more than fix.maxFiles files",
            r"(?is)more than[^.]*fix\.maxFiles[^.]*files",
        ),
        (
            "more than fix.maxLines lines, excluding the new test",
            r"(?is)more than[^.]*fix\.maxLines[^.]*excluding the new test",
        ),
        (
            "root cause unclear after two tested hypotheses",
            r"(?is)root cause[^.]*unclear[^.]*two tested hypotheses",
        ),
        (
            "new dependency, schema, config format or public interface",
            r"(?is)new dependency[^.]*schema[^.]*config[- ]format[^.]*public[- ]interface",
        ),
        ("no failing test possible", r"(?i)\bno failing test\b"),
    ];

    // Act
    let missing: Vec<&str> = rules
        .iter()
        .filter(|(_, pattern)| !Regex::new(pattern).expect("rule pattern").is_match(&text))
        .map(|(name, _)| *name)
        .collect();

    // Assert
    assert!(
        missing.is_empty(),
        "the `{}` section does not state these escalation rules: {missing:?}",
        section.title
    );
}

#[test]
fn fix_reads_both_thresholds_through_the_config_get_intent() {
    // Arrange
    let file = load("fix");
    let text = file.lines[file.body_start..].join("\n");

    // Act
    let missing: Vec<&str> = ["fix.maxFiles", "fix.maxLines"]
        .into_iter()
        .filter(|key| {
            !text
                .lines()
                .any(|l| l.contains("Intent: config-get") && l.contains(*key))
        })
        .collect();

    // Assert
    assert!(
        missing.is_empty(),
        "fix never reads the config-get intent for {missing:?}"
    );
}

#[test]
fn fix_escalation_hands_off_to_plan_and_states_which_rule_fired() {
    // Arrange
    let file = load("fix");
    let section = escalation_section(&file);
    let text = file.text_of(&section);
    let states_rule = Regex::new(r"(?is)\bwhich rule (fired|triggered)\b").expect("rule pattern");

    // Act
    let hands_off = text.contains("/playbook:plan");

    // Assert
    assert!(
        hands_off,
        "the `{}` section must hand off to `/playbook:plan`",
        section.title
    );
    assert!(
        states_rule.is_match(&text),
        "the `{}` section must say it states which rule fired",
        section.title
    );
}

#[test]
fn fix_every_ask_sits_behind_an_if_mode_is_ask_branch() {
    // Arrange
    let mut failures = Vec::new();

    for spec in rows(Auto::AsksOnlyInAskMode) {
        let file = load(spec.name);
        let fenced = fence_flags(&file.lines);

        // Act
        for (i, line) in file.lines.iter().enumerate().skip(file.body_start) {
            if fenced[i] || !is_ask(line) {
                continue;
            }

            // Assert
            if !is_behind_ask_branch(&file.lines, file.body_start, i) {
                failures.push(format!(
                    "{}: line {} asks outside an `{ASK_BRANCH}` branch: {}",
                    spec.name,
                    i + 1,
                    line.trim()
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "unconditional asks:\n{}",
        failures.join("\n")
    );
}

#[test]
fn ask_branch_parser_accepts_headings_and_bullets_and_rejects_the_rest() {
    // Arrange
    let lines = |text: &str| -> Vec<String> { text.lines().map(str::to_owned).collect() };
    let behind = [
        "### If mode is ask\nAsk the user which test to keep.",
        "## Step 3\n- If mode is ask, ask the user which test to keep.",
        "## Step 3\n- If mode is ask:\n  - Ask the user which test to keep.",
        "## Step 3\n- If mode is ask:\n  Ask the user which test to keep.",
    ];
    let not_behind = [
        "## Step 3\nAsk the user which test to keep.",
        "## Step 3\n- Ask the user which test to keep.",
        "### If mode is ask\n## Step 4\nAsk the user which test to keep.",
        "- If mode is ask:\n  - Pick one.\n\n- Ask the user which test to keep.",
        "## Step 3\n```\n# if mode is ask\n```\nAsk the user which test to keep.",
    ];
    let ask_index = |l: &[String]| {
        l.iter()
            .position(|x| is_ask(x))
            .expect("fixture has an ask")
    };

    // Act
    let missed: Vec<&&str> = behind
        .iter()
        .filter(|t| {
            let l = lines(t);
            !is_behind_ask_branch(&l, 0, ask_index(&l))
        })
        .collect();
    let false_hits: Vec<&&str> = not_behind
        .iter()
        .filter(|t| {
            let l = lines(t);
            is_behind_ask_branch(&l, 0, ask_index(&l))
        })
        .collect();

    // Assert
    assert!(missed.is_empty(), "parser rejected gated asks: {missed:?}");
    assert!(
        false_hits.is_empty(),
        "parser accepted ungated asks: {false_hits:?}"
    );
}

#[test]
fn fix_opens_its_pull_request_through_the_create_pull_request_skill() {
    // Arrange
    let file = load("fix");
    let fenced = fence_flags(&file.lines);
    let tools = file.allowed_tools();

    // Act
    let names_skill = file.lines[file.body_start..]
        .iter()
        .any(|l| l.contains("/playbook:create-pull-request"));
    let raw_calls: Vec<usize> = (file.body_start..file.lines.len())
        .filter(|&i| fenced[i] && file.lines[i].contains("gh pr create"))
        .map(|i| i + 1)
        .collect();

    // Assert
    assert!(
        names_skill,
        "fix must open its PR through `/playbook:create-pull-request`"
    );
    assert!(
        tools.iter().any(|t| t == "Skill"),
        "fix allowed-tools {tools:?} lacks Skill, so it cannot invoke the skill"
    );
    assert!(
        raw_calls.is_empty(),
        "fix runs `gh pr create` directly on lines {raw_calls:?}"
    );
}

// ---------------------------------------------------------------------------
// Comments describe the code, not the work that produced it
// ---------------------------------------------------------------------------

/// True when `text` points at internal planning: a work-unit label, a
/// numbered ticket or a planning term that means nothing to a later reader.
fn references_planning(text: &str) -> bool {
    static PLANNING: OnceLock<Regex> = OnceLock::new();
    PLANNING
        .get_or_init(|| {
            Regex::new(
                r"(?i)\bWU-?\d+\b|\bwork units?\b|\bgh-\d+\b|#\d+\b|\bissue\s+\d+\b|\bsegment\s+S\d+\b|\bthe brief\b",
            )
            .expect("planning reference pattern")
        })
        .is_match(text)
}

#[test]
fn planning_reference_scan_flags_references_and_leaves_plain_comments() {
    // Arrange
    let flagged = [
        "// ported in WU-16",
        "# see gh-123 for the design",
        "<!-- Work Unit 3 only -->",
        "# fixes #123",
        "// the brief says so",
    ];
    let clean = [
        "// a section ends at the next heading of the same level",
        "# Set each flag from $ARGUMENTS: true when the flag was passed",
        "# 1) read the mode first",
        "<!-- keep in sync with the help block -->",
    ];

    // Act
    let missed: Vec<&&str> = flagged.iter().filter(|t| !references_planning(t)).collect();
    let false_hits: Vec<&&str> = clean.iter().filter(|t| references_planning(t)).collect();

    // Assert
    assert!(missed.is_empty(), "scan missed references: {missed:?}");
    assert!(
        false_hits.is_empty(),
        "scan flagged plain comments: {false_hits:?}"
    );
}

#[test]
fn comments_in_this_test_and_the_commands_carry_no_planning_references() {
    // Arrange
    let mut offenders = Vec::new();
    let own_source = include_str!("commands_auto_parity.rs");
    assert!(
        COMMANDS
            .iter()
            .any(|spec| !load(spec.name).comments().is_empty()),
        "the comment extractor found no comment in any command"
    );

    // Act
    for (i, line) in own_source.lines().enumerate() {
        if line.trim_start().starts_with("//") && references_planning(line) {
            offenders.push(format!(
                "tests/commands_auto_parity.rs:{}: {}",
                i + 1,
                line.trim()
            ));
        }
    }
    for spec in COMMANDS {
        let file = load(spec.name);
        for (i, text) in file.comments() {
            if references_planning(&text) {
                offenders.push(format!(
                    "commands/{}.md:{}: {}",
                    spec.name,
                    i + 1,
                    text.trim()
                ));
            }
        }
    }

    // Assert
    assert!(
        offenders.is_empty(),
        "comments reference planning artefacts:\n{}",
        offenders.join("\n")
    );
}
