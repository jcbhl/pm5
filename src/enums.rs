use num_enum::TryFromPrimitive;

// TODO consider repr C if crossing the ffi boundary

#[derive(Debug, TryFromPrimitive, PartialEq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum ErgType {
    StaticD,
    StaticC,
    StaticA,
    StaticB,
    StaticE = 5,
    StaticSimulator = 7,
    StaticDynamic = 8,
    SlidesA = 16,
    SlidesB,
    SlidesC,
    SlidesD,
    SlidesE,
    SlidesDynamic = 32,
    StaticDyno = 64,
    StaticSki = 128,
    StaticSkiSimulator = 143,
    Bike = 192,
    BikeArms,
    BikeNoarms,
    BikeSimulator = 207,
    MultiergRow = 224,
    MultiergSki = 225,
    MultiergBike = 226,
    Num,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, TryFromPrimitive, PartialEq)]
pub enum WorkoutType {
    JustRowNoSplits,
    JustRowSplits,
    FixedDistNosplits,
    FixedDistSplits,
    FixedTimeNosplits,
    FixedTimeSplits,
    FixedTimeInterval,
    FixedDistInterval,
    VariableInterval,
    VariableUndefinedrestInterval,
    FixedCalorie,
    FixedWattminutes,
    FixedCalsInterval,
    Num,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, TryFromPrimitive, PartialEq)]
pub enum IntervalType {
    Time,
    Dist,
    Rest,
    TimeRestUndefined,
    DistanceRestUndefined,
    RestUndefined,
    Cal,
    CalRestUndefined,
    WattMinute,
    WattMinuteRestUndefined,
    None = 255,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, TryFromPrimitive, PartialEq)]
pub enum WorkoutState {
    WaitToBegin,
    WorkoutRow,
    CountdownPause,
    IntervalRest,
    IntervalWorkTime,
    IntervalWorkDistance,
    IntervalRestEndToWorkTime,
    IntervalRestEndToWorkDistance,
    IntervalWorkTimeToRest,
    IntervalWorkDistanceToRest,
    WorkoutEnd,
    Terminate,
    WorkoutLogged,
    Rearm,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, TryFromPrimitive, PartialEq)]
pub enum RowingState {
    Inactive,
    Active,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, TryFromPrimitive, PartialEq)]
pub enum StrokeState {
    WaitingForWheelToReachMinSpeedState,
    WaitingForWheelToAccelerateState,
    DrivingState,
    DwellingAfterDriveState,
    RecoveryState,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, TryFromPrimitive, PartialEq)]
pub enum WorkoutDurationType {
    Time = 0,
    Calories = 0x40,
    Distance = 0x80,
    Watts = 0xC0,
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Debug, TryFromPrimitive, PartialEq)]
pub enum GameID {
    None,
    Fish,
    Dart,
    TargetBasic,
    TargetAdvanced,
    Crosstraining,
}
