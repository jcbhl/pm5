use super::enums::*;
use super::helpers::*;
use std::error::Error;
use std::time::Duration;

pub struct GeneralStatus {
    pub elapsed_time: Duration,
    pub distance: u32, // tenths of meters
    pub workout_type: WorkoutType,
    pub interval_type: IntervalType,
    pub workout_state: WorkoutState,
    pub rowing_state: RowingState,
    pub stroke_state: StrokeState,
    pub total_distance: u32,
    pub workout_duration: Duration,
    pub workout_duration_type: WorkoutDurationType,
    pub drag_factor: u8,
}

impl GeneralStatus {
    pub fn from_bytes(b: &[u8]) -> Result<GeneralStatus, Box<dyn Error>> {
        if b.len() != 19 {
            return Err("Length does not match".into());
        }

        Ok(GeneralStatus {
            elapsed_time: decode_to_time(b[0], b[1], b[2]),
            distance: decode_to_distance(b[3], b[4], b[5]),
            workout_type: WorkoutType::try_from(b[6])?,
            interval_type: IntervalType::try_from(b[7])?,
            workout_state: WorkoutState::try_from(b[8])?,
            rowing_state: RowingState::try_from(b[9])?,
            stroke_state: StrokeState::try_from(b[10])?,
            total_distance: decode_to_distance(b[11], b[12], b[13]), // this might have different units? FIXME
            workout_duration: decode_to_time(b[14], b[15], b[16]),
            workout_duration_type: WorkoutDurationType::try_from(b[17])?,
            drag_factor: b[18],
        })
    }
}

pub struct AdditionalStatus1 {
    pub elapsed_time: Duration,
    pub speed: u16,
    pub spm: u8,
    pub hr: u8, // invalid at 255
    pub current_pace: u16,
    pub average_pace: u16,
    pub rest_distance: u16,
    pub rest_time: Duration,
    pub erg_type: ErgType,
}

impl AdditionalStatus1 {
    //FIXME test this
    pub fn from_bytes(b: &[u8]) -> Result<AdditionalStatus1, Box<dyn Error>> {
        if b.len() != 17 {
            return Err("Length does not match".into());
        }

        Ok(AdditionalStatus1 {
            elapsed_time: decode_to_time(b[0], b[1], b[2]),
            speed: decode_pair(b[3], b[4]),
            spm: b[5],
            hr: b[6],
            current_pace: decode_pair(b[7], b[8]),
            average_pace: decode_pair(b[9], b[10]),
            rest_distance: decode_pair(b[11], b[12]),
            rest_time: decode_to_time(b[13], b[14], b[15]),
            erg_type: ErgType::try_from(b[16])?,
        })
    }
}

#[allow(dead_code)]
struct SampleRate(u8);
impl SampleRate {
    pub fn from_bytes(b: &[u8]) -> Result<SampleRate, Box<dyn Error>> {
        if (b.len() != 1) {
            return Err("Length does not match".into());
        }
        Ok(SampleRate { 0: b[0] })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn general_status_parse() {
        let data = [
            0x5F, 0x06, 0x00, 0x5C, 0x01, 0x00, 0x01, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x80, 0x6A,
        ];

        let parsed = GeneralStatus::from_bytes(&data).unwrap();
        assert_eq!(parsed.workout_type, WorkoutType::JustRowSplits);
        assert_eq!(parsed.workout_state, WorkoutState::WorkoutRow);
        assert_eq!(parsed.rowing_state, RowingState::Inactive);
        assert_eq!(
            parsed.stroke_state,
            StrokeState::WaitingForWheelToAccelerateState
        );
    }

    #[test]
    fn general_status_wrong_len() {
        let data = [
            0x5F, 0x06, 0x00, 0x5C, 0x00, 0x01, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x80, 0x6A,
        ];

        assert!(GeneralStatus::from_bytes(&data).is_err());
    }

    #[test]
    fn general_status_invalid_enum() {
        let data = [
            0x5F, 0x06, 0x00, 0x5C, 0x01, 0x00, 0x42, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x80, 0x6A,
        ];

        assert!(GeneralStatus::from_bytes(&data).is_err());
    }
}
