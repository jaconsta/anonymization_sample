pub mod entities {
    #[derive(Debug, Clone, Copy)]
    pub struct ContactDetails<'a> {
        // First name
        pub first_name: &'a str,
        // Last name
        pub last_name: &'a str,
        // phone number
        pub phone_number: &'a str,
    }

    impl ContactDetails<'_> {
        pub fn new<'a>(
            first_name: &'a str,
            last_name: &'a str,
            phone_number: &'a str,
        ) -> ContactDetails<'a> {
            ContactDetails {
                first_name,
                last_name,
                phone_number,
            }
        }
    }
}
pub mod seach_model_projection {
    use super::entities::ContactDetails;

    #[allow(dead_code)] // lead_id
    #[derive(Debug, Clone)]
    pub struct LeadSearchModelProjection {
        lead_id: String,
        first_names: Vec<String>,
        last_names: Vec<String>,
        phone_numbers: Vec<String>,
        version: u8,
    }

    impl LeadSearchModelProjection {
        pub fn lead_initialized<'a>(lead_id: &'a str) -> LeadSearchModelProjection {
            LeadSearchModelProjection {
                lead_id: lead_id.clone().into(),
                first_names: Vec::new(),
                last_names: Vec::new(),
                phone_numbers: Vec::new(),
                version: 0,
            }
        }

        pub fn set_initial_details(&mut self, details: &ContactDetails) {
            if self.version != 0 {
                return;
            }
            self.first_names.push(details.first_name.into());
            self.last_names.push(details.last_name.into());
            self.phone_numbers.push(details.phone_number.into());
        }

        pub fn contact_details_changed(&mut self, details: &ContactDetails) {
            self.first_names.push(details.first_name.clone().into());
            self.last_names.push(details.last_name.clone().into());
            self.phone_numbers.push(details.phone_number.clone().into());
            self.version += 1;
        }

        pub fn contacted(&mut self) {
            self.version += 1;
        }
        pub fn followup_set(&mut self) {
            self.version += 1;
        }
        pub fn oder_submitted(&mut self) {
            self.version += 1;
        }
        pub fn payment_confirmed(&mut self) {
            self.version += 1;
        }
    }
}

pub mod analysis_model_projection {
    use crate::lib::state_based::entity::Status;

    #[allow(dead_code)] // lead_id
    #[derive(Debug, Clone)]
    pub struct LeadAnalysisModelProjection {
        lead_id: String,
        follow_ups: u8,
        status: Status,
        version: u8,
    }

    impl LeadAnalysisModelProjection {
        pub fn lead_initialized(lead_id: &str) -> Self {
            LeadAnalysisModelProjection {
                lead_id: String::from(lead_id),
                follow_ups: 0,
                status: Status::NewLead,
                version: 0,
            }
        }
        pub fn followup_set(&mut self) {
            self.status = Status::FollowUp;
            self.follow_ups += 1;
            self.version += 1;
        }
        pub fn contact_details_changed(&mut self) {
            self.version += 1;
        }
        pub fn contacted(&mut self) {
            self.version += 1;
        }
        pub fn oder_submitted(&mut self) {
            self.status = Status::PendingPayment;
            self.version += 1;
        }
        pub fn payment_confirmed(&mut self) {
            self.status = Status::Converted;
            self.version += 1;
        }
    }
}
