use std::time::Instant;
use time::OffsetDateTime;
use crate::models::{Exercise, Session, Set, WorkoutPattern};

impl Exercise {
    fn new(name: String, targ_musc: crate::models::MuscleGroup, descript: Option<String>) -> Self {
        Exercise {name: name, target_muscle: targ_musc, description: descript, sets: vec![Set::default()]}
    }

    fn add_set(&mut self) {
        self.sets.push(Set::default());
    }

    fn remove_set(&mut self) {
        self.sets.pop();
    }
}

impl WorkoutPattern {
    fn new(name: String) -> Self {
        Self {name: name, exercises: Vec::new()}
    }

    fn add_exercise(&mut self, exer: Exercise) {
        self.exercises.push(exer);
    }

    fn remove_exercise(&mut self, ind: usize) {
        self.exercises.remove(ind);
    }
}

impl Session {
    fn start() -> Self {
        Self { st: Instant::now(), duration: None, date: OffsetDateTime::now_local().unwrap() }
    }

    fn stop(&mut self) {
        self.duration = Some(self.st.elapsed());
    }
}
