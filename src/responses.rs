use super::enums::*;
use std::time::Duration;

pub struct RowingStatusResponse {
    pub elapsed_time: Duration,
    pub distance: u32,
    pub workouttype: WorkoutType,
    pub intervaltype: IntervalType,
    pub workoutstate: WorkoutState,
    pub rowingstate: RowingState,
    pub strokestate: StrokeState,
    pub totaldistance: u32,
    pub workoutduration: u32,
    pub workoutdurationtype: WorkoutDurationType,
    pub df: u8,
}
