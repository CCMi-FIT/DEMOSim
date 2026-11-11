# DEMO Theory Reference for DEMOsim

## Purpose and authority

Use this reference when designing features, debugging execution, reviewing models,
refactoring domain code, or changing the UI and persistence. It covers the domain
beyond revocations. Consult the cited passages for decisions that this summary does
not settle; existing application behavior is not proof of theoretical correctness.

**Source:** Jan L. G. Dietz and Hans B. F. Mulder, *Enterprise Ontology: A
Human-Centric Approach to Understanding the Essence of Organisation*, second edition,
Springer, 2024. DOI: <https://doi.org/10.1007/978-3-031-53361-7>.
The supplied copy is `../../book.pdf` relative to this file, outside the application
repository. Page references below are **printed pages**; PDF links use physical,
one-based PDF page numbers. The book need not be distributed with this repository.

**Edition correction:** this edition has **six** revocation patterns, including
decline and reject. The preface (p. vii) and §19.10 (p. 439) explicitly identify
these two additions. An earlier four-pattern interpretation is incomplete for this
source. Section 8.2.5 introduces transactions; §8.2.6 supplies the detailed patterns.

Sections below distinguish **book theory**, **observed implementation**, and
**engineering implications/open decisions**. Implementation observations were checked
on 2026-11-11 against `main` at `360d75f` and WIP `revokes` at `7eb3a54`; recheck them
after code changes. This is a development reference, not a complete DEMOSL specification
or a claim that the application implements every concept described here.

## 1. Core concepts and levels of abstraction

Sources: §§8.2.2–8.2.7, pp. 126–150; §§11.2.1–11.2.4, pp. 235–241.

| Concept | Meaning and consequence for the simulator |
| --- | --- |
| Subject | A person who can fill actor roles. A subject is not interchangeable with a role. |
| Actor | A subject acting in a particular actor role. The same person can act in several roles. |
| Actor role | Authority to execute transactions of a particular kind; it may also initiate transactions of other kinds. |
| Transaction kind | A reusable kind of cooperation concerning one product kind. Distinguish it from a running instance. |
| Transaction instance | Cooperation between an initiator and an executor concerning a particular product. |
| Production act (P-act) | Work that brings about a product, tangible or intangible: manufacturing, transporting, deciding, judging, etc. |
| Product | An independent production fact together with its dependent facts, such as attributes. A description of a proposed product does not assert that it exists. |
| Coordination act (C-act) | An atomic communicative act with a performer, intention, addressee, and product. |
| Coordination fact (C-fact) | The immediate result of a C-act, such as being requested. Its becoming existent is a coordination event. |

A product kind must identify a fact that can be uniquely instantiated in time and
space. For example, `[membership] is started` identifies a specific membership;
`[person] is a member` alone does not distinguish successive memberships.
Product kind and transaction kind correspond one-to-one. Each transaction kind has
exactly one executor role, and each elementary executor role corresponds to one
transaction kind. This does not restrict a role to one subject or a kind to one instance.

DEMO abstracts from implementation. The O-organisation creates original business
facts; the I-organisation remembers, shares, or derives facts; the D-organisation
handles documents/data; the M-organisation handles their physical carriers/files.
Deriving a total does not create another sale. Saving a simulation does not perform
another business transaction. These distinctions help keep UI, storage, calculations,
and business semantics separate. An enterprise's essential model abstracts from its
supporting layers and implementation choices.

## 2. Transaction phases and the standard pattern

Sources: §§8.2.5–8.2.6.2, pp. 133–138, Figs. 8.7–8.8; §8.2.6.11, pp. 146–147.

The **order phase** negotiates the proposed product until the executor promises it.
The **execution phase** is the executor's production work, basically private to that
executor. The **result phase** negotiates the actual result until the initiator accepts
it. Declaration communicates the result; the initiator cannot directly know the
private production act merely because it occurred.

