# AGENTS.md

These instructions apply to the complete `thoth-pub/cc-license` repository.

This file **specializes** the canonical Shared Engineering Control doctrine for this
repository. It does not replace it. The canonical doctrine lives in `thoth-pub/thoth`:

- root `AGENTS.md`;
- `docs/engineering/ai-delivery/`;
- `docs/engineering/repository-map/`.

Where this file is silent, the canonical doctrine applies. Where this file is more
restrictive, the more restrictive rule applies. This file must never be read as
relaxing a canonical control.

A more deeply nested `AGENTS.md` adds or narrows instructions for its directory.
Read this file and every applicable nested file before editing.

## 1. Required task identity

Before changing anything, record:

```text
Programme:
Owning GitHub issue:
Repository: thoth-pub/cc-license
Task ID:
Approved specification:
Risk: LOW | MEDIUM | HIGH | CRITICAL
Base branch and exact base commit:
PR target:
Task branch:
Dependencies:
Authorized write paths (existing files):
Authorized new-file paths:
Prohibited paths:
Action authorization: see section 5
Cross-repository impact: see section 5.1
Implementing agent/model:
Independent reviewer/model:
```

Do not implement without an approved written specification. A GitHub issue is
sufficient only when it contains the information required by
`docs/engineering/ai-delivery/task-specification-template.md` in `thoth-pub/thoth`,
including an explicit write budget and action-authorization matrix.

If any item is unknown, treat it as missing work.

GitHub is the live task ledger for this repository: the owning issue, its linked
pull request, review threads and CI hold current lifecycle state.

## 2. Authority

Use this order when sources conflict:

1. merged code in this repository, and the published crate;
2. approved ADRs and technical designs in `thoth-pub/thoth`;
3. approved task specifications;
4. GitHub issues, pull requests, review threads and CI evidence;
5. programme-control and rollout documents;
6. agent reports and conversations.

Do not allow chat history or memory to silently override repository evidence.

Stop and escalate when authoritative sources conflict.

## 3. Repository responsibilities

This repository owns:

- the **public Rust crate `cc_license`** — the canonical Creative Commons licence
  URL parser and metadata provider for the Thoth ecosystem;
- licence URL parsing and validation;
- licence nomenclature and human-readable naming (`src/nomenclature.rs`);
- licence rights/permissions metadata (`src/rights.rs`);
- licence version handling (`src/version.rs`);
- its error type and public error surface (`src/error.rs`);
- its **public Rust API**, exported from `src/lib.rs`.

This is a **published public crate**. Its consumers include repositories outside
this one, and its API and parsing behaviour are a shared contract (section 5.1).

### 3.1 Explicit non-responsibilities

This repository **does not own**, and must not be treated as the authority for:

- Thoth's domain model, database or GraphQL API — owned by `thoth-pub/thoth`;
- metadata export formats or export-server behaviour — owned by
  `thoth-export-server` within `thoth-pub/thoth`;
- how any consumer chooses to present or apply licence information;
- Creative Commons' own canonical licence definitions, which are an external
  upstream authority this crate models but does not define.

Keep licence-domain logic in this crate. Do not duplicate licence parsing in
consumers, and do not push consumer-specific presentation concerns into this crate.

## 4. Branch and pull-request workflow

Current **actual** topology:

```text
develop -> feature/<area>/<task> -> develop
develop -> main   (default / release)
```

Verified branch facts:

- `develop` is the **active development branch** and the normal base for new work;
- `main` is the **default and release branch**.

### 4.1 Branch protection

`develop` is a **protected branch**, currently requiring:

- required status checks — **`test`**, **`format_check`** and **`lint`** — with
  strict (up-to-date-with-base) enforcement;
- at least **1 approving review**;
- dismissal of stale reviews on new commits;
- no force pushes and no branch deletion.

Consequences: a pull request targeting `develop` cannot merge until all three
required checks pass and an independent approval exists. Do not attempt to bypass,
weaken or reconfigure branch protection. Administrator enforcement being disabled is
not permission to bypass it.

