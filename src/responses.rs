use super::enums::*;
use super::helpers::*;
use std::error::Error;
use std::time::Duration;

pub struct RowingStatusResponse {
    pub elapsed_time: Duration,
    // tenths of meters
    pub distance: u32,
    pub workout_type: WorkoutType,
    pub interval_type: IntervalType,
    pub workout_state: WorkoutState,
    pub rowing_state: RowingState,
    pub stroke_state: StrokeState,
    pub total_distance: u32,
    pub workout_duration: Duration,
    pub workoutduration_type: WorkoutDurationType,
    pub drag_factor: u8,
}

impl RowingStatusResponse {
    pub fn from_bytes(b: &[u8]) -> Result<RowingStatusResponse, Box<dyn Error>> {
        if b.len() != 19 {
            return Err("Length does not match".into());
        }

        Ok(RowingStatusResponse {
            elapsed_time: decode_to_time(b[0], b[1], b[2]),
            distance: decode_to_distance(b[3], b[4], b[5]),
            workout_type: WorkoutType::try_from(b[6])?,
            interval_type: IntervalType::try_from(b[7])?,
            workout_state: WorkoutState::try_from(b[8])?,
            rowing_state: RowingState::try_from(b[9])?,
            stroke_state: StrokeState::try_from(b[10])?,
            total_distance: decode_to_distance(b[11], b[12], b[13]), // this might have different units? FIXME
            workout_duration: decode_to_time(b[14], b[15], b[16]),
            workoutduration_type: WorkoutDurationType::try_from(b[17])?,
            drag_factor: b[18],
        })
    }
}

#[test]
fn test_rowing_status_parse() {
    let data = [
        0x5F, 0x06, 0x00, 0x5C, 0x01, 0x00, 0x01, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x80, 0x6A,
    ];

    let parsed = RowingStatusResponse::from_bytes(&data).unwrap();
    assert_eq!(parsed.workout_type, WorkoutType::JustRowSplits);
    assert_eq!(parsed.workout_state, WorkoutState::WorkoutRow);
    assert_eq!(parsed.rowing_state, RowingState::Inactive);
    assert_eq!(
        parsed.stroke_state,
        StrokeState::WaitingForWheelToAccelerateState
    );
}

#[test]
fn test_rowing_status_wrong_len() {
    let data = [
        0x5F, 0x06, 0x00, 0x5C, 0x00, 0x01, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x80, 0x6A,
    ];

    assert!(RowingStatusResponse::from_bytes(&data).is_err());
}

#[test]
fn test_rowing_status_invalid_enum() {
    let data = [
        0x5F, 0x06, 0x00, 0x5C, 0x01, 0x00, 0x42, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x80, 0x6A,
    ];

    assert!(RowingStatusResponse::from_bytes(&data).is_err());
}