| Situation | Actor and act | Result / next situation |
| --- | --- | --- |
| Initial (`in`) | Initiator requests (`rq`) | Requested (`rq`) |
| Requested | Executor promises (`pm`) | Promised (`pm`); execution can proceed subject to waits |
| Requested | Executor declines (`dc`) | Declined (`dc`), a discussion state |
| Declined | Initiator optionally renews/adapts request | Requested again |
| Promised | Executor performs the P-act (`ex`) | Production performed; declaration follows |
| Production performed | Executor declares (`da`) | Declared (`da`) |
| Declared | Initiator accepts (`ac`) | Accepted (`ac`), successful completion |
| Declared | Initiator rejects (`rj`) | Rejected (`rj`), a discussion state |
| Rejected | Executor optionally renews/adapts declaration | Declared again |

The two renewal links have cardinality `0..1`: the participant may do nothing and
leave the transaction in discussion. Declined and rejected therefore do not mean
success or irrevocable cancellation. Re-declaring after discussion need not redo
production: the parties might agree on a different price for the same result. To
redo production, the complete pattern provides revocation of declaration.

Agreement concerns the **latest** propositions: successful order means latest
promised product equals latest requested product; successful result means latest
accepted product equals latest declared product. Final attributes need not equal the
original request. Paying is a separate transaction even when closely connected.

The product's existence in the shared production world starts at acceptance
(pp. 136, 147). Distinguish the executor's earlier work from this accepted result.
Acts may be explicit, nonverbal, or tacit; tacit acts still count. Requiring a button
press for each act is an educational interaction choice, not a theoretical requirement.

### Reading the diagrams correctly

Table 8.1 (p. 135) distinguishes causal links (act creates fact), response links
(event prompts action), conditional links (a state must hold), and reversion links
(an allowed revocation changes the main state). These are different relationships.
`F+` means state `F` or further in the main process; it is not an enum ordering.
For example, `da+` includes declared, rejected, and accepted. A historical occurrence
of `F` is not by itself evidence that the current main state still satisfies `F+`
after a reversion. Preserve the difference between an event history and a current
state projection.

## 3. Actors, authority, action rules, and agendas

Sources: §§8.2.7–8.3.3, pp. 147–156; §12.3.3, pp. 278–280.

Subjects and roles have a many-to-many relationship. Several subjects can fill a
role, and one subject can fill several roles; a collective assignment is also possible.
Authority concerns particular acts in response to particular events. Authorisation
and delegation assign authority; delegation can cover only part of it. Role membership,
the participating actor in a particular transaction, and permission for a particular
act are separate questions.

Responsibility concerns how authority is exercised; accountability concerns explaining
one's acts. Actors retain autonomy. Action rules guide decisions in circumstances
that cannot all be anticipated. Do not silently turn every business recommendation
into a forced transition or equate an automated action with absence of a responsible actor.

An action rule has three parts:

1. **Event:** `when` identifies the triggering coordination event; optional `given`
   supplies product parameters; optional `while` identifies an impeding event.
2. **Assess:** evaluate rightness (including authority), sincerity, and truth
   (including relevant production facts and constraints).
3. **Response:** perform the response considered justifiable by the actor.

The same triggering event kind may have rules with different impeding event kinds;
the book disallows duplicate `when`/`while` combinations. Rules are often specified
only for the basic pattern, leaving exceptional discussions to professional judgment.
Omission from an action-rule listing does not remove a transition from the CTP.

An **agendum** represents an event still to be dealt with. The actor selects one,
assesses the situation, acts, and continues the operating cycle. A transaction list,
event history, and agenda serve different purposes. An unmet wait condition should
not be mistaken for a settled event. Engineering implication: identify the particular
event being settled, especially when loops can produce the same fact kind repeatedly.

## 4. Time and product data

Source: §8.3.1, pp. 151–153, Figs. 8.18–8.19.

Keep these meanings distinct:

- **C-fact creation time:** when its C-act was performed.
- **Intended settlement time:** when the performer wants the addressee to respond.
  Actual settlement can be earlier or later because actors are autonomous.
- **Product event time:** the time of acceptance; for tacit acceptance, the book
  takes the declaration time.
- **Product operative time:** the interval during which the accepted product is
  effective. It can be future-dated or, where meaningful, retrospective.

