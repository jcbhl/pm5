#[derive(Debug)]
#[repr(C)]
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

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
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

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
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

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
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

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum RowingState {
    Inactive,
    Active,
}

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum StrokeState {
    WaitingForWheelToReachMinSpeedState,
    WaitingForWheelToAccelerateState,
    DrivingState,
    DwellingAfterDriveState,
    RecoveryState,
}

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum WorkoutDurationType {
    Time = 0,
    Calories = 0x40,
    Distance = 0x80,
    Watts = 0xC0,
}

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum GameID {
    None,
    Fish,
    Dart,
    TargetBasic,
    TargetAdvanced,
    Crosstraining,
}
