use super::*;
use crate::model::{CAct, CFact, CPAct, CPFact, MaxMultiplicity};
use crate::test_support::{Fixture, UiHarness};

#[test]
fn empty_editors_and_dashboard_render_without_creating_domain_data() {
    let mut app = AppContext::default();
    let mut ui = UiHarness::new();
    ui.settle(&mut |ui| {
        actor_roles::actor_roles_ui(
            ui,
            &app.model.transactions,
            &mut app.model.actor_roles,
            &app.model.adt,
        )
    });
    ui.settle(&mut |ui| {
        transactions::transactions_ui(
            ui,
            &app.model.actor_roles,
            &mut app.model.transactions,
            &app.execution,
        )
    });
    ui.settle(&mut |ui| subjects::subjects_ui(ui, &mut app.model, &app.execution));
    ui.settle(&mut |ui| subjects_dashboard::subjects_tabs_ui(ui, &mut app));
    ui.settle(&mut |ui| c_p_world::view(ui, &mut app));
    assert!(app.model.transactions.is_empty());
    assert!(app.model.actor_roles.is_empty());
    assert!(app.model.subjects.is_empty());
    assert!(app.execution.c_p_world.is_empty());
    assert!(app.subject_context.focused_subject_id_o.is_none());
}

#[test]
fn clicking_commit_performs_one_act_and_clears_the_selection() {
    let mut f = Fixture::new();
    let instance = f.root();
    let mut ui = UiHarness::new();
    ui.click("Commit", 0, &mut |ui| {
        subjects_dashboard::subject_pane_ui(ui, &mut f.context, &f.supplier.id, false, |_, _| {})
    });
    assert_eq!(f.context.execution.c_p_world.len(), 2);
    assert_eq!(
        f.context.execution.c_p_world[1].to_fact(),
        CPFact::CFact(CFact::Promised)
    );
    assert_eq!(f.context.execution.agendas.len(), 1);
    // Subsequent frames select Execute for the new agendum, not the old Promise.
    assert_eq!(
        f.context
            .subject_context
            .get_selected_next_act(&f.supplier.id, &instance.id),
        Some(&CPAct::PAct)
    );
}

#[test]
fn blocked_commit_is_inert_and_keeps_the_agendum() {
    let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
    let parent = f.root();
    f.act(&parent, CPAct::CAct(CAct::Promise));
    let before = f.context.execution.c_p_world.clone();
    let agenda = f.context.execution.agendas.clone();
    UiHarness::new().click("Commit", 0, &mut |ui| {
        subjects_dashboard::subject_pane_ui(ui, &mut f.context, &f.supplier.id, false, |_, _| {})
    });
    assert_eq!(f.context.execution.c_p_world, before);
    assert_eq!(f.context.execution.agendas, agenda);
}

#[test]
fn populated_dashboard_idle_frames_do_not_execute_acts() {
    let mut f = Fixture::new();
    f.root();
    let before = f.context.execution.c_p_world.clone();
    let mut ui = UiHarness::new();
    for _ in 0..10 {
        ui.frame(vec![], &mut |ui| {
            subjects_dashboard::subject_pane_ui(
                ui,
                &mut f.context,
                &f.supplier.id,
                false,
                |_, _| {},
            )
        });
    }
    assert_eq!(f.context.execution.c_p_world, before);
}

#[test]
fn role_referenced_by_a_transaction_cannot_be_deleted() {
    let mut f = Fixture::new();
    UiHarness::new().click("❌", 0, &mut |ui| {
        actor_roles::actor_roles_ui(
            ui,
            &f.context.model.transactions,
            &mut f.context.model.actor_roles,
            &f.context.model.adt,
        )
    });
    assert_eq!(f.context.model.actor_roles.len(), 3);
}

#[test]
fn request_modal_requires_both_product_and_addressee() {
    let mut f = Fixture::new();
    f.context.subject_context.focused_subject_id_o = Some(f.customer.id.clone());
    f.context
        .initiate_transaction_modal_context
        .initiated_transaction_id_o = Some(f.parent.id.clone());
    let mut closed = 0;
    let mut ui = UiHarness::new();
    ui.click("Request product", 0, &mut |ui| {
        transaction_initiate_modal::view(ui, &mut f.context, || closed += 1)
    });
    f.context
        .initiate_transaction_modal_context
        .requested_product = "Order #1".into();
    ui.click("Request product", 0, &mut |ui| {
        transaction_initiate_modal::view(ui, &mut f.context, || closed += 1)
    });
    assert_eq!(closed, 0);
    assert!(f.context.execution.transactions_instances.is_empty());
    f.context.initiate_transaction_modal_context.addressee_id_o = Some(f.supplier.id.clone());
    ui.click("Request product", 0, &mut |ui| {
        transaction_initiate_modal::view(ui, &mut f.context, || closed += 1)
    });
    assert_eq!(closed, 1);
    assert_eq!(f.context.execution.transactions_instances.len(), 1);
    let instance = &f.context.execution.transactions_instances[0];
    assert_eq!(instance.initiator_id, f.customer.id);
    assert_eq!(instance.executor_id, f.supplier.id);
    assert_eq!(instance.product_instance, "Order #1");
}

#[test]
fn world_filters_combine_instance_and_performer() {
    let mut f = Fixture::new();
    let first = f.root();
    let second = f.root();
    f.act(&first, CPAct::CAct(CAct::Promise));
    f.context.c_p_world_context.transaction_instance_id_filter = first.id.to_string();
    f.context.c_p_world_context.performer_filter = f.supplier.name.clone();
    let mut ui = UiHarness::new();
    ui.settle(&mut |ui| c_p_world::view(ui, &mut f.context));
    let texts: Vec<_> = ui.texts.iter().map(|(text, _)| text.as_str()).collect();
    assert!(texts.contains(&"Promised"));
    assert!(!texts.contains(&"Requested"));
    assert!(!texts.contains(&second.id.to_string().as_str()));
    assert_eq!(f.context.execution.c_p_world.len(), 3);
}

