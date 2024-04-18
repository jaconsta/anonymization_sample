
pub mod analysis_projection {
    use std::collections::HashMap;

    use crate::lib::event_based::{sample_events::{ET, event_stream}, aggregate::analysis_model_projection::LeadAnalysisModelProjection};


    fn get_lead(lead_id: Option<&ET>) -> String {
        lead_id.unwrap_or(&ET::default()).as_string()
    }

    pub fn run_me_print<'a>() -> HashMap<String, LeadAnalysisModelProjection> {
        let lead_events = event_stream();
        let mut leads: HashMap<String, LeadAnalysisModelProjection> = HashMap::new();

        for lead_event in lead_events.iter() {
            let lead_id = get_lead(lead_event.get("lead-id"));
            let mut lead = match leads.get(&lead_id) {
                Some(l) => l.clone(),
                None => LeadAnalysisModelProjection::lead_initialized(&lead_id.clone()),
            };
            if let Some(l) = lead_event.get("event-type") {
                match l {
                    ET::S(i) => match i.as_str() {
                        "lead-initialized" => {
                        }
                        "contact-details-updated" => 
                            lead.contact_details_changed(),
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
mod test_analysis_projection {
    use crate::lib::event_based::analysis_projection::analysis_projection::run_me_print;


    #[test]
    fn test_generates_stream_events() {
        let events = run_me_print();
        let events_expected = 
        "{\"12\": LeadAnalysisModelProjection { lead_id: \"12\", follow_ups: 1, status: Converted, version: 6 }}";
        assert_eq!(format!("{:?}",events),events_expected );
    }
}