A reservation accepted today for next month's hotel stay illustrates the distinction
between event time and operative time. Check-in is another transaction. Dependent
product facts may be provided beforehand, but their event time is that of the
corresponding independent product fact. A single recording timestamp cannot express
all these meanings.

Engineering implication: product attributes and their agreed versions need explicit
semantics before adding negotiation, scheduling, reporting, or historical replay.
Changing a display label must not accidentally change the identity of a product.

## 5. Transaction networks, waits, and cardinalities

Sources: §§10.2.1–10.2.4, pp. 200–210; §12.3.4, pp. 281–284;
§§9.2.3, 9.3.2, 9.4.3, pp. 175–181, 187–189, 195.

Three relationships must remain distinguishable:

| Relationship | Meaning |
| --- | --- |
| Interaction / initiation | An actor initiates another transaction as part of producing a product; these relationships form business-process trees. |
| Interstriction / access | Actors consult facts in transaction banks to make decisions. Access is not initiation or execution authority. |
| Interimpediment / wait | A particular action cannot proceed until the relevant event/state in another transaction satisfies its wait condition. |

A response link triggers an act; a wait link constrains an act. The PRISMA discussion
and the comparison with Petri nets underline that an impeding event is not the same
thing as the triggering event. Releasing a wait does not itself create a new request
or settle an unrelated agendum.

In the Volley example, promising membership starting triggers a fee-payment request;
membership production waits for acceptance of the corresponding payment. In the
Rent-A-Car example (Fig. 12.14), a daily transport-management transaction can initiate
`0..*` transport-completion transactions and finishes only after all those initiated
transactions complete. Zero children is allowed; an unfinished existing child is
not excused by the zero minimum.

Both response and wait links can carry cardinalities. The default is `1..1`.
A response cardinality `k..n` constrains the number of occurrences of the target act.
Validate both bounds; merely finding one child does not satisfy a minimum of two.
Do not assume wait cardinality is always identical to initiation cardinality.

The same transaction kind can occur under different initiating roles or at different
positions in a process tree (e.g. TAR08 in §10.2.1). Correlate **instances in the
relevant case**, not just transaction-kind IDs. The initiator need not know the
executor's internal production subtree.

Engineering decisions still needed for a richer simulator include how links identify
the relevant instances and event occurrences, when an optional child set is considered
closed, and what happens to dependent work after a prerequisite is revoked. These
must be specified explicitly; the book's examples do not supply a complete software
algorithm for concurrent execution and replay.

## 6. The four integrated DEMO models

Source: §§12.3–12.3.5, pp. 272–287.

| Model | Questions it answers | Application relevance |
| --- | --- | --- |
| Cooperation Model (CM) | Which transactor roles, products, and coordination structures constitute the organisation? | Transaction/role editors, links, product kinds, and access to transaction banks. |
| Action Model (AM) | How do actors assess events and decide responses; how is production performed? | Agenda handling, decision support, action rules, work instructions. |
| Process Model (PM) | Which coordination states and transitions are possible, including inter-transaction occurrence laws and cardinalities? | Transition logic, initiations, waits, execution validation. |
| Fact Model (FM) | Which entities, values, properties, attributes, events, and laws describe the production world? | Typed products, data validation, derived facts, reporting. |

The models must agree. A production event type in the FM is a product kind in the CM.
The Create Use Table connects process steps with the facts they create or consult.
Existence laws constrain allowable states; occurrence laws constrain changes and
event ordering. Work instructions describe production work; that workflow is not
itself the coordination process. For a new feature, identify the affected model(s)
before choosing fields or widgets.

## 7. Complete pattern and revocations

Sources: §§8.2.6.3–8.2.6.11, pp. 138–147, Figs. 8.9–8.15.

Revocations are R-conversations at a meta level relative to the main transaction.
They can be prompted by events outside that main process, but their state guards
must hold. The actor who performed the targeted act proposes its revocation; the
other participant allows or refuses it.

