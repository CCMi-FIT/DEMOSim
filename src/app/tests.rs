use super::DemosimApp;
use crate::execution::TransactionInstance;
use crate::model::{CAct, CPAct, Model, Subject, Transaction};

fn running_app() -> DemosimApp {
    let mut app = DemosimApp::default();
    let subject = Subject::default();
    let transaction = Transaction::new();
    let instance = TransactionInstance::new(
        None,
        transaction.id.clone(),
        "Existing product".into(),
        subject.id.clone(),
        subject.id.clone(),
    );
    app.app_context.model.name = "Original model".into();
    app.app_context.model.subjects.push(subject.clone());
    app.app_context.model.transactions.push(transaction.clone());
    app.app_context
        .execution
        .add_transaction_instance(&app.app_context.model, instance.clone());
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