**Branch normalization is separate work, tracked as `BR-LIC-01`.** Nothing in this
file asserts that normalization has occurred. Until `BR-LIC-01` completes, the
topology above is the accurate description of this repository, including its
`develop`/`main` naming divergence from `thoth-pub/thoth`'s `develop`/`master`. A
task must not perform branch normalization, default-branch changes or protection
changes as a side effect of unrelated work.

Rules:

- verify the actual base commit immediately before branching, and record it;
- branch from the **exact authorized base SHA**, not from a branch name alone;
- if the base has moved since authorization, **stop** and return HOLD rather than
  silently rebasing onto a newer head;
- use one bounded task per branch and PR;
- do not target normal implementation directly at `main`;
- do not merge or approve your own work;
- do not rewrite shared branch history after others depend on it.

## 5. Granular action authorization

Authorization is granted action-by-action and is **not transitive**. Authorization
for one action never implies authorization for another. A task's specification or
implementation-handoff prompt must state exactly which actions are authorized. Any
action not explicitly authorized is **denied by default**.

Distinct actions:

- repository/GitHub read inspection;
- source/worktree modification within the approved write budget;
- creation of new files at explicitly authorized paths;
- deletion, move or rename of files;
- branch creation;
- commit;
- push;
- pull-request creation/update;
- issue/comment mutation;
- manual CI dispatch or rerun;
- crate version change;
- tag creation;
- GitHub release creation or publication;
- crates.io publication;
- repository-secret access;
- merge;
- deployment;
- production activation.

Without limiting the list above:

- source-write authorization does not include commit authorization;
- commit authorization does not include push authorization;
- push authorization does not include pull-request mutation authorization;
- repository-write authorization does not include GitHub issue/comment mutation
  authorization;
- crate-source authorization does not include version-change authorization;
- version-change authorization does not include tag, release or publication
  authorization;
- merge authorization does not include release or publication authorization;
- no source or PR authorization ever implies secret access.

An implementing agent must not:

- merge a PR;
- mark a draft PR ready for review;
- approve its own work;
- change the crate version;
- create a tag or a GitHub release;
- publish to crates.io;
- access `CRATES_TOKEN` or any other repository secret;
- dispatch, rerun or cancel CI;
- modify branch protection or required checks;
- broaden scope, write budget or action authorization without an approved
  specification update;
- change the public crate API or parsing behaviour silently.

### 5.1 Cross-repository impact and downstream consumers

This repository publishes a **public crate contract**. Before scope affecting that
contract — public functions and types, error surface, parsing behaviour, licence
nomenclature or rights metadata, MSRV or edition — is approved, identify known
consumers from `docs/engineering/repository-map/contracts.md` in `thoth-pub/thoth`
and record whether each requires a change or remains compatible and why.

**Verified downstream consumer:**

- **`thoth-pub/thoth`**, specifically **`thoth-export-server`**, whose
  `Cargo.toml` depends directly on the published crate **`cc_license = "0.1.0"`**.

Note that this is a dependency on the **published crates.io version**, not on a git
reference. Changes on this branch are therefore invisible to Thoth until they are
released.

**Downstream compatibility and release-before-consumption ordering.** For any future
breaking change, the ordering is strict:

1. assess downstream impact on `thoth-export-server` explicitly, including licence
   output that appears in exported metadata;
2. the crate change merges;
3. a separately authorized release publishes the new version to crates.io;
4. only then may `thoth-pub/thoth` update its dependency to consume it.

A downstream repository must **never** guess an unpublished or unmerged crate API,
and must never depend on a version that does not exist on the registry. Prefer
additive, backwards-compatible changes. A change in **parsing behaviour** can be
breaking even when the type signatures are unchanged — treat behavioural change as
contract change.

Never give one implementing agent unrestricted write access to more than one
repository for the same task. Each affected repository gets its own bounded task,
branch and pull request, independently reviewed.

