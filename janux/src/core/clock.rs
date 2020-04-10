
#[derive(Clone, Copy)]
pub struct Time
{
    value: u128,
}

impl Time // Static
{
    pub fn from_nanos(nanos: u128)      -> Self { Self{ value: nanos } }
    pub fn from_micros(micros: u128)    -> Self { Self::from_nanos(micros * 1000) }
    pub fn from_millis(millis: u128)    -> Self { Self::from_micros(millis * 1000) }
    pub fn from_seconds(seconds: u128)  -> Self { Self::from_millis(seconds * 1000) }
    pub fn from_minutes(minutes: u128)  -> Self { Self::from_seconds(minutes * 60) }
    pub fn from_hours(hours: u128)      -> Self { Self::from_minutes(hours * 60) }
}

impl Time // Member
{
    pub fn nanos(&self)                 -> u128 { self.value }
    pub fn micros(&self)                -> u128 { self.nanos() / 1000 }
    pub fn millis(&self)                -> u128 { self.micros() / 1000 }
    pub fn seconds(&self)               -> u128 { self.millis() / 1000 }
    pub fn minutes(&self)               -> u128 { self.seconds() / 1000 }
    pub fn hours(&self)                 -> u128 { self.minutes() / 60 }
}

#[derive(Clone, Copy)]
pub struct Clock
{
    current_time: Time,
}

impl Clock // Static
{
    pub fn new() -> Self
    {
        Self
        {
            current_time: Time{ value: std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_nanos() },
        }
    }
}

impl Clock // Member
{
    pub fn now(&mut self) -> Time
    {
        self.current_time = Time{ value: std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_nanos() };
        self.current_time
    }

    pub fn time_since(&mut self, time: Time) -> Time
    {
        Time::from_nanos(self.now().nanos() - time.nanos())
    }
}