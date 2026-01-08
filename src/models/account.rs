use chrono::NaiveDateTime;
use chrono::TimeDelta;

#[derive(Debug, Clone)]
pub enum CommandStyle {
    Unix,
    PlainEnglish,
}

#[derive(Debug, Clone)]
pub enum AutoCommitBehaviour {
    // automatically commit every [x] minutes
    Timer(TimeDelta),
    // automatically commit every [x] commits
    Count(u32),
    Off,
}

#[derive(Debug, Clone)]
pub enum AutoPullBehaviour {
    // automatically pull commits every [x] minutes
    Timer(TimeDelta),
    // automatically pull commits when online
    On,
    Off,
}

#[derive(Debug, Clone)]
pub enum AutoPushBehaviour {
    // automatically push commits every [x] minutes
    Timer(TimeDelta),
    // automatically push commits every [x] commits
    Count(u32),
    Off,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub preferred_command_style: CommandStyle,
    pub auto_commit_behaviour: AutoCommitBehaviour,
    pub auto_pull_behaviour: AutoPullBehaviour,
    pub auto_push_behaviour: AutoPushBehaviour,
}

/*
#[derive(Debug, Clone)]
pub struct PaymentInfo {
    pub paid_until: NaiveDateTime,
    pub subscription_type: ,
}
*/
