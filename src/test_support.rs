//! Deterministic scenarios and input-driven egui tests, with no native window/GPU.
use crate::app::AppContext;
use crate::execution::TransactionInstance;
use crate::model::{
    ActorRole, AdtOption, CAct, CFact, CPAct, Impediment, Initiation, MaxMultiplicity,
    Multiplicity, Subject, Transaction,
};
use egui::epaint::Shape;

pub(crate) struct Fixture {
    pub context: AppContext,
    pub customer: Subject,
    pub supplier: Subject,
    pub worker: Subject,
    pub parent: Transaction,
    pub child: Transaction,
}

impl Fixture {
    pub fn new() -> Self {
        let mut context = AppContext::default();
        let roles: Vec<_> = ["customer", "supplier", "worker"]
            .into_iter()
            .map(|name| ActorRole {
                name: name.into(),
                ..Default::default()
            })
            .collect();
        let subjects: Vec<_> = ["Customer", "Supplier", "Worker"]
            .into_iter()
            .map(|name| Subject {
                name: name.into(),
                ..Default::default()
            })
            .collect();
        let mut parent = Transaction::new();
        parent.t_id = "T1".into();
        parent.name = "Parent".into();
        parent.product = "Parent product".into();
        parent.initiator_id = roles[0].id.clone();
        parent.executor_id = roles[1].id.clone();
        let mut child = Transaction::new();
        child.t_id = "T2".into();
        child.name = "Child".into();
        child.product = "Child product".into();
        child.initiator_id = roles[1].id.clone();
        child.executor_id = roles[2].id.clone();
        for (role, subject) in roles.iter().zip(&subjects) {
            context.model.adt.mappings.insert(
                (role.id.clone(), subject.id.clone()),
                AdtOption::Authorisation,
            );
        }
        context.model.actor_roles = roles;
        context.model.subjects = subjects.clone();
        context.model.transactions = vec![parent.clone(), child.clone()];
        Self {
            context,
            customer: subjects[0].clone(),
            supplier: subjects[1].clone(),
            worker: subjects[2].clone(),
            parent,
            child,
        }
    }

    pub fn linked(min: u8, max: MaxMultiplicity) -> Self {
        let mut f = Self::new();
        f.parent.initiations.push(Initiation {
            initiating_c_fact: CFact::Promised,
            initiated_transaction_id: f.child.id.clone(),
            initiated_c_act: CAct::Request,
            multiplicity: Multiplicity { min, max },
            multiplicity_tmp_str: String::new(),
        });
        f.parent.impediments.push(Impediment {
            impeded_act: CPAct::PAct,
            impeding_transaction_id: f.child.id.clone(),
            impeding_c_fact: CFact::Accepted,
        });
        f.context.model.transactions[0] = f.parent.clone();
        f
    }

    pub fn root(&mut self) -> TransactionInstance {
        let instance = TransactionInstance::new(
            None,
            self.parent.id.clone(),
            "Parent instance".into(),
            self.customer.id.clone(),
            self.supplier.id.clone(),
        );
        self.context
            .execution
            .add_transaction_instance(&self.context.model, instance.clone())
            .unwrap();
        instance
    }

    pub fn child_of(&mut self, parent: &TransactionInstance) -> TransactionInstance {
        let instance = TransactionInstance::new(
            Some(parent.id.clone()),
            self.child.id.clone(),
            "Child instance".into(),
            self.supplier.id.clone(),
            self.worker.id.clone(),
        );
        self.context
            .execution
            .add_transaction_instance(&self.context.model, instance.clone())
            .unwrap();
        instance
    }

    pub fn blocked(&self, instance: &TransactionInstance, act: &CPAct) -> bool {
        self.context
            .execution
            .get_act_impediments(
                &self.context.model,
                self.context.model.get_transaction(&instance.transaction_id),
                &Some(instance.id.clone()),
                act,
            )
            .is_some()
    }

    pub fn act(&mut self, instance: &TransactionInstance, act: CPAct) {
        let actor = match act {
            CPAct::CAct(CAct::Request | CAct::Accept | CAct::Reject) => &instance.initiator_id,
            _ => &instance.executor_id,
        };
        let agenda = self
            .context
            .execution
            .agenda_for(actor)
            .into_iter()
            .find(|item| item.transaction_instance_id == instance.id)
            .expect("actor must have an agendum for this instance");
        assert!(
            agenda.fact.next_acts().contains(&act),
            "act must be offered"
        );
        assert!(!self.blocked(instance, &act), "act must satisfy wait links");
        self.context
            .execution
            .process_new_fact(
                &self.context.model,
                instance.id.clone(),
                actor.clone(),
                act.to_fact(),
            )
            .unwrap();
        self.context.execution.remove_agenda_item(&agenda);
    }

    pub fn finish(&mut self, instance: &TransactionInstance) {
        for act in [
            CPAct::CAct(CAct::Promise),
            CPAct::PAct,
            CPAct::CAct(CAct::Declare),
            CPAct::CAct(CAct::Accept),
        ] {
            self.act(instance, act);
        }
    }
}

pub(crate) struct UiHarness {
    context: egui::Context,
    time: f64,
    pub texts: Vec<(String, egui::Pos2)>,
}

impl UiHarness {
    pub fn new() -> Self {
        Self {
            context: egui::Context::default(),
            time: 0.0,
            texts: Vec::new(),
        }
    }

    pub fn frame(&mut self, events: Vec<egui::Event>, draw: &mut impl FnMut(&mut egui::Ui)) {
        self.time += 1.0 / 60.0;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(4096.0, 2048.0),
            )),
            time: Some(self.time),
            events,
            ..Default::default()
        };
        let output = self.context.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| draw(ui));
        });
        self.texts.clear();
        for shape in output.shapes {
            self.collect(&shape.shape);
        }
    }

    fn collect(&mut self, shape: &Shape) {
        match shape {
            Shape::Text(text) => self.texts.push((
                text.galley.job.text.clone(),
                text.pos + text.galley.rect.center().to_vec2(),
            )),
            Shape::Vec(shapes) => {
                for shape in shapes {
                    self.collect(shape);
                }
            }
            _ => {}
        }
    }

    pub fn settle(&mut self, draw: &mut impl FnMut(&mut egui::Ui)) {
        // Grids need a measuring frame; do not assert against first-frame geometry.
        for _ in 0..3 {
            self.frame(Vec::new(), draw);
        }
    }

    pub fn click(&mut self, label: &str, occurrence: usize, draw: &mut impl FnMut(&mut egui::Ui)) {
        self.settle(draw);
        let pos = self
            .texts
            .iter()
            .filter(|(text, _)| text == label)
            .nth(occurrence)
            .unwrap_or_else(|| panic!("missing label {label:?}"))
            .1;
        for pressed in [true, false] {
            self.frame(
                vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
                draw,
            );
        }
        self.settle(draw);
    }
}