## 6. Stack

Verified current stack:

- **Rust, edition 2021**;
- single crate `cc_license`, currently version `0.1.0`;
- sole runtime dependency: `regex`.

Do not add, upgrade or remove dependencies as an incidental side effect of an
unrelated task. Dependency and edition changes require their own authorization, and
a dependency addition to a published crate propagates to every downstream consumer.

## 7. Validation

The canonical gate, matching the required CI checks:

```bash
cargo test --workspace --verbose
cargo clippy --all --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

These correspond to the required `test`, `lint` and `format_check` checks on
`develop` (section 4.1). Run all three unless the approved specification explicitly
narrows the gate.

For a documentation-only change, at minimum:

```bash
git diff --check
```

Record the exact commands run and their concise results. Do not report only "tests
passed". A check that was not run must be reported as not run, never implied to have
passed.

### 7.1 Known validation gap: clippy fails on current stable

As verified on 2026-08-15 against `develop` @ `db1a4d6`, the clippy gate **fails on
a current stable toolchain** with a single pre-existing error:

```text
error: direct implementation of `ToString`
  --> src/lib.rs:153:1
  = help: prefer implementing `Display` instead
  = note: `-D clippy::to-string-trait-impl` implied by `-D warnings`
```

`cargo test --workspace --verbose` and `cargo fmt --all -- --check` both pass. Only
the clippy/`lint` gate fails.

This is a **toolchain-drift gap**: the lint post-dates the code, and both workflows
pin `toolchain: stable`, which floats. Because `lint` is a **required check** on
protected `develop` (section 4.1), this failure blocks merge for any pull request
until it is resolved.

Repairing it is **separate work** requiring its own authorization and write budget,
since the fix touches `src/lib.rs`. Do not fix it opportunistically inside an
unrelated task, and do not silence it with a blanket `allow` to make an unrelated PR
go green. If an unrelated task encounters this failure, report it as a deferred gap
and return HOLD rather than expanding the source write budget (section 14).

## 8. CI, release and publication

These are **distinct pipelines** and must never be conflated.

### 8.1 Ordinary pull-request CI

`.github/workflows/test_and_check.yml` runs on pull requests and on pushes to `main`
and `develop`, providing the `test`, `lint` and `format_check` jobs that satisfy the
required checks on `develop`.

> **Ordinary PR CI does not publish to crates.io.** Opening or updating a pull
> request runs verification only. Allowing and observing this automatic CI is normal
> and expected.

### 8.2 crates.io publication

`.github/workflows/publish_crate.yml` runs on **`release: published`** and executes
`cargo publish --token ${{ secrets.CRATES_TOKEN }}`.

> **A published GitHub release can therefore publish this crate to crates.io using
> the `CRATES_TOKEN` secret.**

Consequences:

- **version changes, tag creation, GitHub release creation, crates.io publication
  and secret access are separate, denied-by-default actions.** They are never
  implied by authorization to modify source, commit, push, open a PR, or merge;
- publication is **irreversible**: crates.io versions are immutable and cannot be
  deleted, only yanked, and downstream consumers may resolve them immediately;
- an implementing agent must not create a release or tag, must not bump the
  version, must not invoke `cargo publish`, and must not read or use `CRATES_TOKEN`;
- do not modify the publication workflow without explicit authorization.

If a task appears to require a release in order to be useful downstream, **stop**
and request separate release authorization. Do not release to unblock yourself.

### 8.3 Actions modernization gap

The current workflows use **older, unmaintained GitHub Actions** — `actions/checkout@v3`
and the archived `actions-rs/toolchain@v1` / `actions-rs/cargo@v1` actions.

This is a **recorded modernization gap, not a satisfied control**, and it is
**separate work**. Do not modernize the workflows opportunistically inside an
unrelated task. Equally, do not treat the presence of these workflows as evidence
that CI tooling is current.

## 9. Public API and compatibility

Prefer additive and backwards-compatible changes.

Do not:

- rename or remove public functions, types, variants or error cases without an
  approved deprecation path;
- change the meaning of an existing licence parsing result while retaining the same
  signature;
- alter licence nomenclature or rights metadata output without downstream impact
  analysis — these values surface in Thoth's exported metadata;
- allow a downstream repository to guess an unpublished API.

Licence data must be accurate to the upstream Creative Commons definitions. A
correctness change to licence semantics is a domain change, not a cosmetic one, and
requires explicit test evidence.

## 10. Security

This crate parses **untrusted input**: licence URLs originating from external
metadata.

- parsing must fail closed, returning an error rather than a wrong licence;
- never panic on malformed input in library code — return the error type;
- be careful with regular expressions on unbounded input;
- do not log secrets, tokens or raw credentials.

Do not add network access, filesystem access or process execution to this crate. It
is a pure parsing/metadata library and must remain one.

## 11. Change-specific evidence

### Documentation or control-file change

Verify:

- `git diff --check`;
- the whole-task diff contains exactly the authorized paths and nothing else;
- no existing file is modified, deleted, moved or renamed outside the write budget;
- internal links, paths, repository names and terminology;
- every factual statement against live repository, workflow and source evidence;
- that branch, release and publication wording describes the **current** state and
  does not claim normalization or modernization has already occurred.

### Parser or public-API change

Run the full gate in section 7 and additionally prove:

- positive and negative parsing cases, including malformed and unknown URLs;
- licence version and nomenclature correctness;
- rights/permissions metadata correctness;
- downstream impact on `thoth-export-server`;
- whether a release is required, and that release authorization is separate.

## 12. Implementation report

Before review, produce a report following
`docs/engineering/ai-delivery/implementation-report-template.md` in `thoth-pub/thoth`,
including:

- programme, task ID, owning issue and repository;
- authorized base and actual base commit;
- task branch and pull request;
- exact head commit and the commits contained;
- files changed;
- exact validation commands and results;
- CI status evidence, including the required `test`, `format_check` and `lint`
  checks;
- automatic external effects actually observed;
- runtime/API/parsing-behaviour effect;
- migration/data effect;
- auth/security implementation effect;
- provider/runtime mutation;
- release/publication effect;
- deviations from the authorized scope;
- deferred gaps;
- remaining gates.

State `NONE` explicitly where a category does not apply. Do not omit a category.

The implementing agent may provide a self-assessment but **may not issue the
approval decision**.

## 13. Independent review

Every pull request requires a **fresh, independent review of the exact PR head
commit** by a reviewer other than the implementing agent. Branch protection requires
an approving review, and that approval must never come from the implementing agent.

- review must be performed against the exact head SHA, not against a description,
  a summary, or an earlier revision;
- if the head moves after review, the review is stale and must be repeated —
  `develop` dismisses stale reviews automatically;
- the implementing agent must not approve its own work, mark the PR ready for
  review, or merge it;
- pull requests opened by an implementing agent remain **DRAFT** until a human or
  separately authorized reviewer decides otherwise.

## 14. Stop conditions

Return `HOLD` or `BLOCKED` — do not improvise — when:

- the approved specification is absent or incomplete;
- the authorized base branch does not exist, or has moved since authorization;
- an expected new file already exists unexpectedly;
- a file outside the approved write budget must change;
- branch normalization or a branch-protection change would be required;
- publication or release behaviour would have to change;
- a crate version, tag, GitHub release or crates.io publication would be required;
- secret access, including `CRATES_TOKEN`, would be required;
- an unrelated required-check failure would require expanding the source write
  budget to fix — report it as a deferred gap rather than widening scope;
- a public API or parsing-behaviour change would break `thoth-export-server`
  without a released version to consume;
- an unmerged or unpublished contract would have to be guessed;
- repository state differs materially from the task premise;
- an unexpected external or automatic effect occurs;
- a cross-programme decision is required.

When stopping, record what was observed, what was expected, and what decision is
needed. Do not expand scope to work around a stop condition.
