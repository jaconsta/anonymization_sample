use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub enum Status {
    NewLead,
    FollowUp,
    Converted,
    Closed,
    PendingPayment,
}

#[derive(Debug)]
pub struct StateBase<'a> {
    lead_id: &'a str,
    first_name: &'a str,
    last_name: &'a str,
    status: Status,
    followup_on: Option<usize>,
    created_on: usize,
    updated_on: usize,
}

impl StateBase<'_> {
    pub fn new<'a>(
        lead_id: &'a str,
        first_name: &'a str,
        last_name: &'a str,
        status: Status,
    ) -> StateBase<'a> {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let in_millis = since_the_epoch.as_millis() as usize;
        StateBase {
            lead_id,
            first_name,
            last_name,
            status,
            followup_on: None,
            created_on: in_millis,
            updated_on: in_millis,
        }
    }

    pub fn into_str(&self) -> String {
        let res = format!(
            "lead_id: {}, first_name: {}, last_name: {}, status: {:?}, followup_on: {:?}, created_on: {}, updated_on: {}, ",
            self.lead_id, self.first_name, self.last_name, self.status, self.followup_on, self.created_on, self.updated_on
        );
        res
    }
}
