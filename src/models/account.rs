use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::TimeDelta;

#[derive(Debug, Clone)]
pub enum CommandStyle {
    Unix,
    PlainEnglish,
}

impl CommandStyle {
    pub fn to_string(&self) -> &'static str {
        match self {
            CommandStyle::Unix => "terminal style",
            CommandStyle::PlainEnglish => "plain-english style",
        }
    }
    pub fn from_string(s: &str) -> Option<CommandStyle> {
        match s {
            "terminal style" => Some(CommandStyle::Unix),
            "plain-english style" => Some(CommandStyle::PlainEnglish),
            _ => None,
        }
    }
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

#[derive(Debug, Clone)]
pub enum SubscriptionType {
    CloudSync,
    SyncCollaborate,
}

#[derive(Debug, Clone)]
pub struct SubscriptionInfo {
    pub paid_until: NaiveDateTime,
    pub subscription_type: SubscriptionType,
}

impl SubscriptionInfo {
    pub fn is_active(&self) -> bool {
        let now = chrono::Utc::now().naive_utc();
        return self.paid_until > now;
    }
}

#[derive(Debug, Clone)]
pub struct Payment {
    pub payment_id: String, // e.g., Stripe payment intent ID
    pub user_id: Uuid,
    pub amount_cents: u32,
    pub payment_date: NaiveDateTime,
}

