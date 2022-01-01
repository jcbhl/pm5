use super::enums::*;
use std::time::Duration;

pub struct RowingStatusResponse {
    pub elapsed_time: Duration,
    pub distance: u32,
    pub workout_type: WorkoutType,
    pub interval_type: IntervalType,
    pub workout_state: WorkoutState,
    pub rowing_state: RowingState,
    pub stroke_state: StrokeState,
    pub total_distance: u32,
    pub workout_duration: u32,
    pub workoutduration_type: WorkoutDurationType,
    pub drag_factor: u8,
}
