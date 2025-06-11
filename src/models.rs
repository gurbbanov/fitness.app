use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};

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
pub struct AccountData {
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
    pub workout_name: String,
    pub workout_date: String,
    pub workout_volume: u32,
    pub workout_length: u32,
    pub workout_prs: u32,
}

#[derive(Serialize, Deserialize)]
pub struct MacroData {
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


#[derive(Debug)]
pub enum MuscleGroup{
    Shoulders,
    Chest,
    Back,
    Biceps,
    Triceps,
    Forearms,
    Core,
    Legs,
}

pub struct Rest{}

pub struct Skip{}

pub struct WorkoutPattern {
    pub name: String,
    pub exercises: Vec<Exercise>,
}

pub enum Activities {
    Rest,
    Workout,
    Skip,
}

#[derive(Debug)]
pub struct Session {
    pub st: Instant,
    pub duration: Option<Duration>,
    pub date: OffsetDateTime,
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
