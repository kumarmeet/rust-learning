mod cardio;
mod diet;
mod weightlifting;
pub use cardio::{CardioTool, Exercise as CardioExercise};
use diet as gym_diet;
pub use weightlifting::Exercise as WeightliftExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftExercise,
}

impl GymWorkout {
    pub fn new() -> Self {
        cardio::ask_about_program();
        weightlifting::ask_about_program();
        gym_diet::diet::ask_about_program();

        Self {
            cardio: CardioExercise::new("monday".to_string(), CardioTool::Bike, 10),
            weightlifting: WeightliftExercise::new("dead lift".to_string(), 6),
        }
    }
}
