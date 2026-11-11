use super::transactions_instances_ui;
use crate::app::AppContext;
use crate::execution::TransactionInstance;
use crate::model::{ActorRole, AdtOption, Subject, SubjectId, Transaction};
use egui::epaint::Shape;

fn populated_context() -> (AppContext, SubjectId) {
    let mut context = AppContext::default();
    let focused = Subject {
        name: "Focused subject".into(),
        ..Default::default()
    };
    let other = Subject {
        name: "Other subject".into(),
        ..Default::default()
    };
    let outsider = Subject {
        name: "Unrelated subject".into(),
        ..Default::default()
    };
    let transaction = Transaction::new();
    context.model.subjects = vec![focused.clone(), other.clone(), outsider.clone()];
    context.model.transactions.push(transaction.clone());
    for role_id in [&transaction.initiator_id, &transaction.executor_id] {
        context.model.actor_roles.push(ActorRole {
            id: role_id.clone(),
            name: "Test role".into(),
        });
        for subject in &context.model.subjects {
            context.model.adt.mappings.insert(
                (role_id.clone(), subject.id.clone()),
                AdtOption::Authorisation,
            );
        }
    }

    for (initiator, executor, product) in [
        (&focused, &other, "Initiated product"),
        (&other, &focused, "Executed product"),
        (&other, &outsider, "Unrelated product"),
    ] {
        let instance = TransactionInstance::new(
            None,
            transaction.id.clone(),
            product.into(),
            initiator.id.clone(),
            executor.id.clone(),
        );
        context
            .execution
            .add_transaction_instance(&context.model, instance)
            .unwrap();
    }
    (context, focused.id)
}

fn collect_text(shape: &Shape, text: &mut String) {
    match shape {
        Shape::Text(shape) => {
            text.push_str(&shape.galley.job.text);
            text.push('\n');
        }
        Shape::Vec(shapes) => {
            for shape in shapes {
                collect_text(shape, text);
            }
        }
        _ => {}
    }
}

fn render_table(
    app_context: &mut AppContext,
    subject_id: Option<SubjectId>,
    can_delete: bool,
) -> String {
    let context = egui::Context::default();
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(4096.0, 2048.0),
        )),
        ..Default::default()
    };
    let mut draw = |context: &egui::Context| {
        egui::CentralPanel::default().show(context, |ui| {
            transactions_instances_ui(
                ui,
                app_context,
                |instance| match &subject_id {
                    Some(subject_id) => {
                        instance.initiator_id == *subject_id || instance.executor_id == *subject_id
                    }
                    None => true,
                },
                can_delete,
            );
        });
    };
    // The first frame measures the grid before the next frame paints its contents.
    let _ = context.run(input.clone(), &mut draw);
    let output = context.run(input, &mut draw);
    let mut text = String::new();
    for shape in output.shapes {
        collect_text(&shape.shape, &mut text);
    }
    text
}

#[test]
fn subject_view_shows_related_instances_without_delete_controls() {
    let (mut context, subject_id) = populated_context();

    let text = render_table(&mut context, Some(subject_id), false);

    assert!(text.contains("Initiated product"));
    assert!(text.contains("Executed product"));
    assert!(!text.contains("Unrelated product"));
    assert!(!text.contains('❌'));
    assert_eq!(context.execution.transactions_instances.len(), 3);
}

#[test]
fn global_view_shows_all_instances_with_delete_controls() {
    let (mut context, _) = populated_context();

    let text = render_table(&mut context, None, true);

    assert!(text.contains("Initiated product"));
    assert!(text.contains("Executed product"));
    assert!(text.contains("Unrelated product"));
    assert!(text.contains('❌'));
    assert_eq!(context.execution.transactions_instances.len(), 3);
}
