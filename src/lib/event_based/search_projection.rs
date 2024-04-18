pub mod search_projection {
    use std::collections::HashMap;

    use crate::lib::event_based::{
        aggregate::{entities::ContactDetails, seach_model_projection::LeadSearchModelProjection},
        sample_events::{event_stream, ET},
    };


    fn get_lead(lead_id: Option<&ET>) -> String {
        lead_id.unwrap_or(&ET::default()).as_string()
    }

    pub fn run_me_print<'a>() -> HashMap<String, LeadSearchModelProjection> {
        let lead_events = event_stream();
        let mut leads: HashMap<String, LeadSearchModelProjection> = HashMap::new();

        for lead_event in lead_events.iter() {
            let lead_id = get_lead(lead_event.get("lead-id"));
            let mut lead = match leads.get(&lead_id) {
                Some(l) => l.clone(),
                None => LeadSearchModelProjection::lead_initialized(&lead_id.clone()),
            };
            if let Some(l) = lead_event.get("event-type") {
                match l {
                    ET::S(i) => match i.as_str() {
                        "lead-initialized" => {
                            let first_name = lead_event
                                .get("first-name")
                                .unwrap_or(&ET::default())
                                .as_string();
                            let last_name = lead_event
                                .get("last-name")
                                .unwrap_or(&ET::default())
                                .as_string();
                            let phone_number = lead_event
                                .get("phone-number")
                                .unwrap_or(&ET::default())
                                .as_string();

                            let details =
                                &ContactDetails::new(&first_name, &last_name, &phone_number);
                            lead.set_initial_details(details);
                        }
                        "contact-details-updated" => {
                            let first_name = lead_event
                                .get("first-name")
                                .unwrap_or(&ET::default())
                                .as_string();
                            let last_name = lead_event
                                .get("last-name")
                                .unwrap_or(&ET::default())
                                .as_string();
                            let phone_number = lead_event
                                .get("phone-number")
                                .unwrap_or(&ET::default())
                                .as_string();

                            let details =
                                &ContactDetails::new(&first_name, &last_name, &phone_number);

                            lead.contact_details_changed(details);
                        }
                        "followup-set" => lead.followup_set(),
                        "contacted" => lead.contacted(),
                        "order-submitted" => lead.oder_submitted(),
                        "payment-confirmed" => lead.payment_confirmed(),
                        _ => {}
                    },
                    _ => {}
                };
                leads.insert(lead_id.clone(), lead);
            }
        }

        leads
    }
}

#[cfg(test)]
mod test_search_projection {
    use crate::lib::event_based::search_projection::search_projection::run_me_print;


    #[test]
    fn test_generates_stream_events() {
        let events = run_me_print();
        let events_expected = 
        "{\"12\": LeadSearchModelProjection { lead_id: \"12\", first_names: [\"Casey\", \"Casey\"], last_names: [\"David\", \"Davis\"], phone_numbers: [\"444-2951\", \"555-8101\"], version: 6 }}";
        assert_eq!(format!("{:?}",events),events_expected );
    }
}






