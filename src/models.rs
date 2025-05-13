use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct UserInformation {
    pub name: String,
    pub username: String,
    pub age: u32,
    pub weight: u32,
    pub height: u32,
    pub registration_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub level: u32,
    pub xp: u32,
    pub current_streak: u32,
    pub lifted_weight: u32,
    pub registrated_cals: u32,
    pub registrated_meals: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AllWorkoutData {
    pub total_volume: u32,
    pub total_sets: u32,
    pub total_reps: u32,
    pub total_time: u32,
    pub worked_out: u32,
    pub prs: u32,
    pub week_volume: u32,
    pub week_sets: u32,
    pub week_reps: u32,
    pub week_time: u32,
}

#[derive(Serialize, Deserialize)]
pub struct WorkoutData {
    pub workout_name: Workout
    
}

#[derive(Serialize, Deserialize)]
pub struct CaloryData {
    pub calory_goal: u32,
    pub protein_goal: u32,
    pub carb_goal: u32,
    pub fat_goal: u32,

    pub calory_registered: u32,
    pub protein_registered: u32,
    pub carb_registered: u32,
    pub fat_registered: u32,
    pub meal_registered: u32,
}

#[derive(Debug)]
pub struct Exercise {
    pub name: String,
    pub target_muscle: MuscleGroup,
    pub description: Option<String>,
    pub sets: Vec<Set>,
}

impl Exercise {
    fn new(name: String, targ_musc: MuscleGroup, descript: Option<String>) -> Self {
        Exercise {name: name, target_muscle: targ_musc, description: descript, sets: vec![Set::default()]}
    }

    fn add_set(&mut self) {
        self.sets.push(Set::default());
    }

    fn remove_set(&mut self) {
        self.sets.pop();
    }
}

#[derive(Debug)]
enum MuscleGroup{
    Shoulders,
    Chest,
    Back,
    Biceps,
    Triceps,
    Forearms,
    Core,
    Legs,
}

struct Rest{}

#[derive(Serialize, Deserialize)]
struct Workout {
    name: String,
    exercises: Vec<Exercise>,
}

impl Workout {
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

enum Activities {
    Rest,
    Workout,
}

#[derive(Debug)]
struct Session {
    st: Instant,
    duration: Option<Duration>,
    date: OffsetDateTime,
}

impl Session {
    fn start() -> Self {
        Self { st: Instant::now(), duration: None, date: OffsetDateTime::now_local().unwrap() }
    }

    fn stop(&mut self) {
        self.duration = Some(self.st.elapsed());
    }
}

#[derive(Debug, Default)]
pub struct Set {
    reps: u32,
    weight: u32,
}

#[derive(Serialize, Deserialize)]
pub struct States {
    pub calory_add_modal: bool,
    pub calories: u32,
    pub proteins: u32,
    pub carbs: u32,
    pub fats: u32,
}