#[test]
fn second_agenda_commit_does_not_settle_the_first_instance() {
    let mut f = Fixture::new();
    let first = f.root();
    let second = f.root();
    UiHarness::new().click("Commit", 1, &mut |ui| {
        subjects_dashboard::subject_pane_ui(ui, &mut f.context, &f.supplier.id, false, |_, _| {})
    });
    assert_eq!(
        f.context
            .execution
            .get_facts_for_transaction_instance(&first.id)
            .len(),
        1
    );
    assert_eq!(
        f.context
            .execution
            .get_facts_for_transaction_instance(&second.id)
            .len(),
        2
    );
    assert_eq!(f.context.execution.agendas.len(), 2);
}

#[test]
fn all_main_windows_can_render_together_without_advancing_execution() {
    let mut f = Fixture::new();
    f.root();
    let before = f.context.execution.c_p_world.clone();
    let mut windows = EguiWindows {
        actor_roles: true,
        transactions: true,
        subjects: true,
        adt: true,
        subjects_dashboard: true,
        transactions_instances: true,
        c_p_world: true,
        ..Default::default()
    };
    let context = egui::Context::default();
    for _ in 0..3 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(4096.0, 2048.0),
            )),
            ..Default::default()
        };
        let _ = context.run(input, |ctx| windows.windows(ctx, &mut f.context));
    }
    assert_eq!(f.context.execution.c_p_world, before);
}

mod known_bugs {
    use super::*;

    #[test]
    fn adding_initiation_with_no_other_transaction_is_safe() {
        let f = Fixture::new();
        let mut parent = f.parent;
        let transactions = vec![parent.clone()];
        UiHarness::new().click("➕", 0, &mut |ui| {
            transactions::initiations_ui(ui, &transactions, &mut parent)
        });
        assert!(parent.initiations.is_empty());
    }

    #[test]
    fn adding_wait_with_no_other_transaction_is_safe() {
        let f = Fixture::new();
        let mut parent = f.parent;
        let transactions = vec![parent.clone()];
        UiHarness::new().click("➕", 0, &mut |ui| {
            transactions::impediments_ui(ui, &transactions, &mut parent)
        });
        assert!(parent.impediments.is_empty());
    }

    #[test]
    fn deleting_a_referenced_subject_preserves_model_integrity() {
        let mut f = Fixture::new();
        f.root();
        UiHarness::new().click("❌", 0, &mut |ui| {
            subjects::subjects_ui(ui, &mut f.context.model, &f.context.execution)
        });
        for (_, subject_id) in f.context.model.adt.mappings.keys() {
            assert!(f.context.model.subjects.iter().any(|s| &s.id == subject_id));
        }
        UiHarness::new().settle(&mut |ui| {
            crate::components::transactions_instances_ui(ui, &mut f.context, |_| true, true)
        });
    }

    #[test]
    fn deleting_an_initiation_target_preserves_links() {
        let mut f = Fixture::linked(1, MaxMultiplicity::Numeric(1));
        // Remove the wait so only the incoming initiation protects this kind.
        f.context.model.transactions[0].impediments.clear();
        // T1 has delete + initiation-delete controls; T2's delete is the third cross.
        UiHarness::new().click("❌", 2, &mut |ui| {
            transactions::transactions_ui(
                ui,
                &f.context.model.actor_roles,
                &mut f.context.model.transactions,
                &f.context.execution,
            )
        });
        let target = &f.context.model.transactions[0].initiations[0].initiated_transaction_id;
        assert!(f.context.model.transactions.iter().any(|t| &t.id == target));
    }

    #[test]
    fn switching_to_unauthorised_subject_cannot_submit_open_request() {
        let mut f = Fixture::new();
        f.context.subject_context.focused_subject_id_o = Some(f.customer.id.clone());
        f.context
            .initiate_transaction_modal_context
            .initiated_transaction_id_o = Some(f.parent.id.clone());
        f.context
            .initiate_transaction_modal_context
            .requested_product = "Order #1".into();
        f.context.initiate_transaction_modal_context.addressee_id_o = Some(f.supplier.id.clone());
        let mut ui = UiHarness::new();
        ui.settle(&mut |ui| transaction_initiate_modal::view(ui, &mut f.context, || {}));
        f.context.subject_context.focused_subject_id_o = Some(f.worker.id.clone());
        ui.click("Request product", 0, &mut |ui| {
            transaction_initiate_modal::view(ui, &mut f.context, || {})
        });
        // Retaining the original initiator or refusing submission both preserve authority.
        assert!(f
            .context
            .execution
            .transactions_instances
            .iter()
            .all(|i| i.initiator_id == f.customer.id));
    }

    #[test]
    fn stale_agenda_selection_cannot_skip_the_promise() {
        let mut f = Fixture::new();
        let instance = f.root();
        f.context.subject_context.selected_next_act.insert(
            (f.supplier.id.clone(), instance.id),
            CPAct::CAct(CAct::Accept),
        );
        UiHarness::new().click("Commit", 0, &mut |ui| {
            subjects_dashboard::subject_pane_ui(
                ui,
                &mut f.context,
                &f.supplier.id,
                false,
                |_, _| {},
            )
        });
        assert!(f
            .context
            .execution
            .c_p_world
            .iter()
            .all(|e| e.to_fact() != CPFact::CFact(CFact::Accepted)));
    }
}
