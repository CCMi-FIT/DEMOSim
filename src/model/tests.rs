use super::*;
use crate::test_support::Fixture;

#[test]
fn standard_pattern_offers_only_the_expected_responses() {
    let cases = [
        (
            CPFact::CFact(CFact::Requested),
            vec![CPAct::CAct(CAct::Promise), CPAct::CAct(CAct::Decline)],
        ),
        (CPFact::CFact(CFact::Promised), vec![CPAct::PAct]),
        (CPFact::PFact, vec![CPAct::CAct(CAct::Declare)]),
        (
            CPFact::CFact(CFact::Declared),
            vec![CPAct::CAct(CAct::Accept), CPAct::CAct(CAct::Reject)],
        ),
        (
            CPFact::CFact(CFact::Rejected),
            vec![CPAct::CAct(CAct::Declare)],
        ),
        (CPFact::CFact(CFact::Accepted), vec![]),
    ];
    for (fact, expected) in cases {
        assert_eq!(fact.next_acts(), expected, "{fact}");
    }
}

#[test]
fn multiplicities_parse_and_round_trip_at_boundaries() {
    for text in ["0..0", "0..1", "1..1", "2..5", "0..*", "255..255", "255..*"] {
        let parsed: Multiplicity = text.parse().unwrap();
        assert_eq!(parsed.to_string(), text);
    }
    assert_eq!(
        " 1..2 ".parse::<Multiplicity>().unwrap().to_string(),
        "1..2"
    );
    assert_eq!(Multiplicity::default().to_string(), "1..1");
}

#[test]
fn malformed_and_out_of_range_multiplicities_are_rejected() {
    for text in [
        "", "1", "1-2", "-1..2", "1..-2", "256..*", "1..256", "a..2", "1..", "1..2..3",
    ] {
        assert!(text.parse::<Multiplicity>().is_err(), "accepted {text:?}");
    }
}

#[test]
fn maximum_limits_creation_before_the_next_instance() {
    assert!(!MaxMultiplicity::Numeric(0).is_within_bound(0_usize));
    assert!(MaxMultiplicity::Numeric(2).is_within_bound(1_usize));
    assert!(!MaxMultiplicity::Numeric(2).is_within_bound(2_usize));
    assert!(!MaxMultiplicity::Numeric(2).is_within_bound(300_usize));
    assert!(MaxMultiplicity::Any.is_within_bound(usize::MAX));
}

#[test]
fn role_assignments_support_multiple_subjects_and_multiple_roles() {
    let mut f = Fixture::new();
    f.context.model.adt.mappings.insert(
        (f.parent.initiator_id.clone(), f.worker.id.clone()),
        AdtOption::Delegation,
    );
    let ids = f.context.model.get_initiator_subjects_ids(&f.parent.id);
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&f.customer.id) && ids.contains(&f.worker.id));
    assert_eq!(f.context.model.adt.get_roles_of_subject(&f.worker).len(), 2);
    assert!(!ids.contains(&f.supplier.id));
}

#[test]
fn only_authorised_root_kinds_are_directly_startable() {
    let f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
    let model = &f.context.model;
    let roots = model.directly_startable_transactions(&f.customer.id);
    assert_eq!(roots.len(), 1);
    assert_eq!(roots[0].id, f.parent.id);
    assert!(model
        .directly_startable_transactions(&f.supplier.id)
        .is_empty());
    assert!(model
        .directly_startable_transactions(&f.worker.id)
        .is_empty());
}

mod known_bugs {
    use super::*;

    #[test]
    fn declined_allows_a_renewed_request() {
        assert_eq!(
            CPFact::CFact(CFact::Declined).next_acts(),
            vec![CPAct::CAct(CAct::Request)]
        );
    }

    #[test]
    fn reversed_multiplicity_is_rejected() {
        assert!("2..1".parse::<Multiplicity>().is_err());
    }
}
