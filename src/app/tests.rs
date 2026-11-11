use super::DemosimApp;
use crate::model::{CAct, CPAct, Model, Subject};

fn running_app() -> DemosimApp {
    let mut app = DemosimApp::default();
    let mut fixture = crate::test_support::Fixture::new();
    let instance = fixture.root();
    let subject = fixture.customer;
    let transaction = fixture.parent;
    app.app_context = fixture.context;
    app.app_context.model.name = "Original model".into();
    app.app_context.subject_context.focused_subject_id_o = Some(subject.id.clone());
    app.app_context.subject_context.selected_next_act.insert(
        (subject.id.clone(), instance.id.clone()),
        CPAct::CAct(CAct::Promise),
    );
    app.app_context
        .initiate_transaction_modal_context
        .initiated_transaction_id_o = Some(transaction.id);
    app.app_context
        .initiate_transaction_modal_context
        .addressee_id_o = Some(subject.id);
    app.app_context.c_p_world_context.performer_filter = "Previous performer".into();
    app.app_context.hi_transaction_instance_id_o = Some(instance.id);
    app
}

#[test]
fn loading_another_model_clears_previous_execution_and_selections() {
    let mut app = running_app();
    let replacement = Model {
        name: "Replacement model".into(),
        subjects: vec![Subject::default()],
        ..Default::default()
    };
    app.model_text_channel
        .0
        .send(ron::ser::to_string(&replacement).unwrap())
        .unwrap();

    app.try_load_model();

    let context = &app.app_context;
    assert_eq!(context.model.name, replacement.name);
    assert_eq!(context.model.subjects[0].id, replacement.subjects[0].id);
    assert!(context.execution.transactions_instances.is_empty());
    assert!(context.execution.c_p_world.is_empty());
    assert!(context.execution.agendas.is_empty());
    assert!(context.subject_context.focused_subject_id_o.is_none());
    assert!(context.subject_context.selected_next_act.is_empty());
    assert!(context
        .initiate_transaction_modal_context
        .initiated_transaction_id_o
        .is_none());
    assert!(context
        .initiate_transaction_modal_context
        .addressee_id_o
        .is_none());
    assert!(context.c_p_world_context.performer_filter.is_empty());
    assert!(context.hi_transaction_instance_id_o.is_none());
}

#[test]
fn waiting_for_a_model_keeps_the_current_execution() {
    let mut app = running_app();

    app.try_load_model();

    assert_eq!(app.app_context.model.name, "Original model");
    assert_eq!(app.app_context.execution.transactions_instances.len(), 1);
    assert_eq!(app.app_context.execution.c_p_world.len(), 1);
    assert_eq!(app.app_context.execution.agendas.len(), 1);
    assert!(app
        .app_context
        .subject_context
        .focused_subject_id_o
        .is_some());
}

#[test]
fn app_round_trip_restores_execution_but_not_transient_action_selection() {
    let app = running_app();
    let encoded = ron::to_string(&app).unwrap();
    let mut restored: DemosimApp = ron::from_str(&encoded).unwrap();
    assert_eq!(
        restored.app_context.execution.c_p_world,
        app.app_context.execution.c_p_world
    );
    assert_eq!(
        restored.app_context.execution.agendas,
        app.app_context.execution.agendas
    );
    assert_eq!(
        restored.app_context.subject_context.focused_subject_id_o,
        app.app_context.subject_context.focused_subject_id_o
    );
    assert!(restored
        .app_context
        .subject_context
        .selected_next_act
        .is_empty());
    // Deserialisation must recreate a working channel for subsequent file loads.
    let replacement = Model::default();
    restored
        .model_text_channel
        .0
        .send(ron::to_string(&replacement).unwrap())
        .unwrap();
    restored.try_load_model();
    assert!(restored
        .app_context
        .execution
        .transactions_instances
        .is_empty());
}

#[test]
fn empty_legacy_app_state_uses_defaults() {
    let mut app: DemosimApp = ron::from_str("()").unwrap();
    assert!(app.app_context.execution.c_p_world.is_empty());
    app.try_load_model();
}

mod known_bugs {
    use super::*;

    #[test]
    fn malformed_model_does_not_crash_or_replace_the_running_model() {
        let mut app = running_app();
        let before = app.app_context.execution.c_p_world.clone();
        app.model_text_channel
            .0
            .send("not a RON model".into())
            .unwrap();
        app.try_load_model();
        assert_eq!(app.app_context.model.name, "Original model");
        assert_eq!(app.app_context.execution.c_p_world, before);
    }
}

#[test]
fn malformed_and_dangling_models_report_errors_and_keep_running_state() {
    let mut app = running_app();
    let before = app.app_context.execution.c_p_world.clone();
    app.model_text_channel.0.send("invalid".into()).unwrap();
    app.try_load_model();
    assert!(app
        .app_context
        .error
        .as_ref()
        .unwrap()
        .contains("Cannot load model"));
    assert_eq!(app.app_context.execution.c_p_world, before);
    let mut fixture = crate::test_support::Fixture::new();
    fixture.context.model.actor_roles.clear();
    app.model_text_channel
        .0
        .send(ron::to_string(&fixture.context.model).unwrap())
        .unwrap();
    app.try_load_model();
    assert!(app
        .app_context
        .error
        .as_ref()
        .unwrap()
        .contains("missing role"));
    assert_eq!(app.app_context.execution.c_p_world, before);
    app.model_text_channel
        .0
        .send(ron::to_string(&Model::default()).unwrap())
        .unwrap();
    app.try_load_model();
    assert!(app.app_context.error.is_none());
}