| Target act | Proposer | Allows / refuses | Main-state guard | Main state after allowance |
| --- | --- | --- | --- | --- |
| Request | Initiator | Executor | `rq+` | Initial (`in`) |
| Promise | Executor | Initiator | `pm+` | Requested (`rq`) |
| Decline | Executor | Initiator | `dc` | Requested (`rq`) |
| Declare | Executor | Initiator | `da+` | Promised (`pm`) |
| Accept | Initiator | Executor | `ac` | Declared (`da`) |
| Reject | Initiator | Executor | `rj` | Declared (`da`) |

A revocation proposal does not itself revert the main state. Refusal leaves the main
state unchanged. Allowance performs the indicated reversion; it does not automatically
perform the next ordinary act. For example, allowing revoke-accept returns to declared;
rejecting is a subsequent act. Likewise, allowing revoke-decline returns to requested;
promising is a subsequent act.

Accepted transactions can be reopened, including by revoking the request. The book
allows arbitrarily many loops. A mistaken act cancelled before the addressee responds
(p. 139) is distinguished from revoking an intentionally performed act after changing
one's mind. Neither is equivalent to deleting a transaction from storage.

If reversion crosses back before an already performed P-act, to promised, requested,
or initial, production must somehow be undone. Tangible work may not be physically
reversible; compensation and costs depend on the agreement (§8.2.6.10). A purchase
and its payment remain separate transactions, each requiring the relevant reversal.
Do not implement business compensation as erasing evidence or silently cascading
deletion through child instances.

Engineering implications: maintain main-process state separately from the state of
a revocation conversation; correlate allow/refuse with its proposal; preserve history.
Decide explicitly how ordinary actions behave while a revocation is pending and how
overlapping proposals are handled. The six diagrams alone do not settle those
concurrency policies.

## 8. Implementation review and subsequent fixes

The table records the initial observations at the revisions named above. Subsequent
fixes restore decline/request recovery, validate domain actions, protect references,
and correct dependency counts and scope. Regression tests live in
`src/execution/tests.rs`, `src/model/tests.rs`, `src/app/tests.rs`,
`src/windows/tests.rs`, and `tests/sample_models.rs`. Preserve the
distinction between this historical review, implemented fixes, and remaining limits.

| Code | Domain meaning and limitation at initial review |
| --- | --- |
| [`model.rs`](../src/model.rs): `Transaction`, `ActorRole`, `Subject`, `Adt` | `Transaction` represents a **kind**. It has one initiator-role field and one executor-role field. `Adt` represents subject/role assignments, not a detailed model of delegated acts. |
| [`execution.rs`](../src/execution.rs): `TransactionInstance` | Represents an instance, including parent instance, kind, participating subjects, and a free-text product. This does not yet represent the FM's typed/versioned product facts. |
| `CPFact::next_acts` | Encodes the basic route and reject → declare, but **declined has no next act**. The standard pattern's optional renewed request is missing. |
| `Execution::process_new_fact` | Records the supplied fact and creates agenda work; it does not centrally validate the transition or performer's authority. UI restrictions alone do not constitute domain validation. |
| `PWorldItem` / `CPFact::PFact` | Recorded at Execute and displayed as Executed. Treat this as an execution marker; do not infer that its timestamp is the accepted product's event time. |
| `CWorldItem`, `PWorldItem`, `AgendaItem` | Have timestamps but no explicit event-occurrence IDs, intended settlement time, or operative-time model. Repeated acts need careful correlation. |
| `Initiation::initiated_c_act` / `add_transaction_instance` | The editor stores a target C-act, but creating an instance always records Requested. Non-request response links are not thereby implemented. |
| `get_c_p_world_item_by_fact` | Finds the first matching historical fact. Suitable historical evidence is not necessarily a current-state or current-occurrence test. |
| `startable_subtransactions`, `get_instances_of_transaction` | Use historical trigger presence and instance counts; instance lookup includes ancestry. Review case/link correlation before extending loops or shared transaction kinds. |
| `get_act_impediments` | Checks whether matching instances have the required historical fact. With no instances it consults initiation minimum; with some instances it does not enforce a minimum greater than one. `Impediment` has no independent cardinality field. |
| `Model::get_initiation_of_transaction` | Picks the first incoming initiation and warns about multiple matches. This is a simulator restriction, not a general uniqueness rule for DEMO transaction kinds. |
| [`transactions_instances.rs`](../src/components/transactions_instances.rs) | Displays the last recorded fact as status. A future revocation meta-event cannot serve directly as the main transaction state. |
| `delete_transaction_instance` | Removes an instance and its recorded facts/agendas. This administrative operation is not the negotiated revocation protocol. |

The `revokes` WIP adds action/fact variants and addressee mapping for request,
promise, decline, accept, and reject revocations. **Declaration revocation is missing**;
decline and reject belong in this edition. The WIP does not yet implement complete
availability, reversion, and agenda behavior. Enum coverage is not protocol coverage.

## 9. Applying this reference to changes and debugging

For each behavior change, identify the transaction **kind**, **instance**, participating
**actor**, relevant **event occurrence**, current **state**, and product proposition.
Then distinguish theoretical constraints, the model's case-specific rules, and deliberate
educational simplifications. Document new simplifications rather than silently treating
them as DEMO rules.

Useful acceptance scenarios, selected according to the change:

- Basic success: correct performers and addressees for all acts; execution precedes
  declaration, and acceptance establishes the agreed result.
- Discussion: request → decline → renewed request; declare → reject → renewed
  declaration. Allow discussion to remain unresolved without declaring success.
- Identity and authority: several subjects in one role, one subject in several roles,
  two simultaneous instances of the same kind, and an unauthorised performer.
- Dependencies: required child absent, optional child absent, optional child present
  but incomplete, minimum greater than one, maximum reached, and unrelated cases
  that must not satisfy each other's waits.
- Repetition: repeated requests/declarations and newly enabled child work must not
  accidentally reuse an already settled event or a stale agenda selection.
- Time/data: creation versus settlement, execution versus acceptance, and accepted
  product event time versus its validity period.
- Revocations: all six proposer/responder pairs, allow and refuse, exact restored
  state, reopening after acceptance, production compensation, and related payments.
- Persistence/UI: save/load retains identities and the same available behavior;
  displayed status, available actions, agendas, and history agree. Loading another
  model must not retain selections or references into the previous model.

Software regression tests and successful builds do not by themselves establish DEMO
conformance. Conversely, a conceptual gap does not mean an unrelated refactoring is
unsafe. Use a focused scenario tied to the relevant theory and affected code.

## Source navigation and maintenance

| Topic | Printed pages | Local PDF entry |
| --- | --- | --- |
| Production, coordination, conversations | 126–132 | [§§8.2.2–8.2.4](../../book.pdf#page=149) |
| Business transactions and patterns | 133–147 | [§§8.2.5–8.2.6](../../book.pdf#page=156) |
| Actors, time, agendas, authority | 147–156 | [§§8.2.7–8.3.3](../../book.pdf#page=170) |
| PRISMA state, events, and impediments | 175–181, 187–189, 195 | [§9.2.3 onward](../../book.pdf#page=197) |
| Organisational structure and dependencies | 200–210 | [§§10.2.1–10.2.4](../../book.pdf#page=221) |
| Original production and supporting layers | 235–241 | [§11.2](../../book.pdf#page=255) |
| Four models, action rules, process and fact constraints | 272–287 | [§12.3](../../book.pdf#page=291) |
| Practical motivation for decline/reject revocation | 437–439 | [§19.10](../../book.pdf#page=454) |

The transaction diagrams (Figs. 8.8–8.15), link legend (Table 8.1), and process
examples (Figs. 12.13–12.15) were visually checked as well as read from extracted
text. This is a focused reading of these sections and supporting passages, not a
claim to have reviewed the entire book or all sample models for conformance.

Source-file SHA-256:
`26ce843c675d1af560a42efdaaff807e6a6ebd0cb310759785103717b0478b59`.
Keep book-derived rules separate from implementation observations when updating this
reference; record the inspected revision and supporting section for new conclusions.
