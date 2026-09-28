use phf_macros::phf_map;

use super::v2_tables::{V2Table, V2TableRow};

pub static TABLE_0001: V2Table = V2Table {
    number: 1,
    metadata: &super::metadata::TABLE_0001_METADATA,
    rows: phf_map! {
        "F" => V2TableRow { value: "F", display_name: "Female", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Male", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Ambiguous", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Not applicable", definition: "", comment_usage_note: "", status: "" },
        "X" => V2TableRow { value: "X", display_name: "Non-Binary", definition: "Intended for situations where the gender or sex representation of the individual is not strictly male, or strictly female, and is driven by jurisdictional requirements, personal needs, or legal boundaries.", comment_usage_note: "A universally agreed upon single definition for non- binary does not exist. Non- binary should be used in jurisdictions which have implemented an option to declare a gender of “non-binary”", status: "N" },
    },
};

pub static TABLE_0002: V2Table = V2Table {
    number: 2,
    metadata: &super::metadata::TABLE_0002_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Separated", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Divorced", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Married", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Single", definition: "", comment_usage_note: "", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Widowed", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Common law", definition: "", comment_usage_note: "", status: "" },
        "G" => V2TableRow { value: "G", display_name: "Living together", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Domestic partner", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Registered domestic partner", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Legally Separated", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Annulled", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Interlocutory", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Unmarried", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Unreported", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0003: V2Table = V2Table {
    number: 3,
    metadata: &super::metadata::TABLE_0003_METADATA,
    rows: phf_map! {
        "A01" => V2TableRow { value: "A01", display_name: "ADT/ACK - Admit/visit notification", definition: "", comment_usage_note: "", status: "" },
        "A02" => V2TableRow { value: "A02", display_name: "ADT/ACK - Transfer a patient", definition: "", comment_usage_note: "", status: "" },
        "A03" => V2TableRow { value: "A03", display_name: "ADT/ACK - Discharge/end visit", definition: "", comment_usage_note: "", status: "" },
        "A04" => V2TableRow { value: "A04", display_name: "ADT/ACK - Register a patient", definition: "", comment_usage_note: "", status: "" },
        "A05" => V2TableRow { value: "A05", display_name: "ADT/ACK - Pre-admit a patient", definition: "", comment_usage_note: "", status: "" },
        "A06" => V2TableRow { value: "A06", display_name: "ADT/ACK - Change an outpatient to an inpatient", definition: "", comment_usage_note: "", status: "" },
        "A07" => V2TableRow { value: "A07", display_name: "ADT/ACK - Change an inpatient to an outpatient", definition: "", comment_usage_note: "", status: "" },
        "A08" => V2TableRow { value: "A08", display_name: "ADT/ACK - Update patient information", definition: "", comment_usage_note: "", status: "" },
        "A09" => V2TableRow { value: "A09", display_name: "ADT/ACK - Patient departing - tracking", definition: "", comment_usage_note: "", status: "" },
        "A10" => V2TableRow { value: "A10", display_name: "ADT/ACK - Patient arriving - tracking", definition: "", comment_usage_note: "", status: "" },
        "A11" => V2TableRow { value: "A11", display_name: "ADT/ACK - Cancel admit/visit notification", definition: "", comment_usage_note: "", status: "" },
        "A12" => V2TableRow { value: "A12", display_name: "ADT/ACK - Cancel transfer", definition: "", comment_usage_note: "", status: "" },
        "A13" => V2TableRow { value: "A13", display_name: "ADT/ACK - Cancel discharge/end visit", definition: "", comment_usage_note: "", status: "" },
        "A14" => V2TableRow { value: "A14", display_name: "ADT/ACK - Pending admit", definition: "", comment_usage_note: "", status: "" },
        "A15" => V2TableRow { value: "A15", display_name: "ADT/ACK - Pending transfer", definition: "", comment_usage_note: "", status: "" },
        "A16" => V2TableRow { value: "A16", display_name: "ADT/ACK - Pending discharge", definition: "", comment_usage_note: "", status: "" },
        "A17" => V2TableRow { value: "A17", display_name: "ADT/ACK - Swap patients", definition: "", comment_usage_note: "", status: "" },
        "A18" => V2TableRow { value: "A18", display_name: "ADT/ACK - Merge patient information", definition: "", comment_usage_note: "Deprecated", status: "D" },
        "A19" => V2TableRow { value: "A19", display_name: "QRY/ADR - Patient query", definition: "", comment_usage_note: "Deprecated", status: "D" },
        "A20" => V2TableRow { value: "A20", display_name: "ADT/ACK - Bed status update", definition: "", comment_usage_note: "", status: "" },
        "A21" => V2TableRow { value: "A21", display_name: "ADT/ACK - Patient goes on a \"leave of absence\"", definition: "", comment_usage_note: "", status: "" },
        "A22" => V2TableRow { value: "A22", display_name: "ADT/ACK - Patient returns from a \"leave of absence\"", definition: "", comment_usage_note: "", status: "" },
        "A23" => V2TableRow { value: "A23", display_name: "ADT/ACK - Delete a patient record", definition: "", comment_usage_note: "", status: "" },
        "A24" => V2TableRow { value: "A24", display_name: "ADT/ACK - Link patient information", definition: "", comment_usage_note: "", status: "" },
        "A25" => V2TableRow { value: "A25", display_name: "ADT/ACK - Cancel pending discharge", definition: "", comment_usage_note: "", status: "" },
        "A26" => V2TableRow { value: "A26", display_name: "ADT/ACK - Cancel pending transfer", definition: "", comment_usage_note: "", status: "" },
        "A27" => V2TableRow { value: "A27", display_name: "ADT/ACK - Cancel pending admit", definition: "", comment_usage_note: "", status: "" },
        "A28" => V2TableRow { value: "A28", display_name: "ADT/ACK - Add person information", definition: "", comment_usage_note: "", status: "" },
        "A29" => V2TableRow { value: "A29", display_name: "ADT/ACK - Delete person information", definition: "", comment_usage_note: "", status: "" },
        "A30" => V2TableRow { value: "A30", display_name: "ADT/ACK - Merge person information", definition: "", comment_usage_note: "Deprecated", status: "D" },
        "A31" => V2TableRow { value: "A31", display_name: "ADT/ACK - Update person information", definition: "", comment_usage_note: "", status: "" },
        "A32" => V2TableRow { value: "A32", display_name: "ADT/ACK - Cancel patient arriving - tracking", definition: "", comment_usage_note: "", status: "" },
        "A33" => V2TableRow { value: "A33", display_name: "ADT/ACK - Cancel patient departing - tracking", definition: "", comment_usage_note: "", status: "" },
        "A34" => V2TableRow { value: "A34", display_name: "ADT/ACK - Merge patient information - patient ID only", definition: "", comment_usage_note: "Deprecated", status: "D" },
        "A35" => V2TableRow { value: "A35", display_name: "ADT/ACK - Merge patient information - account number only", definition: "", comment_usage_note: "Deprecated", status: "D" },
        "A36" => V2TableRow { value: "A36", display_name: "ADT/ACK - Merge patient information - patient ID and account number", definition: "", comment_usage_note: "Deprecated", status: "D" },
        "A37" => V2TableRow { value: "A37", display_name: "ADT/ACK - Unlink patient information", definition: "", comment_usage_note: "", status: "" },
        "A38" => V2TableRow { value: "A38", display_name: "ADT/ACK - Cancel pre-admit", definition: "", comment_usage_note: "", status: "" },
        "A39" => V2TableRow { value: "A39", display_name: "ADT/ACK - Merge person - patient ID", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "A40" => V2TableRow { value: "A40", display_name: "ADT/ACK - Merge patient - patient identifier list", definition: "", comment_usage_note: "", status: "" },
        "A41" => V2TableRow { value: "A41", display_name: "ADT/ACK - Merge account - patient account number", definition: "", comment_usage_note: "", status: "" },
        "A42" => V2TableRow { value: "A42", display_name: "ADT/ACK - Merge visit - visit number", definition: "", comment_usage_note: "", status: "" },
        "A43" => V2TableRow { value: "A43", display_name: "ADT/ACK - Move patient information - patient identifier list", definition: "", comment_usage_note: "", status: "" },
        "A44" => V2TableRow { value: "A44", display_name: "ADT/ACK - Move account information - patient account number", definition: "", comment_usage_note: "", status: "" },
        "A45" => V2TableRow { value: "A45", display_name: "ADT/ACK - Move visit information - visit number", definition: "", comment_usage_note: "", status: "" },
        "A46" => V2TableRow { value: "A46", display_name: "ADT/ACK - Change patient ID", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "A47" => V2TableRow { value: "A47", display_name: "ADT/ACK - Change patient identifier list", definition: "", comment_usage_note: "", status: "" },
        "A48" => V2TableRow { value: "A48", display_name: "ADT/ACK - Change alternate patient ID", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "A49" => V2TableRow { value: "A49", display_name: "ADT/ACK - Change patient account number", definition: "", comment_usage_note: "", status: "" },
        "A50" => V2TableRow { value: "A50", display_name: "ADT/ACK - Change visit number", definition: "", comment_usage_note: "", status: "" },
        "A51" => V2TableRow { value: "A51", display_name: "ADT/ACK - Change alternate visit ID", definition: "", comment_usage_note: "", status: "" },
        "A52" => V2TableRow { value: "A52", display_name: "ADT/ACK - Cancel leave of absence for a patient", definition: "", comment_usage_note: "", status: "" },
        "A53" => V2TableRow { value: "A53", display_name: "ADT/ACK - Cancel patient returns from a leave of absence", definition: "", comment_usage_note: "", status: "" },
        "A54" => V2TableRow { value: "A54", display_name: "ADT/ACK - Change attending doctor", definition: "", comment_usage_note: "", status: "" },
        "A55" => V2TableRow { value: "A55", display_name: "ADT/ACK - Cancel change attending doctor", definition: "", comment_usage_note: "", status: "" },
        "A60" => V2TableRow { value: "A60", display_name: "ADT/ACK - Update allergy information", definition: "", comment_usage_note: "", status: "" },
        "A61" => V2TableRow { value: "A61", display_name: "ADT/ACK - Change consulting doctor", definition: "", comment_usage_note: "", status: "" },
        "A62" => V2TableRow { value: "A62", display_name: "ADT/ACK - Cancel change consulting doctor", definition: "", comment_usage_note: "", status: "" },
        "B01" => V2TableRow { value: "B01", display_name: "PMU/ACK - Add personnel record", definition: "", comment_usage_note: "", status: "" },
        "B02" => V2TableRow { value: "B02", display_name: "PMU/ACK - Update personnel record", definition: "", comment_usage_note: "", status: "" },
        "B03" => V2TableRow { value: "B03", display_name: "PMU/ACK - Delete personnel re cord", definition: "", comment_usage_note: "", status: "" },
        "B04" => V2TableRow { value: "B04", display_name: "PMU/ACK - Active practicing person", definition: "", comment_usage_note: "", status: "" },
        "B05" => V2TableRow { value: "B05", display_name: "PMU/ACK - Deactivate practicing person", definition: "", comment_usage_note: "", status: "" },
        "B06" => V2TableRow { value: "B06", display_name: "PMU/ACK - Terminate practicing person", definition: "", comment_usage_note: "", status: "" },
        "B07" => V2TableRow { value: "B07", display_name: "PMU/ACK - Grant Certificate/Permission", definition: "", comment_usage_note: "", status: "" },
        "B08" => V2TableRow { value: "B08", display_name: "PMU/ACK - Revoke Certificate/Permission", definition: "", comment_usage_note: "", status: "" },
        "C01" => V2TableRow { value: "C01", display_name: "CRM - Register a patient on a clinical trial", definition: "", comment_usage_note: "", status: "" },
        "C02" => V2TableRow { value: "C02", display_name: "CRM - Cancel a patient registration on clinical trial (for clerical mistakes only)", definition: "", comment_usage_note: "", status: "" },
        "C03" => V2TableRow { value: "C03", display_name: "CRM - Correct/update registration information", definition: "", comment_usage_note: "", status: "" },
        "C04" => V2TableRow { value: "C04", display_name: "CRM - Patient has gone off a clinical trial", definition: "", comment_usage_note: "", status: "" },
        "C05" => V2TableRow { value: "C05", display_name: "CRM - Patient enters phase of clinical trial", definition: "", comment_usage_note: "", status: "" },
        "C06" => V2TableRow { value: "C06", display_name: "CRM - Cancel patient entering a phase (clerical mistake)", definition: "", comment_usage_note: "", status: "" },
        "C07" => V2TableRow { value: "C07", display_name: "CRM - Correct/update phase information", definition: "", comment_usage_note: "", status: "" },
        "C08" => V2TableRow { value: "C08", display_name: "CRM - Patient has gone off phase of clinical trial", definition: "", comment_usage_note: "", status: "" },
        "C09" => V2TableRow { value: "C09", display_name: "CSU - Automated time intervals for reporting, like monthly", definition: "", comment_usage_note: "", status: "" },
        "C10" => V2TableRow { value: "C10", display_name: "CSU - Patient completes the clinical trial", definition: "", comment_usage_note: "", status: "" },
        "C11" => V2TableRow { value: "C11", display_name: "CSU - Patient completes a phase of the clinical trial", definition: "", comment_usage_note: "", status: "" },
        "C12" => V2TableRow { value: "C12", display_name: "CSU - Update/correction of patient order/result information", definition: "", comment_usage_note: "", status: "" },
        "CNQ" => V2TableRow { value: "CNQ", display_name: "Cancel Query", definition: "", comment_usage_note: "", status: "" },
        "E01" => V2TableRow { value: "E01", display_name: "Submit HealthCare Services Invoice", definition: "", comment_usage_note: "", status: "" },
        "E02" => V2TableRow { value: "E02", display_name: "Cancel HealthCare Services Invoice", definition: "", comment_usage_note: "", status: "" },
        "E03" => V2TableRow { value: "E03", display_name: "HealthCare Services Invoice Status", definition: "", comment_usage_note: "", status: "" },
        "E04" => V2TableRow { value: "E04", display_name: "Re-Assess HealthCare Services Invoice Request", definition: "", comment_usage_note: "", status: "" },
        "E10" => V2TableRow { value: "E10", display_name: "Edit/Adjudication Results", definition: "", comment_usage_note: "", status: "" },
        "E12" => V2TableRow { value: "E12", display_name: "Request Additional Information", definition: "", comment_usage_note: "", status: "" },
        "E13" => V2TableRow { value: "E13", display_name: "Additional Information Response", definition: "", comment_usage_note: "", status: "" },
        "E15" => V2TableRow { value: "E15", display_name: "Payment/Remittance Advice", definition: "", comment_usage_note: "", status: "" },
        "E20" => V2TableRow { value: "E20", display_name: "Submit Authorization Request", definition: "", comment_usage_note: "", status: "" },
        "E21" => V2TableRow { value: "E21", display_name: "Cancel Authorization Request", definition: "", comment_usage_note: "", status: "" },
        "E22" => V2TableRow { value: "E22", display_name: "Authorization Request Status", definition: "", comment_usage_note: "", status: "" },
        "E24" => V2TableRow { value: "E24", display_name: "Authorization Response", definition: "", comment_usage_note: "", status: "" },
        "E30" => V2TableRow { value: "E30", display_name: "Submit Health Document related to Authorization Request", definition: "res fut def", comment_usage_note: "erved for ure/not yet ined", status: "" },
        "E31" => V2TableRow { value: "E31", display_name: "Cancel Health Document related to Authorization Request", definition: "res fut def", comment_usage_note: "erved for ure/not yet ined", status: "" },
        "I01" => V2TableRow { value: "I01", display_name: "RQI/RPI - Request for insurance information", definition: "", comment_usage_note: "", status: "" },
        "I02" => V2TableRow { value: "I02", display_name: "RQI/RPL - Request/receipt of patient selection display list", definition: "", comment_usage_note: "", status: "" },
        "I03" => V2TableRow { value: "I03", display_name: "RQI/RPR - Request/receipt of patient selection list", definition: "", comment_usage_note: "", status: "" },
        "I04" => V2TableRow { value: "I04", display_name: "RQD/ RPI - Request for patient demographic data", definition: "", comment_usage_note: "", status: "" },
        "I05" => V2TableRow { value: "I05", display_name: "RQC/RCI - Request for patient clinical information", definition: "Dep", comment_usage_note: "recated D", status: "" },
        "I06" => V2TableRow { value: "I06", display_name: "RQC/RCL - Request/receipt of clinical data listing", definition: "Dep", comment_usage_note: "recated D", status: "" },
        "I07" => V2TableRow { value: "I07", display_name: "PIN/ACK - Unsolicited insura nce information", definition: "", comment_usage_note: "", status: "" },
        "I08" => V2TableRow { value: "I08", display_name: "RQA/RPA - Request for treatment authorization information", definition: "", comment_usage_note: "", status: "" },
        "I09" => V2TableRow { value: "I09", display_name: "RQA/RPA - Request for modification to an authorization", definition: "", comment_usage_note: "", status: "" },
        "I10" => V2TableRow { value: "I10", display_name: "RQA/RPA - Request for resubmission of an authorization", definition: "", comment_usage_note: "", status: "" },
        "I11" => V2TableRow { value: "I11", display_name: "RQA/RPA - Request for cancellation of an authorization", definition: "", comment_usage_note: "", status: "" },
        "I12" => V2TableRow { value: "I12", display_name: "REF/ RRI - Patient referral", definition: "", comment_usage_note: "", status: "" },
        "I13" => V2TableRow { value: "I13", display_name: "REF/ RRI - Modify patient referral", definition: "", comment_usage_note: "", status: "" },
        "I14" => V2TableRow { value: "I14", display_name: "REF/ RRI - Cancel patient referral", definition: "", comment_usage_note: "", status: "" },
        "I15" => V2TableRow { value: "I15", display_name: "REF/ RRI - Request patient referral status", definition: "", comment_usage_note: "", status: "" },
        "I16" => V2TableRow { value: "I16", display_name: "Collaborative Care Referral", definition: "", comment_usage_note: "", status: "" },
        "I17" => V2TableRow { value: "I17", display_name: "Modify Collaborative Care Referral", definition: "Dep", comment_usage_note: "recated D", status: "" },
        "I18" => V2TableRow { value: "I18", display_name: "Cancel Collaborative Care Referral", definition: "Dep", comment_usage_note: "recated D", status: "" },
        "I19" => V2TableRow { value: "I19", display_name: "Collaborative Care Query/Collaborative Care Query Update", definition: "", comment_usage_note: "", status: "" },
        "I20" => V2TableRow { value: "I20", display_name: "Asynchronous Collaborative Care Update", definition: "", comment_usage_note: "", status: "" },
        "I21" => V2TableRow { value: "I21", display_name: "Collaborative Care Message", definition: "", comment_usage_note: "", status: "" },
        "I22" => V2TableRow { value: "I22", display_name: "Collaborative Care Fetch / Collaborative Care Information", definition: "", comment_usage_note: "", status: "" },
        "J01" => V2TableRow { value: "J01", display_name: "QCN/ACK - Cancel query/acknowledge message", definition: "", comment_usage_note: "", status: "" },
        "J02" => V2TableRow { value: "J02", display_name: "QSX/ACK - Cancel subscription/acknowledge message", definition: "", comment_usage_note: "", status: "" },
        "K11" => V2TableRow { value: "K11", display_name: "RSP - Segment pattern response in response to QBP^Q11", definition: "", comment_usage_note: "", status: "" },
        "K13" => V2TableRow { value: "K13", display_name: "RTB - Tabular response in response to QBP^Q13", definition: "", comment_usage_note: "", status: "" },
        "K15" => V2TableRow { value: "K15", display_name: "RDY - Display response in response to QBP^Q15", definition: "", comment_usage_note: "", status: "" },
        "K21" => V2TableRow { value: "K21", display_name: "RSP - Get person demographics response", definition: "", comment_usage_note: "", status: "" },
        "K22" => V2TableRow { value: "K22", display_name: "RSP - Find candidates response", definition: "", comment_usage_note: "", status: "" },
        "K23" => V2TableRow { value: "K23", display_name: "RSP - Get corresponding identifiers response", definition: "", comment_usage_note: "", status: "" },
        "K24" => V2TableRow { value: "K24", display_name: "RSP - Allocate identifiers response", definition: "", comment_usage_note: "", status: "" },
        "K25" => V2TableRow { value: "K25", display_name: "RSP - Personnel Information by Segment Response", definition: "", comment_usage_note: "", status: "" },
        "K31" => V2TableRow { value: "K31", display_name: "RSP -Dispense History Response", definition: "", comment_usage_note: "", status: "" },
        "K32" => V2TableRow { value: "K32", display_name: "Find Candidates including Visit Information Response", definition: "", comment_usage_note: "", status: "" },
        "K33" => V2TableRow { value: "K33", display_name: "Get Donor Record Candidates Response Message", definition: "", comment_usage_note: "", status: "" },
        "K34" => V2TableRow { value: "K34", display_name: "Segment Pattern Response Message", definition: "", comment_usage_note: "", status: "" },
        "M01" => V2TableRow { value: "M01", display_name: "MFN/MFK - Master file not otherwise specified", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "M02" => V2TableRow { value: "M02", display_name: "MFN/MFK - Master file - staff practitioner", definition: "", comment_usage_note: "", status: "" },
        "M03" => V2TableRow { value: "M03", display_name: "MFN/MFK - Master file - test/observation", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "M04" => V2TableRow { value: "M04", display_name: "MFN/MFK - Master files charge description", definition: "", comment_usage_note: "", status: "" },
        "M05" => V2TableRow { value: "M05", display_name: "MFN/MFK - Patient location master file", definition: "", comment_usage_note: "", status: "" },
        "M06" => V2TableRow { value: "M06", display_name: "MFN/MFK - Clinical study with phases and schedules master file", definition: "", comment_usage_note: "", status: "" },
        "M07" => V2TableRow { value: "M07", display_name: "MFN/MFK - Clinical study without phases but with schedules master file", definition: "", comment_usage_note: "", status: "" },
        "M08" => V2TableRow { value: "M08", display_name: "MFN/MFK - Test/observation (numeric) master file", definition: "", comment_usage_note: "", status: "" },
        "M09" => V2TableRow { value: "M09", display_name: "MFN/MFK - Test/observation (categorical) master file", definition: "", comment_usage_note: "", status: "" },
        "M10" => V2TableRow { value: "M10", display_name: "MFN/MFK - Test /observation batteries master file", definition: "", comment_usage_note: "", status: "" },
        "M11" => V2TableRow { value: "M11", display_name: "MFN/MFK - Test/ca lculated observations master file", definition: "", comment_usage_note: "", status: "" },
        "M12" => V2TableRow { value: "M12", display_name: "MFN/MFK - Master file notification message", definition: "", comment_usage_note: "", status: "" },
        "M13" => V2TableRow { value: "M13", display_name: "MFN/MFK - Master file notification - general", definition: "", comment_usage_note: "", status: "" },
        "M14" => V2TableRow { value: "M14", display_name: "MFN/MFK - Master file notification - site defined", definition: "", comment_usage_note: "", status: "" },
        "M15" => V2TableRow { value: "M15", display_name: "MFN/MFK - Inventory item master file notification", definition: "", comment_usage_note: "", status: "" },
        "M16" => V2TableRow { value: "M16", display_name: "MFN/MFK - Master File Notification Inventory Item Enhanced", definition: "", comment_usage_note: "", status: "" },
        "M17" => V2TableRow { value: "M17", display_name: "DRG Master File Message", definition: "", comment_usage_note: "", status: "" },
        "M18" => V2TableRow { value: "M18", display_name: "MFN/MFK - Master file notification - Test/Observation (Payer)", definition: "", comment_usage_note: "", status: "" },
        "N01" => V2TableRow { value: "N01", display_name: "NMQ/NM R - Application management query message", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "N02" => V2TableRow { value: "N02", display_name: "NMD/ACK - Application management data message (unsolicited)", definition: "", comment_usage_note: "", status: "" },
        "O01" => V2TableRow { value: "O01", display_name: "ORM - Order message (also RDE, RDS, RG V, RAS)", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "O02" => V2TableRow { value: "O02", display_name: "ORR - Order response (also RRE, RRD, RRG, RRA)", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "O03" => V2TableRow { value: "O03", display_name: "OMD - Diet order", definition: "", comment_usage_note: "", status: "" },
        "O04" => V2TableRow { value: "O04", display_name: "ORD - Diet order acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O05" => V2TableRow { value: "O05", display_name: "OMS - Stock requisition order", definition: "", comment_usage_note: "", status: "" },
        "O06" => V2TableRow { value: "O06", display_name: "ORS - Stock requisition acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O07" => V2TableRow { value: "O07", display_name: "OMN - Non-stock requisition order", definition: "", comment_usage_note: "", status: "" },
        "O08" => V2TableRow { value: "O08", display_name: "ORN - Non-stock requisition acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O09" => V2TableRow { value: "O09", display_name: "OMP - Pharmacy/treatment order", definition: "", comment_usage_note: "", status: "" },
        "O10" => V2TableRow { value: "O10", display_name: "ORP - Pharmacy/treatment order acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O11" => V2TableRow { value: "O11", display_name: "RDE - Pharmacy/treatment encoded order", definition: "", comment_usage_note: "", status: "" },
        "O12" => V2TableRow { value: "O12", display_name: "RRE - Pharmacy/treatment encoded order acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O13" => V2TableRow { value: "O13", display_name: "RDS - Pharmacy/treatment dispense", definition: "", comment_usage_note: "", status: "" },
        "O14" => V2TableRow { value: "O14", display_name: "RRD - Pharmacy/treatment dispense acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O15" => V2TableRow { value: "O15", display_name: "RGV - Pharmacy/treatment give", definition: "", comment_usage_note: "", status: "" },
        "O16" => V2TableRow { value: "O16", display_name: "RRG - Pharmacy/treatment give acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O17" => V2TableRow { value: "O17", display_name: "RAS - Pharmacy/treatment administration", definition: "", comment_usage_note: "", status: "" },
        "O18" => V2TableRow { value: "O18", display_name: "RRA - Pharmacy/treatment administration acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O19" => V2TableRow { value: "O19", display_name: "OMG - General clinical order", definition: "", comment_usage_note: "", status: "" },
        "O20" => V2TableRow { value: "O20", display_name: "ORG/O RL - General clinical order response", definition: "", comment_usage_note: "", status: "" },
        "O21" => V2TableRow { value: "O21", display_name: "OML - Laboratory order", definition: "", comment_usage_note: "", status: "" },
        "O22" => V2TableRow { value: "O22", display_name: "ORL - General laboratory order response message to any OML", definition: "", comment_usage_note: "", status: "" },
        "O23" => V2TableRow { value: "O23", display_name: "OMI - Imaging order", definition: "", comment_usage_note: "", status: "" },
        "O24" => V2TableRow { value: "O24", display_name: "ORI - Imaging order response message to any OMI", definition: "", comment_usage_note: "", status: "" },
        "O25" => V2TableRow { value: "O25", display_name: "RDE - Pharmacy/treatment refill authorization request", definition: "", comment_usage_note: "", status: "" },
        "O26" => V2TableRow { value: "O26", display_name: "RRE - Pharmacy/Treatment Refill Authorization Acknowledgement", definition: "", comment_usage_note: "", status: "" },
        "O27" => V2TableRow { value: "O27", display_name: "OMB - Blood product order", definition: "", comment_usage_note: "", status: "" },
        "O28" => V2TableRow { value: "O28", display_name: "ORB - Blood product order acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O29" => V2TableRow { value: "O29", display_name: "BPS - Blood product dispense status", definition: "", comment_usage_note: "", status: "" },
        "O30" => V2TableRow { value: "O30", display_name: "BRP - Blood product dispense status acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O31" => V2TableRow { value: "O31", display_name: "BTS - Blood product transfusion/disposition", definition: "", comment_usage_note: "", status: "" },
        "O32" => V2TableRow { value: "O32", display_name: "BRT - Blood product transfusion/disposition acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O33" => V2TableRow { value: "O33", display_name: "OML - Laboratory order for multiple orders related to a single specimen", definition: "", comment_usage_note: "", status: "" },
        "O34" => V2TableRow { value: "O34", display_name: "ORL - Laboratory order response message to a multiple order related to single specimen OML", definition: "", comment_usage_note: "", status: "" },
        "O35" => V2TableRow { value: "O35", display_name: "OML - Laboratory order for multiple orders related to a single container of a specimen", definition: "", comment_usage_note: "", status: "" },
        "O36" => V2TableRow { value: "O36", display_name: "ORL - Laboratory order response message to a single container of a specimen OML", definition: "", comment_usage_note: "", status: "" },
        "O37" => V2TableRow { value: "O37", display_name: "OPL - Population/Location-Based Laboratory Order Message", definition: "", comment_usage_note: "", status: "" },
        "O38" => V2TableRow { value: "O38", display_name: "OPR - Population/Location-Based Laboratory Order Acknowledgment Message", definition: "", comment_usage_note: "", status: "" },
        "O39" => V2TableRow { value: "O39", display_name: "Specimen shipment centric laboratory order", definition: "", comment_usage_note: "", status: "" },
        "O40" => V2TableRow { value: "O40", display_name: "Specimen Shipment Centric Laboratory Order Acknowledgment Message", definition: "", comment_usage_note: "", status: "" },
        "O41" => V2TableRow { value: "O41", display_name: "DBC - Create Donor Record Message", definition: "", comment_usage_note: "", status: "" },
        "O42" => V2TableRow { value: "O42", display_name: "DBU - Update Donor Record Message", definition: "", comment_usage_note: "", status: "" },
        "O43" => V2TableRow { value: "O43", display_name: "General Order Message with Document Payload Acknowledgement Message", definition: "", comment_usage_note: "", status: "" },
        "O44" => V2TableRow { value: "O44", display_name: "Donor Registration - Minimal Message", definition: "", comment_usage_note: "", status: "" },
        "O45" => V2TableRow { value: "O45", display_name: "Donor Eligibility Observations Message", definition: "", comment_usage_note: "", status: "" },
        "O46" => V2TableRow { value: "O46", display_name: "Donor Eligiblity Message", definition: "", comment_usage_note: "", status: "" },
        "O47" => V2TableRow { value: "O47", display_name: "Donor Request to Collect Message", definition: "", comment_usage_note: "", status: "" },
        "O48" => V2TableRow { value: "O48", display_name: "Donation Procedure Message", definition: "", comment_usage_note: "", status: "" },
        "O49" => V2TableRow { value: "O49", display_name: "Pharmacy/Treatment Dispense Request Message", definition: "", comment_usage_note: "", status: "" },
        "O50" => V2TableRow { value: "O50", display_name: "Pharmacy/Treatment Encoded Order Acknowledgment", definition: "", comment_usage_note: "", status: "" },
        "O51" => V2TableRow { value: "O51", display_name: "OSU – Order Status Update", definition: "", comment_usage_note: "N", status: "" },
        "O52" => V2TableRow { value: "O52", display_name: "OSU – Order Status Update Acknowledgement", definition: "", comment_usage_note: "N", status: "" },
        "O53" => V2TableRow { value: "O53", display_name: "ORL - General Laboratory Order Acknowledgment Message (Patient Optional)", definition: "", comment_usage_note: "N", status: "" },
        "O54" => V2TableRow { value: "O54", display_name: "ORL - Laboratory Order Acknowledgment Message – Multiple Order Per Specimen (Patient Optional)", definition: "", comment_usage_note: "N", status: "" },
        "O55" => V2TableRow { value: "O55", display_name: "ORL - Laboratory Order Acknowledgment Message – Multiple Order Per Container of Specimen (Patient Optional)", definition: "", comment_usage_note: "N", status: "" },
        "O56" => V2TableRow { value: "O56", display_name: "ORL - Specimen Shipment Centric Laboratory Order Acknowledgment Message (Patient Optional)", definition: "", comment_usage_note: "N", status: "" },
        "O57" => V2TableRow { value: "O57", display_name: "OMQ- General Order Message with Document Payload", definition: "", comment_usage_note: "B", status: "" },
        "O58" => V2TableRow { value: "O58", display_name: "ORX - General Order Message with Document Payload Acknowledgement Message", definition: "", comment_usage_note: "N", status: "" },
        "O59" => V2TableRow { value: "O59", display_name: "OML - Laboratory order for additional work up Fulfillm work up previous communic result o specimen", definition: "ent order for An example on a the IHE LC ly profile ated describes r submitted message in LAB-7 transactio", comment_usage_note: "is N C this the n", status: "" },
        "P01" => V2TableRow { value: "P01", display_name: "BAR/ACK - Add patient accounts", definition: "", comment_usage_note: "", status: "" },
        "P02" => V2TableRow { value: "P02", display_name: "BAR/ACK - Purge patient accounts", definition: "", comment_usage_note: "", status: "" },
        "P03" => V2TableRow { value: "P03", display_name: "DFT/ACK - Post detail financial transaction", definition: "", comment_usage_note: "", status: "" },
        "P04" => V2TableRow { value: "P04", display_name: "QRY/D SP - Generate bill and A/R statements", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "P05" => V2TableRow { value: "P05", display_name: "BAR/ACK - Update account", definition: "", comment_usage_note: "", status: "" },
        "P06" => V2TableRow { value: "P06", display_name: "BAR/ACK - End account", definition: "", comment_usage_note: "", status: "" },
        "P07" => V2TableRow { value: "P07", display_name: "PEX - Unsolicited initial individual product experience report", definition: "", comment_usage_note: "", status: "" },
        "P08" => V2TableRow { value: "P08", display_name: "PEX - Unsolicited update individual product experience report", definition: "", comment_usage_note: "", status: "" },
        "P09" => V2TableRow { value: "P09", display_name: "SUR - Summary product experience report", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "P10" => V2TableRow { value: "P10", display_name: "BAR/ACK -Transmit Ambulatory Payment Classification(APC)", definition: "", comment_usage_note: "", status: "" },
        "P11" => V2TableRow { value: "P11", display_name: "DFT/ACK - Post Detail Finan cial Transactions - New", definition: "", comment_usage_note: "", status: "" },
        "P12" => V2TableRow { value: "P12", display_name: "BAR/ACK - Update Diagnosis/Procedure", definition: "", comment_usage_note: "", status: "" },
        "PC1" => V2TableRow { value: "PC1", display_name: "PPR - PC/ problem add", definition: "", comment_usage_note: "", status: "" },
        "PC2" => V2TableRow { value: "PC2", display_name: "PPR - PC/ problem update", definition: "", comment_usage_note: "", status: "" },
        "PC3" => V2TableRow { value: "PC3", display_name: "PPR - PC/ problem delete", definition: "", comment_usage_note: "", status: "" },
        "PC4" => V2TableRow { value: "PC4", display_name: "QRY - PC/ problem query", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "PC5" => V2TableRow { value: "PC5", display_name: "PRR - PC/ problem response", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "PC6" => V2TableRow { value: "PC6", display_name: "PGL - PC/ goal add", definition: "", comment_usage_note: "", status: "" },
        "PC7" => V2TableRow { value: "PC7", display_name: "PGL - PC/ goal update", definition: "", comment_usage_note: "", status: "" },
        "PC8" => V2TableRow { value: "PC8", display_name: "PGL - PC/ goal delete", definition: "", comment_usage_note: "", status: "" },
        "PC9" => V2TableRow { value: "PC9", display_name: "QRY - PC/ goal query", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "PCA" => V2TableRow { value: "PCA", display_name: "PPV - PC/ goal response", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "PCB" => V2TableRow { value: "PCB", display_name: "PPP - PC/ pathway (problem-oriented) add", definition: "", comment_usage_note: "", status: "" },
        "PCC" => V2TableRow { value: "PCC", display_name: "PPP - PC/ pathway (problem-oriented) update", definition: "", comment_usage_note: "", status: "" },
        "PCD" => V2TableRow { value: "PCD", display_name: "PPP - PC/ pathway (problem-oriented) delete", definition: "", comment_usage_note: "", status: "" },
        "PCE" => V2TableRow { value: "PCE", display_name: "QRY - PC/ pathway (problem-oriented) query", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "PCF" => V2TableRow { value: "PCF", display_name: "PTR - PC/ pathway (problem-oriented) query response", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "PCG" => V2TableRow { value: "PCG", display_name: "PPG - PC/ pathway (goal-oriented) add", definition: "", comment_usage_note: "", status: "" },
        "PCH" => V2TableRow { value: "PCH", display_name: "PPG - PC/ pathway (goal-oriented) update", definition: "", comment_usage_note: "", status: "" },
        "PCJ" => V2TableRow { value: "PCJ", display_name: "PPG - PC/ pathway (goal-oriented) delete", definition: "", comment_usage_note: "", status: "" },
        "PCK" => V2TableRow { value: "PCK", display_name: "QRY - PC/ pathway (goal-oriented) query", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "PCL" => V2TableRow { value: "PCL", display_name: "PPT - PC/ pathway (goal-oriented) query response", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "Q01" => V2TableRow { value: "Q01", display_name: "QRY/D SR - Query sent for immediate response", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "Q02" => V2TableRow { value: "Q02", display_name: "QRY/QCK - Query sent for deferred response", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "Q03" => V2TableRow { value: "Q03", display_name: "DSR/A CK - Deferred response to a query", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "Q05" => V2TableRow { value: "Q05", display_name: "UDM/ACK - Unsolicited display update message", definition: "", comment_usage_note: "", status: "" },
        "Q06" => V2TableRow { value: "Q06", display_name: "OSQ/OSR - Query for order status", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "Q11" => V2TableRow { value: "Q11", display_name: "QBP - Query by parameter requesting an RSP segment pattern response", definition: "", comment_usage_note: "", status: "" },
        "Q13" => V2TableRow { value: "Q13", display_name: "QBP - Query by parameter requesting an RTB - tabular response", definition: "", comment_usage_note: "", status: "" },
        "Q15" => V2TableRow { value: "Q15", display_name: "QBP - Query by parameter requesting an RDY display response", definition: "", comment_usage_note: "", status: "" },
        "Q16" => V2TableRow { value: "Q16", display_name: "QSB - Create subscription", definition: "", comment_usage_note: "", status: "" },
        "Q17" => V2TableRow { value: "Q17", display_name: "QVR - Query for previous events", definition: "", comment_usage_note: "", status: "" },
        "Q21" => V2TableRow { value: "Q21", display_name: "QBP - Get person demographics", definition: "", comment_usage_note: "", status: "" },
        "Q22" => V2TableRow { value: "Q22", display_name: "QBP - Find candidates", definition: "", comment_usage_note: "", status: "" },
        "Q23" => V2TableRow { value: "Q23", display_name: "QBP - Get corresponding identifiers", definition: "", comment_usage_note: "", status: "" },
        "Q24" => V2TableRow { value: "Q24", display_name: "QBP - Allocate identifiers", definition: "", comment_usage_note: "", status: "" },
        "Q25" => V2TableRow { value: "Q25", display_name: "QBP - Personnel Information by Segment Query", definition: "", comment_usage_note: "", status: "" },
        "Q26" => V2TableRow { value: "Q26", display_name: "ROR - Pharmacy/treatment order response", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "Q27" => V2TableRow { value: "Q27", display_name: "RAR - Pharmacy/treatment administration information", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "Q28" => V2TableRow { value: "Q28", display_name: "RDR - Pharmacy/treatment dispense information", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "Q29" => V2TableRow { value: "Q29", display_name: "RER - Pharmacy/treatment encoded order information", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "Q30" => V2TableRow { value: "Q30", display_name: "RGR - Pharmacy/treatment dose information", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "Q31" => V2TableRow { value: "Q31", display_name: "QBP Query Dispense history", definition: "", comment_usage_note: "", status: "" },
        "Q32" => V2TableRow { value: "Q32", display_name: "Find Candidates including Visit Information", definition: "", comment_usage_note: "", status: "" },
        "Q33" => V2TableRow { value: "Q33", display_name: "QBP - Get Donor Record Candidates", definition: "", comment_usage_note: "", status: "" },
        "Q34" => V2TableRow { value: "Q34", display_name: "QBP - Get Donor Record", definition: "", comment_usage_note: "", status: "" },
        "R01" => V2TableRow { value: "R01", display_name: "ORU/ACK - Unsolicited transmission of an observation message", definition: "", comment_usage_note: "", status: "" },
        "R02" => V2TableRow { value: "R02", display_name: "QRY - Query for results of observation", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "R04" => V2TableRow { value: "R04", display_name: "ORF - Response to query; transmission of requested observation", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "R21" => V2TableRow { value: "R21", display_name: "OUL - Unsolicited laboratory observation", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "R22" => V2TableRow { value: "R22", display_name: "OUL - Unsolicited Specimen Oriented Observation Message", definition: "", comment_usage_note: "", status: "" },
        "R23" => V2TableRow { value: "R23", display_name: "OUL - Unsolicited Specimen Container Oriented Observation Message", definition: "", comment_usage_note: "", status: "" },
        "R24" => V2TableRow { value: "R24", display_name: "OUL - Unsolicited Order Oriented Observation Message", definition: "", comment_usage_note: "", status: "" },
        "R25" => V2TableRow { value: "R25", display_name: "OPU - Unsolicited Population/Location-Based Laboratory Observation Message", definition: "", comment_usage_note: "", status: "" },
        "R26" => V2TableRow { value: "R26", display_name: "OSM - Unsolicited Specimen Shipment Manifest Message", definition: "", comment_usage_note: "", status: "" },
        "R30" => V2TableRow { value: "R30", display_name: "ORU - Unsolicited Point-Of-Care Observation Message Without Existing Order - Place An Order", definition: "", comment_usage_note: "", status: "" },
        "R31" => V2TableRow { value: "R31", display_name: "ORU - Unsolicited New Point-Of-Care Observation Message - Search For An Order", definition: "", comment_usage_note: "", status: "" },
        "R32" => V2TableRow { value: "R32", display_name: "ORU - Unsolicited Pre-Ordered Point-Of-Care Observation", definition: "", comment_usage_note: "", status: "" },
        "R33" => V2TableRow { value: "R33", display_name: "ORA - Observation Report Acknowledgement", definition: "", comment_usage_note: "", status: "" },
        "R40" => V2TableRow { value: "R40", display_name: "ORU - Unsolicited Report Alarm", definition: "", comment_usage_note: "", status: "" },
        "R41" => V2TableRow { value: "R41", display_name: "Observation Report Alert Acknowledgement", definition: "", comment_usage_note: "", status: "" },
        "R42" => V2TableRow { value: "R42", display_name: "ORU – Unsolicited Device Event Observation Message", definition: "", comment_usage_note: "N", status: "" },
        "R43" => V2TableRow { value: "R43", display_name: "ORU – Unsolicited Patient-De vice Association Observation Message", definition: "", comment_usage_note: "N", status: "" },
        "ROR" => V2TableRow { value: "ROR", display_name: "ROR - Pharmacy prescription order query response", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "S01" => V2TableRow { value: "S01", display_name: "SRM/ SRR - Request new appointment booking", definition: "", comment_usage_note: "", status: "" },
        "S02" => V2TableRow { value: "S02", display_name: "SRM/ SRR - Request appointment rescheduling", definition: "", comment_usage_note: "", status: "" },
        "S03" => V2TableRow { value: "S03", display_name: "SRM/ SRR - Request appointment modification", definition: "", comment_usage_note: "", status: "" },
        "S04" => V2TableRow { value: "S04", display_name: "SRM/ SRR - Request appointment cancellation", definition: "", comment_usage_note: "", status: "" },
        "S05" => V2TableRow { value: "S05", display_name: "SRM/ SRR - Request appointment discontinuation", definition: "", comment_usage_note: "", status: "" },
        "S06" => V2TableRow { value: "S06", display_name: "SRM/ SRR - Request appointment deletion", definition: "", comment_usage_note: "", status: "" },
        "S07" => V2TableRow { value: "S07", display_name: "SRM/ SRR - Request addition of service/resource on appointment", definition: "", comment_usage_note: "", status: "" },
        "S08" => V2TableRow { value: "S08", display_name: "SRM/ SRR - Request modification of service/resource on appointment", definition: "", comment_usage_note: "", status: "" },
        "S09" => V2TableRow { value: "S09", display_name: "SRM/ SRR - Request cancellation of service/resource on appointment", definition: "", comment_usage_note: "", status: "" },
        "S10" => V2TableRow { value: "S10", display_name: "SRM/ SRR - Request discontinuation of service/resource on appointment", definition: "", comment_usage_note: "", status: "" },
        "S11" => V2TableRow { value: "S11", display_name: "SRM/ SRR - Request deletion of service/resource on appointment", definition: "", comment_usage_note: "", status: "" },
        "S12" => V2TableRow { value: "S12", display_name: "SIU/ACK - Notification of new appointment booking", definition: "", comment_usage_note: "", status: "" },
        "S13" => V2TableRow { value: "S13", display_name: "SIU/ACK - Notification of appointment rescheduling", definition: "", comment_usage_note: "", status: "" },
        "S14" => V2TableRow { value: "S14", display_name: "SIU/ACK - Notification of appointment modification", definition: "", comment_usage_note: "", status: "" },
        "S15" => V2TableRow { value: "S15", display_name: "SIU/ACK - Notification of appointment cancellation", definition: "", comment_usage_note: "", status: "" },
        "S16" => V2TableRow { value: "S16", display_name: "SIU/ACK - Notification of appointment discontinuation", definition: "", comment_usage_note: "", status: "" },
        "S17" => V2TableRow { value: "S17", display_name: "SIU/ACK - Notification of appointment deletion", definition: "", comment_usage_note: "", status: "" },
        "S18" => V2TableRow { value: "S18", display_name: "SIU/ACK - Notification of addition of service/resource on appointment", definition: "", comment_usage_note: "", status: "" },
        "S19" => V2TableRow { value: "S19", display_name: "SIU/ACK - Notification of modification of service/resource on appointment", definition: "", comment_usage_note: "", status: "" },
        "S20" => V2TableRow { value: "S20", display_name: "SIU/ACK - Notification of cancellation of service/resource on appointment", definition: "", comment_usage_note: "", status: "" },
        "S21" => V2TableRow { value: "S21", display_name: "SIU/ACK - Notification of discontinuation of service/resource on appointment", definition: "", comment_usage_note: "", status: "" },
        "S22" => V2TableRow { value: "S22", display_name: "SIU/ACK - Notification of deletion of service/resource on appointment", definition: "", comment_usage_note: "", status: "" },
        "S23" => V2TableRow { value: "S23", display_name: "SIU/ACK - Notification of blocked schedule time slot(s)", definition: "", comment_usage_note: "", status: "" },
        "S24" => V2TableRow { value: "S24", display_name: "SIU/ACK - Notification of opened (\"unblocked\") schedule time slot(s)", definition: "", comment_usage_note: "", status: "" },
        "S25" => V2TableRow { value: "S25", display_name: "SQM/SQ R - Schedule query message and response", definition: "", comment_usage_note: "Deprecated", status: "D" },
        "S26" => V2TableRow { value: "S26", display_name: "SIU/ACK Notification that patient did not show up for schedule appointment", definition: "", comment_usage_note: "", status: "" },
        "S27" => V2TableRow { value: "S27", display_name: "SIU/ACK - Broadcast Notification of Scheduled Appointments", definition: "", comment_usage_note: "", status: "" },
        "S28" => V2TableRow { value: "S28", display_name: "SLR/SL S - Request new sterilization lot", definition: "", comment_usage_note: "", status: "" },
        "S29" => V2TableRow { value: "S29", display_name: "SLR/SL S - Request Sterilization lot deletion", definition: "", comment_usage_note: "", status: "" },
        "S30" => V2TableRow { value: "S30", display_name: "STI/STS - Request item", definition: "", comment_usage_note: "", status: "" },
        "S31" => V2TableRow { value: "S31", display_name: "SDR/ SDS - Request anti-microbial device data", definition: "", comment_usage_note: "", status: "" },
        "S32" => V2TableRow { value: "S32", display_name: "SMD/SM S - Request anti-microbial device cycle data", definition: "", comment_usage_note: "", status: "" },
        "S33" => V2TableRow { value: "S33", display_name: "STC/A CK - Notification of sterilization configuration", definition: "", comment_usage_note: "", status: "" },
        "S34" => V2TableRow { value: "S34", display_name: "SLN/ACK - Notification of sterilization lot", definition: "", comment_usage_note: "", status: "" },
        "S35" => V2TableRow { value: "S35", display_name: "SLN/ACK - Notification of sterilization lot deletion", definition: "", comment_usage_note: "", status: "" },
        "S36" => V2TableRow { value: "S36", display_name: "SDN/ACK - Notification of anti-microbial device data", definition: "", comment_usage_note: "", status: "" },
        "S37" => V2TableRow { value: "S37", display_name: "SCN/A CK - Notification of anti-microbial device cycle data", definition: "", comment_usage_note: "", status: "" },
        "S38" => V2TableRow { value: "S38", display_name: "Containers Prepared for Specimen Collection", definition: "Describes the event before specimen collection, when containers have been prepared", comment_usage_note: "", status: "N" },
        "S39" => V2TableRow { value: "S39", display_name: "Specimen Collection Successful", definition: "Describes the event when specimen collection was successful", comment_usage_note: "", status: "N" },
        "S40" => V2TableRow { value: "S40", display_name: "Specimen Collection Unsuccessful", definition: "Describes the event when specimen collection was not successful and provides a means to document the reason", comment_usage_note: "", status: "N" },
        "S41" => V2TableRow { value: "S41", display_name: "Specimen Departed", definition: "Describes the event when a specimen has been moved from a location", comment_usage_note: "", status: "N" },
        "S42" => V2TableRow { value: "S42", display_name: "Specimen Arrived", definition: "Describes the event when a specimen has been moved to a location", comment_usage_note: "", status: "N" },
        "S43" => V2TableRow { value: "S43", display_name: "Specimen Accepted", definition: "Describes the event when a specimen has been accepted on the receiver side of a specimen movement", comment_usage_note: "", status: "N" },
        "S44" => V2TableRow { value: "S44", display_name: "Specimen Rejected", definition: "Describes the event when a specimen has been rejected by the receiver side of a specimen movement", comment_usage_note: "", status: "N" },
        "S45" => V2TableRow { value: "S45", display_name: "Specimen Re-identified", definition: "Describes the event when a specimen has been assigned an identifier", comment_usage_note: "", status: "N" },
        "S46" => V2TableRow { value: "S46", display_name: "Specimen De-identified", definition: "Describes the event when a specimen identifier has been removed to anonymize it", comment_usage_note: "", status: "N" },
        "S47" => V2TableRow { value: "S47", display_name: "Specimen Sent to Archive", definition: "Describes the event when a specimen has been moved into storage", comment_usage_note: "", status: "N" },
        "S48" => V2TableRow { value: "S48", display_name: "Specimen Retrieved from Archive", definition: "Describes the event when a specimen has been moved out of storage", comment_usage_note: "", status: "N" },
        "S49" => V2TableRow { value: "S49", display_name: "Specimen Disposed of", definition: "Describes the event when a specimen has been permanently disposed of", comment_usage_note: "", status: "N" },
        "S50" => V2TableRow { value: "S50", display_name: "Specimen Procedure Step Successful , with Derived Specimen(s)", definition: "Describes the event when one or more specimen(s) has(ve) been created from one or more specimen(s)", comment_usage_note: "", status: "N" },
        "S51" => V2TableRow { value: "S51", display_name: "Specimen Procedure Step Successful, no Derived Specimen(s)", definition: "Describes the event when a specimen has been successfully processed without producing any child specimen(s)", comment_usage_note: "", status: "N" },
        "S52" => V2TableRow { value: "S52", display_name: "Specimen Procedure Step Unsuccessful", definition: "Describes the event when a specimen could not be successfully processed and provides a means to document the reason", comment_usage_note: "", status: "N" },
        "T01" => V2TableRow { value: "T01", display_name: "MDM/ACK - Original document notification", definition: "", comment_usage_note: "", status: "" },
        "T02" => V2TableRow { value: "T02", display_name: "MDM/ACK - Original document notification and content", definition: "", comment_usage_note: "", status: "" },
        "T03" => V2TableRow { value: "T03", display_name: "MDM/ACK - Document status change notification", definition: "", comment_usage_note: "", status: "" },
        "T04" => V2TableRow { value: "T04", display_name: "MDM/ACK - Document status change notification and content", definition: "", comment_usage_note: "", status: "" },
        "T05" => V2TableRow { value: "T05", display_name: "MDM/ACK - Document addendum notification", definition: "", comment_usage_note: "", status: "" },
        "T06" => V2TableRow { value: "T06", display_name: "MDM/ACK - Document addendum notification and content", definition: "", comment_usage_note: "", status: "" },
        "T07" => V2TableRow { value: "T07", display_name: "MDM/ACK - Document edit notification", definition: "", comment_usage_note: "", status: "" },
        "T08" => V2TableRow { value: "T08", display_name: "MDM/ACK - Document edit notification and content", definition: "", comment_usage_note: "", status: "" },
        "T09" => V2TableRow { value: "T09", display_name: "MDM/ACK - Document replacement notification", definition: "", comment_usage_note: "", status: "" },
        "T10" => V2TableRow { value: "T10", display_name: "MDM/ACK - Document replacement notification and content", definition: "", comment_usage_note: "", status: "" },
        "T11" => V2TableRow { value: "T11", display_name: "MDM/ACK - Document cancel notification", definition: "", comment_usage_note: "", status: "" },
        "T12" => V2TableRow { value: "T12", display_name: "QRY/DOC - Document query", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "U01" => V2TableRow { value: "U01", display_name: "ESU/ACK - Automated equipment status update", definition: "", comment_usage_note: "", status: "" },
        "U02" => V2TableRow { value: "U02", display_name: "ESR/A CK - Automated equipment status request", definition: "", comment_usage_note: "", status: "" },
        "U03" => V2TableRow { value: "U03", display_name: "SSU/A CK - Specimen status update", definition: "", comment_usage_note: "", status: "" },
        "U04" => V2TableRow { value: "U04", display_name: "SSR/ ACK - specimen status r equest", definition: "", comment_usage_note: "", status: "" },
        "U05" => V2TableRow { value: "U05", display_name: "INU/ACK - Automated equipment inventory update", definition: "", comment_usage_note: "", status: "" },
        "U06" => V2TableRow { value: "U06", display_name: "INR/ACK - Automated equipment inventory request", definition: "", comment_usage_note: "", status: "" },
        "U07" => V2TableRow { value: "U07", display_name: "EAC/A CK - Automated equipment command", definition: "", comment_usage_note: "", status: "" },
        "U08" => V2TableRow { value: "U08", display_name: "EAR/A CK - Automated equipment response", definition: "", comment_usage_note: "", status: "" },
        "U09" => V2TableRow { value: "U09", display_name: "EAN/ACK - Automated equipment notification", definition: "", comment_usage_note: "", status: "" },
        "U10" => V2TableRow { value: "U10", display_name: "TCU/A CK - Automated equipment test code settings update", definition: "", comment_usage_note: "", status: "" },
        "U11" => V2TableRow { value: "U11", display_name: "TCR/A CK - Automated equipment test code settings request", definition: "", comment_usage_note: "", status: "" },
        "U12" => V2TableRow { value: "U12", display_name: "LSU/ACK - Automated equipment log/service update", definition: "", comment_usage_note: "", status: "" },
        "U13" => V2TableRow { value: "U13", display_name: "LSR/A CK - Automated equipment log/service request", definition: "", comment_usage_note: "", status: "" },
        "U14" => V2TableRow { value: "U14", display_name: "INR/ACK – Automated Equipment Inventory Request", definition: "", comment_usage_note: "", status: "" },
        "V01" => V2TableRow { value: "V01", display_name: "VXQ - Query for vaccination record", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "V02" => V2TableRow { value: "V02", display_name: "VXX - Response to vaccination query returning multiple PID matches", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "V03" => V2TableRow { value: "V03", display_name: "VXR - Vaccination record response", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "V04" => V2TableRow { value: "V04", display_name: "VXU - Unsolicited vaccination record update", definition: "", comment_usage_note: "", status: "" },
        "Varies" => V2TableRow { value: "Varies", display_name: "MFQ/MFR - Master files query (use event same as asking for e.g., M05 - location)", definition: "", comment_usage_note: "D", status: "" },
        "W01" => V2TableRow { value: "W01", display_name: "ORU - Waveform result, unsolicited transmission of requested information", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "W02" => V2TableRow { value: "W02", display_name: "QRF - Waveform result, response to query", definition: "Deprecated", comment_usage_note: "D", status: "" },
    },
};

pub static TABLE_0004: V2Table = V2Table {
    number: 4,
    metadata: &super::metadata::TABLE_0004_METADATA,
    rows: phf_map! {
        "E" => V2TableRow { value: "E", display_name: "Emergency", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Inpatient", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Outpatient", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Preadmit", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Recurring patient", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Obstetrics", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Commercial Account", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Not Applicable", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0005: V2Table = V2Table {
    number: 5,
    metadata: &super::metadata::TABLE_0005_METADATA,
    rows: phf_map! {
        "1002-5" => V2TableRow { value: "1002-5", display_name: "American Indian or Alaska Native", definition: "", comment_usage_note: "", status: "" },
        "2028-9" => V2TableRow { value: "2028-9", display_name: "Asian", definition: "", comment_usage_note: "", status: "" },
        "2054-5" => V2TableRow { value: "2054-5", display_name: "Black or African American", definition: "", comment_usage_note: "", status: "" },
        "2076-8" => V2TableRow { value: "2076-8", display_name: "Native Hawaiian or Other Pacific Islander", definition: "", comment_usage_note: "", status: "" },
        "2106-3" => V2TableRow { value: "2106-3", display_name: "White", definition: "", comment_usage_note: "", status: "" },
        "2131-1" => V2TableRow { value: "2131-1", display_name: "Other Race", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0006: V2Table = V2Table {
    number: 6,
    metadata: &super::metadata::TABLE_0006_METADATA,
    rows: phf_map! {
        "AGN" => V2TableRow { value: "AGN", display_name: "Agnostic", definition: "", comment_usage_note: "", status: "" },
        "ATH" => V2TableRow { value: "ATH", display_name: "Atheist", definition: "", comment_usage_note: "", status: "" },
        "BAH" => V2TableRow { value: "BAH", display_name: "Baha'i", definition: "", comment_usage_note: "", status: "" },
        "BRE" => V2TableRow { value: "BRE", display_name: "Brethren", definition: "", comment_usage_note: "", status: "" },
        "BUD" => V2TableRow { value: "BUD", display_name: "Buddhist", definition: "", comment_usage_note: "", status: "" },
        "BMA" => V2TableRow { value: "BMA", display_name: "Buddhist: Mahayana", definition: "", comment_usage_note: "", status: "" },
        "BTH" => V2TableRow { value: "BTH", display_name: "Buddhist: Theravada", definition: "", comment_usage_note: "", status: "" },
        "BTA" => V2TableRow { value: "BTA", display_name: "Buddhist: Tantrayana", definition: "", comment_usage_note: "", status: "" },
        "BOT" => V2TableRow { value: "BOT", display_name: "Buddhist: Other", definition: "", comment_usage_note: "", status: "" },
        "CF" => V2TableRow { value: "CF", display_name: "R Chinese Folk Religionist", definition: "", comment_usage_note: "", status: "" },
        "CHR" => V2TableRow { value: "CHR", display_name: "Christian", definition: "", comment_usage_note: "", status: "" },
        "ABC" => V2TableRow { value: "ABC", display_name: "Christian: American Baptist Church", definition: "", comment_usage_note: "", status: "" },
        "AMT" => V2TableRow { value: "AMT", display_name: "Christian: African Methodist Episcopal", definition: "", comment_usage_note: "", status: "" },
        "AME" => V2TableRow { value: "AME", display_name: "Christian: African Methodist Episcopal Zion", definition: "", comment_usage_note: "", status: "" },
        "ANG" => V2TableRow { value: "ANG", display_name: "Christian: Anglican", definition: "", comment_usage_note: "", status: "" },
        "AOG" => V2TableRow { value: "AOG", display_name: "Christian: Assembly of God", definition: "", comment_usage_note: "", status: "" },
        "BAP" => V2TableRow { value: "BAP", display_name: "Christian: Baptist", definition: "", comment_usage_note: "", status: "" },
        "CRR" => V2TableRow { value: "CRR", display_name: "Christian: Christian Reformed", definition: "", comment_usage_note: "", status: "" },
        "CHS" => V2TableRow { value: "CHS", display_name: "Christian: Christian Science", definition: "", comment_usage_note: "", status: "" },
        "CMA" => V2TableRow { value: "CMA", display_name: "Christian: Christian Missionary Alliance", definition: "", comment_usage_note: "", status: "" },
        "COC" => V2TableRow { value: "COC", display_name: "Christian: Church of Christ", definition: "", comment_usage_note: "", status: "" },
        "COG" => V2TableRow { value: "COG", display_name: "Christian: Church of God", definition: "", comment_usage_note: "", status: "" },
        "COI" => V2TableRow { value: "COI", display_name: "Christian: Church of God in Christ", definition: "", comment_usage_note: "", status: "" },
        "COM" => V2TableRow { value: "COM", display_name: "Christian: Community", definition: "", comment_usage_note: "", status: "" },
        "COL" => V2TableRow { value: "COL", display_name: "Christian: Congregational", definition: "", comment_usage_note: "", status: "" },
        "EOT" => V2TableRow { value: "EOT", display_name: "Christian: Eastern Orthodox", definition: "", comment_usage_note: "", status: "" },
        "EVC" => V2TableRow { value: "EVC", display_name: "Christian: Evangelical Church", definition: "", comment_usage_note: "", status: "" },
        "EPI" => V2TableRow { value: "EPI", display_name: "Christian: Episcopalian", definition: "", comment_usage_note: "", status: "" },
        "FWB" => V2TableRow { value: "FWB", display_name: "Christian: Free Will Baptist", definition: "", comment_usage_note: "", status: "" },
        "FRQ" => V2TableRow { value: "FRQ", display_name: "Christian: Friends", definition: "", comment_usage_note: "", status: "" },
        "FUL" => V2TableRow { value: "FUL", display_name: "Christian: Full Gospel", definition: "", comment_usage_note: "", status: "" },
        "GRE" => V2TableRow { value: "GRE", display_name: "Christian: Greek Orthodox", definition: "", comment_usage_note: "", status: "" },
        "JWN" => V2TableRow { value: "JWN", display_name: "Christian: Jehovah's Witness", definition: "", comment_usage_note: "", status: "" },
        "MOM" => V2TableRow { value: "MOM", display_name: "Christian: Latter-day Saints", definition: "", comment_usage_note: "", status: "" },
        "LUT" => V2TableRow { value: "LUT", display_name: "Christian: Lutheran", definition: "", comment_usage_note: "", status: "" },
        "LMS" => V2TableRow { value: "LMS", display_name: "Christian: Lutheran Missouri Synod", definition: "", comment_usage_note: "", status: "" },
        "MEN" => V2TableRow { value: "MEN", display_name: "Christian: Mennonite", definition: "", comment_usage_note: "", status: "" },
        "MET" => V2TableRow { value: "MET", display_name: "Christian: Methodist", definition: "", comment_usage_note: "", status: "" },
        "NAZ" => V2TableRow { value: "NAZ", display_name: "Christian: Church of the Nazarene", definition: "", comment_usage_note: "", status: "" },
        "ORT" => V2TableRow { value: "ORT", display_name: "Christian: Orthodox", definition: "", comment_usage_note: "", status: "" },
        "PEN" => V2TableRow { value: "PEN", display_name: "Christian: Pentecostal", definition: "", comment_usage_note: "", status: "" },
        "COP" => V2TableRow { value: "COP", display_name: "Christian: Other Pentecostal", definition: "", comment_usage_note: "", status: "" },
        "PRE" => V2TableRow { value: "PRE", display_name: "Christian: Presbyterian", definition: "", comment_usage_note: "", status: "" },
        "PRO" => V2TableRow { value: "PRO", display_name: "Christian: Protestant", definition: "", comment_usage_note: "", status: "" },
        "PRC" => V2TableRow { value: "PRC", display_name: "Christian: Other Protestant", definition: "", comment_usage_note: "", status: "" },
        "REC" => V2TableRow { value: "REC", display_name: "Christian: Reformed Church", definition: "", comment_usage_note: "", status: "" },
        "REO" => V2TableRow { value: "REO", display_name: "Christian: Reorganized Church of Jesus Christ-LDS", definition: "", comment_usage_note: "", status: "" },
        "CAT" => V2TableRow { value: "CAT", display_name: "Christian: Roman Catholic", definition: "", comment_usage_note: "", status: "" },
        "SAA" => V2TableRow { value: "SAA", display_name: "Christian: Salvation Army", definition: "", comment_usage_note: "", status: "" },
        "SEV" => V2TableRow { value: "SEV", display_name: "Christian: Seventh Day Adventist", definition: "", comment_usage_note: "", status: "" },
        "SOU" => V2TableRow { value: "SOU", display_name: "Christian: Southern Baptist", definition: "", comment_usage_note: "", status: "" },
        "UCC" => V2TableRow { value: "UCC", display_name: "Christian: United Church of Christ", definition: "", comment_usage_note: "", status: "" },
        "UMD" => V2TableRow { value: "UMD", display_name: "Christian: United Methodist", definition: "", comment_usage_note: "", status: "" },
        "UNI" => V2TableRow { value: "UNI", display_name: "Christian: Unitarian", definition: "", comment_usage_note: "", status: "" },
        "UNU" => V2TableRow { value: "UNU", display_name: "Christian: Unitarian Universalist", definition: "", comment_usage_note: "", status: "" },
        "WES" => V2TableRow { value: "WES", display_name: "Christian: We sleyan", definition: "", comment_usage_note: "", status: "" },
        "WMC" => V2TableRow { value: "WMC", display_name: "Christian: We sleyan Methodist", definition: "", comment_usage_note: "", status: "" },
        "COT" => V2TableRow { value: "COT", display_name: "Christian: Other", definition: "", comment_usage_note: "", status: "" },
        "CNF" => V2TableRow { value: "CNF", display_name: "Confucian", definition: "", comment_usage_note: "", status: "" },
        "DOC" => V2TableRow { value: "DOC", display_name: "Disciples of Christ", definition: "", comment_usage_note: "", status: "" },
        "ERL" => V2TableRow { value: "ERL", display_name: "Ethnic Religionist", definition: "", comment_usage_note: "", status: "" },
        "HIN" => V2TableRow { value: "HIN", display_name: "Hindu", definition: "", comment_usage_note: "", status: "" },
        "HSH" => V2TableRow { value: "HSH", display_name: "Hindu: Shaivites", definition: "", comment_usage_note: "", status: "" },
        "HVA" => V2TableRow { value: "HVA", display_name: "Hindu: Vaishnavites", definition: "", comment_usage_note: "", status: "" },
        "HOT" => V2TableRow { value: "HOT", display_name: "Hindu: Other", definition: "", comment_usage_note: "", status: "" },
        "JAI" => V2TableRow { value: "JAI", display_name: "Jain", definition: "", comment_usage_note: "", status: "" },
        "JEW" => V2TableRow { value: "JEW", display_name: "Jewish", definition: "", comment_usage_note: "", status: "" },
        "JCO" => V2TableRow { value: "JCO", display_name: "Jewish: Conservative", definition: "", comment_usage_note: "", status: "" },
        "JOR" => V2TableRow { value: "JOR", display_name: "Jewish: Orthodox", definition: "", comment_usage_note: "", status: "" },
        "JRC" => V2TableRow { value: "JRC", display_name: "Jewish: Reconstructionist", definition: "", comment_usage_note: "", status: "" },
        "JRF" => V2TableRow { value: "JRF", display_name: "Jewish: Reform", definition: "", comment_usage_note: "", status: "" },
        "JRN" => V2TableRow { value: "JRN", display_name: "Jewish: Renewal", definition: "", comment_usage_note: "", status: "" },
        "JOT" => V2TableRow { value: "JOT", display_name: "Jewish: Other", definition: "", comment_usage_note: "", status: "" },
        "MOS" => V2TableRow { value: "MOS", display_name: "Muslim", definition: "", comment_usage_note: "", status: "" },
        "MSH" => V2TableRow { value: "MSH", display_name: "Muslim: Shiite", definition: "", comment_usage_note: "", status: "" },
        "MSU" => V2TableRow { value: "MSU", display_name: "Muslim: Sunni", definition: "", comment_usage_note: "", status: "" },
        "MOT" => V2TableRow { value: "MOT", display_name: "Muslim: Other", definition: "", comment_usage_note: "", status: "" },
        "NAM" => V2TableRow { value: "NAM", display_name: "Native American", definition: "", comment_usage_note: "", status: "" },
        "NRL" => V2TableRow { value: "NRL", display_name: "New Religionist", definition: "", comment_usage_note: "", status: "" },
        "NOE" => V2TableRow { value: "NOE", display_name: "Nonreligious", definition: "", comment_usage_note: "", status: "" },
        "SHN" => V2TableRow { value: "SHN", display_name: "Shintoist", definition: "", comment_usage_note: "", status: "" },
        "SIK" => V2TableRow { value: "SIK", display_name: "Sikh", definition: "", comment_usage_note: "", status: "" },
        "SPI" => V2TableRow { value: "SPI", display_name: "Spiritist", definition: "", comment_usage_note: "", status: "" },
        "OTH" => V2TableRow { value: "OTH", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "VAR" => V2TableRow { value: "VAR", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0007: V2Table = V2Table {
    number: 7,
    metadata: &super::metadata::TABLE_0007_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Accident", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Emergency", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Labor and Delivery", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Routine", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Newborn (Birth in healthcare facility)", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Urgent", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Elective", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0008: V2Table = V2Table {
    number: 8,
    metadata: &super::metadata::TABLE_0008_METADATA,
    rows: phf_map! {
        "AA" => V2TableRow { value: "AA", display_name: "Original mode: Application Accept - Enhanced mode: Application acknowledgment: Accept", definition: "", comment_usage_note: "", status: "" },
        "AE" => V2TableRow { value: "AE", display_name: "Original mode: Application Error - Enhanced mode: Application acknowledgment: Error", definition: "", comment_usage_note: "", status: "" },
        "AR" => V2TableRow { value: "AR", display_name: "Original mode: Application Reject - Enhanced mode: Application acknowledgment: Reject", definition: "", comment_usage_note: "", status: "" },
        "CA" => V2TableRow { value: "CA", display_name: "Enhanced mode: Accept acknowledgment: Commit Accept", definition: "", comment_usage_note: "", status: "" },
        "CE" => V2TableRow { value: "CE", display_name: "Enhanced mode: Accept acknowledgment: Commit Error", definition: "", comment_usage_note: "", status: "" },
        "CR" => V2TableRow { value: "CR", display_name: "Enhanced mode: Accept acknowledgment: Commit Reject", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0009: V2Table = V2Table {
    number: 9,
    metadata: &super::metadata::TABLE_0009_METADATA,
    rows: phf_map! {
        "A0" => V2TableRow { value: "A0", display_name: "No functional limitations", definition: "", comment_usage_note: "", status: "" },
        "A1" => V2TableRow { value: "A1", display_name: "Ambulates with assistive device", definition: "", comment_usage_note: "", status: "" },
        "A2" => V2TableRow { value: "A2", display_name: "Wheelchair/stretcher bound", definition: "", comment_usage_note: "", status: "" },
        "A3" => V2TableRow { value: "A3", display_name: "Comatose; non-responsive", definition: "", comment_usage_note: "", status: "" },
        "A4" => V2TableRow { value: "A4", display_name: "Disoriented", definition: "", comment_usage_note: "", status: "" },
        "A5" => V2TableRow { value: "A5", display_name: "Vision impaired", definition: "", comment_usage_note: "", status: "" },
        "A6" => V2TableRow { value: "A6", display_name: "Hearing impaired", definition: "", comment_usage_note: "", status: "" },
        "A7" => V2TableRow { value: "A7", display_name: "Speech impaired", definition: "", comment_usage_note: "", status: "" },
        "A8" => V2TableRow { value: "A8", display_name: "Non-English speaking", definition: "", comment_usage_note: "", status: "" },
        "A9" => V2TableRow { value: "A9", display_name: "Functional level unknown", definition: "", comment_usage_note: "", status: "" },
        "B1" => V2TableRow { value: "B1", display_name: "Oxygen therapy", definition: "", comment_usage_note: "", status: "" },
        "B2" => V2TableRow { value: "B2", display_name: "Special equipment (tubes, IVs, catheters)", definition: "", comment_usage_note: "", status: "" },
        "B3" => V2TableRow { value: "B3", display_name: "Amputee", definition: "", comment_usage_note: "", status: "" },
        "B4" => V2TableRow { value: "B4", display_name: "Mastectomy", definition: "", comment_usage_note: "", status: "" },
        "B5" => V2TableRow { value: "B5", display_name: "Paraplegic", definition: "", comment_usage_note: "", status: "" },
        "B6" => V2TableRow { value: "B6", display_name: "Pregnant", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0017: V2Table = V2Table {
    number: 17,
    metadata: &super::metadata::TABLE_0017_METADATA,
    rows: phf_map! {
        "CG" => V2TableRow { value: "CG", display_name: "Charge", definition: "", comment_usage_note: "", status: "" },
        "CD" => V2TableRow { value: "CD", display_name: "Credit", definition: "", comment_usage_note: "", status: "" },
        "PY" => V2TableRow { value: "PY", display_name: "Payment", definition: "", comment_usage_note: "", status: "" },
        "AJ" => V2TableRow { value: "AJ", display_name: "Adjustment", definition: "", comment_usage_note: "", status: "" },
        "CO" => V2TableRow { value: "CO", display_name: "Co-payment", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0027: V2Table = V2Table {
    number: 27,
    metadata: &super::metadata::TABLE_0027_METADATA,
    rows: phf_map! {
        "S" => V2TableRow { value: "S", display_name: "Stat (do immediately)", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "As soon as possible (a priority lower than stat)", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Routine", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Preoperative (to be done prior to surgery)", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Timing critical (do as near as possible to requested time)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0038: V2Table = V2Table {
    number: 38,
    metadata: &super::metadata::TABLE_0038_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Some, but not all, results available", definition: "", comment_usage_note: "", status: "" },
        "CA" => V2TableRow { value: "CA", display_name: "Order was canceled", definition: "", comment_usage_note: "", status: "" },
        "CM" => V2TableRow { value: "CM", display_name: "Order is completed", definition: "", comment_usage_note: "", status: "" },
        "DC" => V2TableRow { value: "DC", display_name: "Order was discontinued", definition: "", comment_usage_note: "", status: "" },
        "ER" => V2TableRow { value: "ER", display_name: "Error, order not found", definition: "", comment_usage_note: "", status: "" },
        "HD" => V2TableRow { value: "HD", display_name: "Order is on hold", definition: "", comment_usage_note: "", status: "" },
        "IP" => V2TableRow { value: "IP", display_name: "In process, unspecified", definition: "", comment_usage_note: "", status: "" },
        "RP" => V2TableRow { value: "RP", display_name: "Order has been replaced", definition: "", comment_usage_note: "", status: "" },
        "SC" => V2TableRow { value: "SC", display_name: "In process, scheduled", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0052: V2Table = V2Table {
    number: 52,
    metadata: &super::metadata::TABLE_0052_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Admitting", definition: "", comment_usage_note: "", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Working", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Final", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0061: V2Table = V2Table {
    number: 61,
    metadata: &super::metadata::TABLE_0061_METADATA,
    rows: phf_map! {
        "BCV" => V2TableRow { value: "BCV", display_name: "Bank Card Validation Number", definition: "", comment_usage_note: "A non-embossed number included on bank cards and used to validate authenticity of the card and the person presenting the card", status: "" },
        "NPI" => V2TableRow { value: "NPI", display_name: "Check digit algorithm in the US National Provider Identifier", definition: "", comment_usage_note: "", status: "" },
        "ISO" => V2TableRow { value: "ISO", display_name: "ISO 7064: 1983", definition: "", comment_usage_note: "", status: "" },
        "M10" => V2TableRow { value: "M10", display_name: "Mod 10 algorithm", definition: "", comment_usage_note: "", status: "" },
        "M11" => V2TableRow { value: "M11", display_name: "Mod 11 algorithm", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0062: V2Table = V2Table {
    number: 62,
    metadata: &super::metadata::TABLE_0062_METADATA,
    rows: phf_map! {
        "01" => V2TableRow { value: "01", display_name: "Patient request", definition: "", comment_usage_note: "", status: "" },
        "02" => V2TableRow { value: "02", display_name: "Physician/health practitioner order", definition: "", comment_usage_note: "", status: "" },
        "03" => V2TableRow { value: "03", display_name: "Census management", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0063: V2Table = V2Table {
    number: 63,
    metadata: &super::metadata::TABLE_0063_METADATA,
    rows: phf_map! {
        "SEL" => V2TableRow { value: "SEL", display_name: "Self", definition: "", comment_usage_note: "", status: "" },
        "SPO" => V2TableRow { value: "SPO", display_name: "Spouse", definition: "", comment_usage_note: "", status: "" },
        "DOM" => V2TableRow { value: "DOM", display_name: "Life partner", definition: "", comment_usage_note: "", status: "" },
        "CHD" => V2TableRow { value: "CHD", display_name: "Child", definition: "", comment_usage_note: "", status: "" },
        "GCH" => V2TableRow { value: "GCH", display_name: "Grandchild", definition: "", comment_usage_note: "", status: "" },
        "NCH" => V2TableRow { value: "NCH", display_name: "Natural child", definition: "", comment_usage_note: "", status: "" },
        "SCH" => V2TableRow { value: "SCH", display_name: "Stepchild", definition: "", comment_usage_note: "", status: "" },
        "FCH" => V2TableRow { value: "FCH", display_name: "Foster child", definition: "", comment_usage_note: "", status: "" },
        "DEP" => V2TableRow { value: "DEP", display_name: "Handicapped dependent", definition: "", comment_usage_note: "", status: "" },
        "WRD" => V2TableRow { value: "WRD", display_name: "Ward of court", definition: "", comment_usage_note: "", status: "" },
        "PAR" => V2TableRow { value: "PAR", display_name: "Parent", definition: "", comment_usage_note: "", status: "" },
        "MTH" => V2TableRow { value: "MTH", display_name: "Mother", definition: "", comment_usage_note: "", status: "" },
        "FTH" => V2TableRow { value: "FTH", display_name: "Father", definition: "", comment_usage_note: "", status: "" },
        "CGV" => V2TableRow { value: "CGV", display_name: "Care giver", definition: "", comment_usage_note: "", status: "" },
        "GRD" => V2TableRow { value: "GRD", display_name: "Guardian", definition: "", comment_usage_note: "", status: "" },
        "GRP" => V2TableRow { value: "GRP", display_name: "Grandparent", definition: "", comment_usage_note: "", status: "" },
        "EXF" => V2TableRow { value: "EXF", display_name: "Extended family", definition: "", comment_usage_note: "", status: "" },
        "SIB" => V2TableRow { value: "SIB", display_name: "Sibling", definition: "", comment_usage_note: "", status: "" },
        "BRO" => V2TableRow { value: "BRO", display_name: "Brother", definition: "", comment_usage_note: "", status: "" },
        "SIS" => V2TableRow { value: "SIS", display_name: "Sister", definition: "", comment_usage_note: "", status: "" },
        "FND" => V2TableRow { value: "FND", display_name: "Friend", definition: "", comment_usage_note: "", status: "" },
        "OAD" => V2TableRow { value: "OAD", display_name: "Other adult", definition: "", comment_usage_note: "", status: "" },
        "EME" => V2TableRow { value: "EME", display_name: "Employee", definition: "", comment_usage_note: "", status: "" },
        "EMR" => V2TableRow { value: "EMR", display_name: "Employer", definition: "", comment_usage_note: "", status: "" },
        "ASC" => V2TableRow { value: "ASC", display_name: "Associate", definition: "", comment_usage_note: "", status: "" },
        "EMC" => V2TableRow { value: "EMC", display_name: "Emergency contact", definition: "", comment_usage_note: "", status: "" },
        "OWN" => V2TableRow { value: "OWN", display_name: "Owner", definition: "", comment_usage_note: "", status: "" },
        "TRA" => V2TableRow { value: "TRA", display_name: "Trainer", definition: "", comment_usage_note: "", status: "" },
        "MGR" => V2TableRow { value: "MGR", display_name: "Manager", definition: "", comment_usage_note: "", status: "" },
        "NON" => V2TableRow { value: "NON", display_name: "None", definition: "", comment_usage_note: "", status: "" },
        "UNK" => V2TableRow { value: "UNK", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
        "OTH" => V2TableRow { value: "OTH", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0065: V2Table = V2Table {
    number: 65,
    metadata: &super::metadata::TABLE_0065_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Add ordered tests to the existing specimen", definition: "", comment_usage_note: "", status: "" },
        "G" => V2TableRow { value: "G", display_name: "Generated order; reflex order", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Lab to obtain specimen from patient", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Specimen obtained by service other than Lab", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Pending specimen; Order sent prior to delivery", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Revised order", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Schedule the tests specified below", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0066: V2Table = V2Table {
    number: 66,
    metadata: &super::metadata::TABLE_0066_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Full time employed", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Part time employed", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Self-employed", definition: "Self-employed,", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Contract, per diem", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Leave of absence (e.g., family leave, sabbatical, etc.)", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Temporarily unemployed", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Unemployed", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Retired", definition: "", comment_usage_note: "", status: "" },
        "6" => V2TableRow { value: "6", display_name: "On active military duty", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "9" => V2TableRow { value: "9", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0069: V2Table = V2Table {
    number: 69,
    metadata: &super::metadata::TABLE_0069_METADATA,
    rows: phf_map! {
        "MED" => V2TableRow { value: "MED", display_name: "Medical Service", definition: "", comment_usage_note: "", status: "" },
        "SUR" => V2TableRow { value: "SUR", display_name: "Surgical Service", definition: "", comment_usage_note: "", status: "" },
        "URO" => V2TableRow { value: "URO", display_name: "Urology Service", definition: "", comment_usage_note: "", status: "" },
        "PUL" => V2TableRow { value: "PUL", display_name: "Pulmonary Service", definition: "", comment_usage_note: "", status: "" },
        "CAR" => V2TableRow { value: "CAR", display_name: "Cardiac Service", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0074: V2Table = V2Table {
    number: 74,
    metadata: &super::metadata::TABLE_0074_METADATA,
    rows: phf_map! {
        "AU" => V2TableRow { value: "AU", display_name: "Audiology", definition: "", comment_usage_note: "", status: "" },
        "BG" => V2TableRow { value: "BG", display_name: "Blood Gases", definition: "", comment_usage_note: "", status: "" },
        "BLB" => V2TableRow { value: "BLB", display_name: "Blood Bank", definition: "", comment_usage_note: "", status: "" },
        "CG" => V2TableRow { value: "CG", display_name: "Cytogenetics", definition: "", comment_usage_note: "", status: "" },
        "CUS" => V2TableRow { value: "CUS", display_name: "Cardiac Ultrasound", definition: "", comment_usage_note: "", status: "" },
        "CTH" => V2TableRow { value: "CTH", display_name: "Cardiac Catheterization", definition: "", comment_usage_note: "", status: "" },
        "CT" => V2TableRow { value: "CT", display_name: "CAT Scan", definition: "", comment_usage_note: "", status: "" },
        "CH" => V2TableRow { value: "CH", display_name: "Chemistry", definition: "", comment_usage_note: "", status: "" },
        "CP" => V2TableRow { value: "CP", display_name: "Cytopathology", definition: "", comment_usage_note: "", status: "" },
        "EC" => V2TableRow { value: "EC", display_name: "Electrocardiac (e.g., EKG, EEC, Holter)", definition: "", comment_usage_note: "", status: "" },
        "EN" => V2TableRow { value: "EN", display_name: "Electroneuro (EEG, EMG,EP,PSG)", definition: "", comment_usage_note: "", status: "" },
        "GE" => V2TableRow { value: "GE", display_name: "Genetics", definition: "", comment_usage_note: "", status: "" },
        "HM" => V2TableRow { value: "HM", display_name: "Hematology", definition: "", comment_usage_note: "", status: "" },
        "ICU" => V2TableRow { value: "ICU", display_name: "Bedside ICU Monitoring", definition: "", comment_usage_note: "", status: "" },
        "IMM" => V2TableRow { value: "IMM", display_name: "Immunology", definition: "", comment_usage_note: "", status: "" },
        "LAB" => V2TableRow { value: "LAB", display_name: "Laboratory", definition: "", comment_usage_note: "", status: "" },
        "MB" => V2TableRow { value: "MB", display_name: "Microbiology", definition: "", comment_usage_note: "", status: "" },
        "MCB" => V2TableRow { value: "MCB", display_name: "Mycobacteriolog y", definition: "", comment_usage_note: "", status: "" },
        "MYC" => V2TableRow { value: "MYC", display_name: "Mycology", definition: "", comment_usage_note: "", status: "" },
        "NMS" => V2TableRow { value: "NMS", display_name: "Nuclear Medicine Scan", definition: "", comment_usage_note: "", status: "" },
        "NMR" => V2TableRow { value: "NMR", display_name: "Nuclear Magnetic Resonance", definition: "", comment_usage_note: "", status: "" },
        "NRS" => V2TableRow { value: "NRS", display_name: "Nursing Service Measures", definition: "", comment_usage_note: "", status: "" },
        "OUS" => V2TableRow { value: "OUS", display_name: "OB Ultrasound", definition: "", comment_usage_note: "", status: "" },
        "OT" => V2TableRow { value: "OT", display_name: "Occupational Therapy", definition: "", comment_usage_note: "", status: "" },
        "OTH" => V2TableRow { value: "OTH", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "OSL" => V2TableRow { value: "OSL", display_name: "Outside Lab", definition: "", comment_usage_note: "", status: "" },
        "PHR" => V2TableRow { value: "PHR", display_name: "Pharmacy", definition: "", comment_usage_note: "", status: "" },
        "PT" => V2TableRow { value: "PT", display_name: "Physical Therapy", definition: "", comment_usage_note: "", status: "" },
        "PHY" => V2TableRow { value: "PHY", display_name: "Physician (Hx. Dx, admission note, etc.)", definition: "", comment_usage_note: "", status: "" },
        "PF" => V2TableRow { value: "PF", display_name: "Pulmonary Function", definition: "", comment_usage_note: "", status: "" },
        "RAD" => V2TableRow { value: "RAD", display_name: "Radiology", definition: "", comment_usage_note: "", status: "" },
        "RX" => V2TableRow { value: "RX", display_name: "Radiograph", definition: "", comment_usage_note: "", status: "" },
        "RUS" => V2TableRow { value: "RUS", display_name: "Radiology Ultrasound", definition: "", comment_usage_note: "", status: "" },
        "RC" => V2TableRow { value: "RC", display_name: "Respiratory Care (therapy)", definition: "", comment_usage_note: "", status: "" },
        "RT" => V2TableRow { value: "RT", display_name: "Radiation Therapy", definition: "", comment_usage_note: "", status: "" },
        "SR" => V2TableRow { value: "SR", display_name: "Serology", definition: "", comment_usage_note: "", status: "" },
        "SP" => V2TableRow { value: "SP", display_name: "Surgical Pathology", definition: "", comment_usage_note: "", status: "" },
        "TX" => V2TableRow { value: "TX", display_name: "Toxicology", definition: "", comment_usage_note: "", status: "" },
        "VUS" => V2TableRow { value: "VUS", display_name: "Vascular Ultrasound", definition: "", comment_usage_note: "", status: "" },
        "VR" => V2TableRow { value: "VR", display_name: "Virology", definition: "", comment_usage_note: "", status: "" },
        "XRC" => V2TableRow { value: "XRC", display_name: "Cineradiograph", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0076: V2Table = V2Table {
    number: 76,
    metadata: &super::metadata::TABLE_0076_METADATA,
    rows: phf_map! {
        "ACK" => V2TableRow { value: "ACK", display_name: "General acknowledgment message", definition: "", comment_usage_note: "2", status: "" },
        "ADR" => V2TableRow { value: "ADR", display_name: "ADT response", definition: "", comment_usage_note: "3 - Deprecated", status: "" },
        "ADT" => V2TableRow { value: "ADT", display_name: "ADT message", definition: "", comment_usage_note: "3", status: "" },
        "BAR" => V2TableRow { value: "BAR", display_name: "Add/change billing account", definition: "", comment_usage_note: "6", status: "" },
        "BPS" => V2TableRow { value: "BPS", display_name: "Blood product dispense status message", definition: "", comment_usage_note: "4", status: "" },
        "BRP" => V2TableRow { value: "BRP", display_name: "Blood product dispense status acknowledgement message", definition: "", comment_usage_note: "4", status: "" },
        "BRT" => V2TableRow { value: "BRT", display_name: "Blood product transfusion/disposition acknowledgement message", definition: "", comment_usage_note: "4", status: "" },
        "BTS" => V2TableRow { value: "BTS", display_name: "Blood product transfusion/disposition message", definition: "", comment_usage_note: "4", status: "" },
        "CCF" => V2TableRow { value: "CCF", display_name: "Collaborative Care Fetch", definition: "", comment_usage_note: "7", status: "" },
        "CCI" => V2TableRow { value: "CCI", display_name: "Collaborative Care Information", definition: "", comment_usage_note: "7", status: "" },
        "CCM" => V2TableRow { value: "CCM", display_name: "Collaborative Care Message", definition: "", comment_usage_note: "7", status: "" },
        "CCQ" => V2TableRow { value: "CCQ", display_name: "Collaborative Care Referral", definition: "", comment_usage_note: "7", status: "" },
        "CCU" => V2TableRow { value: "CCU", display_name: "Collaborative Care Referral", definition: "", comment_usage_note: "7", status: "" },
        "CQU" => V2TableRow { value: "CQU", display_name: "Collaborative Care Referral", definition: "", comment_usage_note: "7", status: "" },
        "CRM" => V2TableRow { value: "CRM", display_name: "Clinical study registration message", definition: "", comment_usage_note: "7", status: "" },
        "CSU" => V2TableRow { value: "CSU", display_name: "Unsolicited study data message", definition: "", comment_usage_note: "7", status: "" },
        "DBC" => V2TableRow { value: "DBC", display_name: "Create Donor Record", definition: "", comment_usage_note: "4", status: "" },
        "DBU" => V2TableRow { value: "DBU", display_name: "Update Donor Record", definition: "", comment_usage_note: "4", status: "" },
        "DEL" => V2TableRow { value: "DEL", display_name: "Donor Eligibility", definition: "", comment_usage_note: "4", status: "" },
        "DEO" => V2TableRow { value: "DEO", display_name: "Donor Eligibility Observation", definition: "", comment_usage_note: "4", status: "" },
        "DER" => V2TableRow { value: "DER", display_name: "Donor Eligibility Request", definition: "", comment_usage_note: "4", status: "" },
        "DFT" => V2TableRow { value: "DFT", display_name: "Detail financial transactions", definition: "", comment_usage_note: "6", status: "" },
        "DOC" => V2TableRow { value: "DOC", display_name: "Document response", definition: "9 - Deprecated", comment_usage_note: "", status: "" },
        "DPR" => V2TableRow { value: "DPR", display_name: "Donation Procedure", definition: "4", comment_usage_note: "", status: "" },
        "DRC" => V2TableRow { value: "DRC", display_name: "Donor Request to Collect", definition: "4", comment_usage_note: "", status: "" },
        "DSR" => V2TableRow { value: "DSR", display_name: "Display response", definition: "5 - Deprecated", comment_usage_note: "", status: "" },
        "EAC" => V2TableRow { value: "EAC", display_name: "Automated equipment command message", definition: "13", comment_usage_note: "", status: "" },
        "EAN" => V2TableRow { value: "EAN", display_name: "Automated equipment notification message", definition: "13", comment_usage_note: "", status: "" },
        "EAR" => V2TableRow { value: "EAR", display_name: "Automated equipment response message", definition: "13", comment_usage_note: "", status: "" },
        "EHC" => V2TableRow { value: "EHC", display_name: "Health Care Invoice", definition: "16", comment_usage_note: "", status: "" },
        "ESR" => V2TableRow { value: "ESR", display_name: "Automated equipment status update acknowledgment message", definition: "13", comment_usage_note: "", status: "" },
        "ESU" => V2TableRow { value: "ESU", display_name: "Automated equipment status update message", definition: "13", comment_usage_note: "", status: "" },
        "INR" => V2TableRow { value: "INR", display_name: "Automated equipment inventory request message", definition: "13", comment_usage_note: "", status: "" },
        "INU" => V2TableRow { value: "INU", display_name: "Automated equipment inventory update message", definition: "13", comment_usage_note: "", status: "" },
        "LSR" => V2TableRow { value: "LSR", display_name: "Automated equipment log/service request message", definition: "13", comment_usage_note: "", status: "" },
        "LSU" => V2TableRow { value: "LSU", display_name: "Automated equipment log/service update message", definition: "13", comment_usage_note: "", status: "" },
        "MDM" => V2TableRow { value: "MDM", display_name: "Medical document management", definition: "9", comment_usage_note: "", status: "" },
        "MFD" => V2TableRow { value: "MFD", display_name: "Master files delayed application acknowledgment", definition: "8 - Deprecated", comment_usage_note: "", status: "" },
        "MFK" => V2TableRow { value: "MFK", display_name: "Master files application acknowledgment", definition: "8", comment_usage_note: "", status: "" },
        "MFN" => V2TableRow { value: "MFN", display_name: "Master files notification", definition: "8", comment_usage_note: "", status: "" },
        "MFQ" => V2TableRow { value: "MFQ", display_name: "Master files query", definition: "8 - Deprecated", comment_usage_note: "", status: "" },
        "MFR" => V2TableRow { value: "MFR", display_name: "Master files response", definition: "8 - Deprecated", comment_usage_note: "", status: "" },
        "NMD" => V2TableRow { value: "NMD", display_name: "Application management data message", definition: "14", comment_usage_note: "", status: "" },
        "NMQ" => V2TableRow { value: "NMQ", display_name: "Application management query message", definition: "14 - Deprecated", comment_usage_note: "", status: "" },
        "NMR" => V2TableRow { value: "NMR", display_name: "Application management response message", definition: "14 - Deprecated", comment_usage_note: "", status: "" },
        "OMB" => V2TableRow { value: "OMB", display_name: "Blood product order message", definition: "4", comment_usage_note: "", status: "" },
        "OMD" => V2TableRow { value: "OMD", display_name: "Dietary order", definition: "4", comment_usage_note: "", status: "" },
        "OMG" => V2TableRow { value: "OMG", display_name: "General clinical order message", definition: "4", comment_usage_note: "", status: "" },
        "OMI" => V2TableRow { value: "OMI", display_name: "Imaging order", definition: "4", comment_usage_note: "", status: "" },
        "OML" => V2TableRow { value: "OML", display_name: "Laboratory order message", definition: "4", comment_usage_note: "", status: "" },
        "OMN" => V2TableRow { value: "OMN", display_name: "Non-stock requisition order message", definition: "4", comment_usage_note: "", status: "" },
        "OMP" => V2TableRow { value: "OMP", display_name: "Pharmacy/treatment order message", definition: "4", comment_usage_note: "", status: "" },
        "OMQ" => V2TableRow { value: "OMQ", display_name: "General order message with document payload", definition: "4", comment_usage_note: "", status: "" },
        "OMS" => V2TableRow { value: "OMS", display_name: "Stock requisition order message", definition: "4", comment_usage_note: "", status: "" },
        "OPL" => V2TableRow { value: "OPL", display_name: "Population/Location-Based Laboratory Order Message", definition: "4", comment_usage_note: "", status: "" },
        "OPR" => V2TableRow { value: "OPR", display_name: "Population/Location-Based Laboratory Order Acknowledgment Message", definition: "4", comment_usage_note: "", status: "" },
        "OPU" => V2TableRow { value: "OPU", display_name: "Unsolicited Population/Location-Based Laboratory Observation Message", definition: "7", comment_usage_note: "", status: "" },
        "ORA" => V2TableRow { value: "ORA", display_name: "Observation Report Acknowledgment", definition: "7", comment_usage_note: "", status: "" },
        "ORB" => V2TableRow { value: "ORB", display_name: "Blood product order acknowledgement message", definition: "4", comment_usage_note: "", status: "" },
        "ORD" => V2TableRow { value: "ORD", display_name: "Dietary order acknowledgment message", definition: "4", comment_usage_note: "", status: "" },
        "ORF" => V2TableRow { value: "ORF", display_name: "Query for results of observation", definition: "7 - Deprecated", comment_usage_note: "", status: "" },
        "ORG" => V2TableRow { value: "ORG", display_name: "General clinical order acknowledgment message", definition: "4", comment_usage_note: "", status: "" },
        "ORI" => V2TableRow { value: "ORI", display_name: "Imaging order acknowledgement message", definition: "4", comment_usage_note: "", status: "" },
        "ORL" => V2TableRow { value: "ORL", display_name: "Laboratory acknowledgment message (unsolicited)", definition: "7", comment_usage_note: "", status: "" },
        "ORM" => V2TableRow { value: "ORM", display_name: "Pharmacy/treatment order message", definition: "4 - Deprecated", comment_usage_note: "", status: "" },
        "ORN" => V2TableRow { value: "ORN", display_name: "Non-stock requisition - General order acknowledgment message", definition: "4", comment_usage_note: "", status: "" },
        "ORP" => V2TableRow { value: "ORP", display_name: "Pharmacy/treatment order acknowledgment message", definition: "4", comment_usage_note: "", status: "" },
        "ORR" => V2TableRow { value: "ORR", display_name: "General order response message response to any ORM", definition: "4 - Deprecated", comment_usage_note: "", status: "" },
        "ORS" => V2TableRow { value: "ORS", display_name: "Stock requisition - Order acknowledgment message", definition: "4", comment_usage_note: "", status: "" },
        "ORU" => V2TableRow { value: "ORU", display_name: "Unsolicited transmission of an observation message", definition: "7", comment_usage_note: "", status: "" },
        "ORX" => V2TableRow { value: "ORX", display_name: "General Order Message with Document Payload Acknowledgement", definition: "4", comment_usage_note: "", status: "" },
        "OSM" => V2TableRow { value: "OSM", display_name: "Specimen Shipment Message", definition: "7", comment_usage_note: "", status: "" },
        "OSQ" => V2TableRow { value: "OSQ", display_name: "Query response for order status", definition: "4 - Deprecated", comment_usage_note: "", status: "" },
        "OSR" => V2TableRow { value: "OSR", display_name: "Query response for order status", definition: "4 - Deprecated", comment_usage_note: "", status: "" },
        "OSU" => V2TableRow { value: "OSU", display_name: "Order status update", definition: "4", comment_usage_note: "", status: "" },
        "OUL" => V2TableRow { value: "OUL", display_name: "Unsolicited laboratory observation message", definition: "7", comment_usage_note: "", status: "" },
        "PEX" => V2TableRow { value: "PEX", display_name: "Product experience message", definition: "7", comment_usage_note: "", status: "" },
        "PGL" => V2TableRow { value: "PGL", display_name: "Patient goal message", definition: "12", comment_usage_note: "", status: "" },
        "PIN" => V2TableRow { value: "PIN", display_name: "Patient insurance information", definition: "11", comment_usage_note: "", status: "" },
        "PMU" => V2TableRow { value: "PMU", display_name: "Add personnel record", definition: "15", comment_usage_note: "", status: "" },
        "PPG" => V2TableRow { value: "PPG", display_name: "Patient pathway message (goal-oriented)", definition: "12", comment_usage_note: "", status: "" },
        "PPP" => V2TableRow { value: "PPP", display_name: "Patient pathway message (problem- oriented)", definition: "12", comment_usage_note: "", status: "" },
        "PPR" => V2TableRow { value: "PPR", display_name: "Patient problem message", definition: "12", comment_usage_note: "", status: "" },
        "PPT" => V2TableRow { value: "PPT", display_name: "Patient pathway goal-oriented response", definition: "Deprecated", comment_usage_note: "", status: "" },
        "PPV" => V2TableRow { value: "PPV", display_name: "Patient goal response", definition: "Deprecated", comment_usage_note: "", status: "" },
        "PRR" => V2TableRow { value: "PRR", display_name: "Patient problem response", definition: "Deprecated", comment_usage_note: "", status: "" },
        "PTR" => V2TableRow { value: "PTR", display_name: "Patient pathway problem-oriented response", definition: "Deprecated", comment_usage_note: "", status: "" },
        "QBP" => V2TableRow { value: "QBP", display_name: "Query by parameter", definition: "5", comment_usage_note: "", status: "" },
        "QCK" => V2TableRow { value: "QCK", display_name: "Query general acknowledgment Deferred query", definition: "5 - Deprecated", comment_usage_note: "", status: "" },
        "QCN" => V2TableRow { value: "QCN", display_name: "Cancel query", definition: "5", comment_usage_note: "", status: "" },
        "QRY" => V2TableRow { value: "QRY", display_name: "Query, original mode", definition: "3", comment_usage_note: "", status: "" },
        "QSB" => V2TableRow { value: "QSB", display_name: "Create subscription", definition: "5", comment_usage_note: "", status: "" },
        "QSX" => V2TableRow { value: "QSX", display_name: "Cancel subscription/acknowledge message", definition: "5", comment_usage_note: "", status: "" },
        "QVR" => V2TableRow { value: "QVR", display_name: "Query for previous events", definition: "5", comment_usage_note: "", status: "" },
        "RAR" => V2TableRow { value: "RAR", display_name: "Pharmacy/treatment administration information", definition: "4 - Deprecated", comment_usage_note: "", status: "" },
        "RAS" => V2TableRow { value: "RAS", display_name: "Pharmacy/treatment administration message", definition: "4", comment_usage_note: "", status: "" },
        "RCI" => V2TableRow { value: "RCI", display_name: "Return clinical information", definition: "11", comment_usage_note: "", status: "" },
        "RCL" => V2TableRow { value: "RCL", display_name: "Return clinical list", definition: "11", comment_usage_note: "", status: "" },
        "RDE" => V2TableRow { value: "RDE", display_name: "Pharmacy/treatment encoded order message", definition: "4", comment_usage_note: "", status: "" },
        "RDR" => V2TableRow { value: "RDR", display_name: "Pharmacy/treatment dispense information", definition: "4", comment_usage_note: "", status: "" },
        "RDS" => V2TableRow { value: "RDS", display_name: "Pharmacy/treatment dispense message", definition: "4", comment_usage_note: "", status: "" },
        "RDY" => V2TableRow { value: "RDY", display_name: "Display based response", definition: "5", comment_usage_note: "", status: "" },
        "REF" => V2TableRow { value: "REF", display_name: "Patient referral", definition: "11", comment_usage_note: "", status: "" },
        "RER" => V2TableRow { value: "RER", display_name: "Pharmacy/treatment encoded order information", definition: "4 - De", comment_usage_note: "precated", status: "" },
        "RGR" => V2TableRow { value: "RGR", display_name: "Pharmacy/treatment dose information", definition: "4 - De", comment_usage_note: "precated", status: "" },
        "RGV" => V2TableRow { value: "RGV", display_name: "Pharmacy/treatment give message", definition: "4", comment_usage_note: "", status: "" },
        "ROR" => V2TableRow { value: "ROR", display_name: "Pharmacy/treatment order response", definition: "4 - De", comment_usage_note: "precated", status: "" },
        "RPA" => V2TableRow { value: "RPA", display_name: "Return patient authorization", definition: "11", comment_usage_note: "", status: "" },
        "RPI" => V2TableRow { value: "RPI", display_name: "Return patient information", definition: "11", comment_usage_note: "", status: "" },
        "RPL" => V2TableRow { value: "RPL", display_name: "Return patient display list", definition: "11", comment_usage_note: "", status: "" },
        "RP" => V2TableRow { value: "RP", display_name: "R Return patient list", definition: "11", comment_usage_note: "", status: "" },
        "RQA" => V2TableRow { value: "RQA", display_name: "Request patient authorization", definition: "11", comment_usage_note: "", status: "" },
        "RQC" => V2TableRow { value: "RQC", display_name: "Request clinical information", definition: "11", comment_usage_note: "", status: "" },
        "RQI" => V2TableRow { value: "RQI", display_name: "Request patient information", definition: "11", comment_usage_note: "", status: "" },
        "RQP" => V2TableRow { value: "RQP", display_name: "Request patient demographics", definition: "11", comment_usage_note: "", status: "" },
        "RRA" => V2TableRow { value: "RRA", display_name: "Pharmacy/treatment administration acknowledgment message", definition: "4", comment_usage_note: "", status: "" },
        "RRD" => V2TableRow { value: "RRD", display_name: "Pharmacy/treatment dispense acknowledgment message", definition: "4", comment_usage_note: "", status: "" },
        "RRE" => V2TableRow { value: "RRE", display_name: "Pharmacy/treatment encoded order acknowledgment message", definition: "4", comment_usage_note: "", status: "" },
        "RRG" => V2TableRow { value: "RRG", display_name: "Pharmacy/treatment give acknowledgment message", definition: "4", comment_usage_note: "", status: "" },
        "RRI" => V2TableRow { value: "RRI", display_name: "Return referral information", definition: "11", comment_usage_note: "", status: "" },
        "RSP" => V2TableRow { value: "RSP", display_name: "Segment pattern response", definition: "5", comment_usage_note: "", status: "" },
        "RTB" => V2TableRow { value: "RTB", display_name: "Tabular response", definition: "5", comment_usage_note: "", status: "" },
        "SCN" => V2TableRow { value: "SCN", display_name: "Notification of Anti-Microbial Device Cycle Data", definition: "17", comment_usage_note: "", status: "" },
        "SDN" => V2TableRow { value: "SDN", display_name: "Notification of Anti-Microbial Device Data", definition: "17", comment_usage_note: "", status: "" },
        "SDR" => V2TableRow { value: "SDR", display_name: "Sterilization anti-microbial device data request", definition: "17", comment_usage_note: "", status: "" },
        "SET" => V2TableRow { value: "SET", display_name: "Specimen Event Tracking This messa type to re how t speci moves throu lifec from colle ident n, tr accep or re proce to st and dispo", definition: "ge is used port he men gh its ycle ction, ificatio ansport, tance jection, ssing orage sition.", comment_usage_note: "N", status: "" },
        "SIU" => V2TableRow { value: "SIU", display_name: "Schedule information unsolicited", definition: "10", comment_usage_note: "", status: "" },
        "SLN" => V2TableRow { value: "SLN", display_name: "Notification of New Sterilization Lot", definition: "17", comment_usage_note: "", status: "" },
        "SLR" => V2TableRow { value: "SLR", display_name: "Sterilization lot request", definition: "17", comment_usage_note: "", status: "" },
        "SMD" => V2TableRow { value: "SMD", display_name: "Sterilization anti-microbial device cycle data request", definition: "17", comment_usage_note: "", status: "" },
        "SQM" => V2TableRow { value: "SQM", display_name: "Schedule query message", definition: "10 - Deprecated", comment_usage_note: "", status: "" },
        "SQR" => V2TableRow { value: "SQR", display_name: "Schedule query response", definition: "10 - Deprecated", comment_usage_note: "", status: "" },
        "SRM" => V2TableRow { value: "SRM", display_name: "Schedule request message", definition: "10", comment_usage_note: "", status: "" },
        "SRR" => V2TableRow { value: "SRR", display_name: "Scheduled request response", definition: "10", comment_usage_note: "", status: "" },
        "SSR" => V2TableRow { value: "SSR", display_name: "Specimen status request message", definition: "13", comment_usage_note: "", status: "" },
        "SSU" => V2TableRow { value: "SSU", display_name: "Specimen status update message", definition: "13", comment_usage_note: "", status: "" },
        "STC" => V2TableRow { value: "STC", display_name: "Notification of Sterilization Configuration", definition: "17", comment_usage_note: "", status: "" },
        "STI" => V2TableRow { value: "STI", display_name: "Sterilization item request", definition: "17", comment_usage_note: "", status: "" },
        "SUR" => V2TableRow { value: "SUR", display_name: "Summary product experience report", definition: "7 - Deprecated", comment_usage_note: "", status: "" },
        "TBR" => V2TableRow { value: "TBR", display_name: "Tabular data response", definition: "5 - Deprecated", comment_usage_note: "", status: "" },
        "TCR" => V2TableRow { value: "TCR", display_name: "Automated equipment test code settings request message", definition: "13", comment_usage_note: "", status: "" },
        "TCU" => V2TableRow { value: "TCU", display_name: "Automated equipment test code settings update message", definition: "13", comment_usage_note: "", status: "" },
        "UDM" => V2TableRow { value: "UDM", display_name: "Unsolicited display update message", definition: "5", comment_usage_note: "", status: "" },
        "VXQ" => V2TableRow { value: "VXQ", display_name: "Query for vaccination record", definition: "4 - Deprecated", comment_usage_note: "", status: "" },
        "VXR" => V2TableRow { value: "VXR", display_name: "Vaccination record response", definition: "4 - Deprecated", comment_usage_note: "", status: "" },
        "VXU" => V2TableRow { value: "VXU", display_name: "Unsolicited vaccination record update", definition: "4", comment_usage_note: "", status: "" },
        "VXX" => V2TableRow { value: "VXX", display_name: "Response for vaccination query with multiple PID matches", definition: "4 -Deprecated", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0080: V2Table = V2Table {
    number: 80,
    metadata: &super::metadata::TABLE_0080_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "An age- population", definition: "based", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "None - generic normal range", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "A race-based population", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "A sex-based population", definition: "", comment_usage_note: "", status: "" },
        "SP" => V2TableRow { value: "SP", display_name: "Species", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Breed", definition: "", comment_usage_note: "", status: "" },
        "ST" => V2TableRow { value: "ST", display_name: "Strain", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0083: V2Table = V2Table {
    number: 83,
    metadata: &super::metadata::TABLE_0083_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Outlier days", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Outlier cost", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0085: V2Table = V2Table {
    number: 85,
    metadata: &super::metadata::TABLE_0085_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Amended based on adjustments provided by the Placer (Physician) regarding patient demographics (such as age and/or gender or other patient specific information", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Appended Report - Final results reviewed and further information provided for clarity without change to the original result values.", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Record coming over is a correction and thus replaces a final result", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Deletes the OBX record", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Final results", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Specimen in lab; results pending", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Not asked; used to affirmatively document that the observation identified in the OBX was not sought when the universal service ID in OBR-4 implies that it would be sought.", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Order detail description only (no result)", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Preliminary results", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Results entered -- not verified", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Partial results. Deprecated. Retained only for backward compatibility as of V2.6.", definition: "", comment_usage_note: "Deprecated.", status: "" },
        "V" => V2TableRow { value: "V", display_name: "Verified - Final results reviewed and confirmed to be correct, no change to result value, normal range or abnormal flag", definition: "", comment_usage_note: "", status: "" },
        "X" => V2TableRow { value: "X", display_name: "Results cannot be obtained for this observation", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Results status change to final without retransmitting results already sent as 'preliminary.' E.g., radiology changes status from preliminary to final", definition: "", comment_usage_note: "", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Post original as wrong, e.g., transmitted for wrong patient", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0091: V2Table = V2Table {
    number: 91,
    metadata: &super::metadata::TABLE_0091_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Deferred", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Immediate", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0092: V2Table = V2Table {
    number: 92,
    metadata: &super::metadata::TABLE_0092_METADATA,
    rows: phf_map! {
        "R" => V2TableRow { value: "R", display_name: "Re-admission", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0093: V2Table = V2Table {
    number: 93,
    metadata: &super::metadata::TABLE_0093_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Yes", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No", definition: "", comment_usage_note: "", status: "" },
        "..." => V2TableRow { value: "...", display_name: "user-defined codes", definition: "", comment_usage_note: "", status: "D" },
    },
};

pub static TABLE_0098: V2Table = V2Table {
    number: 98,
    metadata: &super::metadata::TABLE_0098_METADATA,
    rows: phf_map! {
        "S" => V2TableRow { value: "S", display_name: "Standard", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unified", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Maternity", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0100: V2Table = V2Table {
    number: 100,
    metadata: &super::metadata::TABLE_0100_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "On discharge", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "On receipt of order", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "At time service is completed", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "At time service is started", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "At a designated date/time", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0103: V2Table = V2Table {
    number: 103,
    metadata: &super::metadata::TABLE_0103_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Debugging", definition: "Messages used for identification and correction of software errors.", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Production", definition: "Messages used for communication of live production data.", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Training", definition: "Messages used for training, where new/updated configurations are utilized to prepare users outside of a production setting.", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Non-Production Testing", definition: "Messages used for testing of an interface for structure, content, and conformance between trading partners, using non-production data.", comment_usage_note: "", status: "" },
        "V" => V2TableRow { value: "V", display_name: "Validation", definition: "Messages used for conformance testing by a third party; for example, as part of certification.", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0104: V2Table = V2Table {
    number: 104,
    metadata: &super::metadata::TABLE_0104_METADATA,
    rows: phf_map! {
        "2.0" => V2TableRow { value: "2.0", display_name: "Release 2.0", definition: "", comment_usage_note: "September 1988", status: "" },
        "2.0D" => V2TableRow { value: "2.0D", display_name: "Demo 2.0", definition: "", comment_usage_note: "October 1988", status: "" },
        "2.1" => V2TableRow { value: "2.1", display_name: "Release 2.1", definition: "", comment_usage_note: "March 1990", status: "" },
        "2.2" => V2TableRow { value: "2.2", display_name: "Release 2.2", definition: "", comment_usage_note: "December 1994", status: "" },
        "2.3" => V2TableRow { value: "2.3", display_name: "Release 2.3", definition: "", comment_usage_note: "March 1997", status: "" },
        "2.3.1" => V2TableRow { value: "2.3.1", display_name: "Release 2.3.1", definition: "", comment_usage_note: "May 1999", status: "" },
        "2.4" => V2TableRow { value: "2.4", display_name: "Release 2.4", definition: "", comment_usage_note: "November 2000", status: "" },
        "2.5" => V2TableRow { value: "2.5", display_name: "Release 2.5", definition: "", comment_usage_note: "May 2003", status: "" },
        "2.5.1" => V2TableRow { value: "2.5.1", display_name: "Release 2.5.1", definition: "", comment_usage_note: "January 2007", status: "" },
        "2.6" => V2TableRow { value: "2.6", display_name: "Release 2.6", definition: "", comment_usage_note: "July 2007", status: "" },
        "2.7" => V2TableRow { value: "2.7", display_name: "Release 2.7", definition: "", comment_usage_note: "November 2010", status: "" },
        "2.7.1" => V2TableRow { value: "2.7.1", display_name: "Release 2.7.1", definition: "", comment_usage_note: "July 2012", status: "" },
        "2.8" => V2TableRow { value: "2.8", display_name: "Release 2.8", definition: "", comment_usage_note: "February 2014", status: "" },
        "2.8.1" => V2TableRow { value: "2.8.1", display_name: "Release 2.8.1", definition: "", comment_usage_note: "April 2014", status: "" },
        "2.8.2" => V2TableRow { value: "2.8.2", display_name: "Release 2.8.2", definition: "", comment_usage_note: "May 2015", status: "" },
        "2.9" => V2TableRow { value: "2.9", display_name: "Draft 2.9", definition: "", comment_usage_note: "Sep. 2017", status: "N" },
    },
};

pub static TABLE_0105: V2Table = V2Table {
    number: 105,
    metadata: &super::metadata::TABLE_0105_METADATA,
    rows: phf_map! {
        "L" => V2TableRow { value: "L", display_name: "Ancillary (filler) department is source of comment", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Orderer (placer) is source of comment", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other system is source of comment", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0116: V2Table = V2Table {
    number: 116,
    metadata: &super::metadata::TABLE_0116_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "Closed", definition: "", comment_usage_note: "", status: "" },
        "H" => V2TableRow { value: "H", display_name: "Housekeeping", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Occupied", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unoccupied", definition: "", comment_usage_note: "", status: "" },
        "K" => V2TableRow { value: "K", display_name: "Contaminated", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Isolated", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0119: V2Table = V2Table {
    number: 119,
    metadata: &super::metadata::TABLE_0119_METADATA,
    rows: phf_map! {
        "AF" => V2TableRow { value: "AF", display_name: "Order/service refill request approval", definition: "AF is a response to RF where the placer authorizing a refill or quantity of refills.", comment_usage_note: "Placer Applications .", status: "" },
        "CA" => V2TableRow { value: "CA", display_name: "Cancel order/service request", definition: "", comment_usage_note: "Placer or Filler Applications", status: "" },
        "CH" => V2TableRow { value: "CH", display_name: "Child order/service", definition: "", comment_usage_note: "Placer or Filler Applications. Usage Note: Used in conjunction with the PA - Parent order control code. Refer to PA order control code for discussion.", status: "" },
        "CN" => V2TableRow { value: "CN", display_name: "Combined result", definition: "", comment_usage_note: "Filler Applications.", status: "" },
        "CP" => V2TableRow { value: "CP", display_name: "Cancel process step", definition: "", comment_usage_note: "Filler Applications.", status: "" },
        "CR" => V2TableRow { value: "CR", display_name: "Canceled as requested", definition: "", comment_usage_note: "Filler or Placer Applications. Usage Note: A response by the filler or placer application that a request to cancel (CA by the placer application) was performed successfully.", status: "" },
        "DC" => V2TableRow { value: "DC", display_name: "Discontinue order/service request", definition: "", comment_usage_note: "Placer or Filler Applications.", status: "" },
        "DE" => V2TableRow { value: "DE", display_name: "Data errors", definition: "", comment_usage_note: "Placer or Filler Applications.", status: "" },
        "DF" => V2TableRow { value: "DF", display_name: "Order/service refill request denied", definition: "", comment_usage_note: "Placer Applications.", status: "" },
        "DR" => V2TableRow { value: "DR", display_name: "Discontinued as requested", definition: "", comment_usage_note: "Filler or Placer Applications. Usage Note: The filler or placer, in response to a request to discontinue (DC from the placer or filler application), has discontinued the order/service.", status: "" },
        "FU" => V2TableRow { value: "FU", display_name: "Order/service refilled, unsolicited", definition: "Usage Note filler iss patient's", comment_usage_note: "Filler Applications. : FU notifies the placer that the ued a refill for the order at the request.", status: "" },
        "HD" => V2TableRow { value: "HD", display_name: "Hold order request", definition: "Placer App Usage Note but are no requested,", comment_usage_note: "lications. : Typical responses include, t limited to, CR - Cancelled as UC - Unable to Cancel.", status: "" },
        "HR" => V2TableRow { value: "HR", display_name: "On hold as requested", definition: "Filler App", comment_usage_note: "lications.", status: "" },
        "LI" => V2TableRow { value: "LI", display_name: "Link order/service to patient care problem or goal", definition: "Placer or Usage Note Care for c", comment_usage_note: "Filler Applications. : Refer to Chapter 12 Patient omplete discussion.", status: "" },
        "MC" => V2TableRow { value: "MC", display_name: "Miscellaneous Charge - not associated with an order", definition: "applies to DFT^P11^DF Usage Note DFT^P03^DF DFT^P11^DF", comment_usage_note: "DFT^P03^DFT_P03 and T_P11 : applies to T_P03 and T_P11", status: "" },
        "NA" => V2TableRow { value: "NA", display_name: "Number assigned", definition: "Placer App", comment_usage_note: "lications.", status: "" },
        "NR" => V2TableRow { value: "NR", display_name: "Notification Received Notifi the Pl cancel discon other notice", definition: "es the Filler that Placer App acer received a lation, tinuance, or state change", comment_usage_note: "lications.", status: "" },
        "NW" => V2TableRow { value: "NW", display_name: "New order/service", definition: "Placer App Usage Note Number Ass", comment_usage_note: "lications. : See comments for NA - igned.", status: "" },
        "OC" => V2TableRow { value: "OC", display_name: "Order/service canceled", definition: "Filler App", comment_usage_note: "lications.", status: "" },
        "OD" => V2TableRow { value: "OD", display_name: "Order/service discontinued", definition: "Filler App", comment_usage_note: "lications.", status: "" },
        "OE" => V2TableRow { value: "OE", display_name: "Order/service released", definition: "Filler App", comment_usage_note: "lications.", status: "" },
        "OF" => V2TableRow { value: "OF", display_name: "Order/service refilled as requested", definition: "Filler App Usage Note placer sys", comment_usage_note: "lications. : OF directly responds to the tem's request for a refill.", status: "" },
        "OH" => V2TableRow { value: "OH", display_name: "Order/service held", definition: "Filler App", comment_usage_note: "lications.", status: "" },
        "OK" => V2TableRow { value: "OK", display_name: "Order/service accepted & OK", definition: "Filler App Usage Note Number Ass", comment_usage_note: "lications. : See comments for NA - igned.", status: "" },
        "OP" => V2TableRow { value: "OP", display_name: "Notification of order for outside dispense", definition: "Placer App", comment_usage_note: "lications.", status: "" },
        "OR" => V2TableRow { value: "OR", display_name: "Released as requested", definition: "Filler App", comment_usage_note: "lications.", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Parent order/service", definition: "Filler App", comment_usage_note: "lications.", status: "" },
        "PR" => V2TableRow { value: "PR", display_name: "Previous Results with new order/service", definition: "Placer App", comment_usage_note: "lications.", status: "" },
        "PY" => V2TableRow { value: "PY", display_name: "Notification of replacement order for outside dispense", definition: "Placer App Usage Note Notificati", comment_usage_note: "lications. : See comments for OP - on of order for outside dispense.", status: "" },
        "RA" => V2TableRow { value: "RA", display_name: "Recommendation Identifies tha Accepted previously recommended replacement or been accepted", definition: "t this Placer Applications: Used in the Recommendati message; includes the as der has order number for the acc order Filler Applications: Used in the acknowledgme message to the Recommend message", comment_usage_note: "N on Accepted signed placer epted replacement nt response ation Accepted", status: "" },
        "RC" => V2TableRow { value: "RC", display_name: "Recommended Identifies tha Change OBR represents recommended replacement or", definition: "t this Filler Applications. a Used in the recommendati to the placer der", comment_usage_note: "N on message sent", status: "" },
        "RD" => V2TableRow { value: "RD", display_name: "Recommendation Identifies tha Declined previously sen recommended replacement or been declined", definition: "t this Placer Applications: t Used in the Recommendati message der has Filler Applications: Used in the acknowledgme message to the Recommend message", comment_usage_note: "N on Declined nt response ation Declined", status: "" },
        "RE" => V2TableRow { value: "RE", display_name: "Observations/Performe d Service to follow", definition: "Placer or Filler Applica", comment_usage_note: "tions.", status: "" },
        "RF" => V2TableRow { value: "RF", display_name: "Refill order/service request", definition: "Placer or Filler Applica", comment_usage_note: "tions.", status: "" },
        "RL" => V2TableRow { value: "RL", display_name: "Release previous hold", definition: "Placer Applications.", comment_usage_note: "", status: "" },
        "RO" => V2TableRow { value: "RO", display_name: "Replacement order", definition: "Placer or Filler Applica", comment_usage_note: "tions.", status: "" },
        "RP" => V2TableRow { value: "RP", display_name: "Order/service replace request", definition: "Placer Applications.", comment_usage_note: "", status: "" },
        "RQ" => V2TableRow { value: "RQ", display_name: "Replaced as requested", definition: "Filler Applications.", comment_usage_note: "", status: "" },
        "RR" => V2TableRow { value: "RR", display_name: "Request received", definition: "Placer or Filler Applica", comment_usage_note: "tions.", status: "" },
        "RU" => V2TableRow { value: "RU", display_name: "Replaced unsolicited", definition: "Filler Applications.", comment_usage_note: "", status: "" },
        "SC" => V2TableRow { value: "SC", display_name: "Status changed", definition: "Placer or Filler Applica", comment_usage_note: "tions.", status: "" },
        "SN" => V2TableRow { value: "SN", display_name: "Send order/service number", definition: "Placer Applications. Usage Note: See comments Number Assigned.", comment_usage_note: "for NA -", status: "" },
        "SQ" => V2TableRow { value: "SQ", display_name: "Supplemented as Supplementatio requested confirmation m indicating tha order was supplemented a requested.", definition: "n This code is used in the essage response to the request t an s", comment_usage_note: "acknowledgment N messages.", status: "" },
        "SR" => V2TableRow { value: "SR", display_name: "Response to send order/service status request", definition: "Filler Applications.", comment_usage_note: "", status: "" },
        "SS" => V2TableRow { value: "SS", display_name: "Send order/service status request", definition: "Placer Applications.", comment_usage_note: "", status: "" },
        "SU" => V2TableRow { value: "SU", display_name: "Supplement this order Identifies exi orders to be supplemented i Supplementatio Recommendation (filler to pla Supplementatio Reque fille", definition: "sting For example this code is order message to identif n missing a recommended or n organizational protocol described in the IHE Lab cer) and Communication (LCC) prof n st (placer to r) messages.", comment_usage_note: "used in the filler’s N y an order that is der based on for example as oratory ile (LAB-6).", status: "" },
        "UA" => V2TableRow { value: "UA", display_name: "Unable to accept order/service", definition: "Filler Ap", comment_usage_note: "plications.", status: "" },
        "UC" => V2TableRow { value: "UC", display_name: "Unable to cancel", definition: "Filler or", comment_usage_note: "Placer Applications.", status: "" },
        "UD" => V2TableRow { value: "UD", display_name: "Unable to discontinue", definition: "Filler or", comment_usage_note: "Placer Applications.", status: "" },
        "UF" => V2TableRow { value: "UF", display_name: "Unable to refill", definition: "Filler Ap Usage Not Refill or the recei complete", comment_usage_note: "plications. e: Negative response to RF der/service request, indicating that ving application was not able to the refill request.", status: "" },
        "UH" => V2TableRow { value: "UH", display_name: "Unable to put on hold", definition: "Filler Ap", comment_usage_note: "plications.", status: "" },
        "UM" => V2TableRow { value: "UM", display_name: "Unable to replace", definition: "Filler Ap", comment_usage_note: "plications.", status: "" },
        "UN" => V2TableRow { value: "UN", display_name: "Unlink order/service from patient care problem or goal", definition: "Placer or Usage Not Care for", comment_usage_note: "Filler Applications. e: Refer to Chapter 12 Patient complete discussion.", status: "" },
        "UR" => V2TableRow { value: "UR", display_name: "Unable to release", definition: "Filler Ap", comment_usage_note: "plications.", status: "" },
        "UX" => V2TableRow { value: "UX", display_name: "Unable to change", definition: "Filler Ap", comment_usage_note: "plications.", status: "" },
        "XO" => V2TableRow { value: "XO", display_name: "Change order/service request", definition: "Placer Ap", comment_usage_note: "plications.", status: "" },
        "XR" => V2TableRow { value: "XR", display_name: "Changed as requested", definition: "Filler Ap", comment_usage_note: "plications.", status: "" },
        "XX" => V2TableRow { value: "XX", display_name: "Order/service changed, unsol.", definition: "Filler Ap", comment_usage_note: "plications.", status: "" },
    },
};

pub static TABLE_0121: V2Table = V2Table {
    number: 121,
    metadata: &super::metadata::TABLE_0121_METADATA,
    rows: phf_map! {
        "E" => V2TableRow { value: "E", display_name: "Report exceptions only", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Same as E, also Replacement and Parent-Child", definition: "Report exceptions, replacement and parent-child", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Same as R, also other associated segments", definition: "Report exceptions, replacement, parent-child and other associated segments", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Same as D, plus confirmations explicitly", definition: "Report exceptions, replacement, parent-child, other associated segments and explicit confirmations", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Only the MSA segment is returned", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0122: V2Table = V2Table {
    number: 122,
    metadata: &super::metadata::TABLE_0122_METADATA,
    rows: phf_map! {
        "CH" => V2TableRow { value: "CH", display_name: "Charge", definition: "", comment_usage_note: "", status: "" },
        "CO" => V2TableRow { value: "CO", display_name: "Contract", definition: "", comment_usage_note: "", status: "" },
        "CR" => V2TableRow { value: "CR", display_name: "Credit", definition: "", comment_usage_note: "", status: "" },
        "DP" => V2TableRow { value: "DP", display_name: "Department", definition: "", comment_usage_note: "", status: "" },
        "GR" => V2TableRow { value: "GR", display_name: "Grant", definition: "", comment_usage_note: "", status: "" },
        "NC" => V2TableRow { value: "NC", display_name: "No Charge", definition: "", comment_usage_note: "", status: "" },
        "PC" => V2TableRow { value: "PC", display_name: "Professional", definition: "", comment_usage_note: "", status: "" },
        "RS" => V2TableRow { value: "RS", display_name: "Research", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0123: V2Table = V2Table {
    number: 123,
    metadata: &super::metadata::TABLE_0123_METADATA,
    rows: phf_map! {
        "O" => V2TableRow { value: "O", display_name: "Order received; specimen not yet received", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "No results available; specimen received, procedure incomplete", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "No results available; procedure scheduled, but not done", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Some, but not all, results available", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Preliminary", definition: "A verified early result is available, final results not yet obtained", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Corrected, final A result under an ord has been finalized ha corrected", definition: "er, th a t s been", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Results stored; not yet verified", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Final results Final results; result and verified.", definition: "s stored Can only be changed with a corrected result.", comment_usage_note: "", status: "" },
        "X" => V2TableRow { value: "X", display_name: "No results available; No results available; Order canceled canceled.", definition: "Order", comment_usage_note: "", status: "" },
        "Y" => V2TableRow { value: "Y", display_name: "No order on record for No order on record fo this test test.", definition: "r this Usage Note: Used only on queries", comment_usage_note: "", status: "" },
        "Z" => V2TableRow { value: "Z", display_name: "No record of this No record of this pat patient", definition: "ient. Usage Note: Used only on queries", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Corrected, not final A result under an ord has not yet been fina been corrected", definition: "er, that lized has", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Procedure completed, No result available; results pending procedure done. To i that a requested test performed but results pending/not yet avail", definition: "requested Usage Note: ndicate Transitions/Relationships: is OBR- are (... to N): O, I, S. Succeeding able. state (N to ...): P, A, R, F, X OBX-11 No OBX segments can be present where OBX-29 = RSLT", comment_usage_note: "25 Valid preceding state", status: "" },
    },
};

pub static TABLE_0124: V2Table = V2Table {
    number: 124,
    metadata: &super::metadata::TABLE_0124_METADATA,
    rows: phf_map! {
        "CART" => V2TableRow { value: "CART", display_name: "Cart - patient travels on cart or gurney", definition: "", comment_usage_note: "", status: "" },
        "PORT" => V2TableRow { value: "PORT", display_name: "The examining device goes to patient's location", definition: "", comment_usage_note: "", status: "" },
        "WALK" => V2TableRow { value: "WALK", display_name: "Patient walks to diagnostic service", definition: "", comment_usage_note: "", status: "" },
        "WHLC" => V2TableRow { value: "WHLC", display_name: "Wheelchair", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0125: V2Table = V2Table {
    number: 125,
    metadata: &super::metadata::TABLE_0125_METADATA,
    rows: phf_map! {
        "AUI" => V2TableRow { value: "AUI", display_name: "Authorization information", definition: "", comment_usage_note: "", status: "" },
        "CCD" => V2TableRow { value: "CCD", display_name: "Charge code and date", definition: "", comment_usage_note: "", status: "" },
        "CCP" => V2TableRow { value: "CCP", display_name: "Channel calibration parameters", definition: "", comment_usage_note: "", status: "" },
        "CD" => V2TableRow { value: "CD", display_name: "Channel definition", definition: "", comment_usage_note: "", status: "" },
        "CF" => V2TableRow { value: "CF", display_name: "Coded element with formatted values", definition: "", comment_usage_note: "", status: "" },
        "CNE" => V2TableRow { value: "CNE", display_name: "Coded with no exceptions", definition: "", comment_usage_note: "", status: "" },
        "CNN" => V2TableRow { value: "CNN", display_name: "Composite ID number and name simplified", definition: "", comment_usage_note: "", status: "" },
        "CP" => V2TableRow { value: "CP", display_name: "Composite price", definition: "", comment_usage_note: "", status: "" },
        "CSU" => V2TableRow { value: "CSU", display_name: "Channel sensitivity and units", definition: "", comment_usage_note: "", status: "" },
        "CWE" => V2TableRow { value: "CWE", display_name: "Coded with exceptions", definition: "", comment_usage_note: "", status: "" },
        "CX" => V2TableRow { value: "CX", display_name: "Extended composite ID with check digit", definition: "", comment_usage_note: "", status: "" },
        "DDI" => V2TableRow { value: "DDI", display_name: "Daily deductible information", definition: "", comment_usage_note: "", status: "" },
        "DIN" => V2TableRow { value: "DIN", display_name: "Date and institution name", definition: "", comment_usage_note: "", status: "" },
        "DLD" => V2TableRow { value: "DLD", display_name: "Discharge to location and date", definition: "", comment_usage_note: "", status: "" },
        "DLN" => V2TableRow { value: "DLN", display_name: "Driver's license number", definition: "", comment_usage_note: "", status: "" },
        "DLT" => V2TableRow { value: "DLT", display_name: "Delta", definition: "", comment_usage_note: "", status: "" },
        "DR" => V2TableRow { value: "DR", display_name: "Date/time range", definition: "", comment_usage_note: "", status: "" },
        "DT" => V2TableRow { value: "DT", display_name: "Date", definition: "", comment_usage_note: "", status: "" },
        "DTM" => V2TableRow { value: "DTM", display_name: "Date/time", definition: "", comment_usage_note: "", status: "" },
        "DTN" => V2TableRow { value: "DTN", display_name: "Day type and number", definition: "", comment_usage_note: "", status: "" },
        "ED" => V2TableRow { value: "ED", display_name: "Encapsulated data", definition: "", comment_usage_note: "", status: "" },
        "EI" => V2TableRow { value: "EI", display_name: "Entity identifier", definition: "", comment_usage_note: "", status: "" },
        "EIP" => V2TableRow { value: "EIP", display_name: "Entity identifier pair", definition: "", comment_usage_note: "", status: "" },
        "ERL" => V2TableRow { value: "ERL", display_name: "Error location", definition: "", comment_usage_note: "", status: "" },
        "FC" => V2TableRow { value: "FC", display_name: "Financial class", definition: "Not", comment_usage_note: "e", status: "" },
        "FT" => V2TableRow { value: "FT", display_name: "Formatted text", definition: "", comment_usage_note: "", status: "" },
        "GTS" => V2TableRow { value: "GTS", display_name: "General timing specification", definition: "", comment_usage_note: "", status: "" },
        "HD" => V2TableRow { value: "HD", display_name: "Hierarchic designator", definition: "", comment_usage_note: "", status: "" },
        "ICD" => V2TableRow { value: "ICD", display_name: "Insurance certification definition", definition: "", comment_usage_note: "", status: "" },
        "IS" => V2TableRow { value: "IS", display_name: "Coded value for user-defined tables", definition: "Thi bee bac com onl", comment_usage_note: "s code has B n marked for kwards patibility use y as of V2.9.", status: "" },
        "JCC" => V2TableRow { value: "JCC", display_name: "Job code/class", definition: "", comment_usage_note: "", status: "" },
        "LA1" => V2TableRow { value: "LA1", display_name: "Location with address variation 1", definition: "Dat bee fro Sta", comment_usage_note: "atype has B n Withdrawn m th e ndard.", status: "" },
        "LA2" => V2TableRow { value: "LA2", display_name: "Location with address variation 2", definition: "Dat bee fro Sta", comment_usage_note: "atype has B n Withdrawn m the ndard.", status: "" },
        "MA" => V2TableRow { value: "MA", display_name: "Multiplexed array", definition: "", comment_usage_note: "", status: "" },
        "MO" => V2TableRow { value: "MO", display_name: "Money", definition: "", comment_usage_note: "", status: "" },
        "MOC" => V2TableRow { value: "MOC", display_name: "Money and charge code", definition: "", comment_usage_note: "", status: "" },
        "MOP" => V2TableRow { value: "MOP", display_name: "Money or percentage", definition: "", comment_usage_note: "", status: "" },
        "MSG" => V2TableRow { value: "MSG", display_name: "Message type", definition: "", comment_usage_note: "", status: "" },
        "NA" => V2TableRow { value: "NA", display_name: "Numeric array", definition: "", comment_usage_note: "", status: "" },
        "NDL" => V2TableRow { value: "NDL", display_name: "Name with date and location", definition: "", comment_usage_note: "", status: "" },
        "NM" => V2TableRow { value: "NM", display_name: "Numeric", definition: "", comment_usage_note: "", status: "" },
        "NR" => V2TableRow { value: "NR", display_name: "Numeric range", definition: "", comment_usage_note: "", status: "" },
        "OCD" => V2TableRow { value: "OCD", display_name: "Occurrence code and date", definition: "", comment_usage_note: "", status: "" },
        "OSP" => V2TableRow { value: "OSP", display_name: "Occurrence span code and date", definition: "", comment_usage_note: "", status: "" },
        "PIP" => V2TableRow { value: "PIP", display_name: "Practitioner institutional privileges", definition: "", comment_usage_note: "", status: "" },
        "PL" => V2TableRow { value: "PL", display_name: "Person location", definition: "", comment_usage_note: "", status: "" },
        "PLN" => V2TableRow { value: "PLN", display_name: "Practitioner license or other ID number", definition: "", comment_usage_note: "", status: "" },
        "PPN" => V2TableRow { value: "PPN", display_name: "Performing person time stamp", definition: "", comment_usage_note: "", status: "" },
        "PRL" => V2TableRow { value: "PRL", display_name: "Parent result link", definition: "", comment_usage_note: "", status: "" },
        "PT" => V2TableRow { value: "PT", display_name: "Processing type", definition: "", comment_usage_note: "", status: "" },
        "PTA" => V2TableRow { value: "PTA", display_name: "Policy type and amount", definition: "", comment_usage_note: "", status: "" },
        "QIP" => V2TableRow { value: "QIP", display_name: "Query input parameter list", definition: "", comment_usage_note: "", status: "" },
        "QSC" => V2TableRow { value: "QSC", display_name: "Query selection criteria", definition: "", comment_usage_note: "", status: "" },
        "RCD" => V2TableRow { value: "RCD", display_name: "Row column definition", definition: "", comment_usage_note: "", status: "" },
        "RF" => V2TableRow { value: "RF", display_name: "R Reference range", definition: "", comment_usage_note: "", status: "" },
        "RI" => V2TableRow { value: "RI", display_name: "Repeat interval", definition: "", comment_usage_note: "", status: "" },
        "RMC" => V2TableRow { value: "RMC", display_name: "Room coverage", definition: "", comment_usage_note: "", status: "" },
        "RP" => V2TableRow { value: "RP", display_name: "Reference pointer", definition: "", comment_usage_note: "", status: "" },
        "RPT" => V2TableRow { value: "RPT", display_name: "Repeat pattern", definition: "", comment_usage_note: "", status: "" },
        "SCV" => V2TableRow { value: "SCV", display_name: "Scheduling class value pair", definition: "", comment_usage_note: "", status: "" },
        "SN" => V2TableRow { value: "SN", display_name: "Structured numeric", definition: "", comment_usage_note: "", status: "" },
        "SNM" => V2TableRow { value: "SNM", display_name: "String of telephone number digits", definition: "", comment_usage_note: "", status: "" },
        "SPD" => V2TableRow { value: "SPD", display_name: "Specialty description", definition: "", comment_usage_note: "", status: "" },
        "SRT" => V2TableRow { value: "SRT", display_name: "Sort order", definition: "", comment_usage_note: "", status: "" },
        "ST" => V2TableRow { value: "ST", display_name: "String data", definition: "", comment_usage_note: "", status: "" },
        "TM" => V2TableRow { value: "TM", display_name: "Time", definition: "", comment_usage_note: "", status: "" },
        "TX" => V2TableRow { value: "TX", display_name: "Text data", definition: "", comment_usage_note: "", status: "" },
        "UVC" => V2TableRow { value: "UVC", display_name: "UB value code and amount", definition: "", comment_usage_note: "", status: "" },
        "VH" => V2TableRow { value: "VH", display_name: "Visiting hours", definition: "", comment_usage_note: "", status: "" },
        "VID" => V2TableRow { value: "VID", display_name: "Version identifier", definition: "", comment_usage_note: "", status: "" },
        "VR" => V2TableRow { value: "VR", display_name: "Value range", definition: "", comment_usage_note: "", status: "" },
        "WVI" => V2TableRow { value: "WVI", display_name: "Channel Identifier", definition: "", comment_usage_note: "", status: "" },
        "WVS" => V2TableRow { value: "WVS", display_name: "Waveform source", definition: "", comment_usage_note: "", status: "" },
        "XAD" => V2TableRow { value: "XAD", display_name: "Extended address", definition: "", comment_usage_note: "", status: "" },
        "XCN" => V2TableRow { value: "XCN", display_name: "Extended composite ID number and name for persons", definition: "", comment_usage_note: "", status: "" },
        "XON" => V2TableRow { value: "XON", display_name: "Extended composite name and ID number for organizations", definition: "", comment_usage_note: "", status: "" },
        "XPN" => V2TableRow { value: "XPN", display_name: "Extended person name", definition: "", comment_usage_note: "", status: "" },
        "XTN" => V2TableRow { value: "XTN", display_name: "Extended telecommunications number", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0126: V2Table = V2Table {
    number: 126,
    metadata: &super::metadata::TABLE_0126_METADATA,
    rows: phf_map! {
        "CH" => V2TableRow { value: "CH", display_name: "Characters", definition: "", comment_usage_note: "RSP/RTB/RD Y", status: "" },
        "LI" => V2TableRow { value: "LI", display_name: "Lines", definition: "", comment_usage_note: "RTB/ RDY", status: "" },
        "PG" => V2TableRow { value: "PG", display_name: "Pages", definition: "", comment_usage_note: "RDY", status: "" },
        "RD" => V2TableRow { value: "RD", display_name: "Records", definition: "", comment_usage_note: "RSP/ RT B/R DY", status: "" },
        "ZO" => V2TableRow { value: "ZO", display_name: "Locally defined", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0127: V2Table = V2Table {
    number: 127,
    metadata: &super::metadata::TABLE_0127_METADATA,
    rows: phf_map! {
        "DA" => V2TableRow { value: "DA", display_name: "Drug allergy", definition: "", comment_usage_note: "", status: "" },
        "FA" => V2TableRow { value: "FA", display_name: "Food allergy", definition: "", comment_usage_note: "", status: "" },
        "MA" => V2TableRow { value: "MA", display_name: "Miscellaneous allergy", definition: "", comment_usage_note: "", status: "" },
        "MC" => V2TableRow { value: "MC", display_name: "Miscellaneous contraindication", definition: "", comment_usage_note: "", status: "" },
        "EA" => V2TableRow { value: "EA", display_name: "Environmental Allergy", definition: "", comment_usage_note: "", status: "" },
        "AA" => V2TableRow { value: "AA", display_name: "Animal Allergy", definition: "", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Plant Allergy", definition: "", comment_usage_note: "", status: "" },
        "LA" => V2TableRow { value: "LA", display_name: "Pollen Allergy", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0128: V2Table = V2Table {
    number: 128,
    metadata: &super::metadata::TABLE_0128_METADATA,
    rows: phf_map! {
        "SV" => V2TableRow { value: "SV", display_name: "Severe", definition: "", comment_usage_note: "", status: "" },
        "MO" => V2TableRow { value: "MO", display_name: "Moderate", definition: "", comment_usage_note: "", status: "" },
        "MI" => V2TableRow { value: "MI", display_name: "Mild", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0130: V2Table = V2Table {
    number: 130,
    metadata: &super::metadata::TABLE_0130_METADATA,
    rows: phf_map! {
        "TE" => V2TableRow { value: "TE", display_name: "Teaching", definition: "", comment_usage_note: "", status: "" },
        "HO" => V2TableRow { value: "HO", display_name: "Home", definition: "", comment_usage_note: "", status: "" },
        "MO" => V2TableRow { value: "MO", display_name: "Mobile Unit", definition: "", comment_usage_note: "", status: "" },
        "PH" => V2TableRow { value: "PH", display_name: "Phone", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0131: V2Table = V2Table {
    number: 131,
    metadata: &super::metadata::TABLE_0131_METADATA,
    rows: phf_map! {
        "E" => V2TableRow { value: "E", display_name: "Employer", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Emergency Contact", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Federal Agency", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Insurance Company", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Next-of-Kin", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "State Agency", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0135: V2Table = V2Table {
    number: 135,
    metadata: &super::metadata::TABLE_0135_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Yes", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Modified assignment", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0136: V2Table = V2Table {
    number: 136,
    metadata: &super::metadata::TABLE_0136_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Yes", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0137: V2Table = V2Table {
    number: 137,
    metadata: &super::metadata::TABLE_0137_METADATA,
    rows: phf_map! {
        "E" => V2TableRow { value: "E", display_name: "Employer", definition: "", comment_usage_note: "", status: "" },
        "G" => V2TableRow { value: "G", display_name: "Guarantor", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Insurance company", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Patient", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0140: V2Table = V2Table {
    number: 140,
    metadata: &super::metadata::TABLE_0140_METADATA,
    rows: phf_map! {
        "USA" => V2TableRow { value: "USA", display_name: "US Army", definition: "", comment_usage_note: "", status: "" },
        "USN" => V2TableRow { value: "USN", display_name: "US Navy", definition: "", comment_usage_note: "", status: "" },
        "USAF" => V2TableRow { value: "USAF", display_name: "US Air Force", definition: "", comment_usage_note: "", status: "" },
        "USMC" => V2TableRow { value: "USMC", display_name: "US Marine Corps", definition: "", comment_usage_note: "", status: "" },
        "USCG" => V2TableRow { value: "USCG", display_name: "US Coast Guard", definition: "", comment_usage_note: "", status: "" },
        "USPHS" => V2TableRow { value: "USPHS", display_name: "US Public Health Service", definition: "", comment_usage_note: "", status: "" },
        "NOAA" => V2TableRow { value: "NOAA", display_name: "National Oceanic and Atmospheric Administration", definition: "", comment_usage_note: "", status: "" },
        "NATO" => V2TableRow { value: "NATO", display_name: "North Atlantic Treaty Organization", definition: "", comment_usage_note: "", status: "" },
        "AUSA" => V2TableRow { value: "AUSA", display_name: "Australian Army", definition: "", comment_usage_note: "", status: "" },
        "AUSN" => V2TableRow { value: "AUSN", display_name: "Australian Navy", definition: "", comment_usage_note: "", status: "" },
        "AUSAF" => V2TableRow { value: "AUSAF", display_name: "Australian Air Force", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0142: V2Table = V2Table {
    number: 142,
    metadata: &super::metadata::TABLE_0142_METADATA,
    rows: phf_map! {
        "ACT" => V2TableRow { value: "ACT", display_name: "Active duty", definition: "", comment_usage_note: "", status: "" },
        "RET" => V2TableRow { value: "RET", display_name: "Retired", definition: "", comment_usage_note: "", status: "" },
        "DEC" => V2TableRow { value: "DEC", display_name: "Deceased", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0144: V2Table = V2Table {
    number: 144,
    metadata: &super::metadata::TABLE_0144_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Insurance company", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Employer", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Insured presented policy", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Insured presented card", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Signed statement on file", definition: "", comment_usage_note: "", status: "" },
        "6" => V2TableRow { value: "6", display_name: "Verbal information", definition: "", comment_usage_note: "", status: "" },
        "7" => V2TableRow { value: "7", display_name: "None", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0145: V2Table = V2Table {
    number: 145,
    metadata: &super::metadata::TABLE_0145_METADATA,
    rows: phf_map! {
        "PRI" => V2TableRow { value: "PRI", display_name: "Private room", definition: "", comment_usage_note: "", status: "" },
        "2PRI" => V2TableRow { value: "2PRI", display_name: "Second private room", definition: "", comment_usage_note: "", status: "" },
        "SPR" => V2TableRow { value: "SPR", display_name: "Semi-private room", definition: "", comment_usage_note: "", status: "" },
        "2SPR" => V2TableRow { value: "2SPR", display_name: "Second semi-private room", definition: "", comment_usage_note: "", status: "" },
        "ICU" => V2TableRow { value: "ICU", display_name: "Intensive care unit", definition: "", comment_usage_note: "", status: "" },
        "2ICU" => V2TableRow { value: "2ICU", display_name: "Second intensive care unit", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0146: V2Table = V2Table {
    number: 146,
    metadata: &super::metadata::TABLE_0146_METADATA,
    rows: phf_map! {
        "DF" => V2TableRow { value: "DF", display_name: "Differential", definition: "", comment_usage_note: "", status: "" },
        "LM" => V2TableRow { value: "LM", display_name: "Limit", definition: "", comment_usage_note: "", status: "" },
        "PC" => V2TableRow { value: "PC", display_name: "Percentage", definition: "", comment_usage_note: "Retained for backward compatibility only as of v 2.5", status: "" },
        "RT" => V2TableRow { value: "RT", display_name: "Rate", definition: "", comment_usage_note: "", status: "" },
        "UL" => V2TableRow { value: "UL", display_name: "Unlimited", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0147: V2Table = V2Table {
    number: 147,
    metadata: &super::metadata::TABLE_0147_METADATA,
    rows: phf_map! {
        "ANC" => V2TableRow { value: "ANC", display_name: "Ancillary", definition: "", comment_usage_note: "", status: "" },
        "2ANC" => V2TableRow { value: "2ANC", display_name: "Second ancillary", definition: "", comment_usage_note: "", status: "" },
        "MMD" => V2TableRow { value: "MMD", display_name: "Major medical", definition: "", comment_usage_note: "", status: "" },
        "2MMD" => V2TableRow { value: "2MMD", display_name: "Second major medical", definition: "", comment_usage_note: "", status: "" },
        "3MMD" => V2TableRow { value: "3MMD", display_name: "Third major medical", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0148: V2Table = V2Table {
    number: 148,
    metadata: &super::metadata::TABLE_0148_METADATA,
    rows: phf_map! {
        "AT" => V2TableRow { value: "AT", display_name: "Currency amount", definition: "", comment_usage_note: "", status: "" },
        "PC" => V2TableRow { value: "PC", display_name: "Percentage", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0149: V2Table = V2Table {
    number: 149,
    metadata: &super::metadata::TABLE_0149_METADATA,
    rows: phf_map! {
        "AP" => V2TableRow { value: "AP", display_name: "Approved", definition: "", comment_usage_note: "", status: "" },
        "DE" => V2TableRow { value: "DE", display_name: "Denied", definition: "", comment_usage_note: "", status: "" },
        "PE" => V2TableRow { value: "PE", display_name: "Pending", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0150: V2Table = V2Table {
    number: 150,
    metadata: &super::metadata::TABLE_0150_METADATA,
    rows: phf_map! {
        "ER" => V2TableRow { value: "ER", display_name: "Emergency", definition: "", comment_usage_note: "", status: "" },
        "IPE" => V2TableRow { value: "IPE", display_name: "Inpatient elective", definition: "", comment_usage_note: "", status: "" },
        "OPE" => V2TableRow { value: "OPE", display_name: "Outpatient elective", definition: "", comment_usage_note: "", status: "" },
        "UR" => V2TableRow { value: "UR", display_name: "Urgent", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0155: V2Table = V2Table {
    number: 155,
    metadata: &super::metadata::TABLE_0155_METADATA,
    rows: phf_map! {
        "AL" => V2TableRow { value: "AL", display_name: "Always", definition: "", comment_usage_note: "", status: "" },
        "NE" => V2TableRow { value: "NE", display_name: "Never", definition: "", comment_usage_note: "", status: "" },
        "ER" => V2TableRow { value: "ER", display_name: "Error/reject conditions only", definition: "", comment_usage_note: "", status: "" },
        "SU" => V2TableRow { value: "SU", display_name: "Successful completion only", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0159: V2Table = V2Table {
    number: 159,
    metadata: &super::metadata::TABLE_0159_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Diet", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Supplement", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Preference", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0160: V2Table = V2Table {
    number: 160,
    metadata: &super::metadata::TABLE_0160_METADATA,
    rows: phf_map! {
        "EARLY" => V2TableRow { value: "EARLY", display_name: "Early tray", definition: "", comment_usage_note: "", status: "" },
        "LATE" => V2TableRow { value: "LATE", display_name: "Late tray", definition: "", comment_usage_note: "", status: "" },
        "GUEST" => V2TableRow { value: "GUEST", display_name: "Guest tray", definition: "", comment_usage_note: "", status: "" },
        "NO" => V2TableRow { value: "NO", display_name: "No tray", definition: "", comment_usage_note: "", status: "" },
        "MSG" => V2TableRow { value: "MSG", display_name: "Tray message only", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0161: V2Table = V2Table {
    number: 161,
    metadata: &super::metadata::TABLE_0161_METADATA,
    rows: phf_map! {
        "N" => V2TableRow { value: "N", display_name: "Substitutions are NOT authorized. (This is the default - null.)", definition: "", comment_usage_note: "", status: "" },
        "G" => V2TableRow { value: "G", display_name: "Allow generic substitutions.", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Allow therapeutic substitutions", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0162: V2Table = V2Table {
    number: 162,
    metadata: &super::metadata::TABLE_0162_METADATA,
    rows: phf_map! {
        "AP" => V2TableRow { value: "AP", display_name: "Apply Externally", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Buccal", definition: "", comment_usage_note: "", status: "" },
        "DT" => V2TableRow { value: "DT", display_name: "Dental", definition: "", comment_usage_note: "", status: "" },
        "EP" => V2TableRow { value: "EP", display_name: "Epidural", definition: "", comment_usage_note: "", status: "" },
        "ET" => V2TableRow { value: "ET", display_name: "Endotrachial Tube", definition: "Endotrachial Tube*", comment_usage_note: "used primarily for respiratory therapy and anesthesia delivery", status: "" },
        "GTT" => V2TableRow { value: "GTT", display_name: "Gastrostomy Tube", definition: "", comment_usage_note: "", status: "" },
        "GU" => V2TableRow { value: "GU", display_name: "GU Irrigant", definition: "", comment_usage_note: "", status: "" },
        "IMR" => V2TableRow { value: "IMR", display_name: "Immerse (Soak) Body Part", definition: "", comment_usage_note: "", status: "" },
        "IA" => V2TableRow { value: "IA", display_name: "Intra-arterial", definition: "", comment_usage_note: "", status: "" },
        "IB" => V2TableRow { value: "IB", display_name: "Intrabursal", definition: "", comment_usage_note: "", status: "" },
        "IC" => V2TableRow { value: "IC", display_name: "Intracardiac", definition: "", comment_usage_note: "", status: "" },
        "ICV" => V2TableRow { value: "ICV", display_name: "Intracervical (uterus)", definition: "", comment_usage_note: "", status: "" },
        "ID" => V2TableRow { value: "ID", display_name: "Intradermal", definition: "", comment_usage_note: "", status: "" },
        "IH" => V2TableRow { value: "IH", display_name: "Inhalation", definition: "", comment_usage_note: "", status: "" },
        "IHA" => V2TableRow { value: "IHA", display_name: "Intrahepatic Artery", definition: "", comment_usage_note: "", status: "" },
        "IM" => V2TableRow { value: "IM", display_name: "Intramuscular", definition: "", comment_usage_note: "", status: "" },
        "IN" => V2TableRow { value: "IN", display_name: "Intranasal", definition: "", comment_usage_note: "", status: "" },
        "IO" => V2TableRow { value: "IO", display_name: "Intraocular", definition: "", comment_usage_note: "", status: "" },
        "IP" => V2TableRow { value: "IP", display_name: "Intraperitoneal", definition: "", comment_usage_note: "", status: "" },
        "IS" => V2TableRow { value: "IS", display_name: "Intrasynovial", definition: "", comment_usage_note: "", status: "" },
        "IT" => V2TableRow { value: "IT", display_name: "Intrathecal", definition: "", comment_usage_note: "", status: "" },
        "IU" => V2TableRow { value: "IU", display_name: "Intrauterine", definition: "", comment_usage_note: "", status: "" },
        "IV" => V2TableRow { value: "IV", display_name: "Intravenous", definition: "", comment_usage_note: "", status: "" },
        "MTH" => V2TableRow { value: "MTH", display_name: "Mouth/Throat", definition: "", comment_usage_note: "", status: "" },
        "MM" => V2TableRow { value: "MM", display_name: "Mucous Membrane", definition: "", comment_usage_note: "", status: "" },
        "NS" => V2TableRow { value: "NS", display_name: "Nasal", definition: "", comment_usage_note: "", status: "" },
        "NG" => V2TableRow { value: "NG", display_name: "Nasogastric", definition: "", comment_usage_note: "", status: "" },
        "NP" => V2TableRow { value: "NP", display_name: "Nasal Prongs Nasal Prongs*", definition: "used primarily for respiratory and anesthesia delivery", comment_usage_note: "therapy", status: "" },
        "NT" => V2TableRow { value: "NT", display_name: "Nasotrachial Tube", definition: "", comment_usage_note: "", status: "" },
        "OP" => V2TableRow { value: "OP", display_name: "Ophthalmic", definition: "", comment_usage_note: "", status: "" },
        "OT" => V2TableRow { value: "OT", display_name: "Otic", definition: "", comment_usage_note: "", status: "" },
        "OTH" => V2TableRow { value: "OTH", display_name: "Other/Miscellane ous", definition: "", comment_usage_note: "", status: "" },
        "PF" => V2TableRow { value: "PF", display_name: "Perfusion", definition: "", comment_usage_note: "", status: "" },
        "PO" => V2TableRow { value: "PO", display_name: "Oral", definition: "", comment_usage_note: "", status: "" },
        "PR" => V2TableRow { value: "PR", display_name: "Rectal", definition: "", comment_usage_note: "", status: "" },
        "RM" => V2TableRow { value: "RM", display_name: "Rebreather Mask Rebreather Mask*", definition: "used primarily for respiratory and anesthesia delivery", comment_usage_note: "therapy", status: "" },
        "SD" => V2TableRow { value: "SD", display_name: "Soaked Dressing", definition: "", comment_usage_note: "", status: "" },
        "SC" => V2TableRow { value: "SC", display_name: "Subcutaneous", definition: "", comment_usage_note: "", status: "" },
        "SL" => V2TableRow { value: "SL", display_name: "Sublingual", definition: "", comment_usage_note: "", status: "" },
        "TP" => V2TableRow { value: "TP", display_name: "Topical", definition: "", comment_usage_note: "", status: "" },
        "TRA" => V2TableRow { value: "TRA", display_name: "Tracheostomy Tracheostomy*", definition: "used primarily for respiratory and anesthesia delivery", comment_usage_note: "therapy", status: "" },
        "TD" => V2TableRow { value: "TD", display_name: "Transdermal", definition: "", comment_usage_note: "", status: "" },
        "TL" => V2TableRow { value: "TL", display_name: "Translingual", definition: "", comment_usage_note: "", status: "" },
        "UR" => V2TableRow { value: "UR", display_name: "Urethral", definition: "", comment_usage_note: "", status: "" },
        "VG" => V2TableRow { value: "VG", display_name: "Vaginal", definition: "", comment_usage_note: "", status: "" },
        "VM" => V2TableRow { value: "VM", display_name: "Ventimask", definition: "", comment_usage_note: "", status: "" },
        "WND" => V2TableRow { value: "WND", display_name: "Wound", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0163: V2Table = V2Table {
    number: 163,
    metadata: &super::metadata::TABLE_0163_METADATA,
    rows: phf_map! {
        "BE" => V2TableRow { value: "BE", display_name: "Bilateral Ears", definition: "", comment_usage_note: "", status: "" },
        "OU" => V2TableRow { value: "OU", display_name: "Bilateral Eyes", definition: "", comment_usage_note: "", status: "" },
        "BN" => V2TableRow { value: "BN", display_name: "Bilateral Nares", definition: "", comment_usage_note: "", status: "" },
        "BU" => V2TableRow { value: "BU", display_name: "Buttock", definition: "", comment_usage_note: "", status: "" },
        "CT" => V2TableRow { value: "CT", display_name: "Chest Tube", definition: "", comment_usage_note: "", status: "" },
        "LA" => V2TableRow { value: "LA", display_name: "Left Arm", definition: "", comment_usage_note: "", status: "" },
        "LAC" => V2TableRow { value: "LAC", display_name: "Left Anterior Chest", definition: "", comment_usage_note: "", status: "" },
        "LACF" => V2TableRow { value: "LACF", display_name: "Left Antecubital Fossa", definition: "", comment_usage_note: "", status: "" },
        "LD" => V2TableRow { value: "LD", display_name: "Left Deltoid", definition: "", comment_usage_note: "", status: "" },
        "LE" => V2TableRow { value: "LE", display_name: "Left Ear", definition: "", comment_usage_note: "", status: "" },
        "LEJ" => V2TableRow { value: "LEJ", display_name: "Left External Jugular", definition: "", comment_usage_note: "", status: "" },
        "OS" => V2TableRow { value: "OS", display_name: "Left Eye", definition: "", comment_usage_note: "", status: "" },
        "LF" => V2TableRow { value: "LF", display_name: "Left Foot", definition: "", comment_usage_note: "", status: "" },
        "LG" => V2TableRow { value: "LG", display_name: "Left Gluteus Medius", definition: "", comment_usage_note: "", status: "" },
        "LH" => V2TableRow { value: "LH", display_name: "Left Hand", definition: "", comment_usage_note: "", status: "" },
        "LIJ" => V2TableRow { value: "LIJ", display_name: "Left Internal Jugular", definition: "", comment_usage_note: "", status: "" },
        "LLAQ" => V2TableRow { value: "LLAQ", display_name: "Left Lower Abd Quadrant", definition: "", comment_usage_note: "", status: "" },
        "LLFA" => V2TableRow { value: "LLFA", display_name: "Left Lower Forearm", definition: "", comment_usage_note: "", status: "" },
        "LMFA" => V2TableRow { value: "LMFA", display_name: "Left Mid Forearm", definition: "", comment_usage_note: "", status: "" },
        "LN" => V2TableRow { value: "LN", display_name: "Left Naris", definition: "", comment_usage_note: "", status: "" },
        "LPC" => V2TableRow { value: "LPC", display_name: "Left Posterior Chest", definition: "", comment_usage_note: "", status: "" },
        "LSC" => V2TableRow { value: "LSC", display_name: "Left Subclavian", definition: "", comment_usage_note: "", status: "" },
        "LT" => V2TableRow { value: "LT", display_name: "Left Thigh", definition: "", comment_usage_note: "", status: "" },
        "LUA" => V2TableRow { value: "LUA", display_name: "Left Upper Arm", definition: "", comment_usage_note: "", status: "" },
        "LUAQ" => V2TableRow { value: "LUAQ", display_name: "Left Upper Abd Quadrant", definition: "", comment_usage_note: "", status: "" },
        "LUFA" => V2TableRow { value: "LUFA", display_name: "Left Upper Forearm", definition: "", comment_usage_note: "", status: "" },
        "LVG" => V2TableRow { value: "LVG", display_name: "Left Ventragluteal", definition: "", comment_usage_note: "", status: "" },
        "LVL" => V2TableRow { value: "LVL", display_name: "Left Vastus Lateralis", definition: "", comment_usage_note: "", status: "" },
        "NB" => V2TableRow { value: "NB", display_name: "Nebulized", definition: "", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Perianal", definition: "", comment_usage_note: "", status: "" },
        "PERIN" => V2TableRow { value: "PERIN", display_name: "Perineal", definition: "", comment_usage_note: "", status: "" },
        "RA" => V2TableRow { value: "RA", display_name: "Right Arm", definition: "", comment_usage_note: "", status: "" },
        "RAC" => V2TableRow { value: "RAC", display_name: "Right Anterior Chest", definition: "", comment_usage_note: "", status: "" },
        "RACF" => V2TableRow { value: "RACF", display_name: "Right Antecubital Fossa", definition: "", comment_usage_note: "", status: "" },
        "RD" => V2TableRow { value: "RD", display_name: "Right Deltoid", definition: "", comment_usage_note: "", status: "" },
        "RE" => V2TableRow { value: "RE", display_name: "Right Ear", definition: "", comment_usage_note: "", status: "" },
        "REJ" => V2TableRow { value: "REJ", display_name: "Right External Jugular", definition: "", comment_usage_note: "", status: "" },
        "OD" => V2TableRow { value: "OD", display_name: "Right Eye", definition: "", comment_usage_note: "", status: "" },
        "RF" => V2TableRow { value: "RF", display_name: "Right Foot", definition: "", comment_usage_note: "", status: "" },
        "RG" => V2TableRow { value: "RG", display_name: "Right Gluteus Medius", definition: "", comment_usage_note: "", status: "" },
        "RH" => V2TableRow { value: "RH", display_name: "Right Hand", definition: "", comment_usage_note: "", status: "" },
        "RIJ" => V2TableRow { value: "RIJ", display_name: "Right Internal Jugular", definition: "", comment_usage_note: "", status: "" },
        "RLAQ" => V2TableRow { value: "RLAQ", display_name: "Rt Lower Abd Quadrant", definition: "", comment_usage_note: "", status: "" },
        "RLFA" => V2TableRow { value: "RLFA", display_name: "Right Lower Forearm", definition: "", comment_usage_note: "", status: "" },
        "RMFA" => V2TableRow { value: "RMFA", display_name: "Right Mid Forearm", definition: "", comment_usage_note: "", status: "" },
        "RN" => V2TableRow { value: "RN", display_name: "Right Naris", definition: "", comment_usage_note: "", status: "" },
        "RP" => V2TableRow { value: "RP", display_name: "C Right Posterior Chest", definition: "", comment_usage_note: "", status: "" },
        "RS" => V2TableRow { value: "RS", display_name: "C Right Subclavian", definition: "", comment_usage_note: "", status: "" },
        "RT" => V2TableRow { value: "RT", display_name: "Right Thigh", definition: "", comment_usage_note: "", status: "" },
        "RUA" => V2TableRow { value: "RUA", display_name: "Right Upper Arm", definition: "", comment_usage_note: "", status: "" },
        "RUAQ" => V2TableRow { value: "RUAQ", display_name: "Right Upper Abd Quadrant", definition: "", comment_usage_note: "", status: "" },
        "RUFA" => V2TableRow { value: "RUFA", display_name: "Right Upper Forearm", definition: "", comment_usage_note: "", status: "" },
        "RVL" => V2TableRow { value: "RVL", display_name: "Right Vastus Lateralis", definition: "", comment_usage_note: "", status: "" },
        "RVG" => V2TableRow { value: "RVG", display_name: "Right Ventragluteal", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0164: V2Table = V2Table {
    number: 164,
    metadata: &super::metadata::TABLE_0164_METADATA,
    rows: phf_map! {
        "AP" => V2TableRow { value: "AP", display_name: "Applicator", definition: "", comment_usage_note: "", status: "" },
        "BT" => V2TableRow { value: "BT", display_name: "Buretrol", definition: "", comment_usage_note: "", status: "" },
        "HL" => V2TableRow { value: "HL", display_name: "Heparin Lock", definition: "", comment_usage_note: "", status: "" },
        "IPPB" => V2TableRow { value: "IPPB", display_name: "IPPB", definition: "", comment_usage_note: "", status: "" },
        "IVP" => V2TableRow { value: "IVP", display_name: "IV Pump", definition: "", comment_usage_note: "", status: "" },
        "IVS" => V2TableRow { value: "IVS", display_name: "IV Soluset", definition: "", comment_usage_note: "", status: "" },
        "MI" => V2TableRow { value: "MI", display_name: "Metered Inhaler", definition: "", comment_usage_note: "", status: "" },
        "NEB" => V2TableRow { value: "NEB", display_name: "Nebulizer", definition: "", comment_usage_note: "", status: "" },
        "PCA" => V2TableRow { value: "PCA", display_name: "PCA Pump", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0165: V2Table = V2Table {
    number: 165,
    metadata: &super::metadata::TABLE_0165_METADATA,
    rows: phf_map! {
        "CH" => V2TableRow { value: "CH", display_name: "Chew", definition: "", comment_usage_note: "", status: "" },
        "DI" => V2TableRow { value: "DI", display_name: "Dissolve", definition: "", comment_usage_note: "", status: "" },
        "DU" => V2TableRow { value: "DU", display_name: "Dust", definition: "", comment_usage_note: "", status: "" },
        "IF" => V2TableRow { value: "IF", display_name: "Infiltrate", definition: "", comment_usage_note: "", status: "" },
        "IS" => V2TableRow { value: "IS", display_name: "Insert", definition: "", comment_usage_note: "", status: "" },
        "IR" => V2TableRow { value: "IR", display_name: "Irrigate", definition: "", comment_usage_note: "", status: "" },
        "IVPB" => V2TableRow { value: "IVPB", display_name: "IV Piggyback", definition: "", comment_usage_note: "", status: "" },
        "IVP" => V2TableRow { value: "IVP", display_name: "IV Push", definition: "", comment_usage_note: "", status: "" },
        "NB" => V2TableRow { value: "NB", display_name: "Nebulized", definition: "", comment_usage_note: "", status: "" },
        "PT" => V2TableRow { value: "PT", display_name: "Paint", definition: "", comment_usage_note: "", status: "" },
        "PF" => V2TableRow { value: "PF", display_name: "Perfuse", definition: "", comment_usage_note: "", status: "" },
        "SH" => V2TableRow { value: "SH", display_name: "Shampoo", definition: "", comment_usage_note: "", status: "" },
        "SO" => V2TableRow { value: "SO", display_name: "Soak", definition: "", comment_usage_note: "", status: "" },
        "WA" => V2TableRow { value: "WA", display_name: "Wash", definition: "", comment_usage_note: "", status: "" },
        "WI" => V2TableRow { value: "WI", display_name: "Wipe", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0166: V2Table = V2Table {
    number: 166,
    metadata: &super::metadata::TABLE_0166_METADATA,
    rows: phf_map! {
        "B" => V2TableRow { value: "B", display_name: "Base", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Additive", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0167: V2Table = V2Table {
    number: 167,
    metadata: &super::metadata::TABLE_0167_METADATA,
    rows: phf_map! {
        "N" => V2TableRow { value: "N", display_name: "No substitute was dispensed. This is equivalent to the default (null) value.", definition: "", comment_usage_note: "", status: "" },
        "G" => V2TableRow { value: "G", display_name: "A generic substitution was dispensed.", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "A therapeutic substitution was dispensed.", definition: "", comment_usage_note: "", status: "" },
        "0" => V2TableRow { value: "0", display_name: "No product selection indicated", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Substitution not allowed by prescriber", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Substitution allowed - patient requested product dispensed", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Substitution allowed - pharmacist selected product dispensed", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Substitution allowed - generic drug not in stock", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Substitution allowed - brand drug dispensed as a generic", definition: "", comment_usage_note: "", status: "" },
        "7" => V2TableRow { value: "7", display_name: "Substitution not allowed - brand drug mandated by law", definition: "", comment_usage_note: "", status: "" },
        "8" => V2TableRow { value: "8", display_name: "Substitution allowed - generic drug not available in marketplace", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0168: V2Table = V2Table {
    number: 168,
    metadata: &super::metadata::TABLE_0168_METADATA,
    rows: phf_map! {
        "S" => V2TableRow { value: "S", display_name: "Stat (do immediately)", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "As soon as possible (a priority lower than stat)", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Routine", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Preoperative (to be done prior to surgery)", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Timing critical (do as near as possible to requested time)", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Measure continuously (e.g., arterial line blood pressure)", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Do at bedside or portable (may be used with other codes)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0169: V2Table = V2Table {
    number: 169,
    metadata: &super::metadata::TABLE_0169_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "Call back results", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Rush reporting", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0170: V2Table = V2Table {
    number: 170,
    metadata: &super::metadata::TABLE_0170_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Parent Observation", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Child Observation", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Not Applicable", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0173: V2Table = V2Table {
    number: 173,
    metadata: &super::metadata::TABLE_0173_METADATA,
    rows: phf_map! {
        "CO" => V2TableRow { value: "CO", display_name: "Coordination", definition: "", comment_usage_note: "", status: "" },
        "IN" => V2TableRow { value: "IN", display_name: "Independent", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0174: V2Table = V2Table {
    number: 174,
    metadata: &super::metadata::TABLE_0174_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Profile or battery consisting of many independent atomic observations (e.g., SMA12, electrolytes), usually done at one instrument on one specimen", definition: "", comment_usage_note: "can be associated with one or more OM4 (specimen) segments See comment for value S", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Functional procedure that may consist of one or more interrelated measures (e.g., glucose tolerance test, creatinine clearance), usually done at different times and/or on different specimens", definition: "", comment_usage_note: "can be associated with one or more OM4 (specimen) segments See comment for value S", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Atomic service/test/observation (test code or treatment code)", definition: "", comment_usage_note: "can be associated with one or more OM4 (specimen) segments a single direct observation and would usually be associated with an OM2 and/or OM3 segments", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Superset-a set of batteries or procedures ordered under a single code unit but processed as separate batteries (e.g., routines = CBC, UA, electrolytes) This set indicates that the code being described is used to order multiple service/test/observation b", definition: "Superset— a set of batteries or procedures ordered under a single code unit but processed as separate batteries (e.g., routines = CBC, UA, electrolytes) This set indicates that the code being described is used to order multiple service/test/observation batteries. For example, a client who routinely orders a CBC, a differential, and a thyroxine as an outpatient profile might use a single, special code to order all three test batteries, instead of having to submit three separate order codes.", comment_usage_note: "can be associated with one or more OM4 (specimen) segments Codes P, F, and S identify sets (batteries) and should be associated with an OM5 segment that defines the list of elements. The definitions for the contained elements would have to be sent in other independent OMx segments, one for each contained element. In the ASTM context, most text reports – such as discharge summaries, admission H&Ps, and chest X-ray reports – are considered as sets, in which each section of the report (e.g., description, impression, and recommendation of an X-ray report) is considered a separate observation", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Single observation calculated via a rule or formula from other independent observations (e.g., Alveolar-arterial ratio, cardiac output)", definition: "", comment_usage_note: "can be associated with one or more OM4 (specimen) segments a derived quantity and would usually be associated with an OM6 segment", status: "" },
    },
};

pub static TABLE_0175: V2Table = V2Table {
    number: 175,
    metadata: &super::metadata::TABLE_0175_METADATA,
    rows: phf_map! {
        "CDM" => V2TableRow { value: "CDM", display_name: "Charge description master file", definition: "", comment_usage_note: "", status: "" },
        "CMA" => V2TableRow { value: "CMA", display_name: "Clinical study with phases and scheduled master file", definition: "", comment_usage_note: "", status: "" },
        "CMB" => V2TableRow { value: "CMB", display_name: "Clinical study without phases but with scheduled master file", definition: "", comment_usage_note: "", status: "" },
        "LOC" => V2TableRow { value: "LOC", display_name: "Location master file", definition: "", comment_usage_note: "", status: "" },
        "OMA" => V2TableRow { value: "OMA", display_name: "Numerical observation master file", definition: "", comment_usage_note: "", status: "" },
        "OMB" => V2TableRow { value: "OMB", display_name: "Categorical observation master file", definition: "", comment_usage_note: "", status: "" },
        "OMC" => V2TableRow { value: "OMC", display_name: "Observation batteries master file", definition: "", comment_usage_note: "", status: "" },
        "OMD" => V2TableRow { value: "OMD", display_name: "Calculated observations master file", definition: "", comment_usage_note: "", status: "" },
        "OMM" => V2TableRow { value: "OMM", display_name: "Mixed type observation master file", definition: "", comment_usage_note: "", status: "" },
        "PRA" => V2TableRow { value: "PRA", display_name: "Practitioner master file", definition: "", comment_usage_note: "", status: "" },
        "STF" => V2TableRow { value: "STF", display_name: "Staff master file", definition: "", comment_usage_note: "", status: "" },
        "CLN" => V2TableRow { value: "CLN", display_name: "Clinic master file", definition: "", comment_usage_note: "", status: "" },
        "OME" => V2TableRow { value: "OME", display_name: "Other Observation/Service Item master file", definition: "", comment_usage_note: "", status: "" },
        "INV" => V2TableRow { value: "INV", display_name: "Inventory master file", definition: "", comment_usage_note: "", status: "" },
        "MLCP" => V2TableRow { value: "MLCP", display_name: "Medicare Limited Coverage Process", definition: "", comment_usage_note: "This identifies Universal Service Identifiers that are not approved for a CPT code based on an ICD.", status: "" },
        "MACP" => V2TableRow { value: "MACP", display_name: "Medicare Approved Coverage Process", definition: "", comment_usage_note: "This identifies Universal Service Identifier that are approved for an ICD based on the CPT.", status: "" },
    },
};

pub static TABLE_0177: V2Table = V2Table {
    number: 177,
    metadata: &super::metadata::TABLE_0177_METADATA,
    rows: phf_map! {
        "V" => V2TableRow { value: "V", display_name: "Very restricted", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Restricted", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Usual control", definition: "", comment_usage_note: "", status: "" },
        "EMP" => V2TableRow { value: "EMP", display_name: "Employee", definition: "", comment_usage_note: "", status: "" },
        "UWM" => V2TableRow { value: "UWM", display_name: "Unwed mother", definition: "", comment_usage_note: "", status: "" },
        "VIP" => V2TableRow { value: "VIP", display_name: "Very important person or celebrity", definition: "", comment_usage_note: "", status: "" },
        "PSY" => V2TableRow { value: "PSY", display_name: "Psychiatric patient", definition: "", comment_usage_note: "", status: "" },
        "AID" => V2TableRow { value: "AID", display_name: "AIDS patient", definition: "", comment_usage_note: "", status: "" },
        "HIV" => V2TableRow { value: "HIV", display_name: "HIV(+) patient", definition: "", comment_usage_note: "", status: "" },
        "ETH" => V2TableRow { value: "ETH", display_name: "Alcohol/drug treatment patient", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0178: V2Table = V2Table {
    number: 178,
    metadata: &super::metadata::TABLE_0178_METADATA,
    rows: phf_map! {
        "REP" => V2TableRow { value: "REP", display_name: "Replace current version of this master file with the version contained in this message", definition: "", comment_usage_note: "", status: "" },
        "UPD" => V2TableRow { value: "UPD", display_name: "Change file records as defined in the record-level event codes for each record that follows", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0179: V2Table = V2Table {
    number: 179,
    metadata: &super::metadata::TABLE_0179_METADATA,
    rows: phf_map! {
        "NE" => V2TableRow { value: "NE", display_name: "Never. No application-level response needed", definition: "", comment_usage_note: "", status: "" },
        "ER" => V2TableRow { value: "ER", display_name: "Error/Reject conditions only. Only MFA segments denoting errors must be returned via the application - level acknowledgment for this message", definition: "", comment_usage_note: "", status: "" },
        "AL" => V2TableRow { value: "AL", display_name: "Always. All MFA segments (whether denoting errors or not) must be returned via the application- level acknowledgment message", definition: "", comment_usage_note: "", status: "" },
        "SU" => V2TableRow { value: "SU", display_name: "Success. Only MFA segments denoting success must be returned via the application- level acknowledgment for this message", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0180: V2Table = V2Table {
    number: 180,
    metadata: &super::metadata::TABLE_0180_METADATA,
    rows: phf_map! {
        "MAD" => V2TableRow { value: "MAD", display_name: "Add record to master file", definition: "", comment_usage_note: "", status: "" },
        "MDL" => V2TableRow { value: "MDL", display_name: "Delete record from master file", definition: "", comment_usage_note: "", status: "" },
        "MUP" => V2TableRow { value: "MUP", display_name: "Update record for master file", definition: "", comment_usage_note: "", status: "" },
        "MDC" => V2TableRow { value: "MDC", display_name: "Deactivate: discontinue using record in master file, but do not delete from database", definition: "", comment_usage_note: "", status: "" },
        "MAC" => V2TableRow { value: "MAC", display_name: "Reactivate deactivated record", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0181: V2Table = V2Table {
    number: 181,
    metadata: &super::metadata::TABLE_0181_METADATA,
    rows: phf_map! {
        "S" => V2TableRow { value: "S", display_name: "Successful posting of the record defined by the MFE segment", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unsuccessful posting of the record defined by the MFE segment", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0183: V2Table = V2Table {
    number: 183,
    metadata: &super::metadata::TABLE_0183_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Active Staff", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Inactive Staff", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0185: V2Table = V2Table {
    number: 185,
    metadata: &super::metadata::TABLE_0185_METADATA,
    rows: phf_map! {
        "B" => V2TableRow { value: "B", display_name: "Beeper Number", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Cellular Phone Number", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "E-Mail Address (for backward compatibility)", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "FAX Number", definition: "", comment_usage_note: "", status: "" },
        "H" => V2TableRow { value: "H", display_name: "Home Phone Number", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Office Phone Number", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0187: V2Table = V2Table {
    number: 187,
    metadata: &super::metadata::TABLE_0187_METADATA,
    rows: phf_map! {
        "I" => V2TableRow { value: "I", display_name: "Institution bills for provider", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Provider does own billing", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0189: V2Table = V2Table {
    number: 189,
    metadata: &super::metadata::TABLE_0189_METADATA,
    rows: phf_map! {
        "H" => V2TableRow { value: "H", display_name: "Hispanic or Latino", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Not Hispanic or Latino", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0190: V2Table = V2Table {
    number: 190,
    metadata: &super::metadata::TABLE_0190_METADATA,
    rows: phf_map! {
        "BA" => V2TableRow { value: "BA", display_name: "Bad address", definition: "", comment_usage_note: "Retained for backward compatibility only as of v2.6. Refer to XAD.17", status: "" },
        "BI" => V2TableRow { value: "BI", display_name: "Billing Address", definition: "", comment_usage_note: "May also be used for the validation/authorization of credit cards", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Birth (nee) (birth address, not otherwise specified)", definition: "", comment_usage_note: "Refers to the birth address, not otherwise specified", status: "" },
        "BDL" => V2TableRow { value: "BDL", display_name: "Birth delivery location (address where birth occurred)", definition: "", comment_usage_note: "Refers to the address where birth occurred.", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Country Of Origin", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Current Or Temporary", definition: "", comment_usage_note: "Retained for backward compatibility only as of v2.6. Refer to XAD.16", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Firm/Business", definition: "", comment_usage_note: "Refers to an address specific to an organization, such as an insurance company or employer, versus an individual’s work location or place of employment. It would be specific to a firm or organization that has some sort of business relationship with the subject", status: "" },
        "H" => V2TableRow { value: "H", display_name: "Home", definition: "", comment_usage_note: "Refers to a residence or domicile, literally the place where the subject resides the majority of the time. Generally speaking most people will have a home address and it will represent their primary address. Home address is mutually exclusive of permanent address.", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Legal Address", definition: "", comment_usage_note: "Refers to a special case address specific to the status of a subject or legal action involving the subject. For example, prisoners being treated at a healthcare facility may have home addresses, but their status mandates an address specific to their place of incarceration. Statutes may require the health information specific to a ward of the state be sent to a legal guardian, the courts, or a state or municipal agency regardless of the ward’s physical location. In cases involving civil or criminal proceedings, a record may be flagged such that all correspondence is sent to any variety of legal entities.", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Mailing", definition: "", comment_usage_note: "Retained for backward compatibility only as of v2.6. Refer to XAD.18", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Office/Business", definition: "", comment_usage_note: "Refers to a work address specific to the subject.", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Permanent", definition: "", comment_usage_note: "Refers to a place where the residents know the subject and where correspondence addressed to the subject will eventually reach the subject regardless of their physical location. A permanent address generally reflects a tax jurisdiction. Members of the military, flight attendants, and executives on rotational assignments are examples of those who typically maintain a permanent address. Although mutually exclusive of home address, in some instances, such as the executives mentioned above, it may be synonymous. In such cases upon return from assignment this address would revert to the home address.", status: "" },
        "RH" => V2TableRow { value: "RH", display_name: "Registry home. Refers to the information system, typically managed by a public health agency, that stores patient information such as immunization histories or cancer data, regardless of where the patient obtains services.", definition: "", comment_usage_note: "Refers to the information system, typically managed by a public health agency that stores patient information such as immunization histories or cancer data, regardless of where the patient obtains services", status: "" },
        "BR" => V2TableRow { value: "BR", display_name: "Residence at birth (home address at time of birth)", definition: "", comment_usage_note: "Refers to the home address at time of birth.", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Service Location", definition: "", comment_usage_note: "Refers to the location in which service is rendered. This would be used if reimbursement is based on the location of the service (to take into account the cost of those services).", status: "" },
        "SH" => V2TableRow { value: "SH", display_name: "Shipping Address", definition: "", comment_usage_note: "", status: "" },
        "TM" => V2TableRow { value: "TM", display_name: "Tube Address", definition: "", comment_usage_note: "Pneumatic tube address (to which letters may be sent). A special transport system to transport small samples/containers and/or normal mail in small carriages on rail or in a tube. (German Rohrpost)", status: "" },
        "V" => V2TableRow { value: "V", display_name: "Vacation", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0191: V2Table = V2Table {
    number: 191,
    metadata: &super::metadata::TABLE_0191_METADATA,
    rows: phf_map! {
        "AP" => V2TableRow { value: "AP", display_name: "Other application data, typically uninterpreted binary data (HL7 V2.3 and later)", definition: "", comment_usage_note: "", status: "" },
        "AU" => V2TableRow { value: "AU", display_name: "Audio data (HL7 V2.3 and later)", definition: "", comment_usage_note: "", status: "" },
        "FT" => V2TableRow { value: "FT", display_name: "Formatted text (HL7 V2.2 only)", definition: "", comment_usage_note: "", status: "" },
        "IM" => V2TableRow { value: "IM", display_name: "Image data (HL7 V2.3 and later)", definition: "", comment_usage_note: "", status: "" },
        "multipart" => V2TableRow { value: "multipart", display_name: "MIME multipart package", definition: "", comment_usage_note: "", status: "" },
        "NS" => V2TableRow { value: "NS", display_name: "Non-scanned image (HL7 V2.2 only)", definition: "", comment_usage_note: "", status: "" },
        "SD" => V2TableRow { value: "SD", display_name: "Scanned document (HL7 V2.2 only)", definition: "", comment_usage_note: "", status: "" },
        "SI" => V2TableRow { value: "SI", display_name: "Scanned image (HL7 V2.2 only)", definition: "", comment_usage_note: "", status: "" },
        "TEXT" => V2TableRow { value: "TEXT", display_name: "Machine readable text document (HL7 V2.3.1 and later)", definition: "", comment_usage_note: "", status: "" },
        "TX" => V2TableRow { value: "TX", display_name: "Machine readable text document (HL7 V2.2 only)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0193: V2Table = V2Table {
    number: 193,
    metadata: &super::metadata::TABLE_0193_METADATA,
    rows: phf_map! {
        "AT" => V2TableRow { value: "AT", display_name: "Amount", definition: "", comment_usage_note: "Retained for backward compatibility only as of v 2.5", status: "" },
        "LM" => V2TableRow { value: "LM", display_name: "Limit", definition: "", comment_usage_note: "", status: "" },
        "PC" => V2TableRow { value: "PC", display_name: "Percentage", definition: "", comment_usage_note: "Retained for backward compatibility only as of v 2.5", status: "" },
        "UL" => V2TableRow { value: "UL", display_name: "Unlimited", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0200: V2Table = V2Table {
    number: 200,
    metadata: &super::metadata::TABLE_0200_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Assigned", definition: "", comment_usage_note: "A name assigned to a person. Reasons some organizations assign alternate names may include not knowing the person's name, or to maintain anonymity. Some, but not necessarily all, of the name types that people call \"alias\" may fit into this category.", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Birth name", definition: "", comment_usage_note: "A name that a person had shortly after being born. Usually for family names but may be used to mark given names at birth that may have changed later. This is not for temporary names assigned at birth while a newborn is not yet named", status: "" },
        "BAD" => V2TableRow { value: "BAD", display_name: "Bad Name", definition: "", comment_usage_note: "A name that was wrongly used in the past and is now maintained only for the purposes of searching", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Adopted Name", definition: "", comment_usage_note: "A name acquired by adoption", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Customary Name", definition: "Known as/conventional also be known asa pre", comment_usage_note: "/the one you use. May ferred name.", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Fathers Name", definition: "Fathers Name ( Patron", comment_usage_note: "ymic Name)", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Licensing Name", definition: "", comment_usage_note: "", status: "" },
        "K" => V2TableRow { value: "K", display_name: "Business name", definition: "A name used in a Prof context. Also include artist’s name, stage An example of use is multiple proper names the particular names", comment_usage_note: "essional or Business s writer’s pseudonym, name, street name, etc. where a person with (i.e. married) uses one of in a professional", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Official Registry Name", definition: "The formal name as re (government) registry be commonly used. May For many people, cust official name", comment_usage_note: "gistered in an official , but which name might not correspond to legal name omary name is also their", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Maiden Name", definition: "A name you had just b", comment_usage_note: "efore you got married", status: "" },
        "MSK" => V2TableRow { value: "MSK", display_name: "Masked", definition: "There is information has not been provided security, privacy or an alternate mechanis information. Note: using this null information that may confidentiality, even provided. Its primary circumstances where i receiver that the inf providing any detail.", comment_usage_note: "on this item available but it by the sender due to other reasons. There may be m for gaining access to this flavor does provide be a breach of though no detail data is purpose is for those t is necessary to inform the ormation does exist without", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Nickname", definition: "Nickname /\"Call me\" N", comment_usage_note: "ame/Street Name", status: "" },
        "NAV" => V2TableRow { value: "NAV", display_name: "Temporarily Unavailable", definition: "Information is not av expected that it will Includes John or Jane", comment_usage_note: "ailable at this time but it is be available later. Doe situations", status: "" },
        "NB" => V2TableRow { value: "NB", display_name: "Newborn Name", definition: "A name assigned on a \"Baby of Smith\"", comment_usage_note: "temporary basis at birth. i.e.", status: "" },
        "NOUSE" => V2TableRow { value: "NOUSE", display_name: "No Longer To Be Used", definition: "Name not to be used a", comment_usage_note: "nymore for personal reasons", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Name of Partner/Spouse", definition: "Retained for backward v2.7.", comment_usage_note: "compatibility only as of", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Registered Name", definition: "(animals only) Retained for backward v2.7. Use \"L\" instead", comment_usage_note: "s compatibility only as of - has same meaning", status: "" },
        "REL" => V2TableRow { value: "REL", display_name: "Religious", definition: "e.g. Sister Mary Fran", comment_usage_note: "cis, Brother John", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Pseudonym", definition: "Coded Pseudo-Name to", comment_usage_note: "ensure anonymity", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Indigenous/Tribal", definition: "Indigenous/Tribal/Com Red Cloud", comment_usage_note: "munity Name e.g. Chief", status: "" },
        "TEMP" => V2TableRow { value: "TEMP", display_name: "Temporary Name", definition: "A temporary name. Not can provide more deta", comment_usage_note: "e that a name valid time iled information.", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "Unknown", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0201: V2Table = V2Table {
    number: 201,
    metadata: &super::metadata::TABLE_0201_METADATA,
    rows: phf_map! {
        "PRN" => V2TableRow { value: "PRN", display_name: "Primary Residence Number", definition: "", comment_usage_note: "", status: "" },
        "ORN" => V2TableRow { value: "ORN", display_name: "Other Residence Number", definition: "", comment_usage_note: "", status: "" },
        "WPN" => V2TableRow { value: "WPN", display_name: "Work Number", definition: "", comment_usage_note: "", status: "" },
        "VHN" => V2TableRow { value: "VHN", display_name: "Vacation Home Number", definition: "", comment_usage_note: "", status: "" },
        "ASN" => V2TableRow { value: "ASN", display_name: "Answering Service Number", definition: "", comment_usage_note: "", status: "" },
        "EMR" => V2TableRow { value: "EMR", display_name: "Emergency Number", definition: "", comment_usage_note: "", status: "" },
        "NET" => V2TableRow { value: "NET", display_name: "Network (email) Address", definition: "", comment_usage_note: "Retained for backward compatibility as of v 2.6", status: "" },
        "BPN" => V2TableRow { value: "BPN", display_name: "Beeper Number", definition: "", comment_usage_note: "Retained for backward compatibility as of v 2.6", status: "" },
        "PRS" => V2TableRow { value: "PRS", display_name: "Personal", definition: "", comment_usage_note: "Not tied to a location or role", status: "" },
    },
};

pub static TABLE_0202: V2Table = V2Table {
    number: 202,
    metadata: &super::metadata::TABLE_0202_METADATA,
    rows: phf_map! {
        "PH" => V2TableRow { value: "PH", display_name: "Telephone", definition: "", comment_usage_note: "", status: "" },
        "FX" => V2TableRow { value: "FX", display_name: "Fax", definition: "", comment_usage_note: "", status: "" },
        "MD" => V2TableRow { value: "MD", display_name: "Modem", definition: "", comment_usage_note: "", status: "" },
        "CP" => V2TableRow { value: "CP", display_name: "Cellular or Mobile Phone", definition: "", comment_usage_note: "", status: "" },
        "SAT" => V2TableRow { value: "SAT", display_name: "Satellite Phone", definition: "", comment_usage_note: "", status: "" },
        "BP" => V2TableRow { value: "BP", display_name: "Beeper", definition: "", comment_usage_note: "", status: "" },
        "Internet" => V2TableRow { value: "Internet", display_name: "Internet Address", definition: "", comment_usage_note: "", status: "" },
        "X.400" => V2TableRow { value: "X.400", display_name: "X.400 email address", definition: "", comment_usage_note: "", status: "" },
        "TDD" => V2TableRow { value: "TDD", display_name: "Telecommunications Device for the Deaf", definition: "", comment_usage_note: "", status: "" },
        "TTY" => V2TableRow { value: "TTY", display_name: "Teletypewriter", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0203: V2Table = V2Table {
    number: 203,
    metadata: &super::metadata::TABLE_0203_METADATA,
    rows: phf_map! {
        "AC" => V2TableRow { value: "AC", display_name: "Accreditation/Certifi cation Identifier", definition: "Identifier that has been assigned by an accreditation or certification organization in specific fields, indicating a recognized skill", comment_usage_note: "In Ask at Order Entry (AOE) questions this can be used to identify the ID with the assigning authority. For instance, a credentialed sonographer whose identifier assigned by the credentialing body has been entered can be properly labeled.", status: "N" },
        "ACSN" => V2TableRow { value: "ACSN", display_name: "Accession ID", definition: "Accession Identifier", comment_usage_note: "", status: "" },
        "AIN" => V2TableRow { value: "AIN", display_name: "Animal Identification Number (US Official)", definition: "A numbering system for the official identification of individual animals in the United States that provides a nationally unique identification number for each animal. The first two numbers on a tag are the numbers assigned to a specific State.", comment_usage_note: "AIN is the official acronym used by USDA", status: "N" },
        "AM" => V2TableRow { value: "AM", display_name: "American Express", definition: "", comment_usage_note: "Deprecated and replaced by BC in v 2.5.", status: "" },
        "AMA" => V2TableRow { value: "AMA", display_name: "American Medical Association Number", definition: "A physician identifier assigned by the AMA.", comment_usage_note: "", status: "" },
        "AN" => V2TableRow { value: "AN", display_name: "Account number", definition: "Account An identifier that is unique to an account.", comment_usage_note: "", status: "" },
        "ANC" => V2TableRow { value: "ANC", display_name: "Account number Creditor", definition: "A more precise definition of an account number", comment_usage_note: "Class: Financial Sometimes two distinct account numbers must be transmitted in the same message, one as the creditor, the other as the debitor. Kreditorenkontonumm er", status: "" },
        "AND" => V2TableRow { value: "AND", display_name: "Account number debitor", definition: "A more precise definition of an account number", comment_usage_note: "Class: Financial Sometimes two distinct account numbers must be transmitted in the same message, one as the creditor, the other as the debitor. Debitorenkontonumme r", status: "" },
        "ANON" => V2TableRow { value: "ANON", display_name: "Anonymous identifier", definition: "An identifier for a living subject whose real identity is protected or suppressed", comment_usage_note: "Justification: For public health reporting purposes, anonymous identifiers are occasionally used for protecting patient identity in reporting certain results. For instance, a state health department may choose to use a scheme for generating an anonymous identifier for reporting a patient that has had a positive human immunodeficiency virus antibody test. Anonymous identifiers can be used in PID 3 by replacing the medical record number or other non- anonymous identifier. The assigning authority for an anonymous identifier would be the state/local health department.", status: "" },
        "ANT" => V2TableRow { value: "ANT", display_name: "Temporary Account Number", definition: "Temporary version of an Account Number", comment_usage_note: "Class: Financial Use Case: An ancillary system that does not normally assign account numbers is the first time to register a patient. This ancillary system will generate a temporary account number that will only be used until an official account number is assigned.", status: "" },
        "APRN" => V2TableRow { value: "APRN", display_name: "Advanced An identi Practice practice Registered Nurse jurisdict number", definition: "fier that is unique to an advanced registered nurse within the ion of a certifying board", comment_usage_note: "", status: "" },
        "ASID" => V2TableRow { value: "ASID", display_name: "Ancestor A unique Specimen ID specimen.", definition: "identifier for the ancestor All child, grandchil etc. specimens of th ancestor specimen share the same Ancestor Specimen ID.", comment_usage_note: "d, e", status: "" },
        "BA" => V2TableRow { value: "BA", display_name: "Bank Account Number", definition: "Class: Financial", comment_usage_note: "", status: "" },
        "BC" => V2TableRow { value: "BC", display_name: "Bank Card An identi Number bank card", definition: "fier that is unique to a person's Class: Financial Replaces AM, DI, DS, MS, and VS beginning in v 2.5.", comment_usage_note: "", status: "" },
        "BCFN" => V2TableRow { value: "BCFN", display_name: "Birth Certificate The ident File Number jurisdict system as the recor certifica", definition: "ifier used within the ional vital records office file an auxiliary means of accessing d associated with the birth te.", comment_usage_note: "N", status: "" },
        "BCT" => V2TableRow { value: "BCT", display_name: "Birth Certificate A number identifyi", definition: "associated with a document ng the event of a person's birth", comment_usage_note: "", status: "" },
        "BR" => V2TableRow { value: "BR", display_name: "Birth registry An identi number Authority a person'", definition: "fier unique within the Assigning that is the official legal record of s birth.", comment_usage_note: "", status: "" },
        "BRN" => V2TableRow { value: "BRN", display_name: "Breed Registry Number", definition: "", comment_usage_note: "", status: "" },
        "BSN" => V2TableRow { value: "BSN", display_name: "R Primary physician office number", definition: "Betriebsstättennumme - for use in the German realm.", comment_usage_note: "r", status: "" },
        "CAAI" => V2TableRow { value: "CAAI", display_name: "Consumer An identi Application patient, Account as a port Identifier", definition: "fier for the consumer (e.g., This may be the same caregiver) for an application such as a username, but al or App. frequently is differ", comment_usage_note: "N ent.", status: "" },
        "CC" => V2TableRow { value: "CC", display_name: "Cost Center number", definition: "Class: Financial Use Case : needed especially for transmitting information about invoices.", comment_usage_note: "", status: "" },
        "CONM" => V2TableRow { value: "CONM", display_name: "Change of Name A number Document identifyi name.", definition: "associated with a document ng a person's legal change of", comment_usage_note: "", status: "" },
        "CY" => V2TableRow { value: "CY", display_name: "County number", definition: "", comment_usage_note: "", status: "" },
        "CZ" => V2TableRow { value: "CZ", display_name: "Citizenship Card A number of reside citizensh", definition: "assigned by a person's country nce to identify a person's ip.", comment_usage_note: "", status: "" },
        "DC" => V2TableRow { value: "DC", display_name: "Death Certificate The ident ID certifica certifica vital rec", definition: "ifier assigned to a death te, and printed on the death te when issued by a jurisdictional ords office", comment_usage_note: "", status: "" },
        "DCFN" => V2TableRow { value: "DCFN", display_name: "Death Certificate The ident File Number jurisdict", definition: "ifier used within the ional vital records office file system as an auxiliary means of accessing the record associated with the death certificate.", comment_usage_note: "", status: "" },
        "DDS" => V2TableRow { value: "DDS", display_name: "Dentist license number", definition: "An identifier that is unique to a dentist within the jurisdiction of the licensing board", comment_usage_note: "", status: "" },
        "DEA" => V2TableRow { value: "DEA", display_name: "Drug Enforcement Administration registration number", definition: "An identifier for an individual or organization relative to controlled substance regulation and transactions.", comment_usage_note: "Use case: This is a registration number that identifies an individual or organization relative to controlled substance regulation and transactions. A DEA number has a very precise and widely accepted meaning within the United States. Surprisingly, the US Drug Enforcement Administration does not solely assign DEA numbers in the United States. Hospitals have the authority to issue DEA numbers to their medical residents. These DEA numbers are based upon the hospital’s DEA number, but the authority rests with the hospital on the assignment to the residents. Thus, DEA as an Identifier Type is necessary in addition to DEA as an Assigning Authority.", status: "" },
        "DFN" => V2TableRow { value: "DFN", display_name: "Drug Furnishing or prescriptive authority Number", definition: "An identifier issued to a health care provider authorizing the person to write drug orders", comment_usage_note: "Use Case : A nurse practitioner has authorization to furnish or prescribe pharmaceutical substances; this identifier is in component 1.", status: "" },
        "DI" => V2TableRow { value: "DI", display_name: "Diner's Club card", definition: "", comment_usage_note: "Deprecated and replaced by BC in v 2.5.", status: "" },
        "DL" => V2TableRow { value: "DL", display_name: "Driver's license number", definition: "", comment_usage_note: "", status: "" },
        "DN" => V2TableRow { value: "DN", display_name: "Doctor number", definition: "", comment_usage_note: "", status: "" },
        "DO" => V2TableRow { value: "DO", display_name: "Osteopathic An ident License number within t", definition: "ifier that is unique to an osteopath he jurisdiction of a licensing board.", comment_usage_note: "", status: "" },
        "DP" => V2TableRow { value: "DP", display_name: "Diplomatic A number Passport passport", definition: "assigned to a diplomatic .", comment_usage_note: "", status: "" },
        "DPM" => V2TableRow { value: "DPM", display_name: "Podiatrist license An ident number within t board.", definition: "ifier that is unique to a podiatrist he jurisdiction of the licensing", comment_usage_note: "", status: "" },
        "DR" => V2TableRow { value: "DR", display_name: "Donor Registration Number", definition: "", comment_usage_note: "", status: "" },
        "DS" => V2TableRow { value: "DS", display_name: "Discover Card", definition: "Deprecated and replaced by BC in 2.5.", comment_usage_note: "v", status: "" },
        "DSG" => V2TableRow { value: "DSG", display_name: "Diagnostic Study Unique I Group orders t", definition: "dentifier that groups several Example: Radiolog hat are to be performed together. studies", comment_usage_note: "y N", status: "" },
        "EI" => V2TableRow { value: "EI", display_name: "Employee A number number employee", definition: "that uniquely identifies an to an employer.", comment_usage_note: "", status: "" },
        "EN" => V2TableRow { value: "EN", display_name: "Employer number", definition: "", comment_usage_note: "", status: "" },
        "ESN" => V2TableRow { value: "ESN", display_name: "Staff Enterprise An ident Number member w by the A", definition: "ifier that is unique to a staff ithin an enterprise (as identified ssigning Authority).", comment_usage_note: "", status: "" },
        "FDR" => V2TableRow { value: "FDR", display_name: "Fetal Death The iden Report ID report, when iss records", definition: "tifier assigned to a fetal death and printed on the fetal death report ued by a jurisdictional vital office", comment_usage_note: "N", status: "" },
        "FDRF" => V2TableRow { value: "FDRF", display_name: "N Fetal Death The iden Report File jurisdic Number system a the reco report c", definition: "tifier used within the tional vital records office file s an auxiliary means of accessing rd associated with the fetal death ertificate.", comment_usage_note: "N", status: "" },
        "FGN" => V2TableRow { value: "FGN", display_name: "Filler Group Unique i Number orders b", definition: "dentifier assigned to a group of This is analogous y the filler application. the Placer Group Number ORC-4, except that it is assigned by the f", comment_usage_note: "to N iller.", status: "" },
        "FI" => V2TableRow { value: "FI", display_name: "Facility ID", definition: "", comment_usage_note: "", status: "" },
        "FILL" => V2TableRow { value: "FILL", display_name: "Filler Identifier An ident identifi service, fulfills", definition: "ifier for a request where the er is issued by the person, or that produces the observations or the request.", comment_usage_note: "", status: "" },
        "GI" => V2TableRow { value: "GI", display_name: "Guarantor internal identifier", definition: "Class: Financial", comment_usage_note: "", status: "" },
        "GIN" => V2TableRow { value: "GIN", display_name: "Animal Group Identifi Identifier (US unambigu Official) of anima", definition: "er that can be used to GIN is the offici ously describe a specific group acronym used by ls. USDA", comment_usage_note: "al N", status: "" },
        "GL" => V2TableRow { value: "GL", display_name: "General ledger number", definition: "Class: Financial", comment_usage_note: "", status: "" },
        "GN" => V2TableRow { value: "GN", display_name: "Guarantor external identifier", definition: "Class: Financial", comment_usage_note: "", status: "" },
        "HC" => V2TableRow { value: "HC", display_name: "Health Card Number", definition: "", comment_usage_note: "", status: "" },
        "IND" => V2TableRow { value: "IND", display_name: "Indigenous/Abori A nu ginal indi Cana", definition: "mber assigned to a member of an genous or aboriginal group outside of da.", comment_usage_note: "", status: "" },
        "JHN" => V2TableRow { value: "JHN", display_name: "Jurisdictional health number", definition: "Class: I 2 uses: jurisdic number; provinci number:", comment_usage_note: "nsurance a) UK tional CHI b) Canadian al health card", status: "" },
        "LACSN" => V2TableRow { value: "LACSN", display_name: "Laboratory A la Accession ID labo", definition: "boratory accession id is used in the The conc ratory domain. accessio other do radiolog LACSN i distingu accessio radiolog", comment_usage_note: "ept of n is used in mains such as y, so the s used to ish a lab n id from an y accession id", status: "" },
        "LANR" => V2TableRow { value: "LANR", display_name: "Lifelong physician number", definition: "Lebensla Arztnumm in Germa", comment_usage_note: "nge er - for use n realm.", status: "" },
        "LI" => V2TableRow { value: "LI", display_name: "Labor and industries number", definition: "", comment_usage_note: "", status: "" },
        "LN" => V2TableRow { value: "LN", display_name: "License number", definition: "", comment_usage_note: "", status: "" },
        "LR" => V2TableRow { value: "LR", display_name: "Local Registry ID", definition: "", comment_usage_note: "", status: "" },
        "MA" => V2TableRow { value: "MA", display_name: "Patient Medicaid number", definition: "Class: I", comment_usage_note: "nsurance", status: "" },
        "MB" => V2TableRow { value: "MB", display_name: "Member Number An i insu subs insu", definition: "dentifier for the insured of an Use Case rance policy (this insured always has a covered criber), usually assigned by the insuranc rance carrier. person m be the s the poli", comment_usage_note: ": Person is by an e policy. This ay or may not ubscriber of cy.", status: "" },
        "MC" => V2TableRow { value: "MC", display_name: "Patient's Medicare number", definition: "Class: I", comment_usage_note: "nsurance", status: "" },
        "MCD" => V2TableRow { value: "MCD", display_name: "Practitioner Medicaid number", definition: "Class: I", comment_usage_note: "nsurance", status: "" },
        "MCN" => V2TableRow { value: "MCN", display_name: "Microchip Number", definition: "", comment_usage_note: "", status: "" },
        "MCR" => V2TableRow { value: "MCR", display_name: "Practitioner Medicare number", definition: "Class: I", comment_usage_note: "nsurance", status: "" },
        "MCT" => V2TableRow { value: "MCT", display_name: "Marriage A nu Certificate iden marr", definition: "mber associated with a document tifying the event of a person's iage.", comment_usage_note: "", status: "" },
        "MD" => V2TableRow { value: "MD", display_name: "Medical License An i number doct boar", definition: "dentifier that is unique to a medical Use Case or within the jurisdiction of a licensing license d. sometime identifi states, authorit three id medical, and phys licenses one stat", comment_usage_note: ": These numbers are s used as ers. In some the same y issues all entifiers, e.g., osteopathic, ician assistant all issued by e medical board. For this case, the CX data type requires distinct identifier types to accurately interpret component 1 . Additionally, the distinction among these license types is critical in most health care settings (this is not to convey full licensing information, which requires a segment to support all related attributes).", status: "" },
        "MI" => V2TableRow { value: "MI", display_name: "Military ID number", definition: "A number assigned to an individual who has had military duty, but is not currently on active duty. The number is assigned by the DOD or Veterans' Affairs (VA).", comment_usage_note: "", status: "" },
        "MR" => V2TableRow { value: "MR", display_name: "Medical record number", definition: "An identifier that is unique to a patient within a set of medical records, not necessarily unique within an application.", comment_usage_note: "", status: "" },
        "MRT" => V2TableRow { value: "MRT", display_name: "Temporary Medical Record Number", definition: "Temporary version of a Medical Record Number", comment_usage_note: "Use Case: An ancillary system that does not normally assign medical record numbers is the first time to register a patient. This ancillary system will generate a temporary medical record number that will only be used until an official medical record number is assigned.", status: "" },
        "MS" => V2TableRow { value: "MS", display_name: "MasterCard", definition: "", comment_usage_note: "Deprecated and replaced by BC in v 2.5.", status: "" },
        "NBSN" => V2TableRow { value: "NBSN", display_name: "R Secondary physician office number", definition: "", comment_usage_note: "Nebenbetriebsstättenn ummer - for use in the German realm.", status: "" },
        "NCT" => V2TableRow { value: "NCT", display_name: "Naturalization Certificate", definition: "A number associated with a document identifying a person's retention of citizenship in a particular country.", comment_usage_note: "", status: "" },
        "NE" => V2TableRow { value: "NE", display_name: "National employer identifier", definition: "", comment_usage_note: "In the US, the Assigning Authority for this value is typically CMS, but it may be used by all providers and insurance companies in HIPAA related transactions.", status: "" },
        "NH" => V2TableRow { value: "NH", display_name: "National Health Plan Identifier", definition: "", comment_usage_note: "Class: Insurance Used for the UK NHS national identifier.", status: "" },
        "NI" => V2TableRow { value: "NI", display_name: "National unique individual identifier", definition: "", comment_usage_note: "Class: Insurance In the US, the Assigning Authority for this value is typically CMS, but it may be used by all providers and insurance companies in HIPAA related transactions.", status: "" },
        "NII" => V2TableRow { value: "NII", display_name: "National Insurance Organization Identifier", definition: "", comment_usage_note: "Class: Insurance In Germany a national identifier for an insurance company. It is printed on the insurance card (health card). It is not to be confused with the health card number itself. Krankenkassen-ID der KV-Karte", status: "" },
        "NIIP" => V2TableRow { value: "NIIP", display_name: "National Insurance Payor Identifier (Payor)", definition: "", comment_usage_note: "Class: Insurance In Germany the insurance identifier addressed as the payor. Krankenkassen-ID des Rechnungsempfängers Use case: a subdivision issues the card with their identifier, but the main division is going to pay the invoices.", status: "" },
        "NNxxx" => V2TableRow { value: "NNxxx", display_name: "National Person Identifier where the xxx is the ISO table 3166 3- character (alphabetic) country code", definition: "", comment_usage_note: "", status: "" },
        "NP" => V2TableRow { value: "NP", display_name: "Nurse practitioner number", definition: "An identifier that is unique to a nurse practitioner within the jurisdiction of a certifying board.", comment_usage_note: "", status: "" },
        "NPI" => V2TableRow { value: "NPI", display_name: "National provider identifier", definition: "may be used by all providers and insurance companie in HIPAA related transactions.", comment_usage_note: "Class: Insurance In the US, the Assigning Authority for this value is typically CMS, but it s", status: "" },
        "OBI" => V2TableRow { value: "OBI", display_name: "Observation Unique a Instance observat Identifier", definition: "nd persistent identifier for an For example in the ion instance IHE-LCC Profile th is used to identif OBX-21 of the resu for which a clarification is requested using an OML^O59_OML_O5 9 message structur", comment_usage_note: "N is y the lt e", status: "" },
        "OD" => V2TableRow { value: "OD", display_name: "Optometrist A number license number optometr licensin", definition: "that is unique to an individual ist within the jurisdiction of the g board.", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Physician An ident Assistant number assistan licensin", definition: "ifier that is unique to a physician t within the jurisdiction of a g board", comment_usage_note: "", status: "" },
        "PC" => V2TableRow { value: "PC", display_name: "Parole Card A number", definition: "identifying a person on parole.", comment_usage_note: "", status: "" },
        "PCN" => V2TableRow { value: "PCN", display_name: "Penitentiary/corre A number ctional institution incarcer Number", definition: "assigned to individual who is ated.", comment_usage_note: "", status: "" },
        "PE" => V2TableRow { value: "PE", display_name: "Living Subject An ident Enterprise subject Number by the A", definition: "ifier that is unique to a living within an enterprise (as identified ssigning Authority).", comment_usage_note: "", status: "" },
        "PEN" => V2TableRow { value: "PEN", display_name: "Pension Number", definition: "", comment_usage_note: "", status: "" },
        "PGN" => V2TableRow { value: "PGN", display_name: "Placer Group Unique i Number orders b", definition: "dentifier assigned to a group of This is analogous y the placer application. the Placer Group Number ORC-4.", comment_usage_note: "to N", status: "" },
        "PHC" => V2TableRow { value: "PHC", display_name: "Public Health Identifi Case Identifier case inv event", definition: "er assigned to a person during a For example every estigation as part of a public health person affected by Norovirus outbreak a cruise ship will assigned a case ID investigation and follow up", comment_usage_note: "N the on be for", status: "" },
        "PHE" => V2TableRow { value: "PHE", display_name: "Public Health Identifi Event Identifier to publ", definition: "er assigned to an event of interest For example an ic health outbreak of Norovi on a cruise ship – is assigned by a p health jurisdictio the local, state o federal level", comment_usage_note: "N rus this ublic n at r", status: "" },
        "PHO" => V2TableRow { value: "PHO", display_name: "Public Health An ident Official ID public h issued b", definition: "ifier for a person working at a May need to identi ealth agency (PHA), assigned or contact in a PHA t y the agency approved a test re or is in charge of investigation.", comment_usage_note: "fy N hat quest an", status: "" },
        "PI" => V2TableRow { value: "PI", display_name: "Patient internal A number identifier an Assig", definition: "that is unique to a patient within ning Authority.", comment_usage_note: "", status: "" },
        "PIN" => V2TableRow { value: "PIN", display_name: "Premises I Identifier Number g (US Official)", definition: "dentifier that uniquely identifies a The eographic location in the US. pre des own can loc ide or per to ass mai the add pro coo and use des A p ide (PI dig inc and Exa", comment_usage_note: "owner of the N mises, or a person ignated by the er of the premises, register his/her ation. A premises ntification number, PIN, is then manently assigned that location ociating it with the ling address. If re is no mailing ress at the perty, geographic rdinates—latitude longitude—can be d instead to cribe the location. remises ntification number N) is a unique, 7- it code that ludes both letters numbers. mple: A123R69", status: "" },
        "PLAC" => V2TableRow { value: "PLAC", display_name: "Placer Identifier A i m", definition: "n identifier for a request where the dentifier is issued by the person or service aking the request.", comment_usage_note: "", status: "" },
        "PN" => V2TableRow { value: "PN", display_name: "Person number A w", definition: "number that is unique to a living subject ithin an Assigning Authority.", comment_usage_note: "", status: "" },
        "PNT" => V2TableRow { value: "PNT", display_name: "Temporary T Living Subject N Number", definition: "emporary version of a Living Subject umber.", comment_usage_note: "", status: "" },
        "PPIN" => V2TableRow { value: "PPIN", display_name: "Medicare/CMS Performing Provider Identification Number", definition: "Cla Usa Ins", comment_usage_note: "ss: Insurance ge Note: Class: urance", status: "" },
        "PPN" => V2TableRow { value: "PPN", display_name: "Passport number A d c", definition: "unique number assigned to the In ocument affirming that a person is a is itizen of the country. Sta", comment_usage_note: "the US this number issued only by the te Department.", status: "" },
        "PRC" => V2TableRow { value: "PRC", display_name: "Permanent Resident Card Number", definition: "", comment_usage_note: "", status: "" },
        "PRN" => V2TableRow { value: "PRN", display_name: "Provider number A p o A", definition: "number that is unique to an individual Use rovider, a provider group or an PRN rganization within an Assigning eit uthority. nur gro (or tea", comment_usage_note: "case: This allows to represent her an individual (a se) or a up/organization thopedic surgery m).", status: "" },
        "PT" => V2TableRow { value: "PT", display_name: "Patient external identifier", definition: "", comment_usage_note: "", status: "" },
        "QA" => V2TableRow { value: "QA", display_name: "QA number", definition: "", comment_usage_note: "", status: "" },
        "RI" => V2TableRow { value: "RI", display_name: "Resource identifier", definition: "A generalized resource identifier.", comment_usage_note: "Use Case : An identifier type is needed to accommodate what are commonly known as resources. The resources can include human (e.g. a respiratory therapist), non- h companion animal), inanimate object (e.g., an exam room), organization (e.g., diabetic education class) or any other physical or logical entity.", status: "uman (e.g., a" },
        "RN" => V2TableRow { value: "RN", display_name: "Registered Nurse Number", definition: "An identifier that is unique to a registered nurse within the jurisdiction of the licensing board.", comment_usage_note: "", status: "" },
        "RPH" => V2TableRow { value: "RPH", display_name: "Pharmacist license number", definition: "An identifier that is unique to a pharmacist within the jurisdiction of the licensing board.", comment_usage_note: "", status: "" },
        "RR" => V2TableRow { value: "RR", display_name: "Railroad Retirement number", definition: "An identifier for an individual enrolled with the Railroad Retirement Administration. Analogous to, but distinct from, a Social Security Number", comment_usage_note: "", status: "" },
        "RRI" => V2TableRow { value: "RRI", display_name: "Regional registry ID", definition: "", comment_usage_note: "", status: "" },
        "RRP" => V2TableRow { value: "RRP", display_name: "Railroad Retirement Provider", definition: "", comment_usage_note: "Class: Insurance", status: "" },
        "SAMN" => V2TableRow { value: "SAMN", display_name: "SAMN# accession Number", definition: "The accession number for the BioSample data repository at the National Center for Biotechnology Information (NCBI)", comment_usage_note: "This accession is a permanent record locator for the BioSample record which contains metadata about the biological sample.", status: "N" },
        "SB" => V2TableRow { value: "SB", display_name: "Social Beneficiary Identifier", definition: "An identifier issued by a governmental organization to a person to identify the person should they apply for or receive social services and/or benefits", comment_usage_note: "", status: "" },
        "SID" => V2TableRow { value: "SID", display_name: "Specimen ID", definition: "Identifier for a specimen.", comment_usage_note: "Used when it is not known if the specimen ID is a unique specimen ID (USID) or an ancestor ID (ASID).", status: "" },
        "SL" => V2TableRow { value: "SL", display_name: "State license", definition: "", comment_usage_note: "", status: "" },
        "SN" => V2TableRow { value: "SN", display_name: "Subscriber An identifier Number insurance poli usually assign", definition: "for a subscriber of an Class: Insurance cy which is unique for, and Use Case : A person is ed by, the insurance carrier. the subscriber of an insurance policy. The person’s family may be plan members, but are not the subscriber.", comment_usage_note: "", status: "" },
        "SNBS" => V2TableRow { value: "SNBS", display_name: "N State assigned The identifier NDBS card Dried Bloodspo Identifier assigned by th sample collect information mu", definition: "on a Newborn Screening For use either with t (NDBS) card that is OBX-5 as CX e state which provided the datatype, where OBX- ion cards and to whom this 3 uses LOINC 57716- st be reported 3^State printed on filter paper card [Identifier] in NBS card^LN, or in SPM- 31", comment_usage_note: "N", status: "" },
        "SNO" => V2TableRow { value: "SNO", display_name: "Serial Number An identifier manufacturer w each item has", definition: "affixed to an item by the hen it is first made, where a different identifier.", comment_usage_note: "", status: "" },
        "SP" => V2TableRow { value: "SP", display_name: "Study Permit A number assoc identifying a jurisdiction f", definition: "iated with a permit person who is a resident of a or the purpose of education.", comment_usage_note: "", status: "" },
        "SR" => V2TableRow { value: "SR", display_name: "State registry ID", definition: "", comment_usage_note: "", status: "" },
        "SRX" => V2TableRow { value: "SRX", display_name: "SRA Acce ssion The accession number Sequence Read National Cente Information (N are uploaded t", definition: "number generated by the This provides both the Archive (SRA) at the sequence data and r for Biotechnology metadata on how the CBI) when sequence data sample was o NCBI. sequenced. – This accession is a permanent record locator for the submitted un- assembled sequence data.", comment_usage_note: "N", status: "" },
        "SS" => V2TableRow { value: "SS", display_name: "Social Security number", definition: "", comment_usage_note: "", status: "" },
        "STN" => V2TableRow { value: "STN", display_name: "Shipment Identifier ass Tracking Number shipped", definition: "igned to a package being For example the Fed Ex / UPS / DHS / USPS tracking number", comment_usage_note: "N", status: "" },
        "TAX" => V2TableRow { value: "TAX", display_name: "Tax ID number", definition: "", comment_usage_note: "", status: "" },
        "TN" => V2TableRow { value: "TN", display_name: "Treaty Number/ A number assig (Canada) indigenous gro", definition: "ned to a member of an Use Case : First up in Canada. Nation.", comment_usage_note: "", status: "" },
        "TPR" => V2TableRow { value: "TPR", display_name: "Temporary A number assoc Permanent identifying a Resident permanent resi (Canada)", definition: "iated with a document person's temporary dent status.", comment_usage_note: "", status: "" },
        "TRL" => V2TableRow { value: "TRL", display_name: "Training License The license nu Number", definition: "mber used during training.", comment_usage_note: "N", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unspecified identifier", definition: "", comment_usage_note: "", status: "" },
        "UDI" => V2TableRow { value: "UDI", display_name: "Universal Device An identifier Identifier the Unique Dev framework as d (http://imdrf.", definition: "assigned to a device using ice Identification efined by IMDRF org).", comment_usage_note: "", status: "" },
        "UPIN" => V2TableRow { value: "UPIN", display_name: "Medicare/CMS An id (formerly CMS/M HCFA) 's uniqu Universal Medic Physician Identification numbers", definition: "entifier for a provider within the Class: Insur edicare program. A globally e identifier for the provider in the are program.", comment_usage_note: "ance", status: "" },
        "USID" => V2TableRow { value: "USID", display_name: "Unique Specimen A uni ID", definition: "que identifier for a specimen.", comment_usage_note: "", status: "" },
        "VN" => V2TableRow { value: "VN", display_name: "Visit number", definition: "", comment_usage_note: "", status: "" },
        "VP" => V2TableRow { value: "VP", display_name: "Visitor Permit A num ident juris", definition: "ber associated with a document ifying a person as a visitor of a diction or country.", comment_usage_note: "", status: "" },
        "VS" => V2TableRow { value: "VS", display_name: "VISA", definition: "Deprecated a replaced by 2.5.", comment_usage_note: "nd BC in v", status: "" },
        "WC" => V2TableRow { value: "WC", display_name: "WIC identifier", definition: "", comment_usage_note: "", status: "" },
        "WCN" => V2TableRow { value: "WCN", display_name: "Workers' Comp Number", definition: "", comment_usage_note: "", status: "" },
        "WP" => V2TableRow { value: "WP", display_name: "Work Permit A num perso in a", definition: "ber associated with a permit for a n who is granted permission to work country for a specified time period.", comment_usage_note: "", status: "" },
        "XV" => V2TableRow { value: "XV", display_name: "Health Plan Natio Identifier requi and H and M Realm", definition: "nal unique health plan identifier Also referre red by the US Department of Health HPID (Health uman Services, Centers for Medicare Identifier). edicaid Services (CMS) in the US . Usage Note: value ‘XV’ i CMS mandated Insurance Po and Accounta Act (HIPAA) transactions", comment_usage_note: "d to as N Plan The code s used in Health rtability bility .", status: "" },
        "XX" => V2TableRow { value: "XX", display_name: "Organization identifier", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0204: V2Table = V2Table {
    number: 204,
    metadata: &super::metadata::TABLE_0204_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Alias name", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Legal name", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Display name", definition: "", comment_usage_note: "", status: "" },
        "SL" => V2TableRow { value: "SL", display_name: "Stock exchange listing name", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0205: V2Table = V2Table {
    number: 205,
    metadata: &super::metadata::TABLE_0205_METADATA,
    rows: phf_map! {
        "AP" => V2TableRow { value: "AP", display_name: "administrative price or handling fee", definition: "", comment_usage_note: "", status: "" },
        "DC" => V2TableRow { value: "DC", display_name: "direct unit cost", definition: "", comment_usage_note: "", status: "" },
        "IC" => V2TableRow { value: "IC", display_name: "indirect unit cost", definition: "", comment_usage_note: "", status: "" },
        "PF" => V2TableRow { value: "PF", display_name: "professional fee for performing provider", definition: "", comment_usage_note: "", status: "" },
        "TF" => V2TableRow { value: "TF", display_name: "technology fee for use of equipment", definition: "", comment_usage_note: "", status: "" },
        "TP" => V2TableRow { value: "TP", display_name: "total price", definition: "", comment_usage_note: "", status: "" },
        "UP" => V2TableRow { value: "UP", display_name: "unit price, may be based on length of procedure or service", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0206: V2Table = V2Table {
    number: 206,
    metadata: &super::metadata::TABLE_0206_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Add/Insert", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Delete", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Used in Snapshot mode", definition: "Declares when segment falls under snapshot mode handling, i.e. all elements that were previously sent will be sent, not just any changes", comment_usage_note: "Snapshot mode is the expected default; use this code to explicitly state that.", status: "N" },
        "U" => V2TableRow { value: "U", display_name: "Update", definition: "", comment_usage_note: "", status: "" },
        "X" => V2TableRow { value: "X", display_name: "No Change", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0207: V2Table = V2Table {
    number: 207,
    metadata: &super::metadata::TABLE_0207_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Archive", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Restore from archive", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Initial load", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Current processing, transmitted at intervals (scheduled or on demand)", definition: "", comment_usage_note: "", status: "" },
        "Not" => V2TableRow { value: "Not", display_name: "present Not present (the default, meaning current processing)", definition: "", comment_usage_note: "", status: "D" },
    },
};

pub static TABLE_0208: V2Table = V2Table {
    number: 208,
    metadata: &super::metadata::TABLE_0208_METADATA,
    rows: phf_map! {
        "OK" => V2TableRow { value: "OK", display_name: "Data found, no errors (this is the default)", definition: "", comment_usage_note: "", status: "" },
        "NF" => V2TableRow { value: "NF", display_name: "No data found, no errors", definition: "", comment_usage_note: "", status: "" },
        "AE" => V2TableRow { value: "AE", display_name: "Application error", definition: "", comment_usage_note: "", status: "" },
        "AR" => V2TableRow { value: "AR", display_name: "Application reject", definition: "", comment_usage_note: "", status: "" },
        "TM" => V2TableRow { value: "TM", display_name: "Too much data found", definition: "The response would exceed the maximum length designated in RCP-2 of the query message.", comment_usage_note: "", status: "N" },
        "PD" => V2TableRow { value: "PD", display_name: "Protected data", definition: "Data matching the query parameters was found but could not be shared with the querying system for reasons including local policy or legal restrictions.", comment_usage_note: "", status: "N" },
    },
};

pub static TABLE_0209: V2Table = V2Table {
    number: 209,
    metadata: &super::metadata::TABLE_0209_METADATA,
    rows: phf_map! {
        "EQ" => V2TableRow { value: "EQ", display_name: "Equal", definition: "", comment_usage_note: "", status: "" },
        "NE" => V2TableRow { value: "NE", display_name: "Not Equal", definition: "", comment_usage_note: "", status: "" },
        "LT" => V2TableRow { value: "LT", display_name: "Less than", definition: "", comment_usage_note: "", status: "" },
        "GT" => V2TableRow { value: "GT", display_name: "Greater than", definition: "", comment_usage_note: "", status: "" },
        "LE" => V2TableRow { value: "LE", display_name: "Less than or equal", definition: "", comment_usage_note: "", status: "" },
        "GE" => V2TableRow { value: "GE", display_name: "Greater than or equal", definition: "", comment_usage_note: "", status: "" },
        "CT" => V2TableRow { value: "CT", display_name: "Contains", definition: "", comment_usage_note: "", status: "" },
        "GN" => V2TableRow { value: "GN", display_name: "Generic", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0210: V2Table = V2Table {
    number: 210,
    metadata: &super::metadata::TABLE_0210_METADATA,
    rows: phf_map! {
        "AND" => V2TableRow { value: "AND", display_name: "Default", definition: "", comment_usage_note: "", status: "" },
        "OR" => V2TableRow { value: "OR", display_name: "", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0211: V2Table = V2Table {
    number: 211,
    metadata: &super::metadata::TABLE_0211_METADATA,
    rows: phf_map! {
        "ASCII" => V2TableRow { value: "ASCII", display_name: "The printable 7-bit ASCII character set.", definition: "", comment_usage_note: "(This is the default if this field is omitted)", status: "" },
        "8859/1" => V2TableRow { value: "8859/1", display_name: "The printable characters from the ISO 8859/1 Character set", definition: "", comment_usage_note: "", status: "" },
        "8859/2" => V2TableRow { value: "8859/2", display_name: "The printable characters from the ISO 8859/2 Character set", definition: "", comment_usage_note: "", status: "" },
        "8859/3" => V2TableRow { value: "8859/3", display_name: "The printable characters from the ISO 8859/3 Character set", definition: "", comment_usage_note: "", status: "" },
        "8859/4" => V2TableRow { value: "8859/4", display_name: "The printable characters from the ISO 8859/4 Character set", definition: "", comment_usage_note: "", status: "" },
        "8859/5" => V2TableRow { value: "8859/5", display_name: "The printable characters from the ISO 8859/5 Character set", definition: "", comment_usage_note: "", status: "" },
        "8859/6" => V2TableRow { value: "8859/6", display_name: "The printable characters from the ISO 8859/6 Character set", definition: "", comment_usage_note: "", status: "" },
        "8859/7" => V2TableRow { value: "8859/7", display_name: "The printable characters from the ISO 8859/7 Character set", definition: "", comment_usage_note: "", status: "" },
        "8859/8" => V2TableRow { value: "8859/8", display_name: "The printable characters from the ISO 8859/8 Character set", definition: "", comment_usage_note: "", status: "" },
        "8859/9" => V2TableRow { value: "8859/9", display_name: "The printable characters from the ISO 8859/9 Character set", definition: "", comment_usage_note: "", status: "" },
        "8859/15" => V2TableRow { value: "8859/15", display_name: "The printable characters from the ISO 8859/15 (Latin-15)", definition: "", comment_usage_note: "", status: "" },
        "ISO" => V2TableRow { value: "ISO", display_name: "IR6 ASCII graphic character set consisting of 94 characters.", definition: "http://www.itscj.ipsj.or.jp/ IR/006.pdf", comment_usage_note: "ISO-", status: "" },
        "GB" => V2TableRow { value: "GB", display_name: "18030- Code for Chinese Character Set", definition: "Does not need an escape sequ", comment_usage_note: "ence.", status: "" },
        "2000" => V2TableRow { value: "2000", display_name: "(GB 18030-2000)", definition: "", comment_usage_note: "", status: "" },
        "KS" => V2TableRow { value: "KS", display_name: "X 1001 Code for Korean Character Set (KS X 1001)", definition: "", comment_usage_note: "", status: "" },
        "CNS" => V2TableRow { value: "CNS", display_name: "11643- Code for Taiwanese Character Set", definition: "Does not need an escape sequ", comment_usage_note: "ence.", status: "" },
        "1992" => V2TableRow { value: "1992", display_name: "(CNS 11643-1992)", definition: "", comment_usage_note: "", status: "" },
        "BIG-5" => V2TableRow { value: "BIG-5", display_name: "Code for Taiwanese Character Set (BIG-5)", definition: "Does not need an escape sequ BIG-5 does not need an escap sequence. ASCII is a 7 bit character set, which means t top bit of the byte is “0”. parser knows that when the t of the byte is “0”, the char is ASCII. When it is “1”, th following bytes should be ha as 2 bytes (or more). No esc technique is needed. However since some servers do not co interpret when they receive bit “1”, it is advised, in i RFC, to not use these kind o safe non-escape extension.", comment_usage_note: "ence. e hat the The op bit acter set e ndled ape , rrectly a top nternet f non-", status: "" },
        "UNICODE" => V2TableRow { value: "UNICODE", display_name: "The world wide character standard from ISO/IEC 10646-1-1993", definition: "Deprecated. Retained for bac compatibility only as v 2.5. Replaced by spe encoding codes. Usage Note: Ava Unicode Consort 700519, San Jos 0519. See http://www.unic nsortium/consor", comment_usage_note: "kward cific Unicode ilable from The ium, P.O. Box e, CA 95170- ode.org/unicode/co t.html", status: "" },
        "UTF-8" => V2TableRow { value: "UTF-8", display_name: "form", definition: "encoding, each represented by depending on th ASCII is a prop Note that the c before UTF but after the hyphe represents the character set, restriction app 1. UTF-8 must b encoding of the cannot be speci character set i 2. There are no allowed in a me is the default message. In other words, can only be spe value in MSH-18 3. A message en must not use a (BOM).", comment_usage_note: "code value is 1,2 or 3 bytes, e code value. 7 bit er subset of UTF-8. ode contains a space not before and n. Since UTF-8 full UNICODE the following ly to its use: e the default message, UTF-8 fied as an additional n MSH-18 other character sets ssage where UTF -8 encoding in the UNICODE UTF-8 cified as a single coded in UTF-8 Byte Order Mark", status: "" },
    },
};

pub static TABLE_0213: V2Table = V2Table {
    number: 213,
    metadata: &super::metadata::TABLE_0213_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Marked for purge. User is no longer able to update the visit.", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "The visit is marked for deletion and the user cannot enter new data against it.", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "The visit is marked inactive and the user cannot enter new data against it.", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0214: V2Table = V2Table {
    number: 214,
    metadata: &super::metadata::TABLE_0214_METADATA,
    rows: phf_map! {
        "CH" => V2TableRow { value: "CH", display_name: "Child Health Assistance", definition: "", comment_usage_note: "", status: "" },
        "ES" => V2TableRow { value: "ES", display_name: "Elective Surgery Program", definition: "", comment_usage_note: "", status: "" },
        "FP" => V2TableRow { value: "FP", display_name: "Family Planning", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0215: V2Table = V2Table {
    number: 215,
    metadata: &super::metadata::TABLE_0215_METADATA,
    rows: phf_map! {
        "F" => V2TableRow { value: "F", display_name: "Family only", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No Publicity", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0216: V2Table = V2Table {
    number: 216,
    metadata: &super::metadata::TABLE_0216_METADATA,
    rows: phf_map! {
        "AI" => V2TableRow { value: "AI", display_name: "Active Inpatient", definition: "", comment_usage_note: "", status: "" },
        "DI" => V2TableRow { value: "DI", display_name: "Discharged Inpatient", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0217: V2Table = V2Table {
    number: 217,
    metadata: &super::metadata::TABLE_0217_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Emergency", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Urgent", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Elective", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0220: V2Table = V2Table {
    number: 220,
    metadata: &super::metadata::TABLE_0220_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Alone", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Family", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Institution", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Relative", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Spouse Only", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0223: V2Table = V2Table {
    number: 223,
    metadata: &super::metadata::TABLE_0223_METADATA,
    rows: phf_map! {
        "S" => V2TableRow { value: "S", display_name: "Spouse Dependent", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Medical Supervision Required", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Small Children Dependent", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0224: V2Table = V2Table {
    number: 224,
    metadata: &super::metadata::TABLE_0224_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Arranged", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Not Arranged", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0225: V2Table = V2Table {
    number: 225,
    metadata: &super::metadata::TABLE_0225_METADATA,
    rows: phf_map! {
        "R" => V2TableRow { value: "R", display_name: "Required", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Not Required", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0228: V2Table = V2Table {
    number: 228,
    metadata: &super::metadata::TABLE_0228_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "Consultation", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Diagnosis", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Medication (antibiotic)", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Radiological scheduling (not using ICDA codes)", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Sign and symptom", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Tissue diagnosis", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Invasive procedure not classified elsewhere (I.V., catheter, etc.)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0230: V2Table = V2Table {
    number: 230,
    metadata: &super::metadata::TABLE_0230_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Anesthesia", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Procedure for treatment (therapeutic, including operations)", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Invasive procedure not classified elsewhere (e.g., IV, catheter, etc.)", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Diagnostic procedure", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0231: V2Table = V2Table {
    number: 231,
    metadata: &super::metadata::TABLE_0231_METADATA,
    rows: phf_map! {
        "F" => V2TableRow { value: "F", display_name: "Full-time student", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Part-time student", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Not a student", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0232: V2Table = V2Table {
    number: 232,
    metadata: &super::metadata::TABLE_0232_METADATA,
    rows: phf_map! {
        "01" => V2TableRow { value: "01", display_name: "Medicare claim status", definition: "", comment_usage_note: "", status: "" },
        "02" => V2TableRow { value: "02", display_name: "Medicaid claim status", definition: "", comment_usage_note: "", status: "" },
        "03" => V2TableRow { value: "03", display_name: "Name/address change", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0234: V2Table = V2Table {
    number: 234,
    metadata: &super::metadata::TABLE_0234_METADATA,
    rows: phf_map! {
        "CO" => V2TableRow { value: "CO", display_name: "Correction", definition: "", comment_usage_note: "", status: "" },
        "AD" => V2TableRow { value: "AD", display_name: "Additional information", definition: "", comment_usage_note: "", status: "" },
        "RQ" => V2TableRow { value: "RQ", display_name: "Requested information", definition: "", comment_usage_note: "", status: "" },
        "DE" => V2TableRow { value: "DE", display_name: "Device evaluation", definition: "", comment_usage_note: "", status: "" },
        "PD" => V2TableRow { value: "PD", display_name: "Periodic", definition: "", comment_usage_note: "", status: "" },
        "3D" => V2TableRow { value: "3D", display_name: "3 day report", definition: "", comment_usage_note: "", status: "" },
        "7D" => V2TableRow { value: "7D", display_name: "7 day report", definition: "", comment_usage_note: "", status: "" },
        "10D" => V2TableRow { value: "10D", display_name: "10 day report", definition: "", comment_usage_note: "", status: "" },
        "15D" => V2TableRow { value: "15D", display_name: "15 day report", definition: "", comment_usage_note: "", status: "" },
        "30D" => V2TableRow { value: "30D", display_name: "30 day report", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0235: V2Table = V2Table {
    number: 235,
    metadata: &super::metadata::TABLE_0235_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "Clinical trial", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Literature", definition: "", comment_usage_note: "", status: "" },
        "H" => V2TableRow { value: "H", display_name: "Health professional", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Regulatory agency", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Database/registry/ poison control center", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Non-healthcare professional", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Patient", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Manufacturer/mar keting authority holder", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Distributor", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0236: V2Table = V2Table {
    number: 236,
    metadata: &super::metadata::TABLE_0236_METADATA,
    rows: phf_map! {
        "M" => V2TableRow { value: "M", display_name: "Manufacturer", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Local facility/user facility", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Regulatory agency", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Distributor", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0237: V2Table = V2Table {
    number: 237,
    metadata: &super::metadata::TABLE_0237_METADATA,
    rows: phf_map! {
        "I" => V2TableRow { value: "I", display_name: "Interaction", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Overdose", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Abuse", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Misuse", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Dependency", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Lack of expect therapeutic effect", definition: "", comment_usage_note: "", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Drug withdrawal", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Unexpected beneficial effect", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0238: V2Table = V2Table {
    number: 238,
    metadata: &super::metadata::TABLE_0238_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Yes", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Significant", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0239: V2Table = V2Table {
    number: 239,
    metadata: &super::metadata::TABLE_0239_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Yes", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0240: V2Table = V2Table {
    number: 240,
    metadata: &super::metadata::TABLE_0240_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Death", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Life threatening", definition: "", comment_usage_note: "", status: "" },
        "H" => V2TableRow { value: "H", display_name: "Caused hospitalized", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Prolonged hospitalization", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Congenital anomaly/birth defect", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Incapacity which is significant, persistent or permanent", definition: "", comment_usage_note: "", status: "" },
        "J" => V2TableRow { value: "J", display_name: "Disability which is significant, persistent or permanent", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Required intervention to prevent permanent impairment/damage", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0241: V2Table = V2Table {
    number: 241,
    metadata: &super::metadata::TABLE_0241_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Died", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Recovering", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Not recovering/uncha nged", definition: "", comment_usage_note: "", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Worsening", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Sequelae", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Fully recovered", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0242: V2Table = V2Table {
    number: 242,
    metadata: &super::metadata::TABLE_0242_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Physician (osteopath, homeopath)", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Pharmacist", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Mid-level professional (nurse, nurse practitioner, physician's assistant)", definition: "", comment_usage_note: "", status: "" },
        "H" => V2TableRow { value: "H", display_name: "Other health professional", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Health care consumer/patient", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Lawyer/attorney", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other non-health professional", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0243: V2Table = V2Table {
    number: 243,
    metadata: &super::metadata::TABLE_0243_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Yes", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No", definition: "", comment_usage_note: "", status: "" },
        "NA" => V2TableRow { value: "NA", display_name: "Not applicable", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0247: V2Table = V2Table {
    number: 247,
    metadata: &super::metadata::TABLE_0247_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Evaluation completed", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Evaluation in progress", definition: "", comment_usage_note: "", status: "" },
        "K" => V2TableRow { value: "K", display_name: "Problem already known, no evaluation necessary", definition: "", comment_usage_note: "", status: "" },
        "X" => V2TableRow { value: "X", display_name: "Product not made by company", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Evaluation anticipated, but not yet begun", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Product discarded -- unable to follow up", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Product received in condition which made analysis impossible", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Product remains implanted -- unable to follow up", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Product unavailable for follow up investigation", definition: "", comment_usage_note: "", status: "" },
        "Q" => V2TableRow { value: "Q", display_name: "Product under quarantine -- unable to follow up", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Product under recall/corrective action", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0248: V2Table = V2Table {
    number: 248,
    metadata: &super::metadata::TABLE_0248_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Actual product involved in incident was evaluated", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "A product from the same lot as the actual product involved was evaluated", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "A product from a reserve sample was evaluated", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "A product from a controlled/non- related inventory was evaluated", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0250: V2Table = V2Table {
    number: 250,
    metadata: &super::metadata::TABLE_0250_METADATA,
    rows: phf_map! {
        "H" => V2TableRow { value: "H", display_name: "Highly probable", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Moderately probable", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Somewhat probable", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Improbable", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Not related", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0251: V2Table = V2Table {
    number: 251,
    metadata: &super::metadata::TABLE_0251_METADATA,
    rows: phf_map! {
        "WP" => V2TableRow { value: "WP", display_name: "Product withdrawn permanently", definition: "", comment_usage_note: "", status: "" },
        "WT" => V2TableRow { value: "WT", display_name: "Product withdrawn temporarily", definition: "", comment_usage_note: "", status: "" },
        "DR" => V2TableRow { value: "DR", display_name: "Product dose or frequency of use reduced", definition: "", comment_usage_note: "", status: "" },
        "DI" => V2TableRow { value: "DI", display_name: "Product dose or frequency of use increased", definition: "", comment_usage_note: "", status: "" },
        "OT" => V2TableRow { value: "OT", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "None", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0252: V2Table = V2Table {
    number: 252,
    metadata: &super::metadata::TABLE_0252_METADATA,
    rows: phf_map! {
        "AW" => V2TableRow { value: "AW", display_name: "Abatement of event after product withdrawn", definition: "", comment_usage_note: "", status: "" },
        "BE" => V2TableRow { value: "BE", display_name: "Event recurred after product reintroduced", definition: "", comment_usage_note: "", status: "" },
        "LI" => V2TableRow { value: "LI", display_name: "Literature reports association of product with event", definition: "", comment_usage_note: "", status: "" },
        "IN" => V2TableRow { value: "IN", display_name: "Event occurred after product introduced", definition: "", comment_usage_note: "", status: "" },
        "EX" => V2TableRow { value: "EX", display_name: "Alternative explanations for the event available", definition: "", comment_usage_note: "", status: "" },
        "PL" => V2TableRow { value: "PL", display_name: "Effect observed when patient receives placebo", definition: "", comment_usage_note: "", status: "" },
        "TC" => V2TableRow { value: "TC", display_name: "Toxic levels of product documented in blood or body fluids", definition: "", comment_usage_note: "", status: "" },
        "DR" => V2TableRow { value: "DR", display_name: "Dose response observed", definition: "", comment_usage_note: "", status: "" },
        "SE" => V2TableRow { value: "SE", display_name: "Similar events in past for this patient", definition: "", comment_usage_note: "", status: "" },
        "OE" => V2TableRow { value: "OE", display_name: "Occurrence of event was confirmed by objective evidence", definition: "", comment_usage_note: "", status: "" },
        "OT" => V2TableRow { value: "OT", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0253: V2Table = V2Table {
    number: 253,
    metadata: &super::metadata::TABLE_0253_METADATA,
    rows: phf_map! {
        "B" => V2TableRow { value: "B", display_name: "Breast milk", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Transplacental", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Father", definition: "", comment_usage_note: "", status: "" },
        "X" => V2TableRow { value: "X", display_name: "Blood product", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0254: V2Table = V2Table {
    number: 254,
    metadata: &super::metadata::TABLE_0254_METADATA,
    rows: phf_map! {
        "CACT" => V2TableRow { value: "CACT", display_name: "Catalytic Activity", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "CNC" => V2TableRow { value: "CNC", display_name: "Catalytic Concentration", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "CCRTO" => V2TableRow { value: "CCRTO", display_name: "Catalytic Concentration Ratio", definition: "", comment_usage_note: "", status: "" },
        "CCNT" => V2TableRow { value: "CCNT", display_name: "Catalytic Content", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "CF" => V2TableRow { value: "CF", display_name: "R Catalytic Fraction", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "CRAT" => V2TableRow { value: "CRAT", display_name: "Catalytic Rate", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "CRTO" => V2TableRow { value: "CRTO", display_name: "Catalytic Ratio", definition: "", comment_usage_note: "", status: "" },
        "ENT" => V2TableRow { value: "ENT", display_name: "Entitic", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "ENTSUB" => V2TableRow { value: "ENTSUB", display_name: "Entitic Substance of Amount", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "ENTCAT" => V2TableRow { value: "ENTCAT", display_name: "Entitic Catalytic Activity", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "ENTNUM" => V2TableRow { value: "ENTNUM", display_name: "Entitic Number", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "ENTVOL" => V2TableRow { value: "ENTVOL", display_name: "Entitic Volume", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "MASS" => V2TableRow { value: "MASS", display_name: "Mass", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "MCNC" => V2TableRow { value: "MCNC", display_name: "Mass Concentration", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "MCRTO" => V2TableRow { value: "MCRTO", display_name: "Mass Concentration Ratio", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "MCNT" => V2TableRow { value: "MCNT", display_name: "Mass Content", definition: "", comment_usage_note: "", status: "" },
        "MFR" => V2TableRow { value: "MFR", display_name: "Mass Fraction", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "MINC" => V2TableRow { value: "MINC", display_name: "Mass Increment", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "MRAT" => V2TableRow { value: "MRAT", display_name: "Mass Ra te", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "MRTO" => V2TableRow { value: "MRTO", display_name: "Mass Ra tio", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "NUM" => V2TableRow { value: "NUM", display_name: "Number", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "NCN" => V2TableRow { value: "NCN", display_name: "C Number Concentration", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "NCNT" => V2TableRow { value: "NCNT", display_name: "Number Content", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "NFR" => V2TableRow { value: "NFR", display_name: "Number Fraction", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "NRTO" => V2TableRow { value: "NRTO", display_name: "Number Ratio", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "SUB" => V2TableRow { value: "SUB", display_name: "Substance Amount", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "SCN" => V2TableRow { value: "SCN", display_name: "C Substance Concentration", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "SCRTO" => V2TableRow { value: "SCRTO", display_name: "Substance Concentration Ratio", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "SCNT" => V2TableRow { value: "SCNT", display_name: "Substance Content", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "SFR" => V2TableRow { value: "SFR", display_name: "Substance Fraction", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "SRAT" => V2TableRow { value: "SRAT", display_name: "Substance Rate", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "SRTO" => V2TableRow { value: "SRTO", display_name: "Substance Ratio", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "VOL" => V2TableRow { value: "VOL", display_name: "Volume", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "VCNT" => V2TableRow { value: "VCNT", display_name: "Volume Content", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "VFR" => V2TableRow { value: "VFR", display_name: "Volume Fraction", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "VRAT" => V2TableRow { value: "VRAT", display_name: "Volume Rate", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "VRTO" => V2TableRow { value: "VRTO", display_name: "Volume Ratio", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "ACN" => V2TableRow { value: "ACN", display_name: "C Concentration, Arbitrary Substance", definition: "", comment_usage_note: "", status: "" },
        "RLMCN" => V2TableRow { value: "RLMCN", display_name: "C Relative Mass Concentration", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "RLSCNC" => V2TableRow { value: "RLSCNC", display_name: "Relative Substance Concentration", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "THRMCNC" => V2TableRow { value: "THRMCNC", display_name: "Threshold Mass Concentration", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "THRS" => V2TableRow { value: "THRS", display_name: "CNC Threshold Substance Concentration", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "TIME" => V2TableRow { value: "TIME", display_name: "Time (e.g. seconds)", definition: "Usage No Book 199", comment_usage_note: "te: Adopted from the IUPAC Silver 5” to the code", status: "" },
        "TMDF" => V2TableRow { value: "TMDF", display_name: "Time Difference", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "TMSTP" => V2TableRow { value: "TMSTP", display_name: "Time Stamp-Date and Time", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "TRTO" => V2TableRow { value: "TRTO", display_name: "Time Ratio", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "RCRLTM" => V2TableRow { value: "RCRLTM", display_name: "Reciprocal Relative Time", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "RLTM" => V2TableRow { value: "RLTM", display_name: "Relative Time", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "ABS" => V2TableRow { value: "ABS", display_name: "Absorbance", definition: "", comment_usage_note: "", status: "" },
        "ACT" => V2TableRow { value: "ACT", display_name: "Activity", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "APER" => V2TableRow { value: "APER", display_name: "Appearance", definition: "", comment_usage_note: "", status: "" },
        "ARB" => V2TableRow { value: "ARB", display_name: "Arbitrary", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "AREA" => V2TableRow { value: "AREA", display_name: "Area", definition: "", comment_usage_note: "", status: "" },
        "ASPECT" => V2TableRow { value: "ASPECT", display_name: "Aspect", definition: "", comment_usage_note: "", status: "" },
        "CLAS" => V2TableRow { value: "CLAS", display_name: "Class", definition: "", comment_usage_note: "", status: "" },
        "CNST" => V2TableRow { value: "CNST", display_name: "Constant", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "COEF" => V2TableRow { value: "COEF", display_name: "Coefficient", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "COLOR" => V2TableRow { value: "COLOR", display_name: "Color", definition: "", comment_usage_note: "", status: "" },
        "CONS" => V2TableRow { value: "CONS", display_name: "Consistency", definition: "", comment_usage_note: "", status: "" },
        "DEN" => V2TableRow { value: "DEN", display_name: "Density", definition: "", comment_usage_note: "", status: "" },
        "DEV" => V2TableRow { value: "DEV", display_name: "Device", definition: "", comment_usage_note: "", status: "" },
        "DIFF" => V2TableRow { value: "DIFF", display_name: "Difference", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "ELAS" => V2TableRow { value: "ELAS", display_name: "Elasticity", definition: "", comment_usage_note: "", status: "" },
        "ELPOT" => V2TableRow { value: "ELPOT", display_name: "Electrical Potential (Voltage)", definition: "", comment_usage_note: "", status: "" },
        "ELRAT" => V2TableRow { value: "ELRAT", display_name: "Electrical current (amperage)", definition: "", comment_usage_note: "", status: "" },
        "ELRES" => V2TableRow { value: "ELRES", display_name: "Electrical Resi stance", definition: "", comment_usage_note: "", status: "" },
        "ENGR" => V2TableRow { value: "ENGR", display_name: "Energy", definition: "", comment_usage_note: "", status: "" },
        "EQL" => V2TableRow { value: "EQL", display_name: "Equilibrium", definition: "", comment_usage_note: "", status: "" },
        "FORCE" => V2TableRow { value: "FORCE", display_name: "Mechanical force", definition: "", comment_usage_note: "", status: "" },
        "FREQ" => V2TableRow { value: "FREQ", display_name: "Frequency", definition: "", comment_usage_note: "", status: "" },
        "IMP" => V2TableRow { value: "IMP", display_name: "Impression/ interpretation of study", definition: "", comment_usage_note: "", status: "" },
        "KINV" => V2TableRow { value: "KINV", display_name: "Kinematic Viscosity", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "LEN" => V2TableRow { value: "LEN", display_name: "Length", definition: "", comment_usage_note: "", status: "" },
        "LINC" => V2TableRow { value: "LINC", display_name: "Length Increment", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "LIQ" => V2TableRow { value: "LIQ", display_name: "Liquefaction", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "MGFLUX" => V2TableRow { value: "MGFLUX", display_name: "Magnetic flux", definition: "", comment_usage_note: "", status: "" },
        "MORPH" => V2TableRow { value: "MORPH", display_name: "Morphology", definition: "", comment_usage_note: "", status: "" },
        "MOTIL" => V2TableRow { value: "MOTIL", display_name: "Motility", definition: "", comment_usage_note: "", status: "" },
        "OD" => V2TableRow { value: "OD", display_name: "Optical density", definition: "", comment_usage_note: "", status: "" },
        "OSMOL" => V2TableRow { value: "OSMOL", display_name: "Osmolality", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "PRID" => V2TableRow { value: "PRID", display_name: "Presence/Identity/E xistence", definition: "", comment_usage_note: "", status: "" },
        "PRES" => V2TableRow { value: "PRES", display_name: "Pressure (Partial)", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "PWR" => V2TableRow { value: "PWR", display_name: "Power (wattage)", definition: "", comment_usage_note: "", status: "" },
        "RANGE" => V2TableRow { value: "RANGE", display_name: "Ranges", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "RATIO" => V2TableRow { value: "RATIO", display_name: "Ratios", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "RDEN" => V2TableRow { value: "RDEN", display_name: "Relative Density", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "REL" => V2TableRow { value: "REL", display_name: "Relative", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "SATFR" => V2TableRow { value: "SATFR", display_name: "Saturation Fraction", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "SHAPE" => V2TableRow { value: "SHAPE", display_name: "Shape", definition: "", comment_usage_note: "", status: "" },
        "SMELL" => V2TableRow { value: "SMELL", display_name: "Smell", definition: "", comment_usage_note: "", status: "" },
        "SUSC" => V2TableRow { value: "SUSC", display_name: "Susceptibility", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "TASTE" => V2TableRow { value: "TASTE", display_name: "Taste", definition: "", comment_usage_note: "", status: "" },
        "TEMP" => V2TableRow { value: "TEMP", display_name: "Temperature", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "TEMPDF" => V2TableRow { value: "TEMPDF", display_name: "Temperature Difference", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "TEMPIN" => V2TableRow { value: "TEMPIN", display_name: "Temperature Increment", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "TITR" => V2TableRow { value: "TITR", display_name: "Dilution Factor (Titer)", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "TYPE" => V2TableRow { value: "TYPE", display_name: "Type", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "VEL" => V2TableRow { value: "VEL", display_name: "Velocity", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "VELRT" => V2TableRow { value: "VELRT", display_name: "Velocity Ratio", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
        "VISC" => V2TableRow { value: "VISC", display_name: "Viscosity", definition: "", comment_usage_note: "Usage Note: Adopted from the IUPAC Silver Book 1995” to the code", status: "" },
    },
};

pub static TABLE_0255: V2Table = V2Table {
    number: 255,
    metadata: &super::metadata::TABLE_0255_METADATA,
    rows: phf_map! {
        "*" => V2TableRow { value: "*", display_name: "(asterisk) Life of the \"unit\"", definition: "", comment_usage_note: "Used for blood products. Usage Note: Deprecated March 2016; use code LU instead.", status: "D" },
        "30M" => V2TableRow { value: "30M", display_name: "30 minutes", definition: "", comment_usage_note: "", status: "" },
        "1H" => V2TableRow { value: "1H", display_name: "1 hour", definition: "", comment_usage_note: "", status: "" },
        "2H" => V2TableRow { value: "2H", display_name: "2 hours", definition: "", comment_usage_note: "", status: "" },
        "2.5H" => V2TableRow { value: "2.5H", display_name: "2 1/2 hours", definition: "", comment_usage_note: "", status: "" },
        "3H" => V2TableRow { value: "3H", display_name: "3 hours", definition: "", comment_usage_note: "", status: "" },
        "4H" => V2TableRow { value: "4H", display_name: "4 hours", definition: "", comment_usage_note: "", status: "" },
        "5H" => V2TableRow { value: "5H", display_name: "5 hours", definition: "", comment_usage_note: "", status: "" },
        "6H" => V2TableRow { value: "6H", display_name: "6 hours", definition: "", comment_usage_note: "", status: "" },
        "7H" => V2TableRow { value: "7H", display_name: "7 hours", definition: "", comment_usage_note: "", status: "" },
        "8H" => V2TableRow { value: "8H", display_name: "8 hours", definition: "", comment_usage_note: "", status: "" },
        "12H" => V2TableRow { value: "12H", display_name: "12 hours", definition: "", comment_usage_note: "", status: "" },
        "24H" => V2TableRow { value: "24H", display_name: "24 hours", definition: "", comment_usage_note: "", status: "" },
        "2D" => V2TableRow { value: "2D", display_name: "2 days", definition: "", comment_usage_note: "", status: "" },
        "3D" => V2TableRow { value: "3D", display_name: "3 days", definition: "", comment_usage_note: "", status: "" },
        "4D" => V2TableRow { value: "4D", display_name: "4 days", definition: "", comment_usage_note: "", status: "" },
        "5D" => V2TableRow { value: "5D", display_name: "5 days", definition: "", comment_usage_note: "", status: "" },
        "6D" => V2TableRow { value: "6D", display_name: "6 days", definition: "", comment_usage_note: "", status: "" },
        "1W" => V2TableRow { value: "1W", display_name: "1 week", definition: "", comment_usage_note: "", status: "" },
        "2W" => V2TableRow { value: "2W", display_name: "2 weeks", definition: "", comment_usage_note: "", status: "" },
        "3W" => V2TableRow { value: "3W", display_name: "3 weeks", definition: "", comment_usage_note: "", status: "" },
        "4W" => V2TableRow { value: "4W", display_name: "4 weeks", definition: "", comment_usage_note: "", status: "" },
        "1L" => V2TableRow { value: "1L", display_name: "1 months (30 days)", definition: "", comment_usage_note: "", status: "" },
        "2L" => V2TableRow { value: "2L", display_name: "2 months", definition: "", comment_usage_note: "", status: "" },
        "3L" => V2TableRow { value: "3L", display_name: "3 months", definition: "", comment_usage_note: "", status: "" },
        "LU" => V2TableRow { value: "LU", display_name: "Life of the \"unit\" Life of th (for blood products).", definition: "e \"unit\" Usage Note: I update for bl no specific d can be specif 43 “Point Ver field definit", comment_usage_note: "n a master file ood products, uration value ied (see OM1- sus Interval” ion).", status: "" },
        "PT" => V2TableRow { value: "PT", display_name: "point in time To identif measures a point in t This is a synonym fo \"spot\" or \"random\" a applied to measuremen", definition: "y t a ime. r s urine ts.", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0256: V2Table = V2Table {
    number: 256,
    metadata: &super::metadata::TABLE_0256_METADATA,
    rows: phf_map! {
        "BS" => V2TableRow { value: "BS", display_name: "Baseline (time just before the challenge)", definition: "", comment_usage_note: "", status: "" },
        "PEAK" => V2TableRow { value: "PEAK", display_name: "The time post drug dose at which the highest drug level is reached (differs by drug)", definition: "", comment_usage_note: "", status: "" },
        "TROUGH" => V2TableRow { value: "TROUGH", display_name: "The time post drug dose at which the lowest drug level is reached (varies with drug)", definition: "", comment_usage_note: "", status: "" },
        "RANDOM" => V2TableRow { value: "RANDOM", display_name: "Time from the challenge, or dose not specified. (random)", definition: "", comment_usage_note: "", status: "" },
        "1M" => V2TableRow { value: "1M", display_name: "1 minute post challenge", definition: "", comment_usage_note: "", status: "" },
        "2M" => V2TableRow { value: "2M", display_name: "2 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "3M" => V2TableRow { value: "3M", display_name: "3 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "4M" => V2TableRow { value: "4M", display_name: "4 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "5M" => V2TableRow { value: "5M", display_name: "5 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "6M" => V2TableRow { value: "6M", display_name: "6 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "7M" => V2TableRow { value: "7M", display_name: "7 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "8M" => V2TableRow { value: "8M", display_name: "8 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "9M" => V2TableRow { value: "9M", display_name: "9 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "10M" => V2TableRow { value: "10M", display_name: "10 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "15M" => V2TableRow { value: "15M", display_name: "15 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "20M" => V2TableRow { value: "20M", display_name: "20 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "25M" => V2TableRow { value: "25M", display_name: "25 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "30M" => V2TableRow { value: "30M", display_name: "30 minutes post challenge", definition: "", comment_usage_note: "", status: "" },
        "1H" => V2TableRow { value: "1H", display_name: "1 hour post challenge", definition: "", comment_usage_note: "", status: "" },
        "2H" => V2TableRow { value: "2H", display_name: "2 hours post challenge", definition: "", comment_usage_note: "", status: "" },
        "2.5H" => V2TableRow { value: "2.5H", display_name: "2 1/2 hours post challenge", definition: "", comment_usage_note: "", status: "" },
        "3H" => V2TableRow { value: "3H", display_name: "3 hours post challenge", definition: "", comment_usage_note: "", status: "" },
        "4H" => V2TableRow { value: "4H", display_name: "4 hours post challenge", definition: "", comment_usage_note: "", status: "" },
        "5H" => V2TableRow { value: "5H", display_name: "5 hours post challenge", definition: "", comment_usage_note: "", status: "" },
        "6H" => V2TableRow { value: "6H", display_name: "6 hours post challenge", definition: "", comment_usage_note: "", status: "" },
        "7H" => V2TableRow { value: "7H", display_name: "7 hours post challenge", definition: "", comment_usage_note: "", status: "" },
        "8H" => V2TableRow { value: "8H", display_name: "8 hours post challenge", definition: "", comment_usage_note: "", status: "" },
        "12H" => V2TableRow { value: "12H", display_name: "12 hours post challenge", definition: "", comment_usage_note: "", status: "" },
        "24H" => V2TableRow { value: "24H", display_name: "24 hours post challenge", definition: "", comment_usage_note: "", status: "" },
        "2D" => V2TableRow { value: "2D", display_name: "2 days", definition: "", comment_usage_note: "", status: "" },
        "3D" => V2TableRow { value: "3D", display_name: "3 days", definition: "", comment_usage_note: "", status: "" },
        "4D" => V2TableRow { value: "4D", display_name: "4 days", definition: "", comment_usage_note: "", status: "" },
        "5D" => V2TableRow { value: "5D", display_name: "5 days", definition: "", comment_usage_note: "", status: "" },
        "6D" => V2TableRow { value: "6D", display_name: "6 days", definition: "", comment_usage_note: "", status: "" },
        "7D" => V2TableRow { value: "7D", display_name: "7 days", definition: "", comment_usage_note: "", status: "" },
        "1W" => V2TableRow { value: "1W", display_name: "1 week", definition: "", comment_usage_note: "", status: "" },
        "10D" => V2TableRow { value: "10D", display_name: "10 days", definition: "", comment_usage_note: "", status: "" },
        "2W" => V2TableRow { value: "2W", display_name: "2 weeks", definition: "", comment_usage_note: "", status: "" },
        "3W" => V2TableRow { value: "3W", display_name: "3 weeks", definition: "", comment_usage_note: "", status: "" },
        "4W" => V2TableRow { value: "4W", display_name: "4 weeks", definition: "", comment_usage_note: "", status: "" },
        "1L" => V2TableRow { value: "1L", display_name: "1 month (30 days) post challenge", definition: "", comment_usage_note: "", status: "" },
        "2L" => V2TableRow { value: "2L", display_name: "2 months (60 days) post challenge", definition: "", comment_usage_note: "", status: "" },
        "3L" => V2TableRow { value: "3L", display_name: "3 months (90 days) post challenge", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0257: V2Table = V2Table {
    number: 257,
    metadata: &super::metadata::TABLE_0257_METADATA,
    rows: phf_map! {
        "CFST" => V2TableRow { value: "CFST", display_name: "Fasting (no calorie intake) for the period specified in the time component of the term, e.g., 1H POST CFST", definition: "", comment_usage_note: "", status: "" },
        "EXCZ" => V2TableRow { value: "EXCZ", display_name: "Exercise undertaken as challenge (can be quantified)", definition: "", comment_usage_note: "", status: "" },
        "FFST" => V2TableRow { value: "FFST", display_name: "No fluid intake for the period specified in the time component of the term", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0258: V2Table = V2Table {
    number: 258,
    metadata: &super::metadata::TABLE_0258_METADATA,
    rows: phf_map! {
        "CONTROL" => V2TableRow { value: "CONTROL", display_name: "Control", definition: "", comment_usage_note: "", status: "" },
        "PATIENT" => V2TableRow { value: "PATIENT", display_name: "Patient", definition: "", comment_usage_note: "", status: "" },
        "DONOR" => V2TableRow { value: "DONOR", display_name: "Donor", definition: "", comment_usage_note: "", status: "" },
        "BPU" => V2TableRow { value: "BPU", display_name: "Blood product unit", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0260: V2Table = V2Table {
    number: 260,
    metadata: &super::metadata::TABLE_0260_METADATA,
    rows: phf_map! {
        "N" => V2TableRow { value: "N", display_name: "Nursing Unit", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Room", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Bed", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Exam Room", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Operating Room", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Clinic", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Department", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Other Location", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0261: V2Table = V2Table {
    number: 261,
    metadata: &super::metadata::TABLE_0261_METADATA,
    rows: phf_map! {
        "OXY" => V2TableRow { value: "OXY", display_name: "Oxygen", definition: "", comment_usage_note: "", status: "" },
        "SUC" => V2TableRow { value: "SUC", display_name: "Suction", definition: "", comment_usage_note: "", status: "" },
        "VIT" => V2TableRow { value: "VIT", display_name: "Vital signs monitor", definition: "", comment_usage_note: "", status: "" },
        "INF" => V2TableRow { value: "INF", display_name: "Infusion pump", definition: "", comment_usage_note: "", status: "" },
        "IVP" => V2TableRow { value: "IVP", display_name: "IV pump", definition: "", comment_usage_note: "", status: "" },
        "EEG" => V2TableRow { value: "EEG", display_name: "Electro-Encephalogram", definition: "", comment_usage_note: "", status: "" },
        "EKG" => V2TableRow { value: "EKG", display_name: "Electro-Cardiogram", definition: "", comment_usage_note: "", status: "" },
        "VEN" => V2TableRow { value: "VEN", display_name: "Ventilator", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0262: V2Table = V2Table {
    number: 262,
    metadata: &super::metadata::TABLE_0262_METADATA,
    rows: phf_map! {
        "F" => V2TableRow { value: "F", display_name: "Isolation", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Private room", definition: "", comment_usage_note: "", status: "" },
        "J" => V2TableRow { value: "J", display_name: "Private room - medically justified", definition: "", comment_usage_note: "", status: "" },
        "Q" => V2TableRow { value: "Q", display_name: "Private room - due to overflow", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Semi-private room", definition: "", comment_usage_note: "", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Ward", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0263: V2Table = V2Table {
    number: 263,
    metadata: &super::metadata::TABLE_0263_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Ambulatory", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Emergency", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Isolation", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Intensive care", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Critical care", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Routine", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Surgery", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0265: V2Table = V2Table {
    number: 265,
    metadata: &super::metadata::TABLE_0265_METADATA,
    rows: phf_map! {
        "AMB" => V2TableRow { value: "AMB", display_name: "Ambulatory", definition: "", comment_usage_note: "", status: "" },
        "PSY" => V2TableRow { value: "PSY", display_name: "Psychiatric", definition: "", comment_usage_note: "", status: "" },
        "PPS" => V2TableRow { value: "PPS", display_name: "Pediatric psychiatric", definition: "", comment_usage_note: "", status: "" },
        "REH" => V2TableRow { value: "REH", display_name: "Rehabilitation", definition: "", comment_usage_note: "", status: "" },
        "PRE" => V2TableRow { value: "PRE", display_name: "Pediatric rehabilitation", definition: "", comment_usage_note: "", status: "" },
        "ISO" => V2TableRow { value: "ISO", display_name: "Isolation", definition: "", comment_usage_note: "", status: "" },
        "OBG" => V2TableRow { value: "OBG", display_name: "Obstetrics, gynecology", definition: "", comment_usage_note: "", status: "" },
        "PIN" => V2TableRow { value: "PIN", display_name: "Pediatric/neonatal intensive care", definition: "", comment_usage_note: "", status: "" },
        "INT" => V2TableRow { value: "INT", display_name: "Intensive care", definition: "", comment_usage_note: "", status: "" },
        "SUR" => V2TableRow { value: "SUR", display_name: "Surgery", definition: "", comment_usage_note: "", status: "" },
        "PSI" => V2TableRow { value: "PSI", display_name: "Psychiatric intensive care", definition: "", comment_usage_note: "", status: "" },
        "EDI" => V2TableRow { value: "EDI", display_name: "Education", definition: "", comment_usage_note: "", status: "" },
        "CAR" => V2TableRow { value: "CAR", display_name: "Coronary/cardiac care", definition: "", comment_usage_note: "", status: "" },
        "NBI" => V2TableRow { value: "NBI", display_name: "Newborn, nursery, infants", definition: "", comment_usage_note: "", status: "" },
        "CCR" => V2TableRow { value: "CCR", display_name: "Critical care", definition: "", comment_usage_note: "", status: "" },
        "PED" => V2TableRow { value: "PED", display_name: "Pediatrics", definition: "", comment_usage_note: "", status: "" },
        "EMR" => V2TableRow { value: "EMR", display_name: "Emergency", definition: "", comment_usage_note: "", status: "" },
        "OBS" => V2TableRow { value: "OBS", display_name: "Observation", definition: "", comment_usage_note: "", status: "" },
        "WIC" => V2TableRow { value: "WIC", display_name: "Walk-in clinic", definition: "", comment_usage_note: "", status: "" },
        "PHY" => V2TableRow { value: "PHY", display_name: "General/family practice", definition: "", comment_usage_note: "", status: "" },
        "ALC" => V2TableRow { value: "ALC", display_name: "Allergy", definition: "", comment_usage_note: "", status: "" },
        "FPC" => V2TableRow { value: "FPC", display_name: "Family planning", definition: "", comment_usage_note: "", status: "" },
        "CHI" => V2TableRow { value: "CHI", display_name: "Chiropractic", definition: "", comment_usage_note: "", status: "" },
        "CAN" => V2TableRow { value: "CAN", display_name: "Cancer", definition: "", comment_usage_note: "", status: "" },
        "NAT" => V2TableRow { value: "NAT", display_name: "Naturopathic", definition: "", comment_usage_note: "", status: "" },
        "OTH" => V2TableRow { value: "OTH", display_name: "Other specialty", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0267: V2Table = V2Table {
    number: 267,
    metadata: &super::metadata::TABLE_0267_METADATA,
    rows: phf_map! {
        "SAT" => V2TableRow { value: "SAT", display_name: "Saturday", definition: "", comment_usage_note: "", status: "" },
        "SUN" => V2TableRow { value: "SUN", display_name: "Sunday", definition: "", comment_usage_note: "", status: "" },
        "MON" => V2TableRow { value: "MON", display_name: "Monday", definition: "", comment_usage_note: "", status: "" },
        "TUE" => V2TableRow { value: "TUE", display_name: "Tuesday", definition: "", comment_usage_note: "", status: "" },
        "WED" => V2TableRow { value: "WED", display_name: "Wednesday", definition: "", comment_usage_note: "", status: "" },
        "THU" => V2TableRow { value: "THU", display_name: "Thursday", definition: "", comment_usage_note: "", status: "" },
        "FRI" => V2TableRow { value: "FRI", display_name: "Friday", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0268: V2Table = V2Table {
    number: 268,
    metadata: &super::metadata::TABLE_0268_METADATA,
    rows: phf_map! {
        "X" => V2TableRow { value: "X", display_name: "Override not allowed", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Override allowed", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Override required", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0269: V2Table = V2Table {
    number: 269,
    metadata: &super::metadata::TABLE_0269_METADATA,
    rows: phf_map! {
        "O" => V2TableRow { value: "O", display_name: "Charge on Order", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Charge on Result", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0270: V2Table = V2Table {
    number: 270,
    metadata: &super::metadata::TABLE_0270_METADATA,
    rows: phf_map! {
        "AR" => V2TableRow { value: "AR", display_name: "Autopsy report", definition: "", comment_usage_note: "", status: "" },
        "CD" => V2TableRow { value: "CD", display_name: "Cardiodiagnostics", definition: "", comment_usage_note: "", status: "" },
        "CN" => V2TableRow { value: "CN", display_name: "Consultation", definition: "", comment_usage_note: "", status: "" },
        "DI" => V2TableRow { value: "DI", display_name: "Diagnostic imaging", definition: "", comment_usage_note: "", status: "" },
        "DS" => V2TableRow { value: "DS", display_name: "Discharge summary", definition: "", comment_usage_note: "", status: "" },
        "ED" => V2TableRow { value: "ED", display_name: "Emergency department report", definition: "", comment_usage_note: "", status: "" },
        "HP" => V2TableRow { value: "HP", display_name: "History and physical examination", definition: "", comment_usage_note: "", status: "" },
        "OP" => V2TableRow { value: "OP", display_name: "Operative report", definition: "", comment_usage_note: "", status: "" },
        "PC" => V2TableRow { value: "PC", display_name: "Psychiatric consultation", definition: "", comment_usage_note: "", status: "" },
        "PH" => V2TableRow { value: "PH", display_name: "Psychiatric history and physical examination", definition: "", comment_usage_note: "", status: "" },
        "PN" => V2TableRow { value: "PN", display_name: "Procedure note", definition: "", comment_usage_note: "", status: "" },
        "PR" => V2TableRow { value: "PR", display_name: "Progress note", definition: "", comment_usage_note: "", status: "" },
        "SP" => V2TableRow { value: "SP", display_name: "Surgical pathology", definition: "", comment_usage_note: "", status: "" },
        "TS" => V2TableRow { value: "TS", display_name: "Transfer summary", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0271: V2Table = V2Table {
    number: 271,
    metadata: &super::metadata::TABLE_0271_METADATA,
    rows: phf_map! {
        "DI" => V2TableRow { value: "DI", display_name: "Dictated", definition: "", comment_usage_note: "", status: "" },
        "DO" => V2TableRow { value: "DO", display_name: "Documented", definition: "", comment_usage_note: "", status: "" },
        "IP" => V2TableRow { value: "IP", display_name: "In Progress", definition: "", comment_usage_note: "", status: "" },
        "IN" => V2TableRow { value: "IN", display_name: "Incomplete", definition: "", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Pre-authenticated", definition: "", comment_usage_note: "", status: "" },
        "AU" => V2TableRow { value: "AU", display_name: "Authenticated", definition: "", comment_usage_note: "", status: "" },
        "LA" => V2TableRow { value: "LA", display_name: "Legally authenticated", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0272: V2Table = V2Table {
    number: 272,
    metadata: &super::metadata::TABLE_0272_METADATA,
    rows: phf_map! {
        "V" => V2TableRow { value: "V", display_name: "Very restricted", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Restricted", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Usual control", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0273: V2Table = V2Table {
    number: 273,
    metadata: &super::metadata::TABLE_0273_METADATA,
    rows: phf_map! {
        "AV" => V2TableRow { value: "AV", display_name: "Available for patient care", definition: "", comment_usage_note: "", status: "" },
        "CA" => V2TableRow { value: "CA", display_name: "Deleted", definition: "", comment_usage_note: "", status: "" },
        "OB" => V2TableRow { value: "OB", display_name: "Obsolete", definition: "", comment_usage_note: "", status: "" },
        "UN" => V2TableRow { value: "UN", display_name: "Unavailable for patient care", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0275: V2Table = V2Table {
    number: 275,
    metadata: &super::metadata::TABLE_0275_METADATA,
    rows: phf_map! {
        "AC" => V2TableRow { value: "AC", display_name: "A ct iv e", definition: "", comment_usage_note: "", status: "" },
        "AA" => V2TableRow { value: "AA", display_name: "Active and archived", definition: "", comment_usage_note: "", status: "" },
        "AR" => V2TableRow { value: "AR", display_name: "Archived (not active)", definition: "", comment_usage_note: "", status: "" },
        "PU" => V2TableRow { value: "PU", display_name: "Purged", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0276: V2Table = V2Table {
    number: 276,
    metadata: &super::metadata::TABLE_0276_METADATA,
    rows: phf_map! {
        "ROUTINE" => V2TableRow { value: "ROUTINE", display_name: "Routine appointment - default if not valued", definition: "", comment_usage_note: "", status: "" },
        "WALKIN" => V2TableRow { value: "WALKIN", display_name: "A previously unscheduled walk-in visit", definition: "", comment_usage_note: "", status: "" },
        "CHECK" => V2TableRow { value: "CHECK", display_name: "UP A routine check-up, such as an annual physical", definition: "", comment_usage_note: "", status: "" },
        "FOLLOWUP" => V2TableRow { value: "FOLLOWUP", display_name: "A follow up visit from a previous appointment", definition: "", comment_usage_note: "", status: "" },
        "EMERGENCY" => V2TableRow { value: "EMERGENCY", display_name: "Emergency appointment", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0277: V2Table = V2Table {
    number: 277,
    metadata: &super::metadata::TABLE_0277_METADATA,
    rows: phf_map! {
        "Normal" => V2TableRow { value: "Normal", display_name: "Routine schedule request type – default if not valued", definition: "", comment_usage_note: "", status: "" },
        "Tentative" => V2TableRow { value: "Tentative", display_name: "A request for a tentative (e.g., “penciled in”) appointment", definition: "", comment_usage_note: "", status: "" },
        "Complete" => V2TableRow { value: "Complete", display_name: "A request to add a completed appointment, used to maintain records of completed appointments that did not appear in the schedule (e.g., STAT, walk-in, etc.)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0278: V2Table = V2Table {
    number: 278,
    metadata: &super::metadata::TABLE_0278_METADATA,
    rows: phf_map! {
        "Pending" => V2TableRow { value: "Pending", display_name: "Pending", definition: "Appointment has not yet been confirmed", comment_usage_note: "", status: "" },
        "Waitlist" => V2TableRow { value: "Waitlist", display_name: "Waitlist", definition: "Appointment has been placed on a waiting list for a particular slot, or set of slots", comment_usage_note: "", status: "" },
        "Booked" => V2TableRow { value: "Booked", display_name: "Booked", definition: "The indicated appointment is booked", comment_usage_note: "", status: "" },
        "Started" => V2TableRow { value: "Started", display_name: "Started", definition: "The indicated appointment has begun and is currently in progress", comment_usage_note: "", status: "" },
        "Complete" => V2TableRow { value: "Complete", display_name: "Complete", definition: "The indicated appointment has completed normally (was not discontinued, canceled, or deleted)", comment_usage_note: "", status: "" },
        "Cancelled" => V2TableRow { value: "Cancelled", display_name: "Cancelled The indica occurring", definition: "ted appointment was stopped from (canceled prior to starting)", comment_usage_note: "", status: "" },
        "DC" => V2TableRow { value: "DC", display_name: "Discontinued The indica while in p or discont", definition: "ted appointment was discontinued (DC’ed Replaced by rogress, discontinued parent appointment, code inued child appointment) 'Discontinue d'", comment_usage_note: "D", status: "" },
        "Discontinue" => V2TableRow { value: "Discontinue", display_name: "Discontinued The indica", definition: "ted appointment was discontinued (DC’ed", comment_usage_note: "", status: "" },
        "d" => V2TableRow { value: "d", display_name: "while in p or discont", definition: "rogress, discontinued parent appointment, inued child appointment)", comment_usage_note: "", status: "" },
        "Deleted" => V2TableRow { value: "Deleted", display_name: "Deleted The indica filler app", definition: "ted appointment was deleted from the lication", comment_usage_note: "", status: "" },
        "Blocked" => V2TableRow { value: "Blocked", display_name: "Blocked The indica", definition: "ted time slot(s) is(are) blocked", comment_usage_note: "", status: "" },
        "Overbook" => V2TableRow { value: "Overbook", display_name: "Overbook The appoin confirmed", definition: "tment has been confirmed; however it is in an overbooked state", comment_usage_note: "", status: "" },
        "Noshow" => V2TableRow { value: "Noshow", display_name: "Noshow The patien", definition: "t did not show up for the appointment", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0279: V2Table = V2Table {
    number: 279,
    metadata: &super::metadata::TABLE_0279_METADATA,
    rows: phf_map! {
        "No" => V2TableRow { value: "No", display_name: "Substitution of this resource is not allowed", definition: "", comment_usage_note: "", status: "" },
        "Confirm" => V2TableRow { value: "Confirm", display_name: "Contact the Placer Contact Person prior to making any substitutions of this resource", definition: "", comment_usage_note: "", status: "" },
        "Notify" => V2TableRow { value: "Notify", display_name: "Notify the Placer Contact Person, through normal institutional procedures, that a substitution of this resource has been made", definition: "", comment_usage_note: "", status: "" },
        "Yes" => V2TableRow { value: "Yes", display_name: "Substitution of this resource is allowed", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0280: V2Table = V2Table {
    number: 280,
    metadata: &super::metadata::TABLE_0280_METADATA,
    rows: phf_map! {
        "S" => V2TableRow { value: "S", display_name: "STAT", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "ASAP", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Routine", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0281: V2Table = V2Table {
    number: 281,
    metadata: &super::metadata::TABLE_0281_METADATA,
    rows: phf_map! {
        "Lab" => V2TableRow { value: "Lab", display_name: "Laboratory", definition: "", comment_usage_note: "", status: "" },
        "Rad" => V2TableRow { value: "Rad", display_name: "Radiology", definition: "", comment_usage_note: "", status: "" },
        "Med" => V2TableRow { value: "Med", display_name: "Medical", definition: "", comment_usage_note: "", status: "" },
        "Skn" => V2TableRow { value: "Skn", display_name: "Skilled Nursing", definition: "", comment_usage_note: "", status: "" },
        "Psy" => V2TableRow { value: "Psy", display_name: "Psychiatric", definition: "", comment_usage_note: "", status: "" },
        "Hom" => V2TableRow { value: "Hom", display_name: "Home Care", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0282: V2Table = V2Table {
    number: 282,
    metadata: &super::metadata::TABLE_0282_METADATA,
    rows: phf_map! {
        "WR" => V2TableRow { value: "WR", display_name: "Send Written Report", definition: "", comment_usage_note: "", status: "" },
        "RP" => V2TableRow { value: "RP", display_name: "Return Patient After Evaluation", definition: "", comment_usage_note: "", status: "" },
        "AM" => V2TableRow { value: "AM", display_name: "Assume Management", definition: "", comment_usage_note: "", status: "" },
        "SO" => V2TableRow { value: "SO", display_name: "Second Opinion", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0283: V2Table = V2Table {
    number: 283,
    metadata: &super::metadata::TABLE_0283_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Accepted", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Pending", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Rejected", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Expired", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0284: V2Table = V2Table {
    number: 284,
    metadata: &super::metadata::TABLE_0284_METADATA,
    rows: phf_map! {
        "I" => V2TableRow { value: "I", display_name: "Inpatient", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Outpatient", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Ambulatory", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Emergency", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0286: V2Table = V2Table {
    number: 286,
    metadata: &super::metadata::TABLE_0286_METADATA,
    rows: phf_map! {
        "RP" => V2TableRow { value: "RP", display_name: "Referring Provider", definition: "", comment_usage_note: "", status: "" },
        "PP" => V2TableRow { value: "PP", display_name: "Primary Care Provider", definition: "", comment_usage_note: "", status: "" },
        "CP" => V2TableRow { value: "CP", display_name: "Consulting Provider", definition: "", comment_usage_note: "", status: "" },
        "RT" => V2TableRow { value: "RT", display_name: "Referred to Provider", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0287: V2Table = V2Table {
    number: 287,
    metadata: &super::metadata::TABLE_0287_METADATA,
    rows: phf_map! {
        "AD" => V2TableRow { value: "AD", display_name: "ADD", definition: "", comment_usage_note: "", status: "" },
        "CO" => V2TableRow { value: "CO", display_name: "CORRECT", definition: "", comment_usage_note: "", status: "" },
        "DE" => V2TableRow { value: "DE", display_name: "DELETE", definition: "", comment_usage_note: "", status: "" },
        "LI" => V2TableRow { value: "LI", display_name: "LINK", definition: "", comment_usage_note: "", status: "" },
        "SP" => V2TableRow { value: "SP", display_name: "Used in Snapshot mode", definition: "Declares when segment falls under snapshot mode handling, i.e. all elements that were previously sent will be sent, not just any changes", comment_usage_note: "Snapshot mode is the expected default; use this code to explicitly state that.", status: "N" },
        "UC" => V2TableRow { value: "UC", display_name: "UNCHANGED", definition: "UNCHANGED *", comment_usage_note: "The UNCHANGED action code is used to signify to the applications programs that this particular segment includes no information to be modified. It is supplied in order to identify the correct record for which the following modification is intended", status: "" },
        "UN" => V2TableRow { value: "UN", display_name: "UNLINK", definition: "", comment_usage_note: "", status: "" },
        "UP" => V2TableRow { value: "UP", display_name: "UPDATE", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0291: V2Table = V2Table {
    number: 291,
    metadata: &super::metadata::TABLE_0291_METADATA,
    rows: phf_map! {
        "BASIC" => V2TableRow { value: "BASIC", display_name: "ISDN PCM audio data", definition: "", comment_usage_note: "", status: "D" },
        "DICOM" => V2TableRow { value: "DICOM", display_name: "Digital Imaging and Communications in Medicine", definition: "", comment_usage_note: "", status: "D" },
        "FAX" => V2TableRow { value: "FAX", display_name: "Facsimile data", definition: "", comment_usage_note: "", status: "D" },
        "GIF" => V2TableRow { value: "GIF", display_name: "Graphics Interchange Format", definition: "", comment_usage_note: "", status: "D" },
        "HTML" => V2TableRow { value: "HTML", display_name: "Hypertext Markup Language", definition: "", comment_usage_note: "", status: "D" },
        "JOT" => V2TableRow { value: "JOT", display_name: "Electronic ink data (Jot 1.0 standard)", definition: "", comment_usage_note: "", status: "" },
        "JPEG" => V2TableRow { value: "JPEG", display_name: "Joint Photographic Experts Group", definition: "", comment_usage_note: "", status: "D" },
        "Octet-stream" => V2TableRow { value: "Octet-stream", display_name: "Uninterpreted binary data", definition: "", comment_usage_note: "", status: "D" },
        "PICT" => V2TableRow { value: "PICT", display_name: "PICT format image data", definition: "", comment_usage_note: "", status: "" },
        "PostScript" => V2TableRow { value: "PostScript", display_name: "PostScript program", definition: "", comment_usage_note: "", status: "D" },
        "RTF" => V2TableRow { value: "RTF", display_name: "Rich Text Format", definition: "", comment_usage_note: "", status: "D" },
        "SGML" => V2TableRow { value: "SGML", display_name: "Standard Generalized Markup Language (HL7 V2.3.1 and later)", definition: "", comment_usage_note: "", status: "D" },
        "TIFF" => V2TableRow { value: "TIFF", display_name: "TIFF image data", definition: "", comment_usage_note: "", status: "D" },
        "XML" => V2TableRow { value: "XML", display_name: "Extensible Markup Language (HL7 V2.3.1 and later)", definition: "", comment_usage_note: "", status: "D" },
        "x-hl7-cd" => V2TableRow { value: "x-hl7-cd", display_name: "a- HL7 Clinical Document Architecture", definition: "", comment_usage_note: "Retained for", status: "" },
        "level-one" => V2TableRow { value: "level-one", display_name: "Level One document", definition: "", comment_usage_note: "backwards compatibilit y only as of v2.6 and CDA R 2. Preferred value is text/xml.", status: "" },
    },
};

pub static TABLE_0294: V2Table = V2Table {
    number: 294,
    metadata: &super::metadata::TABLE_0294_METADATA,
    rows: phf_map! {
        "Prefstart" => V2TableRow { value: "Prefstart", display_name: "An indicator that there is a preferred start time for the appointment request, service or resource.", definition: "", comment_usage_note: "In component 2, specify any valid time in the format HHMM, using 24- hour clock notation where HH = hour and MM = minutes", status: "" },
        "Prefend" => V2TableRow { value: "Prefend", display_name: "An indicator that there is a preferred end time for the appointment request, service or resource.", definition: "", comment_usage_note: "In component 2, specify any valid time in the format HHMM, using 24- hour clock notation where HH = hour and MM = minutes", status: "" },
        "Mon" => V2TableRow { value: "Mon", display_name: "An indicator that Monday is or is not preferred for the day on which the appointment will occur.", definition: "", comment_usage_note: "In component 2, specify OK or NO. OK = Preferred appointment day NO = Day is not preferred", status: "" },
        "Tue" => V2TableRow { value: "Tue", display_name: "An indicator that Tuesday is or is not preferred for the day on which the appointment will occur.", definition: "", comment_usage_note: "In component 2, specify OK or NO. OK = Preferred appointment day NO = Day is not preferred", status: "" },
        "Wed" => V2TableRow { value: "Wed", display_name: "An indicator that Wednesday is or is not preferred for the day on which the appointment will occur.", definition: "", comment_usage_note: "In component 2, specify OK or NO. OK = Preferred appointment day NO = Day is not preferred", status: "" },
        "Thu" => V2TableRow { value: "Thu", display_name: "An indicator that Thursday is or is not preferred for the day on which the appointment will occur.", definition: "", comment_usage_note: "In component 2, specify OK or NO. OK = Preferred appointment day NO = Day is not preferred", status: "" },
        "Fri" => V2TableRow { value: "Fri", display_name: "An indicator that Friday is or is not preferred for the day on which the appointment will occur.", definition: "", comment_usage_note: "In component 2, specify OK or NO. OK = Preferred appointment day NO = Day is not preferred", status: "" },
        "Sat" => V2TableRow { value: "Sat", display_name: "An indicator that Saturday is or is not preferred for the day on which the appointment will occur.", definition: "", comment_usage_note: "In component 2, specify OK or NO. OK = Preferred appointment day NO = Day is not preferred", status: "" },
        "Sun" => V2TableRow { value: "Sun", display_name: "An indicator that Sunday is or is not preferred for the day on which the appointment will occur.", definition: "", comment_usage_note: "In component 2, specify OK or NO. OK = Preferred appointment day NO = Day is not preferred", status: "" },
    },
};

pub static TABLE_0298: V2Table = V2Table {
    number: 298,
    metadata: &super::metadata::TABLE_0298_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Pro-rate. Apply this price to this interval, pro-rated by whatever portion of the interval has occurred/been consumed", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Flat-rate. Apply the entire price to this interval, do not pro-rate the price if the full interval has not occurred/been consumed", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0299: V2Table = V2Table {
    number: 299,
    metadata: &super::metadata::TABLE_0299_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "No encoding - data are displayable ASCII characters.", definition: "", comment_usage_note: "", status: "" },
        "Hex" => V2TableRow { value: "Hex", display_name: "Hexadecimal encoding - consecutive pairs of hexadecimal digits represent consecutive single octets.", definition: "", comment_usage_note: "", status: "" },
        "Base64" => V2TableRow { value: "Base64", display_name: "Encoding as defined by MIME (Multipurpose Internet Mail Extensions) standard RFC 1521. Four consecutive ASCII characters represent three consecutive octets of binary data. Base64 utilizes a 65-character subset of US- ASCII, consisting of both the upper and", definition: "Encoding as defined by MIME (Multipurpose Internet Mail Extensions) standard RFC 1521. Four consecutive ASCII characters represent three consecutive octets of binary data. Base64 utilizes a 65- character subset of US- ASCII, consisting of both the upper and lower case alphabetic characters, digits “0” through “9”, “+”, “/”, and “=”.", comment_usage_note: "The Request For Comment (RFC) 1521 standard is available at: http://www.i etf.org/rfc/rf c1521.txt", status: "" },
    },
};

pub static TABLE_0301: V2Table = V2Table {
    number: 301,
    metadata: &super::metadata::TABLE_0301_METADATA,
    rows: phf_map! {
        "CAP" => V2TableRow { value: "CAP", display_name: "College of American Pathologist Accreditation Number", definition: "Allows for the ability to designate organization identifier as a \"CAP\" assigned number (for labs)", comment_usage_note: "Use to identify assigning authority IDs, when an OID is not available.", status: "N" },
        "CLIA" => V2TableRow { value: "CLIA", display_name: "Clinical Laboratory Improvement Amendments", definition: "Clinical Laboratory Improvement Amendments. Allows for the ability to designate organization identifier as a \"CLIA\" assigned number (for labs)", comment_usage_note: "Allows for the ability to designate organization identifier as a “CLIA” assigned number (for labs)", status: "" },
        "CLIP" => V2TableRow { value: "CLIP", display_name: "Clinical laboratory Improvement Program", definition: "Clinical laboratory Improvement Program. Allows for the ability to designate organization identifier as a \"CLIP\" assigned number (for labs).Â Used by US Department of Defense.", comment_usage_note: "Allows for the ability to designate organization identifier as a “CLIP” assigned number (for labs). Used by US Department of Defense.", status: "" },
        "DNS" => V2TableRow { value: "DNS", display_name: "Domain Name System", definition: "An Internet host name, in accordance with RFC 1035; or an IP address. Either in ASCII or as integers, with periods between components (\"dotted\" notation).", comment_usage_note: "An Internet host name, in accordance with RFC 1035; or an IP address. Either in ASCII or as integers, with periods between components (“dotted” notation).", status: "" },
        "EUI64" => V2TableRow { value: "EUI64", display_name: "IEEE 64-bit Extended Unique Identifier manuf often (e.g. IPv4 http: i/tut detai", definition: "IEEE 64-bit Extended Unique Identifier is comprised of a 24- bit company identifier and a 40- bit instance identifier. The value shall be formatted as 16 ASCII HEX digits, for example, “AABBCC1122334455”. The 24- bit company identifier, formally known as Organizationally Unique Identifier (OUI- 24), is guaranteed to be globally unique. The 40 -bit extensions are assigned by acturers. This identifier is unique. T used in equipment interfaces are assign , “MAC” address format for This ident & IPv6). [See equipment //standards.ieee.org/regauth/ou “MAC” addr orials/EUI64.html for a & IPv6). led explanation of th e form at.] http://sta oui/tutori detailed e format.]OU administer Registrati", comment_usage_note: "IEEE 64-bit Extended Unique Identifier is comprised of a 24- bit company identifier and a 40- bit instance identifier. The value shall be formatted as 16 ASCII HEX digits, for example, “AABBCC1122334455”. The 24- bit company identifier, formally known as Organizationally Unique Identifier (OUI- 24), is guaranteed to be globally he 40-bit extensions ed by manufacturers. ifier is often used in interfaces (e.g., ess format for IPv4 [See ndards.ieee.org/regauth/ als/EUI64.htmlfor a xplanation of the I- 24 values are ed by the IEEE on Authority.", status: "" },
        "GUID" => V2TableRow { value: "GUID", display_name: "globally unique Same identifier", definition: "as UUID. Same as U backward of v2.7;", comment_usage_note: "UID. Retained for compatibility only as use UUID instead", status: "" },
        "HCD" => V2TableRow { value: "HCD", display_name: "CEN Healthcare The Coding Identifier Sche", definition: "CEN Healthcare Coding The CEN H me Designator Scheme De backward of v2.7; Assigning", comment_usage_note: "ealthcare Coding signator. Retained for compatibility only as does not identify Authorities", status: "" },
        "HL7" => V2TableRow { value: "HL7", display_name: "HL7 registration schemes", definition: "Retained compatibi HL7 assig Assigning", comment_usage_note: "for backward lity only as of v2.7; ns ISO OIDs for Authorities", status: "" },
        "ISO" => V2TableRow { value: "ISO", display_name: "ISO Object An I Identifier Orga (OID 8824 sepa reco char", definition: "nternational Standards An Intern nization Object Identifier Organizat ), in accordance with ISO/IEC (OID), in . Formatted as decimal digits ISO/IEC 8 rated by periods; decimal d mmended limit of 64 periods; acters 64 charac", comment_usage_note: "ational Standards ion Object Identifier accordance with 824. Formatted as igits separated by recommended limit of ters", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Local Thes defi", definition: "e are reserved for locally Locally d ned coding schemes. identifie compatibi", comment_usage_note: "efined coding entity r.Retained for backward lity only as of v 2.8", status: "" },
        "L,M,N" => V2TableRow { value: "L,M,N", display_name: "Local Thes defi", definition: "e are reserved for locally Locally d ned coding schemes. identifie compatibi", comment_usage_note: "efined coding entity D r.Retained for backward lity only as of v 2.8", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Local Thes defi", definition: "e are reserved for locally Locally d ned coding schemes. identifie compatibi", comment_usage_note: "efined coding entity r.Retained for backward lity only as of v 2.8", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Local Thes defi", definition: "e are reserved for locally Locally d ned coding schemes. identifie compatibi", comment_usage_note: "efined coding entity r.Retained for backward lity only as of v 2.8", status: "" },
        "NPI" => V2TableRow { value: "NPI", display_name: "US National Allo Provider orga Identifier assi prov orga", definition: "ws for the ability to designate Use to id nization identifier as a \"NPI\" authority gned number (lab, any medical not avail ider, can be a person or an important nization) Namespace published", comment_usage_note: "entify assigning N IDs, when an OID is able. Especially in the CNN datatype. maintained and in the US.", status: "" },
        "Random" => V2TableRow { value: "Random", display_name: "Random Usua rand Note used than", definition: "lly a base64 encoded string of Usually a om bits. of random : Random IDs are typically backward for instance identifiers, rather of v2.7; an identifier of an Assigning defined e Authority that issues instance identifiers", comment_usage_note: "base64 encoded string bits.Retained for compatibility only as equivalent to a locally ntity identifier scheme; use L. M, or N instead. Note: Random IDs are typically used for instance identifiers, rather than an identifier of an Assigning Authority that issues instance identifiers Usage Note: Retained for backward compatibility only as of v2.7; equivalent to a locally defined entity identifier scheme; use L. M, or N instead.", status: "" },
        "URI" => V2TableRow { value: "URI", display_name: "Uniform Resource Identifier", definition: "", comment_usage_note: "", status: "" },
        "UUID" => V2TableRow { value: "UUID", display_name: "Universal Unique Identifier", definition: "The DCE Universal Unique Identifier, in accordance with RFC 4122. Recommended format is 32 hexadecimal digits separated by hyphens, in the digit grouping 8- 4- 4-4-12", comment_usage_note: "The DCE Universal Unique Identifier, in accordance with RFC 4122. Recommended format is 32 hexadecimal digits separated by hyphens, in the digit grouping 8-4-4-4-12", status: "" },
        "x400" => V2TableRow { value: "x400", display_name: "X.400 MHS identifier", definition: "An X.400 MHS identifier. Recommended format is in accordance with RFC 1649", comment_usage_note: "Recommended format is in accordance with RFC 1649", status: "" },
        "x500" => V2TableRow { value: "x500", display_name: "X500 directory Name", definition: "An X.500 directory name", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0305: V2Table = V2Table {
    number: 305,
    metadata: &super::metadata::TABLE_0305_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "Clinic", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Department", definition: "", comment_usage_note: "", status: "" },
        "H" => V2TableRow { value: "H", display_name: "Home", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Nursing Unit", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Provider's Office", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Phone", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "SNF", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0309: V2Table = V2Table {
    number: 309,
    metadata: &super::metadata::TABLE_0309_METADATA,
    rows: phf_map! {
        "H" => V2TableRow { value: "H", display_name: "Hospital/institutional", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Physician/profess ional", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Both hospital and physician", definition: "", comment_usage_note: "", status: "" },
        "RX" => V2TableRow { value: "RX", display_name: "Pharmacy", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0311: V2Table = V2Table {
    number: 311,
    metadata: &super::metadata::TABLE_0311_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Permanent", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Temporary", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0315: V2Table = V2Table {
    number: 315,
    metadata: &super::metadata::TABLE_0315_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Yes, patient has a living will", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Yes, patient has a living will but it is not on file", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No, patient does not have a living will and no information was provided", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "No, patient does not have a living will but information was provided", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0316: V2Table = V2Table {
    number: 316,
    metadata: &super::metadata::TABLE_0316_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Yes, patient is a documented donor and documentation is on file", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Yes, patient is a documented donor, but documentation is not on file", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No, patient has not agreed to be a donor", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "No, patient is not a documented donor, but information was provided", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Patient leaves organ donation decision to relatives", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Patient leaves organ donation decision to a specific person", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0317: V2Table = V2Table {
    number: 317,
    metadata: &super::metadata::TABLE_0317_METADATA,
    rows: phf_map! {
        "9900" => V2TableRow { value: "9900", display_name: "Pace spike", definition: "", comment_usage_note: "", status: "" },
        "9901" => V2TableRow { value: "9901", display_name: "SAS marker", definition: "", comment_usage_note: "", status: "" },
        "9902" => V2TableRow { value: "9902", display_name: "Sense marker", definition: "", comment_usage_note: "", status: "" },
        "9903" => V2TableRow { value: "9903", display_name: "Beat marker", definition: "", comment_usage_note: "", status: "" },
        "9904" => V2TableRow { value: "9904", display_name: "etc.", definition: "", comment_usage_note: "", status: "D" },
    },
};

pub static TABLE_0321: V2Table = V2Table {
    number: 321,
    metadata: &super::metadata::TABLE_0321_METADATA,
    rows: phf_map! {
        "TR" => V2TableRow { value: "TR", display_name: "Traditional", definition: "", comment_usage_note: "", status: "" },
        "UD" => V2TableRow { value: "UD", display_name: "Unit Dose", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Floor Stock", definition: "", comment_usage_note: "", status: "" },
        "AD" => V2TableRow { value: "AD", display_name: "Automatic Dispensing", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0322: V2Table = V2Table {
    number: 322,
    metadata: &super::metadata::TABLE_0322_METADATA,
    rows: phf_map! {
        "CP" => V2TableRow { value: "CP", display_name: "Complete", definition: "", comment_usage_note: "", status: "" },
        "RE" => V2TableRow { value: "RE", display_name: "Refused", definition: "", comment_usage_note: "", status: "" },
        "NA" => V2TableRow { value: "NA", display_name: "Not Administered", definition: "", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Partially Administered", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0324: V2Table = V2Table {
    number: 324,
    metadata: &super::metadata::TABLE_0324_METADATA,
    rows: phf_map! {
        "SMK" => V2TableRow { value: "SMK", display_name: "S m ok i ng", definition: "", comment_usage_note: "", status: "" },
        "LIC" => V2TableRow { value: "LIC", display_name: "Licensed", definition: "", comment_usage_note: "", status: "" },
        "IMP" => V2TableRow { value: "IMP", display_name: "Implant: can be used for radiation implant patients", definition: "", comment_usage_note: "", status: "" },
        "SHA" => V2TableRow { value: "SHA", display_name: "Shadow: a temporary holding location that does not physically exist", definition: "", comment_usage_note: "", status: "" },
        "INF" => V2TableRow { value: "INF", display_name: "Infectious disease: this location can be used for isolation", definition: "", comment_usage_note: "", status: "" },
        "PRL" => V2TableRow { value: "PRL", display_name: "Privacy level: indicating the level of private versus non-private room", definition: "", comment_usage_note: "", status: "" },
        "LCR" => V2TableRow { value: "LCR", display_name: "Level of care", definition: "", comment_usage_note: "", status: "" },
        "OVR" => V2TableRow { value: "OVR", display_name: "Overflow", definition: "", comment_usage_note: "", status: "" },
        "STF" => V2TableRow { value: "STF", display_name: "Bed is staffed", definition: "", comment_usage_note: "", status: "" },
        "SET" => V2TableRow { value: "SET", display_name: "Bed is set up", definition: "", comment_usage_note: "", status: "" },
        "GEN" => V2TableRow { value: "GEN", display_name: "Gender of patient(s)", definition: "", comment_usage_note: "", status: "" },
        "TEA" => V2TableRow { value: "TEA", display_name: "Teaching location", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0325: V2Table = V2Table {
    number: 325,
    metadata: &super::metadata::TABLE_0325_METADATA,
    rows: phf_map! {
        "RX" => V2TableRow { value: "RX", display_name: "Nearest pharmacy", definition: "", comment_usage_note: "", status: "" },
        "RX2" => V2TableRow { value: "RX2", display_name: "Second nearest pharmacy", definition: "", comment_usage_note: "", status: "" },
        "LAB" => V2TableRow { value: "LAB", display_name: "Nearest lab", definition: "", comment_usage_note: "", status: "" },
        "LB2" => V2TableRow { value: "LB2", display_name: "Second nearest lab", definition: "", comment_usage_note: "", status: "" },
        "DTY" => V2TableRow { value: "DTY", display_name: "Nearest dietary location", definition: "", comment_usage_note: "", status: "" },
        "ALI" => V2TableRow { value: "ALI", display_name: "Location Alias(es)", definition: "", comment_usage_note: "", status: "" },
        "PAR" => V2TableRow { value: "PAR", display_name: "Parent location", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0326: V2Table = V2Table {
    number: 326,
    metadata: &super::metadata::TABLE_0326_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Account level", definition: "Account level (default)", comment_usage_note: "Usage Note: default", status: "" },
        "V" => V2TableRow { value: "V", display_name: "Visit level", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0329: V2Table = V2Table {
    number: 329,
    metadata: &super::metadata::TABLE_0329_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Actual count", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Estimated (see comment)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0330: V2Table = V2Table {
    number: 330,
    metadata: &super::metadata::TABLE_0330_METADATA,
    rows: phf_map! {
        "510K" => V2TableRow { value: "510K", display_name: "510 (K)", definition: "", comment_usage_note: "", status: "" },
        "510E" => V2TableRow { value: "510E", display_name: "510 (K) exempt", definition: "", comment_usage_note: "", status: "" },
        "PMA" => V2TableRow { value: "PMA", display_name: "Premarketing authorization", definition: "", comment_usage_note: "", status: "" },
        "PRE" => V2TableRow { value: "PRE", display_name: "Preamendment", definition: "", comment_usage_note: "", status: "" },
        "TXN" => V2TableRow { value: "TXN", display_name: "Transitional", definition: "", comment_usage_note: "", status: "" },
        "522S" => V2TableRow { value: "522S", display_name: "Post marketing study (522)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0331: V2Table = V2Table {
    number: 331,
    metadata: &super::metadata::TABLE_0331_METADATA,
    rows: phf_map! {
        "U" => V2TableRow { value: "U", display_name: "User", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Manufacturer", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Distributor", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Agent for a foreign manufacturer", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0332: V2Table = V2Table {
    number: 332,
    metadata: &super::metadata::TABLE_0332_METADATA,
    rows: phf_map! {
        "I" => V2TableRow { value: "I", display_name: "Initiate", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Accept", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0334: V2Table = V2Table {
    number: 334,
    metadata: &super::metadata::TABLE_0334_METADATA,
    rows: phf_map! {
        "PT" => V2TableRow { value: "PT", display_name: "Patient", definition: "", comment_usage_note: "", status: "" },
        "GT" => V2TableRow { value: "GT", display_name: "Guarantor", definition: "", comment_usage_note: "", status: "" },
        "IN" => V2TableRow { value: "IN", display_name: "Insured", definition: "", comment_usage_note: "", status: "" },
        "AP" => V2TableRow { value: "AP", display_name: "Associated party", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0335: V2Table = V2Table {
    number: 335,
    metadata: &super::metadata::TABLE_0335_METADATA,
    rows: phf_map! {
        "Q<integer>S" => V2TableRow { value: "Q<integer>S", display_name: "every <integer> seconds", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "Q<integer>M" => V2TableRow { value: "Q<integer>M", display_name: "every <integer> minutes", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "Q<integer>H" => V2TableRow { value: "Q<integer>H", display_name: "every <integer> hours", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "Q<integer>D" => V2TableRow { value: "Q<integer>D", display_name: "every <integer> days", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "Q<integer>W" => V2TableRow { value: "Q<integer>W", display_name: "every <integer> weeks", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "Q<integer>L" => V2TableRow { value: "Q<integer>L", display_name: "every <integer> months (Lunar cycle)", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "Q<integer>J<day" => V2TableRow { value: "Q<integer>J<day", display_name: "repeats on a particular", definition: "", comment_usage_note: "This is not a real code, but guidelines how", status: "D" },
        "#>" => V2TableRow { value: "#>", display_name: "day of the week,", definition: "", comment_usage_note: "to construct the codes.", status: "" },
        "BID" => V2TableRow { value: "BID", display_name: "twice a day at institution-specified times", definition: "", comment_usage_note: "(e.g., 9AM-4PM)", status: "" },
        "TID" => V2TableRow { value: "TID", display_name: "three times a day at institution-specified times", definition: "", comment_usage_note: "(e.g., 9AM-4PM-9PM)", status: "" },
        "QID" => V2TableRow { value: "QID", display_name: "four times a day at institution-specified times", definition: "", comment_usage_note: "(e.g., 9AM-11AM-4PM-9PM)", status: "" },
        "xID" => V2TableRow { value: "xID", display_name: "\"X\" times per day at institution-specified times, where X is a numeral 5 or greater.", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "QAM" => V2TableRow { value: "QAM", display_name: "in the morning at institution-specified time", definition: "", comment_usage_note: "", status: "" },
        "QSHIFT" => V2TableRow { value: "QSHIFT", display_name: "during each of three eight-hour shifts at institution-specified times", definition: "", comment_usage_note: "", status: "" },
        "QOD" => V2TableRow { value: "QOD", display_name: "every other day", definition: "", comment_usage_note: "(same as Q2D)", status: "" },
        "QHS" => V2TableRow { value: "QHS", display_name: "every day before the hour of sleep", definition: "", comment_usage_note: "", status: "" },
        "QPM" => V2TableRow { value: "QPM", display_name: "in the evening at institution-specified time", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "service is provided continuously between start time and stop time", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "<spec> for future use, where <spec> is an interval specification as defined by the UNIX cron specification.", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "PRN" => V2TableRow { value: "PRN", display_name: "given as needed", definition: "", comment_usage_note: "", status: "" },
        "PRNxxx" => V2TableRow { value: "PRNxxx", display_name: "where xxx is some frequency code", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "Once" => V2TableRow { value: "Once", display_name: "one time only.", definition: "", comment_usage_note: "This is also the default when this component is null.", status: "" },
        "Meal" => V2TableRow { value: "Meal", display_name: "Related <timing>C", definition: "", comment_usage_note: "This is not a real code, but guidelines how", status: "D" },
        "Timings" => V2TableRow { value: "Timings", display_name: "(\"cum\")<meal>", definition: "", comment_usage_note: "to construct the codes.", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Ante (before)", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "P" => V2TableRow { value: "P", display_name: "Post (after)", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "I" => V2TableRow { value: "I", display_name: "Inter", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "M" => V2TableRow { value: "M", display_name: "Cibus Matutinus (breakfast)", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "D" => V2TableRow { value: "D", display_name: "Cibus Diurnus (lunch)", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "V" => V2TableRow { value: "V", display_name: "Cibus Vespertinus (dinner)", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
    },
};

pub static TABLE_0336: V2Table = V2Table {
    number: 336,
    metadata: &super::metadata::TABLE_0336_METADATA,
    rows: phf_map! {
        "S" => V2TableRow { value: "S", display_name: "Second Opinion", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Patient Preference", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Provider Ordered", definition: "", comment_usage_note: "", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Work Load", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0337: V2Table = V2Table {
    number: 337,
    metadata: &super::metadata::TABLE_0337_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "Certified", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Eligible", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0338: V2Table = V2Table {
    number: 338,
    metadata: &super::metadata::TABLE_0338_METADATA,
    rows: phf_map! {
        "CY" => V2TableRow { value: "CY", display_name: "County number", definition: "", comment_usage_note: "", status: "" },
        "DEA" => V2TableRow { value: "DEA", display_name: "Drug Enforcement Agency no.", definition: "", comment_usage_note: "", status: "" },
        "GL" => V2TableRow { value: "GL", display_name: "General ledger number", definition: "", comment_usage_note: "", status: "" },
        "LI" => V2TableRow { value: "LI", display_name: "Labor and industries number", definition: "", comment_usage_note: "", status: "" },
        "L&I" => V2TableRow { value: "L&I", display_name: "Labor and industries number", definition: "", comment_usage_note: "Deprecated as of v 2.5; Use LI instead", status: "D" },
        "MCD" => V2TableRow { value: "MCD", display_name: "Medicaid number", definition: "", comment_usage_note: "", status: "" },
        "MCR" => V2TableRow { value: "MCR", display_name: "Medicare number", definition: "", comment_usage_note: "", status: "" },
        "QA" => V2TableRow { value: "QA", display_name: "QA number", definition: "", comment_usage_note: "", status: "" },
        "SL" => V2TableRow { value: "SL", display_name: "State license number", definition: "", comment_usage_note: "", status: "" },
        "TAX" => V2TableRow { value: "TAX", display_name: "Tax ID number", definition: "", comment_usage_note: "", status: "" },
        "TRL" => V2TableRow { value: "TRL", display_name: "Training license number", definition: "", comment_usage_note: "", status: "" },
        "UPIN" => V2TableRow { value: "UPIN", display_name: "Unique physician ID no.", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0339: V2Table = V2Table {
    number: 339,
    metadata: &super::metadata::TABLE_0339_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Service is subject to medical necessity procedures", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Patient has been informed of responsibility, and agrees to pay for service", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Patient has been informed of responsibility, and asks that the payer be billed", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Advanced Beneficiary Notice has not been signed", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0340: V2Table = V2Table {
    number: 340,
    metadata: &super::metadata::TABLE_0340_METADATA,
    rows: phf_map! {
        "CPTM" => V2TableRow { value: "CPTM", display_name: "CPT Modifier Code", definition: "", comment_usage_note: "Available for the AMA at the address listed for CPT above. These codes are found in Appendix A of CPT 2000 Standard Edition. (CPT 2000 Standard Edition, American Medical Associatio n, Chicago, IL)", status: "D" },
        "HPC" => V2TableRow { value: "HPC", display_name: "CMS (formerly HCFA) Procedure Codes (HCPCS)", definition: "", comment_usage_note: "Health Care Financing Administration (HCFA) Common Procedure Coding System (HCPCS) including modifiers. Usage Note: The HCPCS code is divided into three \"levels.\" Level I includes the entire CPT-4 code by reference. Level II includes the American Dental Association’s Current Dental Terminology (CDT-2 ) code by reference. Level II also includes the genuine HCPCS codes, approved and maintained jointly by the Alpha-Numeric Editorial Panel, consisting of CMS, the Health Insurance Association of America, and the Blue Cross and Blue Shield Association. Level III are codes developed locally by Medicare carriers. The HCPCS modifiers are divided into the same three levels, I being CPT-4 modifiers, II CDT-2 and genuine HCPCS modifiers, and III being locally agreed modifiers. The genuine HCPCS codes and modifiers of level II can be found at http://www.hcfa.gov/stats/anhcpcdl.htm. CMS distributes the HCPCS codes via the National Technical Information Service (NTIS, www.ntis.gov) and NTIS distribution includes the CDT-2 part of HCPCS Level II, but does not include the CPT-4 part (Level I). CMS may distribute the CPT-4 part to ist contractors.", status: "D" },
    },
};

pub static TABLE_0344: V2Table = V2Table {
    number: 344,
    metadata: &super::metadata::TABLE_0344_METADATA,
    rows: phf_map! {
        "01" => V2TableRow { value: "01", display_name: "Patient is insured", definition: "", comment_usage_note: "", status: "" },
        "02" => V2TableRow { value: "02", display_name: "Spouse", definition: "", comment_usage_note: "", status: "" },
        "03" => V2TableRow { value: "03", display_name: "Natural child/insured financial responsibility", definition: "", comment_usage_note: "", status: "" },
        "04" => V2TableRow { value: "04", display_name: "Natural child/Insured does not have financial responsibility", definition: "", comment_usage_note: "", status: "" },
        "05" => V2TableRow { value: "05", display_name: "Step child", definition: "", comment_usage_note: "", status: "" },
        "06" => V2TableRow { value: "06", display_name: "Foster child", definition: "", comment_usage_note: "", status: "" },
        "07" => V2TableRow { value: "07", display_name: "Ward of the court", definition: "", comment_usage_note: "", status: "" },
        "08" => V2TableRow { value: "08", display_name: "Employee", definition: "", comment_usage_note: "", status: "" },
        "09" => V2TableRow { value: "09", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
        "10" => V2TableRow { value: "10", display_name: "Handicapped dependent", definition: "", comment_usage_note: "", status: "" },
        "11" => V2TableRow { value: "11", display_name: "Organ donor", definition: "", comment_usage_note: "", status: "" },
        "12" => V2TableRow { value: "12", display_name: "Cadaver donor", definition: "", comment_usage_note: "", status: "" },
        "13" => V2TableRow { value: "13", display_name: "Grandchild", definition: "", comment_usage_note: "", status: "" },
        "14" => V2TableRow { value: "14", display_name: "Niece/nephew", definition: "", comment_usage_note: "", status: "" },
        "15" => V2TableRow { value: "15", display_name: "Injured plaintiff", definition: "", comment_usage_note: "", status: "" },
        "16" => V2TableRow { value: "16", display_name: "Sponsored dependent", definition: "", comment_usage_note: "", status: "" },
        "17" => V2TableRow { value: "17", display_name: "Minor dependent of a minor dependent", definition: "", comment_usage_note: "", status: "" },
        "18" => V2TableRow { value: "18", display_name: "Parent", definition: "", comment_usage_note: "", status: "" },
        "19" => V2TableRow { value: "19", display_name: "Grandparent", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0353: V2Table = V2Table {
    number: 353,
    metadata: &super::metadata::TABLE_0353_METADATA,
    rows: phf_map! {
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
        "UASK" => V2TableRow { value: "UASK", display_name: "Asked but Unknown", definition: "", comment_usage_note: "", status: "" },
        "NAV" => V2TableRow { value: "NAV", display_name: "Not available", definition: "", comment_usage_note: "", status: "" },
        "NA" => V2TableRow { value: "NA", display_name: "Not applicable", definition: "", comment_usage_note: "", status: "" },
        "NASK" => V2TableRow { value: "NASK", display_name: "Not asked", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0354: V2Table = V2Table {
    number: 354,
    metadata: &super::metadata::TABLE_0354_METADATA,
    rows: phf_map! {
        "ACK" => V2TableRow { value: "ACK", display_name: "Varies", definition: "", comment_usage_note: "", status: "" },
        "ADR_A19" => V2TableRow { value: "ADR_A19", display_name: "", definition: "", comment_usage_note: "Deprecated and removed as of V2.7", status: "D" },
        "ADT_A01" => V2TableRow { value: "ADT_A01", display_name: "A01, A04, A08, A13", definition: "", comment_usage_note: "", status: "" },
        "ADT_A02" => V2TableRow { value: "ADT_A02", display_name: "A02", definition: "", comment_usage_note: "", status: "" },
        "ADT_A03" => V2TableRow { value: "ADT_A03", display_name: "A03", definition: "", comment_usage_note: "", status: "" },
        "ADT_A05" => V2TableRow { value: "ADT_A05", display_name: "A05, A14, A28, A31", definition: "", comment_usage_note: "", status: "" },
        "ADT_A06" => V2TableRow { value: "ADT_A06", display_name: "A06, A07", definition: "", comment_usage_note: "", status: "" },
        "ADT_A09" => V2TableRow { value: "ADT_A09", display_name: "A09, A10, A11", definition: "", comment_usage_note: "", status: "" },
        "ADT_A12" => V2TableRow { value: "ADT_A12", display_name: "A12", definition: "", comment_usage_note: "", status: "" },
        "ADT_A15" => V2TableRow { value: "ADT_A15", display_name: "A15", definition: "", comment_usage_note: "", status: "" },
        "ADT_A16" => V2TableRow { value: "ADT_A16", display_name: "A16", definition: "", comment_usage_note: "", status: "" },
        "ADT_A17" => V2TableRow { value: "ADT_A17", display_name: "A17", definition: "", comment_usage_note: "", status: "" },
        "ADT_A18" => V2TableRow { value: "ADT_A18", display_name: "", definition: "", comment_usage_note: "Deprecated and removed as of V2.7", status: "D" },
        "ADT_A20" => V2TableRow { value: "ADT_A20", display_name: "A20", definition: "", comment_usage_note: "", status: "" },
        "ADT_A21" => V2TableRow { value: "ADT_A21", display_name: "A21, A22, A23, A25, A26, A27, A29, A32, A3 3", definition: "", comment_usage_note: "", status: "" },
        "ADT_A24" => V2TableRow { value: "ADT_A24", display_name: "A24", definition: "", comment_usage_note: "", status: "" },
        "ADT_A30" => V2TableRow { value: "ADT_A30", display_name: "", definition: "", comment_usage_note: "Deprecated and removed as of V2.7", status: "D" },
        "ADT_A37" => V2TableRow { value: "ADT_A37", display_name: "A37", definition: "", comment_usage_note: "", status: "" },
        "ADT_A38" => V2TableRow { value: "ADT_A38", display_name: "A38", definition: "", comment_usage_note: "", status: "" },
        "ADT_A39" => V2TableRow { value: "ADT_A39", display_name: "A39, A40, A41, A42", definition: "", comment_usage_note: "", status: "" },
        "ADT_A43" => V2TableRow { value: "ADT_A43", display_name: "A43", definition: "", comment_usage_note: "", status: "" },
        "ADT_A44" => V2TableRow { value: "ADT_A44", display_name: "A44", definition: "", comment_usage_note: "", status: "" },
        "ADT_A45" => V2TableRow { value: "ADT_A45", display_name: "A45", definition: "", comment_usage_note: "", status: "" },
        "ADT_A50" => V2TableRow { value: "ADT_A50", display_name: "A50, A51", definition: "", comment_usage_note: "", status: "" },
        "ADT_A52" => V2TableRow { value: "ADT_A52", display_name: "A52, A53", definition: "", comment_usage_note: "", status: "" },
        "ADT_A54" => V2TableRow { value: "ADT_A54", display_name: "A54, A55", definition: "", comment_usage_note: "", status: "" },
        "ADT_A60" => V2TableRow { value: "ADT_A60", display_name: "A60", definition: "", comment_usage_note: "", status: "" },
        "ADT_A61" => V2TableRow { value: "ADT_A61", display_name: "A61, A62", definition: "", comment_usage_note: "", status: "" },
        "BAR_P01" => V2TableRow { value: "BAR_P01", display_name: "P01", definition: "", comment_usage_note: "", status: "" },
        "BAR_P02" => V2TableRow { value: "BAR_P02", display_name: "P02", definition: "", comment_usage_note: "", status: "" },
        "BAR_P05" => V2TableRow { value: "BAR_P05", display_name: "P05", definition: "", comment_usage_note: "", status: "" },
        "BAR_P06" => V2TableRow { value: "BAR_P06", display_name: "P06", definition: "", comment_usage_note: "", status: "" },
        "BAR_P10" => V2TableRow { value: "BAR_P10", display_name: "P10", definition: "", comment_usage_note: "", status: "" },
        "BAR_P12" => V2TableRow { value: "BAR_P12", display_name: "P12", definition: "", comment_usage_note: "", status: "" },
        "BPS_O29" => V2TableRow { value: "BPS_O29", display_name: "O29", definition: "", comment_usage_note: "", status: "" },
        "BRP_O30" => V2TableRow { value: "BRP_O30", display_name: "O30", definition: "", comment_usage_note: "", status: "" },
        "BRT_O32" => V2TableRow { value: "BRT_O32", display_name: "O32", definition: "", comment_usage_note: "", status: "" },
        "BTS_O31" => V2TableRow { value: "BTS_O31", display_name: "O31", definition: "", comment_usage_note: "", status: "" },
        "CCF_I22" => V2TableRow { value: "CCF_I22", display_name: "I22", definition: "", comment_usage_note: "", status: "" },
        "CCI_I22" => V2TableRow { value: "CCI_I22", display_name: "I22", definition: "", comment_usage_note: "", status: "" },
        "CCM_I21" => V2TableRow { value: "CCM_I21", display_name: "I21", definition: "", comment_usage_note: "", status: "" },
        "CCQ_I19" => V2TableRow { value: "CCQ_I19", display_name: "I19", definition: "", comment_usage_note: "", status: "" },
        "CCR_I16" => V2TableRow { value: "CCR_I16", display_name: "I16, |17, |18", definition: "", comment_usage_note: "", status: "" },
        "CCU_I20" => V2TableRow { value: "CCU_I20", display_name: "I20", definition: "", comment_usage_note: "", status: "" },
        "CQU_I19" => V2TableRow { value: "CQU_I19", display_name: "I19", definition: "", comment_usage_note: "", status: "" },
        "CRM_" => V2TableRow { value: "CRM_", display_name: "C01 C01, C02, C03, C04, C05, C06, C07, C08", definition: "", comment_usage_note: "", status: "" },
        "CSU_C09" => V2TableRow { value: "CSU_C09", display_name: "C09, C10, C11, C12", definition: "", comment_usage_note: "", status: "" },
        "DBC_O41" => V2TableRow { value: "DBC_O41", display_name: "O41", definition: "", comment_usage_note: "", status: "" },
        "DBC_O42" => V2TableRow { value: "DBC_O42", display_name: "O42", definition: "", comment_usage_note: "", status: "" },
        "DEL_O46" => V2TableRow { value: "DEL_O46", display_name: "O46", definition: "", comment_usage_note: "", status: "" },
        "DEO_O45" => V2TableRow { value: "DEO_O45", display_name: "O45", definition: "", comment_usage_note: "", status: "" },
        "DER_O44" => V2TableRow { value: "DER_O44", display_name: "O44", definition: "", comment_usage_note: "", status: "" },
        "DFT_P03" => V2TableRow { value: "DFT_P03", display_name: "P03", definition: "", comment_usage_note: "", status: "" },
        "DFT_P11" => V2TableRow { value: "DFT_P11", display_name: "P11", definition: "", comment_usage_note: "", status: "" },
        "DOC_T12" => V2TableRow { value: "DOC_T12", display_name: "", definition: "", comment_usage_note: "Deprecated and removed as of V2.7", status: "D" },
        "DPR_O48" => V2TableRow { value: "DPR_O48", display_name: "O48", definition: "", comment_usage_note: "", status: "" },
        "DRC_O47" => V2TableRow { value: "DRC_O47", display_name: "O47", definition: "", comment_usage_note: "", status: "" },
        "DRG_O43" => V2TableRow { value: "DRG_O43", display_name: "O43", definition: "", comment_usage_note: "", status: "" },
        "EAC_U07" => V2TableRow { value: "EAC_U07", display_name: "U07", definition: "", comment_usage_note: "", status: "" },
        "EAN_U09" => V2TableRow { value: "EAN_U09", display_name: "U09", definition: "", comment_usage_note: "", status: "" },
        "EAR_U08" => V2TableRow { value: "EAR_U08", display_name: "U08", definition: "", comment_usage_note: "", status: "" },
        "EHC_E01" => V2TableRow { value: "EHC_E01", display_name: "E01", definition: "", comment_usage_note: "", status: "" },
        "EHC_E02" => V2TableRow { value: "EHC_E02", display_name: "E02", definition: "", comment_usage_note: "", status: "" },
        "EHC_E04" => V2TableRow { value: "EHC_E04", display_name: "E04", definition: "", comment_usage_note: "", status: "" },
        "EHC_E10" => V2TableRow { value: "EHC_E10", display_name: "E10", definition: "", comment_usage_note: "", status: "" },
        "EHC_E12" => V2TableRow { value: "EHC_E12", display_name: "E12", definition: "", comment_usage_note: "", status: "" },
        "EHC_E13" => V2TableRow { value: "EHC_E13", display_name: "E13", definition: "", comment_usage_note: "", status: "" },
        "EHC_E15" => V2TableRow { value: "EHC_E15", display_name: "E15", definition: "", comment_usage_note: "", status: "" },
        "EHC_E20" => V2TableRow { value: "EHC_E20", display_name: "E20", definition: "", comment_usage_note: "", status: "" },
        "EHC_E21" => V2TableRow { value: "EHC_E21", display_name: "E21", definition: "", comment_usage_note: "", status: "" },
        "EHC_E24" => V2TableRow { value: "EHC_E24", display_name: "E24", definition: "", comment_usage_note: "", status: "" },
        "ESR_U02" => V2TableRow { value: "ESR_U02", display_name: "U02", definition: "", comment_usage_note: "", status: "" },
        "ESU_U01" => V2TableRow { value: "ESU_U01", display_name: "U01", definition: "", comment_usage_note: "", status: "" },
        "INR_U06" => V2TableRow { value: "INR_U06", display_name: "U06", definition: "", comment_usage_note: "", status: "" },
        "INU_U05" => V2TableRow { value: "INU_U05", display_name: "U05", definition: "", comment_usage_note: "", status: "" },
        "INV_U14" => V2TableRow { value: "INV_U14", display_name: "Events U14 U14", definition: "", comment_usage_note: "", status: "" },
        "LSU_U12" => V2TableRow { value: "LSU_U12", display_name: "U12, U13", definition: "", comment_usage_note: "", status: "" },
        "MDM_T01" => V2TableRow { value: "MDM_T01", display_name: "T01, T03, T05, T07, T09, T11", definition: "", comment_usage_note: "", status: "" },
        "MDM_T02" => V2TableRow { value: "MDM_T02", display_name: "T02, T04, T06, T08, T10", definition: "", comment_usage_note: "", status: "" },
        "MFK_M01" => V2TableRow { value: "MFK_M01", display_name: "M01, M02, M03, M04, M05, M06, M07, M08, M09, M10, M11", definition: "", comment_usage_note: "", status: "" },
        "MFN_M01" => V2TableRow { value: "MFN_M01", display_name: "", definition: "Deprecated and removed as of V2.7", comment_usage_note: "D", status: "" },
        "MFN_M02" => V2TableRow { value: "MFN_M02", display_name: "M02", definition: "", comment_usage_note: "", status: "" },
        "MFN_M03" => V2TableRow { value: "MFN_M03", display_name: "M03", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "MFN_M04" => V2TableRow { value: "MFN_M04", display_name: "M04", definition: "", comment_usage_note: "", status: "" },
        "MFN_M05" => V2TableRow { value: "MFN_M05", display_name: "M05", definition: "", comment_usage_note: "", status: "" },
        "MFN_M06" => V2TableRow { value: "MFN_M06", display_name: "M06", definition: "", comment_usage_note: "", status: "" },
        "MFN_M07" => V2TableRow { value: "MFN_M07", display_name: "M07", definition: "", comment_usage_note: "", status: "" },
        "MFN_M08" => V2TableRow { value: "MFN_M08", display_name: "M08", definition: "", comment_usage_note: "", status: "" },
        "MFN_M09" => V2TableRow { value: "MFN_M09", display_name: "M09", definition: "", comment_usage_note: "", status: "" },
        "MFN_M10" => V2TableRow { value: "MFN_M10", display_name: "M10", definition: "", comment_usage_note: "", status: "" },
        "MFN_M11" => V2TableRow { value: "MFN_M11", display_name: "M11", definition: "", comment_usage_note: "", status: "" },
        "MFN_M12" => V2TableRow { value: "MFN_M12", display_name: "M12", definition: "", comment_usage_note: "", status: "" },
        "MFN_M13" => V2TableRow { value: "MFN_M13", display_name: "M13", definition: "", comment_usage_note: "", status: "" },
        "MFN_M15" => V2TableRow { value: "MFN_M15", display_name: "M15", definition: "", comment_usage_note: "", status: "" },
        "MFN_M16" => V2TableRow { value: "MFN_M16", display_name: "M16", definition: "", comment_usage_note: "", status: "" },
        "MFN_M17" => V2TableRow { value: "MFN_M17", display_name: "M17", definition: "", comment_usage_note: "", status: "" },
        "MFN_M18" => V2TableRow { value: "MFN_M18", display_name: "M18", definition: "", comment_usage_note: "", status: "" },
        "MFQ_M01" => V2TableRow { value: "MFQ_M01", display_name: "M01, M02, M03, M04, M05, M06", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "MFR_M01" => V2TableRow { value: "MFR_M01", display_name: "M01, M02, M03,", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "MFR_M04" => V2TableRow { value: "MFR_M04", display_name: "M04", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "MFR_M05" => V2TableRow { value: "MFR_M05", display_name: "M05", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "MFR_M06" => V2TableRow { value: "MFR_M06", display_name: "M06", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "MFR_M07" => V2TableRow { value: "MFR_M07", display_name: "M07", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "NMD_N02" => V2TableRow { value: "NMD_N02", display_name: "N02", definition: "", comment_usage_note: "", status: "" },
        "NMQ_N01" => V2TableRow { value: "NMQ_N01", display_name: "N01", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "NMR_N01" => V2TableRow { value: "NMR_N01", display_name: "N01", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "OMB_O27" => V2TableRow { value: "OMB_O27", display_name: "O27", definition: "", comment_usage_note: "", status: "" },
        "OMD_O03" => V2TableRow { value: "OMD_O03", display_name: "O03", definition: "", comment_usage_note: "", status: "" },
        "OMG_O19" => V2TableRow { value: "OMG_O19", display_name: "O19", definition: "", comment_usage_note: "", status: "" },
        "OMI_O23" => V2TableRow { value: "OMI_O23", display_name: "O23", definition: "", comment_usage_note: "", status: "" },
        "OML_O21" => V2TableRow { value: "OML_O21", display_name: "O21", definition: "", comment_usage_note: "", status: "" },
        "OML_O33" => V2TableRow { value: "OML_O33", display_name: "O33", definition: "", comment_usage_note: "", status: "" },
        "OML_O35" => V2TableRow { value: "OML_O35", display_name: "O35", definition: "", comment_usage_note: "", status: "" },
        "OML_O39" => V2TableRow { value: "OML_O39", display_name: "O39", definition: "", comment_usage_note: "", status: "" },
        "OML_O59" => V2TableRow { value: "OML_O59", display_name: "Laboratory Fulfillment Laboratory order mes request with REL segment structure for fulfil request with REL seg", definition: "sage An example is the IHE lment LCC profile describes ment this message in the LAB-7 transaction", comment_usage_note: "N", status: "" },
        "OMN_O07" => V2TableRow { value: "OMN_O07", display_name: "O07", definition: "", comment_usage_note: "", status: "" },
        "OMP_O09" => V2TableRow { value: "OMP_O09", display_name: "O09", definition: "", comment_usage_note: "", status: "" },
        "OMQ_O42" => V2TableRow { value: "OMQ_O42", display_name: "O42", definition: "Deprecated; was added erroneously in 2016", comment_usage_note: "D", status: "" },
        "OMQ_O57" => V2TableRow { value: "OMQ_O57", display_name: "O57 Identifier f structure for message with of General Or with Document (O57)", definition: "or the message an OMQ a trigger event der Message Payload", comment_usage_note: "N", status: "" },
        "OMS_O05" => V2TableRow { value: "OMS_O05", display_name: "O05", definition: "", comment_usage_note: "", status: "" },
        "OPL_O37" => V2TableRow { value: "OPL_O37", display_name: "O37", definition: "", comment_usage_note: "", status: "" },
        "OPR_O38" => V2TableRow { value: "OPR_O38", display_name: "O38", definition: "", comment_usage_note: "", status: "" },
        "OPU_R25" => V2TableRow { value: "OPU_R25", display_name: "R25", definition: "", comment_usage_note: "", status: "" },
        "ORA_R33" => V2TableRow { value: "ORA_R33", display_name: "R33", definition: "", comment_usage_note: "", status: "" },
        "ORA_R41" => V2TableRow { value: "ORA_R41", display_name: "R41", definition: "", comment_usage_note: "", status: "" },
        "ORB_O28" => V2TableRow { value: "ORB_O28", display_name: "O28", definition: "", comment_usage_note: "", status: "" },
        "ORD_O04" => V2TableRow { value: "ORD_O04", display_name: "O04", definition: "", comment_usage_note: "", status: "" },
        "ORF_R04" => V2TableRow { value: "ORF_R04", display_name: "R04", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "ORG_O20" => V2TableRow { value: "ORG_O20", display_name: "O20", definition: "", comment_usage_note: "", status: "" },
        "ORI_O24" => V2TableRow { value: "ORI_O24", display_name: "O24", definition: "", comment_usage_note: "", status: "" },
        "ORL_O22" => V2TableRow { value: "ORL_O22", display_name: "O22", definition: "", comment_usage_note: "", status: "" },
        "ORL_O34" => V2TableRow { value: "ORL_O34", display_name: "O34", definition: "", comment_usage_note: "", status: "" },
        "ORL_O36" => V2TableRow { value: "ORL_O36", display_name: "O36", definition: "", comment_usage_note: "", status: "" },
        "ORL_O40" => V2TableRow { value: "ORL_O40", display_name: "O40", definition: "", comment_usage_note: "", status: "" },
        "ORL_O41" => V2TableRow { value: "ORL_O41", display_name: "O41", definition: "Deprecated; was added erroneously in 2016", comment_usage_note: "D", status: "" },
        "ORL_O42" => V2TableRow { value: "ORL_O42", display_name: "O42", definition: "Deprecated; was added erroneously in 2016", comment_usage_note: "D", status: "" },
        "ORL_O43" => V2TableRow { value: "ORL_O43", display_name: "O43", definition: "Deprecated; was added erroneously in 2016", comment_usage_note: "D", status: "" },
        "ORL_O44" => V2TableRow { value: "ORL_O44", display_name: "O44", definition: "Deprecated; was added erroneously in 2016", comment_usage_note: "D", status: "" },
        "ORL_O53" => V2TableRow { value: "ORL_O53", display_name: "O53 Identifier f structure for with a trigge General Labor Acknowledgmen (Patient Opti", definition: "or the message an ORL message r event of - atory Order t Message onal) (O53)", comment_usage_note: "N", status: "" },
        "ORL_O54" => V2TableRow { value: "ORL_O54", display_name: "O54 Identifier fo structure for with a trigge Laboratory Or Acknowledgmen Multiple Orde (Patient Opti", definition: "r the message an ORL message r event of der t Message – r Per Specimen onal) (O54)", comment_usage_note: "N", status: "" },
        "ORL_O55" => V2TableRow { value: "ORL_O55", display_name: "O55 Identifier f structure for with a trigge Laboratory Or Acknowledgmen Multiple Orde of Specimen ( Optional) (O5", definition: "or the message an ORL message r event of der t Message – r Per Container Patient 5)", comment_usage_note: "N", status: "" },
        "ORL_O56" => V2TableRow { value: "ORL_O56", display_name: "O56 Identifier for th structure for an O with a trigger eve Specimen Shipment Laboratory Order Acknowledgment Mes (Patient Optional)", definition: "e message RL message nt of Centric sage (O56)", comment_usage_note: "N", status: "" },
        "ORM_O01" => V2TableRow { value: "ORM_O01", display_name: "O01", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "ORN_O08" => V2TableRow { value: "ORN_O08", display_name: "O08", definition: "", comment_usage_note: "", status: "" },
        "ORP_O10" => V2TableRow { value: "ORP_O10", display_name: "O10", definition: "", comment_usage_note: "", status: "" },
        "ORR_O02" => V2TableRow { value: "ORR_O02", display_name: "O02", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "ORS_O06" => V2TableRow { value: "ORS_O06", display_name: "O06", definition: "", comment_usage_note: "", status: "" },
        "ORU_R01" => V2TableRow { value: "ORU_R01", display_name: "R01", definition: "", comment_usage_note: "", status: "" },
        "ORU_R30" => V2TableRow { value: "ORU_R30", display_name: "R30", definition: "", comment_usage_note: "", status: "" },
        "ORU_W01" => V2TableRow { value: "ORU_W01", display_name: "W01", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "ORX_O43" => V2TableRow { value: "ORX_O43", display_name: "O43", definition: "Deprecated; was added erroneously in 2016", comment_usage_note: "D", status: "" },
        "ORX_O58" => V2TableRow { value: "ORX_O58", display_name: "O58 Identifier for th structure for an O with a trigger eve General Order Mess Document Payload Acknowledgement Me (O58)", definition: "e message RX message nt of age with ssage", comment_usage_note: "N", status: "" },
        "OSM_R26" => V2TableRow { value: "OSM_R26", display_name: "R26", definition: "", comment_usage_note: "", status: "" },
        "OSQ_Q06" => V2TableRow { value: "OSQ_Q06", display_name: "Q06", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "OSR_Q06" => V2TableRow { value: "OSR_Q06", display_name: "Q06", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "OSU_O41" => V2TableRow { value: "OSU_O41", display_name: "O41", definition: "Deprecated; was added erroneously in 2016", comment_usage_note: "D", status: "" },
        "OSU_O51" => V2TableRow { value: "OSU_O51", display_name: "O51 Identifier for the structure for an O with a trigger eve Status Update (O51", definition: "message SU message nt of Order )", comment_usage_note: "N", status: "" },
        "OSU_O52" => V2TableRow { value: "OSU_O52", display_name: "O52 Identifier for the structure for an O with a trigger eve Status Update Acknowledgement (O", definition: "message SU message nt of Order 52)", comment_usage_note: "N", status: "" },
        "OUL_R21" => V2TableRow { value: "OUL_R21", display_name: "R21", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "OUL_R22" => V2TableRow { value: "OUL_R22", display_name: "R22", definition: "", comment_usage_note: "", status: "" },
        "OUL_R23" => V2TableRow { value: "OUL_R23", display_name: "R23", definition: "", comment_usage_note: "", status: "" },
        "OUL_R24" => V2TableRow { value: "OUL_R24", display_name: "R24", definition: "", comment_usage_note: "", status: "" },
        "PEX_P07" => V2TableRow { value: "PEX_P07", display_name: "P07, P08", definition: "", comment_usage_note: "", status: "" },
        "PGL_PC6" => V2TableRow { value: "PGL_PC6", display_name: "PC6, PC7, PC8", definition: "", comment_usage_note: "", status: "" },
        "PMU_B01" => V2TableRow { value: "PMU_B01", display_name: "B01, B02", definition: "", comment_usage_note: "", status: "" },
        "PMU_B03" => V2TableRow { value: "PMU_B03", display_name: "B03", definition: "", comment_usage_note: "", status: "" },
        "PMU_B04" => V2TableRow { value: "PMU_B04", display_name: "B04, B05, B06", definition: "", comment_usage_note: "", status: "" },
        "PMU_B07" => V2TableRow { value: "PMU_B07", display_name: "B07", definition: "", comment_usage_note: "", status: "" },
        "PMU_B08" => V2TableRow { value: "PMU_B08", display_name: "B08", definition: "", comment_usage_note: "", status: "" },
        "PPG_PCG" => V2TableRow { value: "PPG_PCG", display_name: "PCC, PCG, P CH, P CJ", definition: "", comment_usage_note: "", status: "" },
        "PPP_PCB" => V2TableRow { value: "PPP_PCB", display_name: "PCB, PCD", definition: "", comment_usage_note: "", status: "" },
        "PPR_P" => V2TableRow { value: "PPR_P", display_name: "C1 PC1, PC2, PC3", definition: "", comment_usage_note: "", status: "" },
        "PPT_PCL" => V2TableRow { value: "PPT_PCL", display_name: "PCL", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "PPV_PCA" => V2TableRow { value: "PPV_PCA", display_name: "PCA", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "PRR_PC5" => V2TableRow { value: "PRR_PC5", display_name: "PC5", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "PTR_PCF" => V2TableRow { value: "PTR_PCF", display_name: "PCF", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "QBP_E03" => V2TableRow { value: "QBP_E03", display_name: "E03", definition: "", comment_usage_note: "", status: "" },
        "QBP_E22" => V2TableRow { value: "QBP_E22", display_name: "E22", definition: "", comment_usage_note: "", status: "" },
        "QBP_O33" => V2TableRow { value: "QBP_O33", display_name: "O33", definition: "", comment_usage_note: "", status: "" },
        "QBP_O34" => V2TableRow { value: "QBP_O34", display_name: "O34", definition: "", comment_usage_note: "", status: "" },
        "QBP_Q11" => V2TableRow { value: "QBP_Q11", display_name: "Q11", definition: "", comment_usage_note: "", status: "" },
        "QBP_Q13" => V2TableRow { value: "QBP_Q13", display_name: "Q13", definition: "", comment_usage_note: "", status: "" },
        "QBP_Q15" => V2TableRow { value: "QBP_Q15", display_name: "Q15", definition: "", comment_usage_note: "", status: "" },
        "QBP_Q21" => V2TableRow { value: "QBP_Q21", display_name: "Q21, Q22, Q23,Q24, Q25", definition: "", comment_usage_note: "", status: "" },
        "QCK_Q02" => V2TableRow { value: "QCK_Q02", display_name: "Q02", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "QCN_J01" => V2TableRow { value: "QCN_J01", display_name: "J01, J02", definition: "", comment_usage_note: "", status: "" },
        "QRF_W02" => V2TableRow { value: "QRF_W02", display_name: "W02", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "QRY_A19" => V2TableRow { value: "QRY_A19", display_name: "A19", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "QRY_P" => V2TableRow { value: "QRY_P", display_name: "C4 PC4, PC9, PCE, P CK", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "QRY_Q01" => V2TableRow { value: "QRY_Q01", display_name: "Q01, Q26, Q27, Q28, Q29, Q30", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "QRY_Q02" => V2TableRow { value: "QRY_Q02", display_name: "Q02", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "QRY_R02" => V2TableRow { value: "QRY_R02", display_name: "R02", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "QRY_T12" => V2TableRow { value: "QRY_T12", display_name: "T12", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "QSB_Q16" => V2TableRow { value: "QSB_Q16", display_name: "Q16", definition: "", comment_usage_note: "", status: "" },
        "QVR_Q17" => V2TableRow { value: "QVR_Q17", display_name: "Q17", definition: "", comment_usage_note: "", status: "" },
        "RAR_" => V2TableRow { value: "RAR_", display_name: "RAR RAR", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "RAS_O17" => V2TableRow { value: "RAS_O17", display_name: "O17", definition: "", comment_usage_note: "", status: "" },
        "RCI_I05" => V2TableRow { value: "RCI_I05", display_name: "I05", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "RCL_I06" => V2TableRow { value: "RCL_I06", display_name: "I06", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "RDE_O11" => V2TableRow { value: "RDE_O11", display_name: "O11, O25", definition: "", comment_usage_note: "", status: "" },
        "RDE_O49" => V2TableRow { value: "RDE_O49", display_name: "Events O49 O49", definition: "", comment_usage_note: "", status: "" },
        "RDR_" => V2TableRow { value: "RDR_", display_name: "RDR RDR", definition: "", comment_usage_note: "", status: "" },
        "RDS_O13" => V2TableRow { value: "RDS_O13", display_name: "O13", definition: "", comment_usage_note: "", status: "" },
        "RDY_K15" => V2TableRow { value: "RDY_K15", display_name: "K15", definition: "", comment_usage_note: "", status: "" },
        "REF_I12" => V2TableRow { value: "REF_I12", display_name: "I12, I13, I14, I15", definition: "", comment_usage_note: "", status: "" },
        "RER_RE" => V2TableRow { value: "RER_RE", display_name: "R RER", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "RGR_" => V2TableRow { value: "RGR_", display_name: "RGR RGR", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "RGV_O15" => V2TableRow { value: "RGV_O15", display_name: "O15", definition: "", comment_usage_note: "", status: "" },
        "ROR_" => V2TableRow { value: "ROR_", display_name: "ROR ROR", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "RPA_I08" => V2TableRow { value: "RPA_I08", display_name: "I08, I09. I10, I11", definition: "", comment_usage_note: "", status: "" },
        "RPI_I01" => V2TableRow { value: "RPI_I01", display_name: "I01, I04", definition: "", comment_usage_note: "", status: "" },
        "RPI_I04" => V2TableRow { value: "RPI_I04", display_name: "I04", definition: "", comment_usage_note: "", status: "" },
        "RPL_I02" => V2TableRow { value: "RPL_I02", display_name: "I02", definition: "", comment_usage_note: "", status: "" },
        "RP" => V2TableRow { value: "RP", display_name: "R_I03 I03", definition: "", comment_usage_note: "", status: "" },
        "RQA_I08" => V2TableRow { value: "RQA_I08", display_name: "I08, I09, I10, I11", definition: "", comment_usage_note: "", status: "" },
        "RQC_I05" => V2TableRow { value: "RQC_I05", display_name: "I05, I06", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "RQI_I01" => V2TableRow { value: "RQI_I01", display_name: "I01, I02, I03, I07", definition: "", comment_usage_note: "", status: "" },
        "RQP_I04" => V2TableRow { value: "RQP_I04", display_name: "I04", definition: "", comment_usage_note: "", status: "" },
        "RRA_O18" => V2TableRow { value: "RRA_O18", display_name: "O18", definition: "", comment_usage_note: "", status: "" },
        "RRD_O14" => V2TableRow { value: "RRD_O14", display_name: "O14", definition: "", comment_usage_note: "", status: "" },
        "RRE_O12" => V2TableRow { value: "RRE_O12", display_name: "O12, O26", definition: "", comment_usage_note: "", status: "" },
        "RRE_O50" => V2TableRow { value: "RRE_O50", display_name: "Events O50 O50", definition: "", comment_usage_note: "", status: "" },
        "RRG_O16" => V2TableRow { value: "RRG_O16", display_name: "O16", definition: "", comment_usage_note: "", status: "" },
        "RRI_I12" => V2TableRow { value: "RRI_I12", display_name: "I12, I13, I14, I15", definition: "", comment_usage_note: "", status: "" },
        "RSP_E03" => V2TableRow { value: "RSP_E03", display_name: "E03", definition: "", comment_usage_note: "", status: "" },
        "RSP_E22" => V2TableRow { value: "RSP_E22", display_name: "E22", definition: "", comment_usage_note: "", status: "" },
        "RSP_K11" => V2TableRow { value: "RSP_K11", display_name: "K11", definition: "", comment_usage_note: "", status: "" },
        "RSP_K21" => V2TableRow { value: "RSP_K21", display_name: "K21", definition: "", comment_usage_note: "", status: "" },
        "RSP_K22" => V2TableRow { value: "RSP_K22", display_name: "K22", definition: "", comment_usage_note: "", status: "" },
        "RSP_K23" => V2TableRow { value: "RSP_K23", display_name: "K23, K24", definition: "", comment_usage_note: "", status: "" },
        "RSP_K25" => V2TableRow { value: "RSP_K25", display_name: "K25", definition: "", comment_usage_note: "", status: "" },
        "RSP_K31" => V2TableRow { value: "RSP_K31", display_name: "K31", definition: "", comment_usage_note: "", status: "" },
        "RSP_K32" => V2TableRow { value: "RSP_K32", display_name: "K32", definition: "", comment_usage_note: "", status: "" },
        "RSP_O33" => V2TableRow { value: "RSP_O33", display_name: "O33", definition: "", comment_usage_note: "", status: "" },
        "RSP_O34" => V2TableRow { value: "RSP_O34", display_name: "O34", definition: "", comment_usage_note: "", status: "" },
        "RSP_Q11" => V2TableRow { value: "RSP_Q11", display_name: "Q11", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "RTB_K13" => V2TableRow { value: "RTB_K13", display_name: "K13", definition: "", comment_usage_note: "", status: "" },
        "SDR_S31" => V2TableRow { value: "SDR_S31", display_name: "S31, S36", definition: "", comment_usage_note: "", status: "" },
        "SDR_S32" => V2TableRow { value: "SDR_S32", display_name: "S32, S37", definition: "", comment_usage_note: "", status: "" },
        "SET_S38" => V2TableRow { value: "SET_S38", display_name: "Specimen Container This message structure Preparation and Specimen supports tracking of Collection Event information related to preparations for specimen collection and the collection event.", definition: "It can be used to describe the container preparation for collection as wellasthe successful collection of one or more specimen(s).", comment_usage_note: "N", status: "" },
        "SET_S40" => V2TableRow { value: "SET_S40", display_name: "Specimen Collection Event This message structure unsuccessful supports tracking of information when specimen collection is not successful.", definition: "It can be used to describe the reason why a specimen collection was unsuccessful.", comment_usage_note: "N", status: "" },
        "SET_S41" => V2TableRow { value: "SET_S41", display_name: "Specimen Movement Event This message structure supports tracking of information related to the movements of specimens across locations, and placement in and out of storage.", definition: "It can be used to report on specimen departing, arriving, being accepted or rejected, including moving the specimen into storage or retrieving it from storage (e.g. in biobanking).", comment_usage_note: "N", status: "" },
        "SET_S45" => V2TableRow { value: "SET_S45", display_name: "Specimen Identification This message structure Events supports tracking of information to identify or de- identify specimens or at time of final disposal.", definition: "It can be used to report the de-and re- identification of specimen(s) as well as the final disposition of a specimen.", comment_usage_note: "N", status: "" },
        "SET_S50" => V2TableRow { value: "SET_S50", display_name: "Specimen Procedure Step This message structure successful supports tracking of information related to processing of one or more specimen that may result in derived (child) specimen or not.", definition: "", comment_usage_note: "N", status: "" },
        "SET_S52" => V2TableRow { value: "SET_S52", display_name: "Specimen Procedure Step This message structure unsuccessful supports tracking of information when the processing of a specimen was unsuccessful.", definition: "It can be used to describe the reason why the specimen processing was unsuccessful.", comment_usage_note: "N", status: "" },
        "SIU_S12" => V2TableRow { value: "SIU_S12", display_name: "S12, S13, S14, S15, S16, S17, S18, S19, S20, S21, S22, S23, S24, S26", definition: "", comment_usage_note: "", status: "" },
        "SLR_S28" => V2TableRow { value: "SLR_S28", display_name: "S28, S29, S30, S34, S35", definition: "", comment_usage_note: "", status: "" },
        "SQM_S25" => V2TableRow { value: "SQM_S25", display_name: "S25", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "SQR_S25" => V2TableRow { value: "SQR_S25", display_name: "S25", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "SRM_S01" => V2TableRow { value: "SRM_S01", display_name: "S01, S02, S03, S04, S05, S06, S07, S08, S09, S10, S11", definition: "", comment_usage_note: "", status: "" },
        "SRR_S01" => V2TableRow { value: "SRR_S01", display_name: "S01, S02, S03, S04, S05, S06 , S07, S08, S09, S10, S11", definition: "", comment_usage_note: "", status: "" },
        "SSR_U04" => V2TableRow { value: "SSR_U04", display_name: "U04", definition: "", comment_usage_note: "", status: "" },
        "SSU_U03" => V2TableRow { value: "SSU_U03", display_name: "U03", definition: "", comment_usage_note: "", status: "" },
        "STC_S33" => V2TableRow { value: "STC_S33", display_name: "S33", definition: "", comment_usage_note: "", status: "" },
        "SUR_P09" => V2TableRow { value: "SUR_P09", display_name: "P09", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "TCU_U10" => V2TableRow { value: "TCU_U10", display_name: "U10, U11", definition: "", comment_usage_note: "", status: "" },
        "UDM_Q05" => V2TableRow { value: "UDM_Q05", display_name: "Q05", definition: "", comment_usage_note: "", status: "" },
        "VXQ_V01" => V2TableRow { value: "VXQ_V01", display_name: "V01", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "VXR_V03" => V2TableRow { value: "VXR_V03", display_name: "V03", definition: "Deprecated", comment_usage_note: "D", status: "" },
        "VXU_V04" => V2TableRow { value: "VXU_V04", display_name: "V04", definition: "", comment_usage_note: "", status: "" },
        "VXX_V02" => V2TableRow { value: "VXX_V02", display_name: "V02", definition: "Deprecated", comment_usage_note: "D", status: "" },
    },
};

pub static TABLE_0355: V2Table = V2Table {
    number: 355,
    metadata: &super::metadata::TABLE_0355_METADATA,
    rows: phf_map! {
        "PL" => V2TableRow { value: "PL", display_name: "Person location", definition: "", comment_usage_note: "", status: "" },
        "CE" => V2TableRow { value: "CE", display_name: "Coded element", definition: "", comment_usage_note: "Withdrawn as of v2.6 - CE has been replaced by CNE and CWE", status: "" },
        "CWE" => V2TableRow { value: "CWE", display_name: "Coded with Exceptions", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0356: V2Table = V2Table {
    number: 356,
    metadata: &super::metadata::TABLE_0356_METADATA,
    rows: phf_map! {
        "ISO" => V2TableRow { value: "ISO", display_name: "2022- This standard is titled \"Information", definition: "", comment_usage_note: "Note that the escape sequences used in", status: "" },
        "1994" => V2TableRow { value: "1994", display_name: "Technology - Character Code Structure and Extension Technique\". .", definition: "", comment_usage_note: "this mode do not use the ASCII “esc” character, as defined in ISO 2022- They are “HL7 escape sequences” as first defined in HL7 2.3, sec. 2.9.2. (Also, note that sections 2.8.28.6.1and 2.9.2 in HL7 2.3 correspond to sections 2.16.93 and 2.7.2 in HL7 2. 5.)", status: "1994." },
        "2.3" => V2TableRow { value: "2.3", display_name: "The character set switching mode specified in HL7 2.5, section 2.7.2 and section 2.A.46, \"XPN - extended person name\".", definition: "", comment_usage_note: "Note that the escape sequences used in this mode do not use the ASCII “esc” character, as defined in ISO 2022- 1994. They are “HL7 escape sequences” as first defined in HL7 2.3, sec. 2.9.2. (Also, note that sections 2.8.28.6.1and 2.9.2 in HL7 2.3 correspond to sections 2.16.93 and 2.7.2 in HL7 2. 5.)", status: "" },
        "<null>" => V2TableRow { value: "<null>", display_name: "This is the default, indicating that there is no character set switching occurring in this message.", definition: "", comment_usage_note: "This is the default.", status: "D" },
    },
};

pub static TABLE_0357: V2Table = V2Table {
    number: 357,
    metadata: &super::metadata::TABLE_0357_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Message accepted", definition: "", comment_usage_note: "Success. Optional, as the AA conveys success. Used for systems that must always return a status code.", status: "" },
        "100" => V2TableRow { value: "100", display_name: "Segment sequence error", definition: "", comment_usage_note: "Error: The message segments were not in the proper order, or required segments are missing.", status: "" },
        "101" => V2TableRow { value: "101", display_name: "Required field missing", definition: "", comment_usage_note: "Error: A required field is missing from a segment", status: "" },
        "102" => V2TableRow { value: "102", display_name: "Data type error", definition: "", comment_usage_note: "Error: The field contained data of the wrong data type, e.g., an NM field contained \"FOO\".", status: "" },
        "103" => V2TableRow { value: "103", display_name: "Table value not found", definition: "Error: A fi compared ag no match wa", comment_usage_note: "eld of data type ID or IS was ainst the corresponding table, and s found.", status: "" },
        "104" => V2TableRow { value: "104", display_name: "Value too long", definition: "Error: a va or the leng safely hand", comment_usage_note: "lue exceeded the normative length, th that the application is able to le.", status: "" },
        "198" => V2TableRow { value: "198", display_name: "Non-Conformant An err Cardinality encoun to HL7 conten not -c with t specif standa confor or imp profil", definition: "or has been Error: Card tered related than 3 of t message the message t. Message is onformant he applicable ication’s (base rd, mance profile lementation e) cardinality.", comment_usage_note: "inality is listed as [0..3] and more N he identified element are present in .", status: "" },
        "199" => V2TableRow { value: "199", display_name: "Other HL7 Error Any ot the HL is not any of error set.", definition: "her error with Error 7 syntax that captured in the other codes in this", comment_usage_note: "N", status: "" },
        "200" => V2TableRow { value: "200", display_name: "Unsupported message type", definition: "Rejection:", comment_usage_note: "The Message Type is not supported.", status: "" },
        "201" => V2TableRow { value: "201", display_name: "Unsupported event code", definition: "Rejection:", comment_usage_note: "The Event Code is not supported.", status: "" },
        "202" => V2TableRow { value: "202", display_name: "Unsupported processing id", definition: "Rejection:", comment_usage_note: "The Processing ID is not supported.", status: "" },
        "203" => V2TableRow { value: "203", display_name: "Unsupported version id", definition: "Rejection:", comment_usage_note: "The Version ID is not supported.", status: "" },
        "204" => V2TableRow { value: "204", display_name: "Unknown key Retain identifier backwa compat This s should in ERR (Appli Code) 101 (U Identi code s HL7053", definition: "ed for ERR-3 (HL7 rds convey erro ibility only: an applicat ituation reported in be reported - 5 cation Error using code nknown Key fier) from ystem 3.", comment_usage_note: "Error Code) should be used to B rs at the structural level and this is ion level error, which should be ERR- 5 (Application Error Code).", status: "" },
        "205" => V2TableRow { value: "205", display_name: "Duplicate key Retain identifier backwa compat This s should in ERR (Appli Code) 102 (D Identi code syste HL70533.", definition: "ed for ERR-3 (HL7 rds convey erro ibility only: an applicat ituation reported in be reported - 5 cation Error using code uplicate Key fier) from m", comment_usage_note: "Error Code) should be used to B rs at the structural level and this is ion level error, which should be ERR-5 (Application Error Code).", status: "" },
        "206" => V2TableRow { value: "206", display_name: "Application Retained f record locked backwards compatibil This situa should be in ERR- (Applicati Code) usin 103 (Appli record loc code syste HL70533.", definition: "or ERR-3 (HL7 Error convey errors at ity only: an application l tion reported in ERR- reported 5 on Error g code cation ked) from m", comment_usage_note: "Code) should be used to B the structural level and this is evel error, which should be 5 (Application Error Code).", status: "" },
        "207" => V2TableRow { value: "207", display_name: "Application error An applica error has and the de that error identified", definition: "tion level This value is us occurred list is applicab tail for error reported i is ERR- in ERR-5.", comment_usage_note: "ed when no other value in this le and there is an application n ERR-5. It is applicable when 3 is required in an implementation guide.", status: "" },
    },
};

pub static TABLE_0360: V2Table = V2Table {
    number: 360,
    metadata: &super::metadata::TABLE_0360_METADATA,
    rows: phf_map! {
        "PN" => V2TableRow { value: "PN", display_name: "Advanced Practice Nurse", definition: "", comment_usage_note: "", status: "" },
        "AAS" => V2TableRow { value: "AAS", display_name: "Associate of Applied Science", definition: "", comment_usage_note: "", status: "" },
        "AA" => V2TableRow { value: "AA", display_name: "Associate of Arts", definition: "", comment_usage_note: "", status: "" },
        "ABA" => V2TableRow { value: "ABA", display_name: "Associate of Business Admini stration", definition: "", comment_usage_note: "", status: "" },
        "AE" => V2TableRow { value: "AE", display_name: "Associate of Engineering", definition: "", comment_usage_note: "", status: "" },
        "AS" => V2TableRow { value: "AS", display_name: "Associate of Science", definition: "", comment_usage_note: "", status: "" },
        "BA" => V2TableRow { value: "BA", display_name: "Bachelor of Arts", definition: "", comment_usage_note: "", status: "" },
        "BBA" => V2TableRow { value: "BBA", display_name: "Bachelor of Business Administration", definition: "", comment_usage_note: "", status: "" },
        "BE" => V2TableRow { value: "BE", display_name: "Bachelor or Engineering", definition: "", comment_usage_note: "", status: "" },
        "BFA" => V2TableRow { value: "BFA", display_name: "Bachelor of Fine Arts", definition: "", comment_usage_note: "", status: "" },
        "BN" => V2TableRow { value: "BN", display_name: "Bachelor of Nursing", definition: "", comment_usage_note: "", status: "" },
        "BS" => V2TableRow { value: "BS", display_name: "Bachelor of Science", definition: "", comment_usage_note: "", status: "" },
        "BSL" => V2TableRow { value: "BSL", display_name: "Bachelor of Science - Law", definition: "", comment_usage_note: "", status: "" },
        "BSN" => V2TableRow { value: "BSN", display_name: "Bachelor on Science - Nursing", definition: "", comment_usage_note: "", status: "" },
        "BT" => V2TableRow { value: "BT", display_name: "Bachelor of Theology", definition: "", comment_usage_note: "", status: "" },
        "CER" => V2TableRow { value: "CER", display_name: "Certificate", definition: "", comment_usage_note: "", status: "" },
        "CANP" => V2TableRow { value: "CANP", display_name: "Certified Adult Nurse Practitioner", definition: "", comment_usage_note: "", status: "" },
        "CMA" => V2TableRow { value: "CMA", display_name: "Certified Medical Assistant", definition: "", comment_usage_note: "", status: "" },
        "CNP" => V2TableRow { value: "CNP", display_name: "Certified Nurse Practitioner", definition: "", comment_usage_note: "", status: "" },
        "CNM" => V2TableRow { value: "CNM", display_name: "Certified Nurse Midwife", definition: "", comment_usage_note: "", status: "" },
        "CRN" => V2TableRow { value: "CRN", display_name: "Certified Registered Nurse", definition: "", comment_usage_note: "", status: "" },
        "CNS" => V2TableRow { value: "CNS", display_name: "Certified Nurse Specialist", definition: "", comment_usage_note: "", status: "" },
        "CPNP" => V2TableRow { value: "CPNP", display_name: "Certified Pediatric Nurse Practitioner", definition: "", comment_usage_note: "", status: "" },
        "CTR" => V2TableRow { value: "CTR", display_name: "Certified Tumor Registrar", definition: "", comment_usage_note: "", status: "" },
        "DIP" => V2TableRow { value: "DIP", display_name: "Diploma", definition: "", comment_usage_note: "", status: "" },
        "DBA" => V2TableRow { value: "DBA", display_name: "Doctor of Business Administration", definition: "", comment_usage_note: "", status: "" },
        "DED" => V2TableRow { value: "DED", display_name: "Doctor of Education", definition: "", comment_usage_note: "", status: "" },
        "PharmD" => V2TableRow { value: "PharmD", display_name: "Doctor of Pharmacy", definition: "", comment_usage_note: "", status: "" },
        "PHE" => V2TableRow { value: "PHE", display_name: "Doctor of Engineering", definition: "", comment_usage_note: "", status: "" },
        "PHD" => V2TableRow { value: "PHD", display_name: "Doctor of Philosophy", definition: "", comment_usage_note: "", status: "" },
        "PHS" => V2TableRow { value: "PHS", display_name: "Doctor of Science", definition: "", comment_usage_note: "", status: "" },
        "MD" => V2TableRow { value: "MD", display_name: "Doctor of Medicine", definition: "", comment_usage_note: "", status: "" },
        "DO" => V2TableRow { value: "DO", display_name: "Doctor of Osteopathy", definition: "", comment_usage_note: "", status: "" },
        "EMT" => V2TableRow { value: "EMT", display_name: "Emergency Medical Technician", definition: "", comment_usage_note: "", status: "" },
        "EMTP" => V2TableRow { value: "EMTP", display_name: "Emergency Medical Technician - Paramedic", definition: "", comment_usage_note: "", status: "" },
        "FPNP" => V2TableRow { value: "FPNP", display_name: "Family Practice Nurse Practitioner", definition: "", comment_usage_note: "", status: "" },
        "HS" => V2TableRow { value: "HS", display_name: "High School Graduate", definition: "", comment_usage_note: "", status: "" },
        "JD" => V2TableRow { value: "JD", display_name: "Juris Doctor", definition: "", comment_usage_note: "", status: "" },
        "MA" => V2TableRow { value: "MA", display_name: "Master of Arts", definition: "", comment_usage_note: "", status: "" },
        "MBA" => V2TableRow { value: "MBA", display_name: "Master of Business Administr ation", definition: "", comment_usage_note: "", status: "" },
        "MCE" => V2TableRow { value: "MCE", display_name: "Master of Civil Engineering", definition: "", comment_usage_note: "", status: "" },
        "MDI" => V2TableRow { value: "MDI", display_name: "Master of Divinity", definition: "", comment_usage_note: "", status: "" },
        "MED" => V2TableRow { value: "MED", display_name: "Master of Education", definition: "", comment_usage_note: "", status: "" },
        "MEE" => V2TableRow { value: "MEE", display_name: "Master of Electrical Engineering", definition: "", comment_usage_note: "", status: "" },
        "ME" => V2TableRow { value: "ME", display_name: "Master of Engineering", definition: "", comment_usage_note: "", status: "" },
        "MFA" => V2TableRow { value: "MFA", display_name: "Master of Fine Arts", definition: "", comment_usage_note: "", status: "" },
        "MME" => V2TableRow { value: "MME", display_name: "Master of Mechanical Engineering", definition: "", comment_usage_note: "", status: "" },
        "MS" => V2TableRow { value: "MS", display_name: "Master of Science", definition: "", comment_usage_note: "", status: "" },
        "MSL" => V2TableRow { value: "MSL", display_name: "Master of Science - Law", definition: "", comment_usage_note: "", status: "" },
        "MSN" => V2TableRow { value: "MSN", display_name: "Master of Science - Nursing", definition: "", comment_usage_note: "", status: "" },
        "MTH" => V2TableRow { value: "MTH", display_name: "Master of Theology", definition: "", comment_usage_note: "", status: "" },
        "MDA" => V2TableRow { value: "MDA", display_name: "Medical Assistant", definition: "", comment_usage_note: "", status: "" },
        "MT" => V2TableRow { value: "MT", display_name: "Medical Technician", definition: "", comment_usage_note: "", status: "" },
        "NG" => V2TableRow { value: "NG", display_name: "Non-Graduate", definition: "", comment_usage_note: "", status: "" },
        "NP" => V2TableRow { value: "NP", display_name: "Nurse Practitioner", definition: "", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Physician Assi stant", definition: "", comment_usage_note: "", status: "" },
        "RMA" => V2TableRow { value: "RMA", display_name: "Registered Medical Assistant", definition: "", comment_usage_note: "", status: "" },
        "RN" => V2TableRow { value: "RN", display_name: "Registered Nurse", definition: "", comment_usage_note: "", status: "" },
        "RPH" => V2TableRow { value: "RPH", display_name: "Registered Pharmacist", definition: "", comment_usage_note: "", status: "" },
        "SEC" => V2TableRow { value: "SEC", display_name: "Secretarial Certificate", definition: "", comment_usage_note: "", status: "" },
        "TS" => V2TableRow { value: "TS", display_name: "Trade School Graduate", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0364: V2Table = V2Table {
    number: 364,
    metadata: &super::metadata::TABLE_0364_METADATA,
    rows: phf_map! {
        "PI" => V2TableRow { value: "PI", display_name: "Patient Instructions", definition: "", comment_usage_note: "", status: "" },
        "AI" => V2TableRow { value: "AI", display_name: "Ancillary Instructions", definition: "", comment_usage_note: "", status: "" },
        "GI" => V2TableRow { value: "GI", display_name: "General Instructions", definition: "", comment_usage_note: "", status: "" },
        "1R" => V2TableRow { value: "1R", display_name: "Primary Reason", definition: "", comment_usage_note: "", status: "" },
        "2R" => V2TableRow { value: "2R", display_name: "Secondary Reason", definition: "", comment_usage_note: "", status: "" },
        "GR" => V2TableRow { value: "GR", display_name: "General Reason", definition: "", comment_usage_note: "", status: "" },
        "RE" => V2TableRow { value: "RE", display_name: "Remark", definition: "", comment_usage_note: "", status: "" },
        "DR" => V2TableRow { value: "DR", display_name: "Duplicate/Interact ion Reason", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0365: V2Table = V2Table {
    number: 365,
    metadata: &super::metadata::TABLE_0365_METADATA,
    rows: phf_map! {
        "IN" => V2TableRow { value: "IN", display_name: "Initializing", definition: "Software is ready, hardware not yet", comment_usage_note: "", status: "" },
        "CO" => V2TableRow { value: "CO", display_name: "Configuring", definition: "", comment_usage_note: "", status: "" },
        "PU" => V2TableRow { value: "PU", display_name: "Powered Up Software and hardware ar ready", definition: "e not yet", comment_usage_note: "", status: "" },
        "RS" => V2TableRow { value: "RS", display_name: "Ready to start Software and hardware ar user action is required", definition: "e ready, but to start", comment_usage_note: "", status: "" },
        "ID" => V2TableRow { value: "ID", display_name: "Idle Successfully started, ne be accepted, currently n present", definition: "w orders can o orders are", comment_usage_note: "", status: "" },
        "OP" => V2TableRow { value: "OP", display_name: "Normal Operation Successfully started, ne be accepted", definition: "w orders can", comment_usage_note: "", status: "" },
        "CL" => V2TableRow { value: "CL", display_name: "Clearing", definition: "", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Pausing", definition: "", comment_usage_note: "", status: "" },
        "PD" => V2TableRow { value: "PD", display_name: "Paused User action is required", definition: "to continue", comment_usage_note: "", status: "" },
        "ES" => V2TableRow { value: "ES", display_name: "E-stopped Error, remaining orders finished, new orders can accepted", definition: "can be not be", comment_usage_note: "", status: "" },
        "TS" => V2TableRow { value: "TS", display_name: "Transport stopped", definition: "", comment_usage_note: "", status: "" },
        "SS" => V2TableRow { value: "SS", display_name: "Sampling stopped", definition: "", comment_usage_note: "", status: "" },
        "SD" => V2TableRow { value: "SD", display_name: "Shutting down", definition: "", comment_usage_note: "", status: "" },
        "DI" => V2TableRow { value: "DI", display_name: "Diagnose", definition: "", comment_usage_note: "", status: "" },
        "MA" => V2TableRow { value: "MA", display_name: "Maintenance", definition: "", comment_usage_note: "", status: "" },
        "FL" => V2TableRow { value: "FL", display_name: "Failure Failure, remaining order new orders cannot be acc", definition: "s are aborted, epted", comment_usage_note: "", status: "" },
        "UNK" => V2TableRow { value: "UNK", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
        "LT" => V2TableRow { value: "LT", display_name: "Limited test menu For diagnostic instrumen types are unavailable", definition: "ts: some test", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0366: V2Table = V2Table {
    number: 366,
    metadata: &super::metadata::TABLE_0366_METADATA,
    rows: phf_map! {
        "L" => V2TableRow { value: "L", display_name: "Local", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Remote", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0367: V2Table = V2Table {
    number: 367,
    metadata: &super::metadata::TABLE_0367_METADATA,
    rows: phf_map! {
        "N" => V2TableRow { value: "N", display_name: "Normal", definition: "", comment_usage_note: "No Corrective Action Needed", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Warning", definition: "", comment_usage_note: "Corrective Action Anticipated", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Serious", definition: "", comment_usage_note: "Corrective Action Required", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Critical", definition: "", comment_usage_note: "Shut Down, Fix Problem and Re-init", status: "" },
    },
};

pub static TABLE_0368: V2Table = V2Table {
    number: 368,
    metadata: &super::metadata::TABLE_0368_METADATA,
    rows: phf_map! {
        "SA" => V2TableRow { value: "SA", display_name: "Sampling", definition: "", comment_usage_note: "", status: "" },
        "LO" => V2TableRow { value: "LO", display_name: "Load", definition: "", comment_usage_note: "", status: "" },
        "UN" => V2TableRow { value: "UN", display_name: "Unload", definition: "", comment_usage_note: "", status: "" },
        "LK" => V2TableRow { value: "LK", display_name: "Lock", definition: "", comment_usage_note: "", status: "" },
        "UC" => V2TableRow { value: "UC", display_name: "Unlock", definition: "", comment_usage_note: "", status: "" },
        "TT" => V2TableRow { value: "TT", display_name: "Transport To", definition: "", comment_usage_note: "", status: "" },
        "CN" => V2TableRow { value: "CN", display_name: "Clear Notification", definition: "", comment_usage_note: "", status: "" },
        "IN" => V2TableRow { value: "IN", display_name: "Initialize/Initiate", definition: "", comment_usage_note: "", status: "" },
        "SU" => V2TableRow { value: "SU", display_name: "Setup", definition: "", comment_usage_note: "", status: "" },
        "CL" => V2TableRow { value: "CL", display_name: "Clear", definition: "", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Pause", definition: "", comment_usage_note: "", status: "" },
        "RE" => V2TableRow { value: "RE", display_name: "Resume", definition: "", comment_usage_note: "", status: "" },
        "ES" => V2TableRow { value: "ES", display_name: "Emergency -stop", definition: "", comment_usage_note: "", status: "" },
        "LC" => V2TableRow { value: "LC", display_name: "Local Control Request", definition: "", comment_usage_note: "", status: "" },
        "RC" => V2TableRow { value: "RC", display_name: "Remote Control Request", definition: "", comment_usage_note: "", status: "" },
        "AB" => V2TableRow { value: "AB", display_name: "Abort", definition: "", comment_usage_note: "", status: "" },
        "EN" => V2TableRow { value: "EN", display_name: "Enable Sending Events", definition: "", comment_usage_note: "", status: "" },
        "DI" => V2TableRow { value: "DI", display_name: "Disable Sending Events", definition: "", comment_usage_note: "", status: "" },
        "EX" => V2TableRow { value: "EX", display_name: "Execute (command specified in field Parameters (ST) 01394)", definition: "", comment_usage_note: "", status: "" },
        "AF" => V2TableRow { value: "AF", display_name: "Aliquot From container", definition: "See desc. below", comment_usage_note: "", status: "" },
        "AT" => V2TableRow { value: "AT", display_name: "Aliquot To container", definition: "See desc. below", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0369: V2Table = V2Table {
    number: 369,
    metadata: &super::metadata::TABLE_0369_METADATA,
    rows: phf_map! {
        "B" => V2TableRow { value: "B", display_name: "Blind Sample", definition: "Used to test the validity of the measurement process, where the composition of the sample is unknown except to the person submitting it.", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Calibrator", definition: "Used for initial setting of calibration of the instrument.", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Electronic QC", definition: "Used with manufactured reference providing signals that simulate QC results", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Filler Organization Proficiency", definition: "Specimen used for testing proficiency of the organization performing the testing (Filler).", comment_usage_note: "", status: "" },
        "G" => V2TableRow { value: "G", display_name: "Group", definition: "Used when solid specimens consist of multiple individual elements that are not individually identified but can be separated again into the original specimens. The identifiers of the original specimens may be tracked.", comment_usage_note: "", status: "" },
        "H" => V2TableRow { value: "H", display_name: "Historical Specimen", definition: "This identifies a parent specimen to the specimen that is submitted for testing", comment_usage_note: "The order of specimen processing can be derived from the SPM-3 value in all SPM segments in the message. More than one historical SPM can be submitted, each SPM describes one step in the derivation process of the specimen submitted for testing.", status: "N" },
        "L" => V2TableRow { value: "L", display_name: "Pool", definition: "Used when aliquots of liquid individual specimens are combined to form a single specimen representing all of the components that are not individually identified. The identifiers of the original specimens may be tracked.", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Operator Proficiency", definition: "Specimen used for testing Operator Proficiency.", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Patient", definition: "Used for any patient sample.", comment_usage_note: "If the component is not valued (blank) this represents the default meaning.", status: "" },
        "Q" => V2TableRow { value: "Q", display_name: "Control specimen", definition: "Used when specimen is the control specimen (either positive or negative).", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Replicate (of patient Us sample as a control) re te", definition: "ed when a patient sample is -run as a control for a repeat st.", comment_usage_note: "", status: "" },
        "V" => V2TableRow { value: "V", display_name: "Verifying Calibrator Us ch", definition: "ed for periodic calibration ecks.", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0370: V2Table = V2Table {
    number: 370,
    metadata: &super::metadata::TABLE_0370_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Archived", definition: "Archived status is used by one system to inform another that the container was already processed by this system, archived for a longer time period and is re- introduced to the automation system for re-processing.", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Identified", definition: "Identified status is used by one system to inform another that it has received a container. In the exchange between the LAS and LIS the Identified status can be used for reporting of the \"In Lab\" (Specimen Received) status. In some cases this may not be equal to the first event of sample recognition.", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Left Equipment", definition: "Left Equipment status is used by one system to inform another that the container has been released from that system.", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Missing", definition: "Missing status is used by one system to inform another that the container did not arrive at its next expected location.", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "In Process", definition: "In Process status is used by one system to inform another that the specific container is being processed by the equipment. It is useful as a response to a query about Container Status, when the specific step of the process is not relevant.", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "In Position", definition: "In Position status is used by one system to inform another that the container is in position for specimen transfer (e.g., container removal from track, pipetting, etc.).", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Process Completed", definition: "Process Completed status is used by one system to inform another that the processing has been completed, but the container has not been released from that system.", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "Unknown status is used by one system to inform another that the container has not been identified.", comment_usage_note: "", status: "" },
        "X" => V2TableRow { value: "X", display_name: "Container Unavailable", definition: "Cancelled status is used by one system to inform another that the container is no longer available within the scope of the system (e.g., tube broken or discarded).", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0371: V2Table = V2Table {
    number: 371,
    metadata: &super::metadata::TABLE_0371_METADATA,
    rows: phf_map! {
        "F10" => V2TableRow { value: "F10", display_name: "10% Formalin", definition: "", comment_usage_note: "Tissue preservative", status: "" },
        "C32" => V2TableRow { value: "C32", display_name: "3.2% Citrate", definition: "", comment_usage_note: "Blue top tube", status: "" },
        "C38" => V2TableRow { value: "C38", display_name: "3.8% Citrate", definition: "", comment_usage_note: "Blue top tube", status: "" },
        "HCL6" => V2TableRow { value: "HCL6", display_name: "6N HCL", definition: "", comment_usage_note: "24 HR Urine Additive", status: "" },
        "ACDA" => V2TableRow { value: "ACDA", display_name: "ACD Solution A", definition: "", comment_usage_note: "Yellow top tube", status: "" },
        "ACD" => V2TableRow { value: "ACD", display_name: "B ACD Solution B", definition: "", comment_usage_note: "Yellow top tube", status: "" },
        "ACET" => V2TableRow { value: "ACET", display_name: "Acetic Acid", definition: "", comment_usage_note: "Urine preservative", status: "" },
        "AMIES" => V2TableRow { value: "AMIES", display_name: "Amies transport medium", definition: "", comment_usage_note: "Protozoa", status: "" },
        "HEPA" => V2TableRow { value: "HEPA", display_name: "Ammonium heparin", definition: "", comment_usage_note: "Green top tube", status: "" },
        "BACTM" => V2TableRow { value: "BACTM", display_name: "Bacterial Transport medium", definition: "", comment_usage_note: "Microbiological culture", status: "" },
        "BOR" => V2TableRow { value: "BOR", display_name: "Borate Boric Acid", definition: "", comment_usage_note: "24HR Urine Additive", status: "" },
        "BOUIN" => V2TableRow { value: "BOUIN", display_name: "Bouin's solution", definition: "Tissue", comment_usage_note: "", status: "" },
        "BF10" => V2TableRow { value: "BF10", display_name: "Buffered 10% formalin", definition: "Tissue", comment_usage_note: "", status: "" },
        "WEST" => V2TableRow { value: "WEST", display_name: "Buffered Citrate (Westergren Sedimentation Rate)", definition: "Black top tube", comment_usage_note: "", status: "" },
        "BSKM" => V2TableRow { value: "BSKM", display_name: "Buffered skim milk", definition: "Viral isolation", comment_usage_note: "", status: "" },
        "CARS" => V2TableRow { value: "CARS", display_name: "Carson's Modified 10% formalin", definition: "Tissue", comment_usage_note: "", status: "" },
        "CARY" => V2TableRow { value: "CARY", display_name: "Cary Blair Medium", definition: "Stool Cultures", comment_usage_note: "", status: "" },
        "CHLTM" => V2TableRow { value: "CHLTM", display_name: "Chlamydia transport medium", definition: "Chlamydia culture", comment_usage_note: "", status: "" },
        "CTAD" => V2TableRow { value: "CTAD", display_name: "CTAD (this should be spelled out if not universally understood)", definition: "Blue top tube", comment_usage_note: "", status: "" },
        "ENT" => V2TableRow { value: "ENT", display_name: "Enteric bacteria transport medium", definition: "Bacterial culture", comment_usage_note: "", status: "" },
        "ENT+" => V2TableRow { value: "ENT+", display_name: "Enteric plus", definition: "Stool Cultures", comment_usage_note: "", status: "" },
        "JKM" => V2TableRow { value: "JKM", display_name: "Jones Kendrick Medium", definition: "Bordetella pertussis", comment_usage_note: "", status: "" },
        "KARN" => V2TableRow { value: "KARN", display_name: "Karnovsky's fixative", definition: "Tissue", comment_usage_note: "", status: "" },
        "LIA" => V2TableRow { value: "LIA", display_name: "Lithium iodoacetate", definition: "Gray top tube", comment_usage_note: "", status: "" },
        "HEPL" => V2TableRow { value: "HEPL", display_name: "Lithium/Li Heparin", definition: "Green top tube", comment_usage_note: "", status: "" },
        "M4" => V2TableRow { value: "M4", display_name: "M4", definition: "Microbiological culture", comment_usage_note: "", status: "" },
        "M4RT" => V2TableRow { value: "M4RT", display_name: "M4-RT", definition: "Microbiological culture", comment_usage_note: "", status: "" },
        "M5" => V2TableRow { value: "M5", display_name: "M5", definition: "Microbiological culture", comment_usage_note: "", status: "" },
        "MICHTM" => V2TableRow { value: "MICHTM", display_name: "Michel's transport medium", definition: "IF tests", comment_usage_note: "", status: "" },
        "MMDTM" => V2TableRow { value: "MMDTM", display_name: "MMD transport medium", definition: "Immunofluorescence", comment_usage_note: "", status: "" },
        "HNO3" => V2TableRow { value: "HNO3", display_name: "Nitric Acid", definition: "Urine", comment_usage_note: "", status: "" },
        "NONE" => V2TableRow { value: "NONE", display_name: "None", definition: "Red or Pink top tube", comment_usage_note: "", status: "" },
        "PAGE" => V2TableRow { value: "PAGE", display_name: "Pages's Saline", definition: "Acanthaoemba", comment_usage_note: "", status: "" },
        "PHENOL" => V2TableRow { value: "PHENOL", display_name: "Phenol", definition: "24 Hr Urine Additive", comment_usage_note: "", status: "" },
        "KOX" => V2TableRow { value: "KOX", display_name: "Potassium Oxalate", definition: "Gray top tube", comment_usage_note: "", status: "" },
        "EDTK" => V2TableRow { value: "EDTK", display_name: "Potassium/K EDTA", definition: "Deprecated. Replaced by EDTK15 and EDTK75", comment_usage_note: "", status: "" },
        "EDTK15" => V2TableRow { value: "EDTK15", display_name: "Potassium/K EDTA 15%", definition: "Purple top tube", comment_usage_note: "", status: "" },
        "EDTK75" => V2TableRow { value: "EDTK75", display_name: "Potassium/K EDTA 7.5%", definition: "Purple top tube", comment_usage_note: "", status: "" },
        "PVA" => V2TableRow { value: "PVA", display_name: "PVA (polyvinylalcohol)", definition: "O&P", comment_usage_note: "", status: "" },
        "RLM" => V2TableRow { value: "RLM", display_name: "Reagan Lowe Medium", definition: "Bordetella pertussis cu", comment_usage_note: "ltures", status: "" },
        "SST" => V2TableRow { value: "SST", display_name: "Serum Separator Tube (Polymer Gel)", definition: "'Tiger' Top tube", comment_usage_note: "", status: "" },
        "SILICA" => V2TableRow { value: "SILICA", display_name: "Siliceous earth, 12 mg", definition: "Gray top tube", comment_usage_note: "", status: "" },
        "NAF" => V2TableRow { value: "NAF", display_name: "Sodium Fluoride", definition: "Gray top tube", comment_usage_note: "", status: "" },
        "FL100" => V2TableRow { value: "FL100", display_name: "Sodium Fluoride, 100mg", definition: "Urine", comment_usage_note: "", status: "" },
        "FL10" => V2TableRow { value: "FL10", display_name: "Sodium Fluoride, 10mg", definition: "Urine", comment_usage_note: "", status: "" },
        "NAPS" => V2TableRow { value: "NAPS", display_name: "Sodium polyanethol sulfonate 0.35% in 0.85% sodium chloride", definition: "Yellow (Blood Culture)", comment_usage_note: "", status: "" },
        "HEPN" => V2TableRow { value: "HEPN", display_name: "Sodium/Na Heparin", definition: "Green top tube", comment_usage_note: "", status: "" },
        "EDTN" => V2TableRow { value: "EDTN", display_name: "Sodium/Na EDTA", definition: "Dark Blue top tube", comment_usage_note: "", status: "" },
        "SPS" => V2TableRow { value: "SPS", display_name: "SPS(this should be spelled out if not universally understood)", definition: "Anticoagulant w/o bacteriocidal propertie", comment_usage_note: "s", status: "" },
        "STUTM" => V2TableRow { value: "STUTM", display_name: "Stuart transport medium", definition: "Bacterial culture", comment_usage_note: "", status: "" },
        "THROM" => V2TableRow { value: "THROM", display_name: "Thrombin", definition: "Orange or Grey/Yellow (STAT Chem)", comment_usage_note: "", status: "" },
        "FDP" => V2TableRow { value: "FDP", display_name: "Thrombin NIH; soybean trypsin inhibitor (Fibrin Degradation Products)", definition: "Dark Blue top tube", comment_usage_note: "", status: "" },
        "THYMOL" => V2TableRow { value: "THYMOL", display_name: "Thymol", definition: "24 Hr Urine Additive", comment_usage_note: "", status: "" },
        "THYO" => V2TableRow { value: "THYO", display_name: "Thyoglycollate broth", definition: "Bacterial Isolation", comment_usage_note: "", status: "" },
        "TOLU" => V2TableRow { value: "TOLU", display_name: "Toluene", definition: "24 Hr Urine Additive", comment_usage_note: "", status: "" },
        "URETM" => V2TableRow { value: "URETM", display_name: "Ureaplasma transport medium", definition: "Ureaplasma culture", comment_usage_note: "", status: "" },
        "VIRTM" => V2TableRow { value: "VIRTM", display_name: "Viral Transport medium", definition: "", comment_usage_note: "Virus cultures", status: "" },
    },
};

pub static TABLE_0372: V2Table = V2Table {
    number: 372,
    metadata: &super::metadata::TABLE_0372_METADATA,
    rows: phf_map! {
        "SUP" => V2TableRow { value: "SUP", display_name: "Supernatant", definition: "", comment_usage_note: "", status: "" },
        "SED" => V2TableRow { value: "SED", display_name: "Sediment", definition: "", comment_usage_note: "", status: "" },
        "BLD" => V2TableRow { value: "BLD", display_name: "Whole blood, homogeneous", definition: "", comment_usage_note: "", status: "" },
        "BSEP" => V2TableRow { value: "BSEP", display_name: "Whole blood, separated", definition: "", comment_usage_note: "", status: "" },
        "PRP" => V2TableRow { value: "PRP", display_name: "Platelet rich plasma", definition: "", comment_usage_note: "", status: "" },
        "PPP" => V2TableRow { value: "PPP", display_name: "Platelet poor plasma", definition: "", comment_usage_note: "", status: "" },
        "SER" => V2TableRow { value: "SER", display_name: "Serum, NOS (not otherwise specified)", definition: "", comment_usage_note: "", status: "" },
        "PLAS" => V2TableRow { value: "PLAS", display_name: "Plasma, NOS (not otherwise specified)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0373: V2Table = V2Table {
    number: 373,
    metadata: &super::metadata::TABLE_0373_METADATA,
    rows: phf_map! {
        "LDLP" => V2TableRow { value: "LDLP", display_name: "LDL Precipitation", definition: "", comment_usage_note: "", status: "" },
        "RECA" => V2TableRow { value: "RECA", display_name: "Recalification", definition: "", comment_usage_note: "", status: "" },
        "DEFB" => V2TableRow { value: "DEFB", display_name: "Defibrination", definition: "", comment_usage_note: "", status: "" },
        "ACID" => V2TableRow { value: "ACID", display_name: "Acidification", definition: "", comment_usage_note: "", status: "" },
        "NEUT" => V2TableRow { value: "NEUT", display_name: "Neutralization", definition: "", comment_usage_note: "", status: "" },
        "ALK" => V2TableRow { value: "ALK", display_name: "Alkalization", definition: "", comment_usage_note: "", status: "" },
        "FILT" => V2TableRow { value: "FILT", display_name: "Filtration", definition: "", comment_usage_note: "", status: "" },
        "UFIL" => V2TableRow { value: "UFIL", display_name: "Ultrafiltration", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0374: V2Table = V2Table {
    number: 374,
    metadata: &super::metadata::TABLE_0374_METADATA,
    rows: phf_map! {
        "CNTM" => V2TableRow { value: "CNTM", display_name: "Present, type of contamination unspecified", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0375: V2Table = V2Table {
    number: 375,
    metadata: &super::metadata::TABLE_0375_METADATA,
    rows: phf_map! {
        "SFHB" => V2TableRow { value: "SFHB", display_name: "Stromal free hemoglobin preparations", definition: "", comment_usage_note: "", status: "" },
        "FLUR" => V2TableRow { value: "FLUR", display_name: "Fluorocarbons", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0376: V2Table = V2Table {
    number: 376,
    metadata: &super::metadata::TABLE_0376_METADATA,
    rows: phf_map! {
        "C37" => V2TableRow { value: "C37", display_name: "Body temperature", definition: "", comment_usage_note: "Critical to keep at body temperature: 36 - 38( C.", status: "" },
        "AMB" => V2TableRow { value: "AMB", display_name: "Ambient temperature", definition: "", comment_usage_note: "Keep at ambient (room) temperature, approximately 22 ( 2 degrees C. Accidental refrigeration or freezing is of little consequence", status: "" },
        "CAMB" => V2TableRow { value: "CAMB", display_name: "Critical ambient temperature", definition: "", comment_usage_note: "Critical ambient - must not be refrigerated or frozen.", status: "" },
        "REF" => V2TableRow { value: "REF", display_name: "Refrigerated temperature", definition: "", comment_usage_note: "Keep at refrigerated temperature: 4-8( C. Accidental warming or freezing is of little consequence", status: "" },
        "CREF" => V2TableRow { value: "CREF", display_name: "Critical refrigerated temperature", definition: "", comment_usage_note: "Critical refrigerated - must not be allowed to freeze or warm until immediately prior to testing", status: "" },
        "FRZ" => V2TableRow { value: "FRZ", display_name: "Frozen temperature", definition: "", comment_usage_note: "Keep at frozen temperature: -4( C. Accidental thawing is of little consequence", status: "" },
        "CF" => V2TableRow { value: "CF", display_name: "RZ Critical frozen temperature", definition: "", comment_usage_note: "Critical frozen - must not be allowed to thaw until immediately prior to testing", status: "" },
        "DFRZ" => V2TableRow { value: "DFRZ", display_name: "Deep frozen", definition: "", comment_usage_note: "Deep frozen: -16 to -20( C.", status: "" },
        "UFRZ" => V2TableRow { value: "UFRZ", display_name: "Ultra frozen", definition: "", comment_usage_note: "Ultra cold frozen: ~ -75 to -85( C. (ultra cold freezer is typically at temperature of dry ice).", status: "" },
        "NTR" => V2TableRow { value: "NTR", display_name: "Liquid nitrogen", definition: "", comment_usage_note: "Keep in liquid nitrogen.", status: "" },
        "PRTL" => V2TableRow { value: "PRTL", display_name: "Protect from light", definition: "Prote", comment_usage_note: "ct from light (e.g., wrap in aluminum foil).", status: "" },
        "CATM" => V2TableRow { value: "CATM", display_name: "Protect from air", definition: "Criti uncap", comment_usage_note: "cal. Do not expose to atmosphere. Do not .", status: "" },
        "DRY" => V2TableRow { value: "DRY", display_name: "Dry", definition: "Keep", comment_usage_note: "in a dry environment.", status: "" },
        "PSO" => V2TableRow { value: "PSO", display_name: "No shock", definition: "Prote", comment_usage_note: "ct from shock.", status: "" },
        "PSA" => V2TableRow { value: "PSA", display_name: "Do not shake", definition: "Do no", comment_usage_note: "t shake.", status: "" },
        "UPR" => V2TableRow { value: "UPR", display_name: "Upright", definition: "Keep", comment_usage_note: "upright. Do not turn upside down.", status: "" },
        "MTLF" => V2TableRow { value: "MTLF", display_name: "Metal Free", definition: "Conta", comment_usage_note: "iner is free of heavy metals including lead.", status: "" },
    },
};

pub static TABLE_0377: V2Table = V2Table {
    number: 377,
    metadata: &super::metadata::TABLE_0377_METADATA,
    rows: phf_map! {
        "ATM" => V2TableRow { value: "ATM", display_name: "Opened container, atmosphere and duration unspecified", definition: "", comment_usage_note: "", status: "" },
        "A60" => V2TableRow { value: "A60", display_name: "Opened container, indoor atmosphere, 60 minutes duration", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0383: V2Table = V2Table {
    number: 383,
    metadata: &super::metadata::TABLE_0383_METADATA,
    rows: phf_map! {
        "EW" => V2TableRow { value: "EW", display_name: "Expired Warning", definition: "", comment_usage_note: "", status: "" },
        "EE" => V2TableRow { value: "EE", display_name: "Expired Error", definition: "", comment_usage_note: "", status: "" },
        "CW" => V2TableRow { value: "CW", display_name: "Calibration Warning", definition: "", comment_usage_note: "", status: "" },
        "CE" => V2TableRow { value: "CE", display_name: "Calibration Error", definition: "", comment_usage_note: "", status: "" },
        "QW" => V2TableRow { value: "QW", display_name: "QC Warning", definition: "", comment_usage_note: "", status: "" },
        "QE" => V2TableRow { value: "QE", display_name: "QC Error", definition: "", comment_usage_note: "", status: "" },
        "NW" => V2TableRow { value: "NW", display_name: "Not Available Warning", definition: "", comment_usage_note: "", status: "" },
        "NE" => V2TableRow { value: "NE", display_name: "Not Available Error", definition: "", comment_usage_note: "", status: "" },
        "OW" => V2TableRow { value: "OW", display_name: "Other Warning", definition: "", comment_usage_note: "", status: "" },
        "OE" => V2TableRow { value: "OE", display_name: "Other Error", definition: "", comment_usage_note: "", status: "" },
        "OK" => V2TableRow { value: "OK", display_name: "OK Status", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0384: V2Table = V2Table {
    number: 384,
    metadata: &super::metadata::TABLE_0384_METADATA,
    rows: phf_map! {
        "SR" => V2TableRow { value: "SR", display_name: "Single Test Reagent", definition: "", comment_usage_note: "", status: "" },
        "MR" => V2TableRow { value: "MR", display_name: "Multiple Test Reagent", definition: "", comment_usage_note: "The consumption cannot be tied to orders for a single test", status: "" },
        "DI" => V2TableRow { value: "DI", display_name: "Diluent", definition: "", comment_usage_note: "", status: "" },
        "PT" => V2TableRow { value: "PT", display_name: "Pretreatment", definition: "", comment_usage_note: "", status: "" },
        "RC" => V2TableRow { value: "RC", display_name: "Reagent Calibrator", definition: "", comment_usage_note: "", status: "" },
        "CO" => V2TableRow { value: "CO", display_name: "Control Reagent", definition: "", comment_usage_note: "", status: "" },
        "PW" => V2TableRow { value: "PW", display_name: "Purified Water", definition: "", comment_usage_note: "", status: "" },
        "LW" => V2TableRow { value: "LW", display_name: "Liquid Waste", definition: "", comment_usage_note: "", status: "" },
        "SW" => V2TableRow { value: "SW", display_name: "Solid Waste", definition: "", comment_usage_note: "", status: "" },
        "SC" => V2TableRow { value: "SC", display_name: "Countable Solid Item", definition: "E.g., Pipetting tip", comment_usage_note: "", status: "" },
        "LI" => V2TableRow { value: "LI", display_name: "Measurable Liquid Item", definition: "", comment_usage_note: "", status: "" },
        "OT" => V2TableRow { value: "OT", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0387: V2Table = V2Table {
    number: 387,
    metadata: &super::metadata::TABLE_0387_METADATA,
    rows: phf_map! {
        "OK" => V2TableRow { value: "OK", display_name: "Command completed successfully", definition: "", comment_usage_note: "", status: "" },
        "TI" => V2TableRow { value: "TI", display_name: "Command cannot be completed within requested completion time", definition: "", comment_usage_note: "", status: "" },
        "ER" => V2TableRow { value: "ER", display_name: "Command cannot be completed because of error condition", definition: "See response parameters.", comment_usage_note: "", status: "" },
        "ST" => V2TableRow { value: "ST", display_name: "Command cannot be completed because of the status of the requested equipment", definition: "", comment_usage_note: "", status: "" },
        "UN" => V2TableRow { value: "UN", display_name: "Command cannot be completed for unknown reasons", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0388: V2Table = V2Table {
    number: 388,
    metadata: &super::metadata::TABLE_0388_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Regular Production", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Evaluation", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0389: V2Table = V2Table {
    number: 389,
    metadata: &super::metadata::TABLE_0389_METADATA,
    rows: phf_map! {
        "O" => V2TableRow { value: "O", display_name: "Original, first run", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Repeated without dilution", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Repeated with dilution", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Reflex test", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0391: V2Table = V2Table {
    number: 391,
    metadata: &super::metadata::TABLE_0391_METADATA,
    rows: phf_map! {
        "ADMINISTRATION" => V2TableRow { value: "ADMINISTRATION", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ALLERGY" => V2TableRow { value: "ALLERGY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "APP_STATS" => V2TableRow { value: "APP_STATS", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "APP_STATUS" => V2TableRow { value: "APP_STATUS", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ASSOCIATED_PERSON" => V2TableRow { value: "ASSOCIATED_PERSON", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ASSOCIATED_RX_ADMIN" => V2TableRow { value: "ASSOCIATED_RX_ADMIN", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ASSOCIATED_RX_ORDER" => V2TableRow { value: "ASSOCIATED_RX_ORDER", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "AUTHORIZATION" => V2TableRow { value: "AUTHORIZATION", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "AUTHORIZATION_CONTACT" => V2TableRow { value: "AUTHORIZATION_CONTACT", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "CERTIFICATE" => V2TableRow { value: "CERTIFICATE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "CLOCK" => V2TableRow { value: "CLOCK", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "CLOCK_AND_STATISTICS" => V2TableRow { value: "CLOCK_AND_STATISTICS", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "CLOCK_AND_STATS_WIT" => V2TableRow { value: "CLOCK_AND_STATS_WIT", display_name: "H_NOTES", definition: "", comment_usage_note: "", status: "" },
        "COMMAND" => V2TableRow { value: "COMMAND", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "COMMAND_RESPONSE" => V2TableRow { value: "COMMAND_RESPONSE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "COMMON_ORDER" => V2TableRow { value: "COMMON_ORDER", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "COMPONENT" => V2TableRow { value: "COMPONENT", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "COMPONENTS" => V2TableRow { value: "COMPONENTS", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "CONTAINER" => V2TableRow { value: "CONTAINER", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "DEFINITION" => V2TableRow { value: "DEFINITION", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "DIET" => V2TableRow { value: "DIET", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "DISPENSE" => V2TableRow { value: "DISPENSE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ENCODED_ORDER" => V2TableRow { value: "ENCODED_ORDER", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ENCODING" => V2TableRow { value: "ENCODING", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "EXPERIENCE" => V2TableRow { value: "EXPERIENCE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "FINANCIAL" => V2TableRow { value: "FINANCIAL", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "FINANCIAL_COMMON_ORDER" => V2TableRow { value: "FINANCIAL_COMMON_ORDER", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "FINANCIAL_INSURANCE" => V2TableRow { value: "FINANCIAL_INSURANCE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "FINANCIAL_OBSERVATION" => V2TableRow { value: "FINANCIAL_OBSERVATION", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "FINANCIAL_ORDER" => V2TableRow { value: "FINANCIAL_ORDER", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "FINANCIAL_PROCE" => V2TableRow { value: "FINANCIAL_PROCE", display_name: "DURE", definition: "", comment_usage_note: "", status: "" },
        "FINANCIAL_TIMING_QUANTITY" => V2TableRow { value: "FINANCIAL_TIMING_QUANTITY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "GENERAL_RESOURCE" => V2TableRow { value: "GENERAL_RESOURCE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "GIVE" => V2TableRow { value: "GIVE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "GOAL" => V2TableRow { value: "GOAL", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "GOAL_OBSE" => V2TableRow { value: "GOAL_OBSE", display_name: "RVATION", definition: "", comment_usage_note: "", status: "" },
        "GOAL_PATHWAY" => V2TableRow { value: "GOAL_PATHWAY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "GOAL_ROLE" => V2TableRow { value: "GOAL_ROLE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "GUARANTOR_INSURANCE" => V2TableRow { value: "GUARANTOR_INSURANCE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "INSURANCE" => V2TableRow { value: "INSURANCE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "LOCATION_RESOURCE" => V2TableRow { value: "LOCATION_RESOURCE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MERGE_INFO" => V2TableRow { value: "MERGE_INFO", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF" => V2TableRow { value: "MF", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_CDM" => V2TableRow { value: "MF_CDM", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_CLIN_STUDY" => V2TableRow { value: "MF_CLIN_STUDY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_CLIN_STUDY_SCHED" => V2TableRow { value: "MF_CLIN_STUDY_SCHED", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_INV_ITEM" => V2TableRow { value: "MF_INV_ITEM", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_LOC_DEPT" => V2TableRow { value: "MF_LOC_DEPT", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_LOCATION" => V2TableRow { value: "MF_LOCATION", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_OBS_ATTRIBUTES" => V2TableRow { value: "MF_OBS_ATTRIBUTES", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_PHASE_SCHED_DETAIL" => V2TableRow { value: "MF_PHASE_SCHED_DETAIL", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_QUERY" => V2TableRow { value: "MF_QUERY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_SITE_DEFINED" => V2TableRow { value: "MF_SITE_DEFINED", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_STAFF" => V2TableRow { value: "MF_STAFF", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_TEST" => V2TableRow { value: "MF_TEST", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_TEST_BATT_DETAIL" => V2TableRow { value: "MF_TEST_BATT_DETAIL", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_TEST_BATTERIES" => V2TableRow { value: "MF_TEST_BATTERIES", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_TEST_CALC_DETAIL" => V2TableRow { value: "MF_TEST_CALC_DETAIL", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_TEST_CALCULATED" => V2TableRow { value: "MF_TEST_CALCULATED", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_TEST_CAT_DETAIL" => V2TableRow { value: "MF_TEST_CAT_DETAIL", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_TEST_CATEGORICAL" => V2TableRow { value: "MF_TEST_CATEGORICAL", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "MF_TEST_NUMERIC" => V2TableRow { value: "MF_TEST_NUMERIC", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "NK1_TIMING_QTY" => V2TableRow { value: "NK1_TIMING_QTY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "NOTIFICATION" => V2TableRow { value: "NOTIFICATION", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "OBSE" => V2TableRow { value: "OBSE", display_name: "RVATION", definition: "", comment_usage_note: "", status: "" },
        "OMSERVATION" => V2TableRow { value: "OMSERVATION", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ORDER" => V2TableRow { value: "ORDER", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ORDER_" => V2TableRow { value: "ORDER_", display_name: "CHOICE", definition: "", comment_usage_note: "", status: "" },
        "ORDER_DETAIL" => V2TableRow { value: "ORDER_DETAIL", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ORDER_DETAIL_SUPPLEMENT" => V2TableRow { value: "ORDER_DETAIL_SUPPLEMENT", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ORDER_DIET" => V2TableRow { value: "ORDER_DIET", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ORDER_ENCODED" => V2TableRow { value: "ORDER_ENCODED", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ORDER_OBSE" => V2TableRow { value: "ORDER_OBSE", display_name: "RVATION", definition: "", comment_usage_note: "", status: "" },
        "ORDER_P" => V2TableRow { value: "ORDER_P", display_name: "RIO R", definition: "", comment_usage_note: "", status: "" },
        "ORDER_T" => V2TableRow { value: "ORDER_T", display_name: "RAY", definition: "", comment_usage_note: "", status: "" },
        "PATHWAY" => V2TableRow { value: "PATHWAY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "PATHWAY_ROLE" => V2TableRow { value: "PATHWAY_ROLE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "PATIENT" => V2TableRow { value: "PATIENT", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "PATIENT_PRIOR" => V2TableRow { value: "PATIENT_PRIOR", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "PATIENT_RESULT" => V2TableRow { value: "PATIENT_RESULT", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "PATIENT_VISIT" => V2TableRow { value: "PATIENT_VISIT", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "PATIENT_VISIT_PRIOR" => V2TableRow { value: "PATIENT_VISIT_PRIOR", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "PERSONNEL_RESOURCE" => V2TableRow { value: "PERSONNEL_RESOURCE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "PEX_CAUSE" => V2TableRow { value: "PEX_CAUSE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "PEX_OBSE" => V2TableRow { value: "PEX_OBSE", display_name: "RVATION", definition: "", comment_usage_note: "", status: "" },
        "PRIOR_" => V2TableRow { value: "PRIOR_", display_name: "RESULT", definition: "", comment_usage_note: "", status: "" },
        "PRO" => V2TableRow { value: "PRO", display_name: "BLEM", definition: "", comment_usage_note: "", status: "" },
        "PRODU" => V2TableRow { value: "PRODU", display_name: "CT", definition: "", comment_usage_note: "", status: "" },
        "PRODUCT_STATUS" => V2TableRow { value: "PRODUCT_STATUS", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "PROVIDE" => V2TableRow { value: "PROVIDE", display_name: "R", definition: "", comment_usage_note: "", status: "" },
        "QBP" => V2TableRow { value: "QBP", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "QRY_WITH_DETAIL" => V2TableRow { value: "QRY_WITH_DETAIL", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "QUERY_RE" => V2TableRow { value: "QUERY_RE", display_name: "SPONSE", definition: "", comment_usage_note: "", status: "" },
        "REQUEST" => V2TableRow { value: "REQUEST", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "RESOU" => V2TableRow { value: "RESOU", display_name: "RCE", definition: "", comment_usage_note: "", status: "" },
        "RESPO" => V2TableRow { value: "RESPO", display_name: "NSE", definition: "", comment_usage_note: "", status: "" },
        "RESULT" => V2TableRow { value: "RESULT", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "RESULTS" => V2TableRow { value: "RESULTS", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "RESULTS_NOTES" => V2TableRow { value: "RESULTS_NOTES", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ROW_DE" => V2TableRow { value: "ROW_DE", display_name: "FINITION", definition: "", comment_usage_note: "", status: "" },
        "RX_ADMINISTRATION" => V2TableRow { value: "RX_ADMINISTRATION", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "RX_ORDE" => V2TableRow { value: "RX_ORDE", display_name: "R", definition: "", comment_usage_note: "", status: "" },
        "SCHEDULE" => V2TableRow { value: "SCHEDULE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "SERVICE" => V2TableRow { value: "SERVICE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "SPECIMEN" => V2TableRow { value: "SPECIMEN", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "SPECIMEN_CONTAINER" => V2TableRow { value: "SPECIMEN_CONTAINER", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "STAFF" => V2TableRow { value: "STAFF", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "STUDY" => V2TableRow { value: "STUDY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "STUDY_OBSERVATION" => V2TableRow { value: "STUDY_OBSERVATION", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "STUDY_PHASE" => V2TableRow { value: "STUDY_PHASE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "STUDY_SCHEDULE" => V2TableRow { value: "STUDY_SCHEDULE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "TEST_CONFIGURATION" => V2TableRow { value: "TEST_CONFIGURATION", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "TIMING" => V2TableRow { value: "TIMING", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "TIMING_DIET" => V2TableRow { value: "TIMING_DIET", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "TIMING_ENCODED" => V2TableRow { value: "TIMING_ENCODED", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "TIMING_GIVE" => V2TableRow { value: "TIMING_GIVE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "TIMING_PRIOR" => V2TableRow { value: "TIMING_PRIOR", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "TIMING_QTY" => V2TableRow { value: "TIMING_QTY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "TIMING_QUANTITY" => V2TableRow { value: "TIMING_QUANTITY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "TIMING_TRAY" => V2TableRow { value: "TIMING_TRAY", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "TREATMENT" => V2TableRow { value: "TREATMENT", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "VISIT" => V2TableRow { value: "VISIT", display_name: "", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0392: V2Table = V2Table {
    number: 392,
    metadata: &super::metadata::TABLE_0392_METADATA,
    rows: phf_map! {
        "DB" => V2TableRow { value: "DB", display_name: "Match on Date of Birth", definition: "", comment_usage_note: "", status: "" },
        "NA" => V2TableRow { value: "NA", display_name: "Match on Name (Alpha Match)", definition: "", comment_usage_note: "", status: "" },
        "NP" => V2TableRow { value: "NP", display_name: "Match on Name (Phonetic Match)", definition: "", comment_usage_note: "", status: "" },
        "SS" => V2TableRow { value: "SS", display_name: "Match on Social Security Number", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0393: V2Table = V2Table {
    number: 393,
    metadata: &super::metadata::TABLE_0393_METADATA,
    rows: phf_map! {
        "LINKSOFT_2." => V2TableRow { value: "LINKSOFT_2.", display_name: "Proprietary", definition: "", comment_usage_note: "", status: "" },
        "01" => V2TableRow { value: "01", display_name: "algorithm for LinkSoft v2.01", definition: "", comment_usage_note: "", status: "" },
        "MATCH" => V2TableRow { value: "MATCH", display_name: "WA Proprietary", definition: "", comment_usage_note: "", status: "" },
        "RE_1.2" => V2TableRow { value: "RE_1.2", display_name: "algorithm for MatchWare v1.2", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0394: V2Table = V2Table {
    number: 394,
    metadata: &super::metadata::TABLE_0394_METADATA,
    rows: phf_map! {
        "R" => V2TableRow { value: "R", display_name: "Real Time", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Bolus (a series of responses sent at the same time without use of batch formatting)", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Batch", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0395: V2Table = V2Table {
    number: 395,
    metadata: &super::metadata::TABLE_0395_METADATA,
    rows: phf_map! {
        "N" => V2TableRow { value: "N", display_name: "New Subscription", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Modified Subscription", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0396: V2Table = V2Table {
    number: 396,
    metadata: &super::metadata::TABLE_0396_METADATA,
    rows: phf_map! {
        "99zzz" => V2TableRow { value: "99zzz", display_name: "Local general code for a site-defined code system used for a specific set of trading partners. The 'zzz' SHALL be any printable ASCII string. Length of the name SHALL not exceed field width, and is subject to local implementation.", definition: "", comment_usage_note: "The 'zzz' may be any printable ASCII string of variable length, but the recommendation is to generally use only numbers and uppercase letters, as that is how all of the currently existent table 396 entries have been created for many years. In addition, there is no explicit length restriction, but it is recommended that the string be kept below a 20 character maximum.", status: "" },
        "ACR" => V2TableRow { value: "ACR", display_name: "American College of Radiology finding codes", definition: "", comment_usage_note: "Index for Radiological Diagnosis Revised, 3rd Edition 1986, American College of Radiology, Reston, VA.", status: "" },
        "ACTCO" => V2TableRow { value: "ACTCO", display_name: "DE Table of HL7 Version 3 ActCode values", definition: "", comment_usage_note: "For use in v2.x systems interoperating with V3 systems. Identical to the code system 2.16.840.1.113883.5.4 ActCode in the Version 3 vocabulary.", status: "" },
        "ACTRELSS" => V2TableRow { value: "ACTRELSS", display_name: "Used to indicate that the target of the relationship will be a filtered subset of the total related set of targets. Used when there is a need to limit the number of components to the first, the last, the next, the total, the average or some other filtere", definition: "", comment_usage_note: "V3 coding system. Download with V3 materials.", status: "" },
        "ALPHAID2" => V2TableRow { value: "ALPHAID2", display_name: "German Alpha-ID v2006", definition: "", comment_usage_note: "ID of the alphabetical Index ICD-10-GM-", status: "" },
        "006" => V2TableRow { value: "006", display_name: "", definition: "", comment_usage_note: "2006. Alpha-ID.", status: "" },
        "007" => V2TableRow { value: "007", display_name: "", definition: "", comment_usage_note: "2007. Alpha-ID.", status: "" },
        "008" => V2TableRow { value: "008", display_name: "", definition: "", comment_usage_note: "2008. Alpha-ID.", status: "" },
        "009" => V2TableRow { value: "009", display_name: "", definition: "", comment_usage_note: "2009. Alpha-ID.", status: "" },
        "AMTv2" => V2TableRow { value: "AMTv2", display_name: "Australian Medicines Terminology (v2)", definition: "", comment_usage_note: "The national terminology to identify medicines used in Australia, using unique codes to deliver unambiguous, accurate and standardised names for both branded (trade) and generic (medicinal) products.", status: "" },
        "ANS+" => V2TableRow { value: "ANS+", display_name: "HL7 set of units of measure", definition: "", comment_usage_note: "HL7 set of units of measure based upon ANSI X3.50 - 1986, ISO 2988-83, and US customary units / see chapter 7, section 7.4.2.6.", status: "" },
        "ART" => V2TableRow { value: "ART", display_name: "WHO Adverse Reaction Terms", definition: "", comment_usage_note: "WHO Collaborating Centre for International Drug Monitoring, Box 26, S-", status: "" },
        "AS4" => V2TableRow { value: "AS4", display_name: "ASTM E1238/ E1467 Universal", definition: "", comment_usage_note: "American Society for Testing & Materials and CPT4 (see Appendix X1 of Specification E1238 and Appendix X2 of Specification E1467).", status: "" },
        "AS4E" => V2TableRow { value: "AS4E", display_name: "AS4 Neurophysiology Codes", definition: "", comment_usage_note: "ASTM’s diagnostic codes and test result coding/grading systems for clinical neurophysiology. See ASTM Specification E1467, Appendix 2.", status: "" },
        "ATC" => V2TableRow { value: "ATC", display_name: "American Type Culture Collection", definition: "", comment_usage_note: "Reference cultures (microorganisms, tissue cultures, etc.), related biological materials and associated data. American Type Culture Collection, 12301 Parklawn Dr, Rockville MD, 20852. (301) 881-2600.", status: "" },
        "C4" => V2TableRow { value: "C4", display_name: "CPT-4", definition: "", comment_usage_note: "American Medical Association, P.O. Box 10946, Chicago IL 60610.", status: "" },
        "C5" => V2TableRow { value: "C5", display_name: "CPT-5", definition: "", comment_usage_note: "Not currently being worked on, no D proposed release date at this time. American Medical Association, P.O. Box 10946, Chicago IL 60610.", status: "" },
        "CAPE" => V2TableRow { value: "CAPE", display_name: "CC College of American Pathologists Electronic Cancer Checklist", definition: "", comment_usage_note: "Each code in this system represents a single line in a database template for the College of American Pathologists Electronic Cancer Checklist (CAP eCC). Each line and its code corresponds to either a question or an answer selection. The code is in a decimal format of #########.#########, where each \"#\" is an optional number. The nine digits to the right of the Ckey decimal point make up a namespace identifier, which is specific to the center that created the database entries for the checklist line items with their unique Ckey values. The namespace identifier for SNOMED Terminology Solutions at the College of American Pathologists is 1000043. All Ckey values in the 2008 release use namespace 1000043. The digits to the left of the decimal point are a locally assigned sequential key for the ChecklistTemplateItems table in the local CAP eCC database. These codes are used to specify questions and answers selected in a CAP eCC template for transmission in an HL7 message, as defined by the NAACCR Pathology Workgroup and the CDC/NPCR Reporting Pathology Protocols II (RPP II) project. SNOMED Terminology Solutions, College of American Pathologists, 325 Waukegan Road, Northfield, Illinois, 60093, snomedsolutions@cap.org", status: "" },
        "CAS" => V2TableRow { value: "CAS", display_name: "Chemical abstract codes", definition: "", comment_usage_note: "These include unique codes for each unique chemical, including all generic drugs. The", status: "" },
    },
};

pub static TABLE_0397: V2Table = V2Table {
    number: 397,
    metadata: &super::metadata::TABLE_0397_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Ascending", definition: "", comment_usage_note: "", status: "" },
        "AN" => V2TableRow { value: "AN", display_name: "Ascending, case insensitive", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Descending", definition: "", comment_usage_note: "", status: "" },
        "DN" => V2TableRow { value: "DN", display_name: "Descending, case insensitive", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "None", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0398: V2Table = V2Table {
    number: 398,
    metadata: &super::metadata::TABLE_0398_METADATA,
    rows: phf_map! {
        "F" => V2TableRow { value: "F", display_name: "Fragmentation", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Interactive Continuation", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0401: V2Table = V2Table {
    number: 401,
    metadata: &super::metadata::TABLE_0401_METADATA,
    rows: phf_map! {
        "MM" => V2TableRow { value: "MM", display_name: "Medicare", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Medi-Cal", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0402: V2Table = V2Table {
    number: 402,
    metadata: &super::metadata::TABLE_0402_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Dental", definition: "", comment_usage_note: "", status: "" },
        "G" => V2TableRow { value: "G", display_name: "Graduate", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Medical", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Undergraduate", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0403: V2Table = V2Table {
    number: 403,
    metadata: &super::metadata::TABLE_0403_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Read", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Write", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Speak", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Understand", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Sign", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0404: V2Table = V2Table {
    number: 404,
    metadata: &super::metadata::TABLE_0404_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Excellent", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Good", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Fair", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Poor", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Some (level unknown)", definition: "", comment_usage_note: "", status: "" },
        "6" => V2TableRow { value: "6", display_name: "None", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0406: V2Table = V2Table {
    number: 406,
    metadata: &super::metadata::TABLE_0406_METADATA,
    rows: phf_map! {
        "H" => V2TableRow { value: "H", display_name: "Home", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Office", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Hospital", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Physician Clinic", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Long Term Care", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Acute Care", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0409: V2Table = V2Table {
    number: 409,
    metadata: &super::metadata::TABLE_0409_METADATA,
    rows: phf_map! {
        "SU" => V2TableRow { value: "SU", display_name: "Start up", definition: "", comment_usage_note: "", status: "" },
        "SD" => V2TableRow { value: "SD", display_name: "Shut down", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Migrates to different CPU", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0411: V2Table = V2Table {
    number: 411,
    metadata: &super::metadata::TABLE_0411_METADATA,
    rows: phf_map! {
        "NDBS" => V2TableRow { value: "NDBS", display_name: "Observation of type Newborn Dried Blood Screening (NDBS)", definition: "", comment_usage_note: "", status: "N" },
        "CG" => V2TableRow { value: "CG", display_name: "Observation of type Clinical Genomics (CG)", definition: "", comment_usage_note: "", status: "N" },
    },
};

pub static TABLE_0415: V2Table = V2Table {
    number: 415,
    metadata: &super::metadata::TABLE_0415_METADATA,
    rows: phf_map! {
        "N" => V2TableRow { value: "N", display_name: "DRG Non Exempt", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "DRG Exempt", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0416: V2Table = V2Table {
    number: 416,
    metadata: &super::metadata::TABLE_0416_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "1st non-Operative", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "2nd non- Operative", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Major Operative", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "2nd Operative", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "3rd Operative", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0417: V2Table = V2Table {
    number: 417,
    metadata: &super::metadata::TABLE_0417_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Insufficient Tissue", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Not abnormal", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Abnormal-not categorized", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Mechanical abnormal", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Growth alteration", definition: "", comment_usage_note: "", status: "" },
        "6" => V2TableRow { value: "6", display_name: "Degeneration & necrosis", definition: "", comment_usage_note: "", status: "" },
        "7" => V2TableRow { value: "7", display_name: "Non-acute inflammation", definition: "", comment_usage_note: "", status: "" },
        "8" => V2TableRow { value: "8", display_name: "Non-malignant neoplasm", definition: "", comment_usage_note: "", status: "" },
        "9" => V2TableRow { value: "9", display_name: "Malignant neoplasm", definition: "", comment_usage_note: "", status: "" },
        "0" => V2TableRow { value: "0", display_name: "No tissue expected", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Basal cell carcinoma", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Carcinoma- unspecified type", definition: "", comment_usage_note: "", status: "" },
        "G" => V2TableRow { value: "G", display_name: "Additional tissue required", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0418: V2Table = V2Table {
    number: 418,
    metadata: &super::metadata::TABLE_0418_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "the admitting procedure", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "the primary procedure", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "for ranked secondary procedures", definition: "", comment_usage_note: "", status: "" },
        "..." => V2TableRow { value: "...", display_name: "No suggested values defined", definition: "", comment_usage_note: "", status: "D" },
    },
};

pub static TABLE_0421: V2Table = V2Table {
    number: 421,
    metadata: &super::metadata::TABLE_0421_METADATA,
    rows: phf_map! {
        "MI" => V2TableRow { value: "MI", display_name: "Mild", definition: "", comment_usage_note: "", status: "" },
        "MO" => V2TableRow { value: "MO", display_name: "Moderate", definition: "", comment_usage_note: "", status: "" },
        "SE" => V2TableRow { value: "SE", display_name: "Severe", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0422: V2Table = V2Table {
    number: 422,
    metadata: &super::metadata::TABLE_0422_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Non-acute", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Acute", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Urgent", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Severe", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Dead on Arrival (DOA)", definition: "", comment_usage_note: "", status: "" },
        "99" => V2TableRow { value: "99", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0423: V2Table = V2Table {
    number: 423,
    metadata: &super::metadata::TABLE_0423_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Doctor's Office Closed", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0424: V2Table = V2Table {
    number: 424,
    metadata: &super::metadata::TABLE_0424_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Premature / Pre- term", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Full Term", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Overdue / Post- term", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0425: V2Table = V2Table {
    number: 425,
    metadata: &super::metadata::TABLE_0425_METADATA,
    rows: phf_map! {
        "5" => V2TableRow { value: "5", display_name: "Born at home", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Born en route", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Born in facility", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Transfer in", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0426: V2Table = V2Table {
    number: 426,
    metadata: &super::metadata::TABLE_0426_METADATA,
    rows: phf_map! {
        "CRYO" => V2TableRow { value: "CRYO", display_name: "Cryoprecipitated AHF", definition: "", comment_usage_note: "", status: "" },
        "CRY" => V2TableRow { value: "CRY", display_name: "OP Pooled Cryoprecipitate", definition: "", comment_usage_note: "", status: "" },
        "FFP" => V2TableRow { value: "FFP", display_name: "Fresh Frozen Plasma", definition: "", comment_usage_note: "", status: "" },
        "FFPTH" => V2TableRow { value: "FFPTH", display_name: "Fresh Frozen Plasma - T h awed", definition: "", comment_usage_note: "", status: "" },
        "PC" => V2TableRow { value: "PC", display_name: "Packed Cells", definition: "", comment_usage_note: "", status: "" },
        "PCA" => V2TableRow { value: "PCA", display_name: "Autologous Packed Cells", definition: "", comment_usage_note: "", status: "" },
        "PCNEO" => V2TableRow { value: "PCNEO", display_name: "Packed Cells - Neonatal", definition: "", comment_usage_note: "", status: "" },
        "PCW" => V2TableRow { value: "PCW", display_name: "Washed Packed Cells", definition: "", comment_usage_note: "", status: "" },
        "PLT" => V2TableRow { value: "PLT", display_name: "Platelet Concentrate", definition: "", comment_usage_note: "", status: "" },
        "PLTNEO" => V2TableRow { value: "PLTNEO", display_name: "Reduced Volume Platelets", definition: "", comment_usage_note: "", status: "" },
        "PLTP" => V2TableRow { value: "PLTP", display_name: "Pooled Platelets", definition: "", comment_usage_note: "", status: "" },
        "PLTPH" => V2TableRow { value: "PLTPH", display_name: "Platelet Pheresis", definition: "", comment_usage_note: "", status: "" },
        "PLTPHLR" => V2TableRow { value: "PLTPHLR", display_name: "Leukoreduced Platelet Pheresis", definition: "", comment_usage_note: "", status: "" },
        "RWB" => V2TableRow { value: "RWB", display_name: "Reconstituted Whole Blood", definition: "", comment_usage_note: "", status: "" },
        "WBA" => V2TableRow { value: "WBA", display_name: "Autologous Whole Blood", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0427: V2Table = V2Table {
    number: 427,
    metadata: &super::metadata::TABLE_0427_METADATA,
    rows: phf_map! {
        "B" => V2TableRow { value: "B", display_name: "Body fluid exposure", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Contaminated Substance", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Diet Errors", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Equipment problem", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Patient fell (not from bed)", definition: "", comment_usage_note: "", status: "" },
        "H" => V2TableRow { value: "H", display_name: "Patient fell from bed", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Infusion error", definition: "", comment_usage_note: "", status: "" },
        "J" => V2TableRow { value: "J", display_name: "Foreign object left during surgery", definition: "", comment_usage_note: "", status: "" },
        "K" => V2TableRow { value: "K", display_name: "Sterile precaution violated", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Procedure error", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Pharmaceutical error", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Suicide Attempt", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Transfusion error", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0428: V2Table = V2Table {
    number: 428,
    metadata: &super::metadata::TABLE_0428_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Preventable", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "User Error", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0429: V2Table = V2Table {
    number: 429,
    metadata: &super::metadata::TABLE_0429_METADATA,
    rows: phf_map! {
        "BR" => V2TableRow { value: "BR", display_name: "Breeding/genetic stock", definition: "", comment_usage_note: "", status: "" },
        "DA" => V2TableRow { value: "DA", display_name: "Dairy", definition: "", comment_usage_note: "", status: "" },
        "DR" => V2TableRow { value: "DR", display_name: "Draft", definition: "", comment_usage_note: "", status: "" },
        "DU" => V2TableRow { value: "DU", display_name: "Dual Purpose", definition: "", comment_usage_note: "", status: "" },
        "LY" => V2TableRow { value: "LY", display_name: "Layer, Includes Multiplier flocks", definition: "", comment_usage_note: "", status: "" },
        "MT" => V2TableRow { value: "MT", display_name: "Meat", definition: "", comment_usage_note: "", status: "" },
        "OT" => V2TableRow { value: "OT", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "PL" => V2TableRow { value: "PL", display_name: "Pleasure", definition: "", comment_usage_note: "", status: "" },
        "RA" => V2TableRow { value: "RA", display_name: "Racing", definition: "", comment_usage_note: "", status: "" },
        "SH" => V2TableRow { value: "SH", display_name: "Show", definition: "", comment_usage_note: "", status: "" },
        "NA" => V2TableRow { value: "NA", display_name: "Not Applicable", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0430: V2Table = V2Table {
    number: 430,
    metadata: &super::metadata::TABLE_0430_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Ambulance", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Car", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "On foot", definition: "", comment_usage_note: "", status: "" },
        "H" => V2TableRow { value: "H", display_name: "Helicopter", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Public Transport", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0431: V2Table = V2Table {
    number: 431,
    metadata: &super::metadata::TABLE_0431_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Alcohol", definition: "", comment_usage_note: "", status: "" },
        "K" => V2TableRow { value: "K", display_name: "Kava", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Marijuana", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Tobacco - smoked", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Tobacco - chewed", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0432: V2Table = V2Table {
    number: 432,
    metadata: &super::metadata::TABLE_0432_METADATA,
    rows: phf_map! {
        "AC" => V2TableRow { value: "AC", display_name: "A c ut e", definition: "", comment_usage_note: "", status: "" },
        "CH" => V2TableRow { value: "CH", display_name: "Chronic", definition: "", comment_usage_note: "", status: "" },
        "CO" => V2TableRow { value: "CO", display_name: "Comatose", definition: "", comment_usage_note: "", status: "" },
        "CR" => V2TableRow { value: "CR", display_name: "Critical", definition: "", comment_usage_note: "", status: "" },
        "IM" => V2TableRow { value: "IM", display_name: "Improved", definition: "", comment_usage_note: "", status: "" },
        "MO" => V2TableRow { value: "MO", display_name: "Moribund", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0433: V2Table = V2Table {
    number: 433,
    metadata: &super::metadata::TABLE_0433_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Aggressive", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Blind", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Confused", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Deaf", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "On IV", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Do not resuscitate", definition: "\"No-code\" (i.e. Do not resuscitate)", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Paraplegic", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0434: V2Table = V2Table {
    number: 434,
    metadata: &super::metadata::TABLE_0434_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Satisfactory", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Critical", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Poor", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Stable", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0435: V2Table = V2Table {
    number: 435,
    metadata: &super::metadata::TABLE_0435_METADATA,
    rows: phf_map! {
        "DNR" => V2TableRow { value: "DNR", display_name: "Do not resuscitate", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No directive", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0436: V2Table = V2Table {
    number: 436,
    metadata: &super::metadata::TABLE_0436_METADATA,
    rows: phf_map! {
        "AD" => V2TableRow { value: "AD", display_name: "Adverse Reaction (Not otherwise classified)", definition: "", comment_usage_note: "", status: "" },
        "AL" => V2TableRow { value: "AL", display_name: "Allergy", definition: "", comment_usage_note: "", status: "" },
        "CT" => V2TableRow { value: "CT", display_name: "Contraindication", definition: "", comment_usage_note: "", status: "" },
        "IN" => V2TableRow { value: "IN", display_name: "Intolerance", definition: "", comment_usage_note: "", status: "" },
        "SE" => V2TableRow { value: "SE", display_name: "Side Effect", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0437: V2Table = V2Table {
    number: 437,
    metadata: &super::metadata::TABLE_0437_METADATA,
    rows: phf_map! {
        "B" => V2TableRow { value: "B", display_name: "Bracelet", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Necklace", definition: "", comment_usage_note: "", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Wallet Card", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0438: V2Table = V2Table {
    number: 438,
    metadata: &super::metadata::TABLE_0438_METADATA,
    rows: phf_map! {
        "U" => V2TableRow { value: "U", display_name: "Unconfirmed", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Pending", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Suspect", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Confirmed or verified", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Confirmed but inactive", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Erroneous", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Doubt raised", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0440: V2Table = V2Table {
    number: 440,
    metadata: &super::metadata::TABLE_0440_METADATA,
    rows: phf_map! {
        "AD" => V2TableRow { value: "AD", display_name: "Address", definition: "", comment_usage_note: "Replaced by XAD as of v 2.3", status: "" },
        "AUI" => V2TableRow { value: "AUI", display_name: "Authorization information", definition: "", comment_usage_note: "Replaces the CM data type used in sections 6.5.6.14 IN1-14, as of v 2.5.", status: "" },
        "CCD" => V2TableRow { value: "CCD", display_name: "Charge code and date", definition: "", comment_usage_note: "Replaces the CM data type used in section 4.5.2.1 BLG-1, as of v 2.5.", status: "" },
        "CCP" => V2TableRow { value: "CCP", display_name: "Channel calibration parameters", definition: "", comment_usage_note: "Replaces the CM data type used in 7.14.1.5 OBX-5.3 where OBX-5Observation value (*) is data type CD as of v 2.5.", status: "" },
        "CD" => V2TableRow { value: "CD", display_name: "Channel definition", definition: "", comment_usage_note: "For waveform data only;.", status: "" },
        "CE" => V2TableRow { value: "CE", display_name: "Coded element", definition: "", comment_usage_note: "WITHDRA WN", status: "" },
        "CF" => V2TableRow { value: "CF", display_name: "Coded element with formatted values", definition: "", comment_usage_note: "", status: "" },
        "CK" => V2TableRow { value: "CK", display_name: "Composite ID with check digit", definition: "", comment_usage_note: "WITHDRA WN", status: "" },
        "CM" => V2TableRow { value: "CM", display_name: "Composite", definition: "", comment_usage_note: "WITHDRAWN Replaced by numerous new unambiguous data types in v 2.5", status: "" },
        "CN" => V2TableRow { value: "CN", display_name: "Composite ID number and name", definition: "", comment_usage_note: "WITHDRA WN. Replaced by XCN as of v 2.3", status: "" },
        "CNE" => V2TableRow { value: "CNE", display_name: "Coded with no exceptions", definition: "", comment_usage_note: "", status: "" },
        "CNN" => V2TableRow { value: "CNN", display_name: "Composite ID number and name simplified", definition: "", comment_usage_note: "Restores the original data type CN as was initially implementable in the CM used in sections 4.5.3.32 and 7.4.1.32- (OBR- 4.5.3.33 and 7.4.1.33 - (OBR- 33) 4.5.3.34 and 7.4.1.34 - (OBR- 34) 4. 7.4.1.35 - (OBR- 35). Components 7 and 8, however, have been promoted to data type IS to be consistent with current practice without violating backward compatibility.", status: "32), 5.3.35 and" },
        "CP" => V2TableRow { value: "CP", display_name: "Composite price", definition: "", comment_usage_note: ".", status: "" },
        "CQ" => V2TableRow { value: "CQ", display_name: "Composite quantity with units", definition: "", comment_usage_note: "CQ cannot be legally expressed when embedded within another data type. Its use is constrained to a segment field.", status: "" },
        "CSU" => V2TableRow { value: "CSU", display_name: "Channel sensitivity and units", definition: "", comment_usage_note: "Replaces the CM data type used in 7.14.1.5 OBX-5.3 where OBX-5Observation value (*) is data type CD as of v 2.5.", status: "" },
        "CWE" => V2TableRow { value: "CWE", display_name: "Coded with exceptions", definition: "", comment_usage_note: "", status: "" },
        "CX" => V2TableRow { value: "CX", display_name: "Extended composite ID with check digit", definition: "", comment_usage_note: "", status: "" },
        "DDI" => V2TableRow { value: "DDI", display_name: "Daily deductible information", definition: "Replaces th 6.5.7.30", comment_usage_note: "e CM data type used in section IN2-30, as of v 2.5.", status: "" },
        "DIN" => V2TableRow { value: "DIN", display_name: "Date and institution name", definition: "Replaces th 15.4.6.12 S of v 2.5.", comment_usage_note: "e CM data type used in sections TF-12 and 15.4.6.14 STF- 13, as", status: "" },
        "DLD" => V2TableRow { value: "DLD", display_name: "Discharge to location and date", definition: "Replaces th 8.8.4.9 - O", comment_usage_note: "e CM data type used in section M2-9, as of v 2.5", status: "" },
        "DLN" => V2TableRow { value: "DLN", display_name: "Driver's license number", definition: "", comment_usage_note: "", status: "" },
        "DLT" => V2TableRow { value: "DLT", display_name: "Delta", definition: "", comment_usage_note: "", status: "" },
        "DR" => V2TableRow { value: "DR", display_name: "Date/time range", definition: "", comment_usage_note: "", status: "" },
        "DT" => V2TableRow { value: "DT", display_name: "Date", definition: "", comment_usage_note: "", status: "" },
        "DTM" => V2TableRow { value: "DTM", display_name: "Date/time", definition: "", comment_usage_note: "", status: "" },
        "DTN" => V2TableRow { value: "DTN", display_name: "Day type and number", definition: "Replaces th 6.5.8.11", comment_usage_note: "e CM data type used in section IN3-11, as of v 2.5.", status: "" },
        "ED" => V2TableRow { value: "ED", display_name: "Encapsulated data", definition: "Supports AS data.", comment_usage_note: "CII MIME-encoding of binary", status: "" },
        "EI" => V2TableRow { value: "EI", display_name: "Entity identifier", definition: "", comment_usage_note: "", status: "" },
        "EIP" => V2TableRow { value: "EIP", display_name: "Entity identifier pair", definition: "Replaces th 4.5.1.8 - - OBR-29, a", comment_usage_note: "e CM data type used in sections ORC- 8, 4.5.3.29 - OBR- s of v 2.5.", status: "29, 7.3.1.29" },
        "ELD" => V2TableRow { value: "ELD", display_name: "Error location and description", definition: "WITHDRA WN", comment_usage_note: "", status: "" },
        "ERL" => V2TableRow { value: "ERL", display_name: "Error location", definition: "", comment_usage_note: "", status: "" },
        "FC" => V2TableRow { value: "FC", display_name: "Financial class", definition: "", comment_usage_note: "", status: "" },
        "FN" => V2TableRow { value: "FN", display_name: "Family name", definition: "Appears ONL", comment_usage_note: "Y in the PPN, XCN, and XPN.", status: "" },
        "FT" => V2TableRow { value: "FT", display_name: "Formatted text", definition: "", comment_usage_note: "", status: "" },
        "GTS" => V2TableRow { value: "GTS", display_name: "General timing specification", definition: "", comment_usage_note: "", status: "" },
        "HD" => V2TableRow { value: "HD", display_name: "Hierarchic designator", definition: "", comment_usage_note: "", status: "" },
        "ICD" => V2TableRow { value: "ICD", display_name: "Insurance certification definition", definition: "Replaces th 6.5.8.20", comment_usage_note: "e CM data type used in section IN3-20, as of v 2.5.", status: "" },
        "ID" => V2TableRow { value: "ID", display_name: "Coded values for HL7 tables", definition: "", comment_usage_note: "", status: "" },
        "IS" => V2TableRow { value: "IS", display_name: "Coded value for user- defined tables", definition: "", comment_usage_note: "", status: "" },
        "JCC" => V2TableRow { value: "JCC", display_name: "Job code/class", definition: "", comment_usage_note: "", status: "" },
        "LA1" => V2TableRow { value: "LA1", display_name: "Location with address variation 1", definition: "Datatype ha Standard; t were deprec compatibili they have b datapyt is compatible", comment_usage_note: "s been withdrawn from the B he segments RXO- 8 and RXE-8 ated, and kept for backward ty only as of v2.5, and as of v2.9 een witdrawn so the code for the here marked as backwars- use (for historical records only)", status: "" },
        "LA2" => V2TableRow { value: "LA2", display_name: "Location with address variation 2", definition: "Datatype ha Standard; t and RXA-11 backward co as of v2.9 code for th backwards-c records onl", comment_usage_note: "s been withdrawn from the B he segments RXD- 13, RXG-11 were deprecated, and kept for mpatibility only as of v2.5, and they have been withdrawn so the e datatype is here marked as ompatible use (for historical y)", status: "" },
        "MA" => V2TableRow { value: "MA", display_name: "Multiplexed array", definition: "For wavefor", comment_usage_note: "m data only", status: "" },
        "MO" => V2TableRow { value: "MO", display_name: "Money", definition: "", comment_usage_note: "", status: "" },
        "MOC" => V2TableRow { value: "MOC", display_name: "Money and charge code", definition: "Replaces th 4.5.3.23 OB v 2.5.", comment_usage_note: "e CM data type used in sections R-23 and 7.4.1.23- OBR- 23 as of", status: "" },
        "MOP" => V2TableRow { value: "MOP", display_name: "Money or percentage", definition: "Replaces th 6.5.8.5 IN restricted", comment_usage_note: "e CM data type used in section 3- 5, as of v 2.5. This data type is to this field.", status: "" },
        "MSG" => V2TableRow { value: "MSG", display_name: "Message type", definition: "Replaces th MSH-9 as of", comment_usage_note: "e CM data type used in 2.16.9.9 v 2.5.", status: "" },
        "NA" => V2TableRow { value: "NA", display_name: "Numeric array", definition: "For wavefor", comment_usage_note: "m data only", status: "" },
        "NDL" => V2TableRow { value: "NDL", display_name: "Name with date and location", definition: "Replaces th 4.5.3.32 an and 7.4.1.3 7.4.1.34 - OBR-35) as", comment_usage_note: "e CM data type used in sections d 7.4.1.32-( OBR- 32) , 4.5.3.33 3 - ( OBR- 33) 4.5.3.34 and ( OBR- 34) 4.5.3.35 and 7.4.1.35 - of v 2.5.", status: "(" },
        "NM" => V2TableRow { value: "NM", display_name: "Numeric", definition: "", comment_usage_note: "", status: "" },
        "NR" => V2TableRow { value: "NR", display_name: "Numeric range", definition: "Replaces th 8.8.4.6.1- 8.8.4.6.4-", comment_usage_note: "e CM data type used in sections OM2-6.1, 8.8.4.6.3- OM2-6.3and OM2-6.4, as of v 2.5.", status: "" },
        "OCD" => V2TableRow { value: "OCD", display_name: "Occurrence code and date", definition: "Replaces th 6.5.10.10 U v 2.5.", comment_usage_note: "e CM data type used in sections B1- 16 and 6.5.11.7 UB2- 7, as of", status: "" },
        "OSD" => V2TableRow { value: "OSD", display_name: "Order sequence definition", definition: "WITHDRA WN", comment_usage_note: "", status: "" },
        "OSP" => V2TableRow { value: "OSP", display_name: "Occurrence span code and date", definition: "Replaces th 6.5.11.8 UB", comment_usage_note: "e CM data type used in section 2-8, as of v 2.5.", status: "" },
        "PIP" => V2TableRow { value: "PIP", display_name: "Practitioner institutional privileges", definition: "Replaces th PRA-7 as of", comment_usage_note: "e CM data type used in 15.4.5.7 v 2.5.", status: "" },
        "PL" => V2TableRow { value: "PL", display_name: "Person location", definition: "", comment_usage_note: "", status: "" },
        "PLN" => V2TableRow { value: "PLN", display_name: "Practitioner license or other ID number", definition: "Replaces th PRA-6, 11.6 as of v 2.5", comment_usage_note: "e CM data type used in 15.4.5.6 .3.7 PRD-7 and 11.6.4.7 CTD-7 .", status: "" },
        "PN" => V2TableRow { value: "PN", display_name: "Person name", definition: "WITHDRA WN", comment_usage_note: "", status: "" },
        "PPN" => V2TableRow { value: "PPN", display_name: "Performing person time stamp", definition: "equivalent", comment_usage_note: "of an XCN joined with a TS", status: "" },
        "PRL" => V2TableRow { value: "PRL", display_name: "Parent result link", definition: "Replaces th 4.5.3.26 - of v 2.5.", comment_usage_note: "e CM data type used in sections OBR- 26 and 7.4.1.26 - OBR-", status: "26 as" },
        "PT" => V2TableRow { value: "PT", display_name: "Processing type", definition: "", comment_usage_note: "", status: "" },
        "PTA" => V2TableRow { value: "PTA", display_name: "Policy type and amount", definition: "Replaces th 6.5.7.29", comment_usage_note: "e CM data type used in section IN2-29, as of v 2.5.", status: "" },
        "QIP" => V2TableRow { value: "QIP", display_name: "Query input parameter list", definition: "", comment_usage_note: "", status: "" },
        "QSC" => V2TableRow { value: "QSC", display_name: "Query selection criteria", definition: "", comment_usage_note: "", status: "" },
        "RCD" => V2TableRow { value: "RCD", display_name: "Row column definition", definition: "", comment_usage_note: "", status: "" },
        "RF" => V2TableRow { value: "RF", display_name: "R Reference range", definition: "Replaces th 8.8.4.6 - - OM2-8 as", comment_usage_note: "e CM data type used in sections OM2-6, 8.8.4.7 - OM2-7 and8.8.4.8 of v 2.5.", status: "" },
        "RI" => V2TableRow { value: "RI", display_name: "Repeat interval", definition: "", comment_usage_note: "", status: "" },
        "RMC" => V2TableRow { value: "RMC", display_name: "Room coverage", definition: "Replaces th 6.5.7.28 I", comment_usage_note: "e CM data type used in section N2-28, as of v 2.5.", status: "" },
        "RP" => V2TableRow { value: "RP", display_name: "Reference pointer", definition: "", comment_usage_note: "", status: "" },
        "RPT" => V2TableRow { value: "RPT", display_name: "Repeat pattern", definition: "", comment_usage_note: "", status: "" },
        "SAD" => V2TableRow { value: "SAD", display_name: "Street Address", definition: "Appears ONL", comment_usage_note: "Y in the XAD data type.", status: "" },
        "SCV" => V2TableRow { value: "SCV", display_name: "Scheduling class value pair", definition: "For schedul", comment_usage_note: "ing data only. See Chapter 10", status: "" },
        "SI" => V2TableRow { value: "SI", display_name: "Sequence ID", definition: "", comment_usage_note: "", status: "" },
        "SN" => V2TableRow { value: "SN", display_name: "Structured numeric", definition: "", comment_usage_note: "", status: "" },
        "SNM" => V2TableRow { value: "SNM", display_name: "String of telephone number digits", definition: "Definition limited to through 9. always con length: No specified", comment_usage_note: ": a string whose characters are \"+\" and the decimal digits 0 As a string, leading zeros are sidered significant. Maximum t specified for the type. May be in the context of use.", status: "" },
        "SPD" => V2TableRow { value: "SPD", display_name: "Specialty description", definition: "Replaces t PRA-5 as o", comment_usage_note: "he CM data type used in 15.4.5.5 f v 2.5.", status: "" },
        "SPS" => V2TableRow { value: "SPS", display_name: "Specimen source", definition: "WITHDRA WN", comment_usage_note: "", status: "" },
        "SRT" => V2TableRow { value: "SRT", display_name: "Sort order", definition: "", comment_usage_note: "", status: "" },
        "ST" => V2TableRow { value: "ST", display_name: "String data", definition: "", comment_usage_note: "", status: "" },
        "TM" => V2TableRow { value: "TM", display_name: "Time", definition: "", comment_usage_note: "", status: "" },
        "TN" => V2TableRow { value: "TN", display_name: "Telephone number", definition: "WITHDRA WN", comment_usage_note: "", status: "" },
        "TQ" => V2TableRow { value: "TQ", display_name: "Timing/quantity", definition: "WITHDRA WN", comment_usage_note: "", status: "" },
        "TS" => V2TableRow { value: "TS", display_name: "Time stamp", definition: "WITHDRA WN", comment_usage_note: "", status: "" },
        "TX" => V2TableRow { value: "TX", display_name: "Text data", definition: "", comment_usage_note: "", status: "" },
        "UVC" => V2TableRow { value: "UVC", display_name: "UB value code and amount", definition: "Replaces t 6.5.10.10 v 2.5.", comment_usage_note: "he CM data type used in sections UB1- 10 and 6.5.11.6 UB2- 6, as of", status: "" },
        "VH" => V2TableRow { value: "VH", display_name: "Visiting hours", definition: "", comment_usage_note: "", status: "" },
        "VID" => V2TableRow { value: "VID", display_name: "Version identifier", definition: "", comment_usage_note: "", status: "" },
        "VR" => V2TableRow { value: "VR", display_name: "Value range", definition: "Replaces t 5.10.5.3.1", comment_usage_note: "he CM data type used in 1 QRD-11 as of v 2.5.", status: "" },
        "WVI" => V2TableRow { value: "WVI", display_name: "Channel Identifier", definition: "Replaces t OBX-5.1 wh (*) is dat", comment_usage_note: "he CM data type used in 7.14.1.3.1 ere OBX-5 Observation value a type CD as of v 2.5.", status: "" },
        "WVS" => V2TableRow { value: "WVS", display_name: "Waveform source", definition: "Replaces t OBX-5.2 wh (*) is dat", comment_usage_note: "he CM data type used in 7.14.1.4 ere OBX-5 Observation value a type CD as of v 2.5.", status: "" },
        "XAD" => V2TableRow { value: "XAD", display_name: "Extended address", definition: "Replaces A", comment_usage_note: "D as of v 2.3", status: "" },
        "XCN" => V2TableRow { value: "XCN", display_name: "Extended composite ID number and name for persons", definition: "Replaces C", comment_usage_note: "N a s of v 2.3", status: "" },
        "XON" => V2TableRow { value: "XON", display_name: "Extended composite name and ID number for organizations", definition: "", comment_usage_note: "", status: "" },
        "XPN" => V2TableRow { value: "XPN", display_name: "Extended person name", definition: "Replaces P", comment_usage_note: "N as of v 2.3.", status: "" },
        "XTN" => V2TableRow { value: "XTN", display_name: "Extended telecommunications number", definition: "Replaces T", comment_usage_note: "N as of v 2.3", status: "" },
    },
};

pub static TABLE_0441: V2Table = V2Table {
    number: 441,
    metadata: &super::metadata::TABLE_0441_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "A ct iv e", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Inactive", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Inactive - Lost to follow-up (cancel contract)", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Inactive - Moved or gone elsewhere (cancel contract)", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Inactive - Permanently inactive (Do not reactivate or add new entries to the record)", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0442: V2Table = V2Table {
    number: 442,
    metadata: &super::metadata::TABLE_0442_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Diagnostic", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Therapeutic", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Primary Care", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Emergency Room Casualty", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0443: V2Table = V2Table {
    number: 443,
    metadata: &super::metadata::TABLE_0443_METADATA,
    rows: phf_map! {
        "AD" => V2TableRow { value: "AD", display_name: "Admitting", definition: "", comment_usage_note: "PV1-17 Admitting doctor", status: "" },
        "AP" => V2TableRow { value: "AP", display_name: "Administering Provider", definition: "", comment_usage_note: "RXA-10 Administering Provider", status: "" },
        "AT" => V2TableRow { value: "AT", display_name: "Attending", definition: "", comment_usage_note: "PV1-7 Attending doctor", status: "" },
        "CLP" => V2TableRow { value: "CLP", display_name: "Collecting Provider", definition: "", comment_usage_note: "OBR-10 Collector Identifier", status: "" },
        "CP" => V2TableRow { value: "CP", display_name: "Consulting Provider", definition: "", comment_usage_note: "", status: "" },
        "DP" => V2TableRow { value: "DP", display_name: "Dispensing Provider", definition: "", comment_usage_note: "RXD-10 Dispensing Provider", status: "" },
        "EP" => V2TableRow { value: "EP", display_name: "Entering Provider (probably not the same as transcriptionist?)", definition: "", comment_usage_note: "ORC-10 Entered By", status: "" },
        "FHCP" => V2TableRow { value: "FHCP", display_name: "Family Health Care Professional", definition: "", comment_usage_note: "", status: "" },
        "IP" => V2TableRow { value: "IP", display_name: "Initiating Provider (as in action by)", definition: "", comment_usage_note: "ORC-19 Action By", status: "" },
        "MDIR" => V2TableRow { value: "MDIR", display_name: "Medical Director", definition: "", comment_usage_note: "OBX-25 Performing Organization Medical Director", status: "" },
        "OP" => V2TableRow { value: "OP", display_name: "Ordering Provider", definition: "", comment_usage_note: "ORC-12 Ordering Provider, OBR-16 Ordering Provider, RXO- 14 Ordering Provider's DEA Number, RXE-13 Ordering Provider's DEA Number, ORC-24 Ordering Provider Address", status: "" },
        "PH" => V2TableRow { value: "PH", display_name: "Pharmacist (not sure how to dissect Pharmacist/Treatment Supplier's Verifier ID)", definition: "", comment_usage_note: "RXE-14 Pharmacist/Treatment Supplier's Verifier ID", status: "" },
        "PP" => V2TableRow { value: "PP", display_name: "Primary Care Provider", definition: "", comment_usage_note: "", status: "" },
        "RO" => V2TableRow { value: "RO", display_name: "Responsible Observer", definition: "", comment_usage_note: "OBX-16 Responsible Observer", status: "" },
        "RP" => V2TableRow { value: "RP", display_name: "Referring Provider", definition: "", comment_usage_note: "PV1-8 Referring doctor", status: "" },
        "RT" => V2TableRow { value: "RT", display_name: "Referred to Provider", definition: "", comment_usage_note: "", status: "" },
        "TR" => V2TableRow { value: "TR", display_name: "Transcriptionist", definition: "", comment_usage_note: "", status: "" },
        "PI" => V2TableRow { value: "PI", display_name: "Primary Interpreter", definition: "", comment_usage_note: "", status: "" },
        "AI" => V2TableRow { value: "AI", display_name: "Assistant/Alternate Interpreter", definition: "", comment_usage_note: "", status: "" },
        "TN" => V2TableRow { value: "TN", display_name: "Technician", definition: "", comment_usage_note: "", status: "" },
        "VP" => V2TableRow { value: "VP", display_name: "Verifying Provider", definition: "", comment_usage_note: "ORC-11 Verified By", status: "" },
        "VPS" => V2TableRow { value: "VPS", display_name: "Verifying Pharmaceutical Supplier (not sure how to dissect Pharmacist/Treatment Supplier's Verifier ID)", definition: "", comment_usage_note: "RXE-14 Pharmacist/Treatment Supplier's Verifier ID", status: "" },
        "VTS" => V2TableRow { value: "VTS", display_name: "Verifying Treatment Supplier (not sure how to dissect Pharmacist/Treatment Supplier's Verifier ID)", definition: "", comment_usage_note: "RXE-14 Pharmacist/Treatment Supplier's Verifier ID", status: "" },
    },
};

pub static TABLE_0444: V2Table = V2Table {
    number: 444,
    metadata: &super::metadata::TABLE_0444_METADATA,
    rows: phf_map! {
        "G" => V2TableRow { value: "G", display_name: "Prefix Given Middle Family Suffix", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Prefix Family Middle Given Suffix", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0445: V2Table = V2Table {
    number: 445,
    metadata: &super::metadata::TABLE_0445_METADATA,
    rows: phf_map! {
        "US" => V2TableRow { value: "US", display_name: "Unknown/Default Social Security Number", definition: "", comment_usage_note: "", status: "" },
        "UD" => V2TableRow { value: "UD", display_name: "Unknown/Default Date of Birth", definition: "", comment_usage_note: "", status: "" },
        "UA" => V2TableRow { value: "UA", display_name: "Unknown/Default Address", definition: "", comment_usage_note: "", status: "" },
        "AL" => V2TableRow { value: "AL", display_name: "Patient/Person Name is an Alias", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0450: V2Table = V2Table {
    number: 450,
    metadata: &super::metadata::TABLE_0450_METADATA,
    rows: phf_map! {
        "LOG" => V2TableRow { value: "LOG", display_name: "Log Event", definition: "", comment_usage_note: "", status: "" },
        "SER" => V2TableRow { value: "SER", display_name: "Service Event", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0451: V2Table = V2Table {
    number: 451,
    metadata: &super::metadata::TABLE_0451_METADATA,
    rows: phf_map! {
        "ALL" => V2TableRow { value: "ALL", display_name: "Used for query of all inventory items", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0457: V2Table = V2Table {
    number: 457,
    metadata: &super::metadata::TABLE_0457_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "No edits present on claim", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Only edits present are for line item denial or rejection", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Multiple-day claim with one or more days denied or rejected", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Claim denied, rejected, suspended or returned to provider with only post payment edits", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Claim denied, rejected, suspended or returned to provider with only pre payment edits", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0459: V2Table = V2Table {
    number: 459,
    metadata: &super::metadata::TABLE_0459_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "OCE line item denial or rejection is not ignored", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "OCE line item denial or rejection is ignored", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "External line item denial. Line item is denied even if no OCE edits", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "External line item rejection. Line item is rejected even if no OCE edits", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0460: V2Table = V2Table {
    number: 460,
    metadata: &super::metadata::TABLE_0460_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Line item not denied or rejected", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Line item denied or rejected", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Line item is on a multiple-day claim. The line item is not denied or rejected, but occurs on a day that has been denied or rejected.", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0465: V2Table = V2Table {
    number: 465,
    metadata: &super::metadata::TABLE_0465_METADATA,
    rows: phf_map! {
        "I" => V2TableRow { value: "I", display_name: "Ideographic (i.e., Kanji)", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Alphabetic (i.e., Default or some single-byte)", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Phonetic (i.e., ASCII, Katakana, Hiragana, etc.)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0466: V2Table = V2Table {
    number: 466,
    metadata: &super::metadata::TABLE_0466_METADATA,
    rows: phf_map! {
        "031" => V2TableRow { value: "031", display_name: "Dental procedures", definition: "", comment_usage_note: "", status: "" },
        "163" => V2TableRow { value: "163", display_name: "Excision/biopsy", definition: "", comment_usage_note: "", status: "" },
        "181" => V2TableRow { value: "181", display_name: "Level 1 skin repair.", definition: "", comment_usage_note: "", status: "" },
        "..." => V2TableRow { value: "...", display_name: "No suggested values defined", definition: "", comment_usage_note: "", status: "D" },
    },
};

pub static TABLE_0468: V2Table = V2Table {
    number: 468,
    metadata: &super::metadata::TABLE_0468_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "No payment adjustment", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Designated current drug or biological payment adjustment applies to APC (status indicator G)", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Designated new device payment adjustment applies to APC (status indicator H)", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Designated new drug or new biological payment adjustment applies to APC (status indicator J)", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Deductible not applicable (specific list of HCPCS codes)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0469: V2Table = V2Table {
    number: 469,
    metadata: &super::metadata::TABLE_0469_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Not packaged", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Packaged service (status indicator N, or no HCPCS code and certain revenue codes)", definition: "Note", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Packaged as part of partial hospitalization per diem or daily mental health service per diem", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0470: V2Table = V2Table {
    number: 470,
    metadata: &super::metadata::TABLE_0470_METADATA,
    rows: phf_map! {
        "OPPS" => V2TableRow { value: "OPPS", display_name: "Outpatient Prospective Payment System", definition: "", comment_usage_note: "", status: "" },
        "Pckg" => V2TableRow { value: "Pckg", display_name: "Packaged APC", definition: "", comment_usage_note: "", status: "" },
        "Lab" => V2TableRow { value: "Lab", display_name: "Clinical Laboratory APC", definition: "", comment_usage_note: "", status: "" },
        "Thrpy" => V2TableRow { value: "Thrpy", display_name: "Therapy APC", definition: "", comment_usage_note: "", status: "" },
        "DME" => V2TableRow { value: "DME", display_name: "Durable Medical Equipment", definition: "", comment_usage_note: "", status: "" },
        "EPO" => V2TableRow { value: "EPO", display_name: "Epotein", definition: "", comment_usage_note: "", status: "" },
        "Mamm" => V2TableRow { value: "Mamm", display_name: "Screening Mammography APC", definition: "", comment_usage_note: "", status: "" },
        "PartH" => V2TableRow { value: "PartH", display_name: "Partial Hospitalization APC", definition: "", comment_usage_note: "", status: "" },
        "Crnl" => V2TableRow { value: "Crnl", display_name: "Corneal Tissue APC", definition: "", comment_usage_note: "", status: "" },
        "NoPay" => V2TableRow { value: "NoPay", display_name: "This APC i s not paid", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0472: V2Table = V2Table {
    number: 472,
    metadata: &super::metadata::TABLE_0472_METADATA,
    rows: phf_map! {
        "S" => V2TableRow { value: "S", display_name: "Synchronous", definition: "", comment_usage_note: "Do the next specification after this one (unless otherwise constrained by the following fields: TQ1- 7-start date/time and TQ1-8-end date/time). An \"S\" specification implies that the second timing sequence follows the first, e.g., when a service request is written to measure blood pressure Q15 minutes for the 1st hour, then every 2 hours for the next day.", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Asynchronous", definition: "", comment_usage_note: "Do the next specification in parallel with this one (unless otherwise constrained by the following fields: TQ1-7-start date/time and TQ1-8-end date/time). The conjunction of \"A\" specifies two parallel instructions, as are sometimes used in medication, e.g., prednisone given at 1 tab on Monday, Wednesday, Friday, and at 1/2 tab on Tuesday, Thursday, Saturday, Sunday.", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Actuation Time", definition: "", comment_usage_note: "It will be followed by a completion time for the service. This code allows one to distinguish between the time and priority at which a service should be actuated (e.g., blood should be drawn) and the time and priority at which a service should be completed (e.g., results should be reported).", status: "" },
    },
};

pub static TABLE_0473: V2Table = V2Table {
    number: 473,
    metadata: &super::metadata::TABLE_0473_METADATA,
    rows: phf_map! {
        "G" => V2TableRow { value: "G", display_name: "This observation/service is on the formulary, and has guidelines", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "This observation/service is not on the formulary", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "This observation/service is on the formulary, but is restricted", definition: "", comment_usage_note: "", status: "" },
        "Y" => V2TableRow { value: "Y", display_name: "This observation/service is on the formulary", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0474: V2Table = V2Table {
    number: 474,
    metadata: &super::metadata::TABLE_0474_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Department", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Facility", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Subdepartment", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Subdivision", definition: "", comment_usage_note: "", status: "" },
        "V" => V2TableRow { value: "V", display_name: "Division", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0475: V2Table = V2Table {
    number: 475,
    metadata: &super::metadata::TABLE_0475_METADATA,
    rows: phf_map! {
        "01" => V2TableRow { value: "01", display_name: "Allergy", definition: "", comment_usage_note: "", status: "" },
        "02" => V2TableRow { value: "02", display_name: "Intolerance", definition: "", comment_usage_note: "", status: "" },
        "03" => V2TableRow { value: "03", display_name: "Treatment Failure", definition: "", comment_usage_note: "", status: "" },
        "04" => V2TableRow { value: "04", display_name: "Patient Request", definition: "", comment_usage_note: "", status: "" },
        "05" => V2TableRow { value: "05", display_name: "No Exception", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0477: V2Table = V2Table {
    number: 477,
    metadata: &super::metadata::TABLE_0477_METADATA,
    rows: phf_map! {
        "I" => V2TableRow { value: "I", display_name: "Schedule I", definition: "", comment_usage_note: "Includes drugs that have a high potential for abuse, no currently accepted medical use in the United States and a lack of accepted safety for use under medical supervision.", status: "" },
        "II" => V2TableRow { value: "II", display_name: "Schedule II", definition: "", comment_usage_note: "Includes drugs having currently accepted medical use in the United States and a high abuse potential, with severe psychological or physical dependence liability.", status: "" },
        "III" => V2TableRow { value: "III", display_name: "Schedule III", definition: "", comment_usage_note: "Includes drugs having an abuse potential less than that of drugs listed in Schedules I and II. All CS III drugs have a currently accepted medical use in the United States.", status: "" },
        "IV" => V2TableRow { value: "IV", display_name: "Schedule IV", definition: "", comment_usage_note: "Includes drugs having a lesser potential for abuse than those listed in Schedule III. CS IV drugs have a currently accepted medical use in the United States.", status: "" },
        "V" => V2TableRow { value: "V", display_name: "Schedule V", definition: "", comment_usage_note: "Includes drugs having low abuse potential and limited physical or psychological dependence relative to those listed in IV and have an accepted medical use in the United States.", status: "" },
        "VI" => V2TableRow { value: "VI", display_name: "Schedule VI", definition: "", comment_usage_note: "State defined", status: "" },
    },
};

pub static TABLE_0478: V2Table = V2Table {
    number: 478,
    metadata: &super::metadata::TABLE_0478_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Pharmaceutical substance is in the formulary", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Pharmaceutical substance is NOT in the formulary", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Pharmaceutical substance is in the formulary, but restrictions apply", definition: "", comment_usage_note: "", status: "" },
        "G" => V2TableRow { value: "G", display_name: "Pharmaceutical substance is in the formulary, but guidelines apply", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0480: V2Table = V2Table {
    number: 480,
    metadata: &super::metadata::TABLE_0480_METADATA,
    rows: phf_map! {
        "M" => V2TableRow { value: "M", display_name: "Medication", definition: "", comment_usage_note: "Default value. Includes, but is not limited to, tables, capsules, powders, puffs, and other non- injected/non- infused products.", status: "" },
        "S" => V2TableRow { value: "S", display_name: "IV Large Volume Solutions", definition: "", comment_usage_note: "Includes, but is not limited to, TPNs, admixtures, solutions and drips.", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other solution as medication orders", definition: "", comment_usage_note: "Includes, but is not limited to, piggybacks and syringes", status: "" },
    },
};

pub static TABLE_0482: V2Table = V2Table {
    number: 482,
    metadata: &super::metadata::TABLE_0482_METADATA,
    rows: phf_map! {
        "I" => V2TableRow { value: "I", display_name: "Inpatient Order", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Outpatient Order", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0483: V2Table = V2Table {
    number: 483,
    metadata: &super::metadata::TABLE_0483_METADATA,
    rows: phf_map! {
        "EL" => V2TableRow { value: "EL", display_name: "Electronic", definition: "", comment_usage_note: "", status: "" },
        "EM" => V2TableRow { value: "EM", display_name: "E-mail", definition: "", comment_usage_note: "", status: "" },
        "FX" => V2TableRow { value: "FX", display_name: "Fax", definition: "", comment_usage_note: "", status: "" },
        "IP" => V2TableRow { value: "IP", display_name: "In Person", definition: "", comment_usage_note: "", status: "" },
        "MA" => V2TableRow { value: "MA", display_name: "Mail", definition: "", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Paper", definition: "", comment_usage_note: "", status: "" },
        "PH" => V2TableRow { value: "PH", display_name: "Phone", definition: "", comment_usage_note: "", status: "" },
        "RE" => V2TableRow { value: "RE", display_name: "Reflexive (Automated system)", definition: "", comment_usage_note: "", status: "" },
        "VC" => V2TableRow { value: "VC", display_name: "Video-conference", definition: "", comment_usage_note: "", status: "" },
        "VO" => V2TableRow { value: "VO", display_name: "Voice", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0484: V2Table = V2Table {
    number: 484,
    metadata: &super::metadata::TABLE_0484_METADATA,
    rows: phf_map! {
        "B" => V2TableRow { value: "B", display_name: "Trial Quantity Balance", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Compassionate Fill", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "New/Renew - Full Fill", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "New/Renew - Part Fill", definition: "", comment_usage_note: "", status: "" },
        "Q" => V2TableRow { value: "Q", display_name: "Refill - Part Fill", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Refill - Full Fill", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Manufacturer Sample", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Trial Quantity", definition: "", comment_usage_note: "", status: "" },
        "Z" => V2TableRow { value: "Z", display_name: "Non-Prescription Fill", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0485: V2Table = V2Table {
    number: 485,
    metadata: &super::metadata::TABLE_0485_METADATA,
    rows: phf_map! {
        "S" => V2TableRow { value: "S", display_name: "Stat", definition: "", comment_usage_note: "With highest priority", status: "" },
        "A" => V2TableRow { value: "A", display_name: "ASAP", definition: "", comment_usage_note: "Fill after S orders", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Routine", definition: "", comment_usage_note: "Default", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Preop", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Callback", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Timing critical", definition: "", comment_usage_note: "A request implying that it is critical to come as close as possible to the requested time, e.g., for a trough anti-microbial level.", status: "" },
        "TS<integer>" => V2TableRow { value: "TS<integer>", display_name: "Timing critical within <integer> seconds.", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "TM<integer" => V2TableRow { value: "TM<integer", display_name: "Timing critical", definition: "", comment_usage_note: "This is not a real code, but guidelines how to", status: "D" },
        ">" => V2TableRow { value: ">", display_name: "within <integer> minutes.", definition: "", comment_usage_note: "construct the codes.", status: "" },
        "TH<integer>" => V2TableRow { value: "TH<integer>", display_name: "Timing critical within <integer> hours.", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "TD<integer>" => V2TableRow { value: "TD<integer>", display_name: "Timing critical within <integer> days.", definition: "", comment_usage_note: "This is not a real code, but guidelines how to construct the codes.", status: "D" },
        "TW<integer" => V2TableRow { value: "TW<integer", display_name: "Timing critical", definition: "T", comment_usage_note: "his is not a real code, but guidelines how to D", status: "" },
        "TL<integer>" => V2TableRow { value: "TL<integer>", display_name: "Timing critical within <integer> months.", definition: "T c", comment_usage_note: "his is not a real code, but guidelines how to D onstruct the codes.", status: "" },
        "PRN" => V2TableRow { value: "PRN", display_name: "As needed", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0487: V2Table = V2Table {
    number: 487,
    metadata: &super::metadata::TABLE_0487_METADATA,
    rows: phf_map! {
        "ABS" => V2TableRow { value: "ABS", display_name: "Abscess", definition: "", comment_usage_note: "", status: "" },
        "ACNE" => V2TableRow { value: "ACNE", display_name: "Tissue, Acne", definition: "", comment_usage_note: "Tissue", status: "" },
        "ACNFLD" => V2TableRow { value: "ACNFLD", display_name: "Fluid, Acne", definition: "", comment_usage_note: "Fluid", status: "" },
        "AIRS" => V2TableRow { value: "AIRS", display_name: "Air Sample", definition: "", comment_usage_note: "Environment", status: "" },
        "ALL" => V2TableRow { value: "ALL", display_name: "Allograft", definition: "", comment_usage_note: "Tissue", status: "" },
        "AMN" => V2TableRow { value: "AMN", display_name: "Amniotic fluid", definition: "", comment_usage_note: "", status: "" },
        "AMP" => V2TableRow { value: "AMP", display_name: "Amputation", definition: "", comment_usage_note: "Tissue", status: "" },
        "ANGI" => V2TableRow { value: "ANGI", display_name: "Catheter Tip, Angio", definition: "", comment_usage_note: "Device", status: "" },
        "ARTC" => V2TableRow { value: "ARTC", display_name: "Catheter Tip, Arterial", definition: "", comment_usage_note: "Device", status: "" },
        "ASERU" => V2TableRow { value: "ASERU", display_name: "Serum, Acute", definition: "", comment_usage_note: "Blood", status: "" },
        "ASP" => V2TableRow { value: "ASP", display_name: "Aspirate", definition: "", comment_usage_note: "", status: "" },
        "ATTE" => V2TableRow { value: "ATTE", display_name: "Environment, Attest", definition: "", comment_usage_note: "Environment", status: "D" },
        "AUTOA" => V2TableRow { value: "AUTOA", display_name: "Environmental, Autoclave Ampule", definition: "", comment_usage_note: "Environment", status: "" },
        "AUTOC" => V2TableRow { value: "AUTOC", display_name: "Environmental, Autoclave Capsule", definition: "", comment_usage_note: "Environment", status: "D" },
        "AUTP" => V2TableRow { value: "AUTP", display_name: "Autopsy", definition: "", comment_usage_note: "Tissue", status: "" },
        "BBL" => V2TableRow { value: "BBL", display_name: "Blood bag", definition: "", comment_usage_note: "Blood", status: "" },
        "BCY" => V2TableRow { value: "BCY", display_name: "ST Cyst, Baker 's", definition: "", comment_usage_note: "Condition", status: "" },
        "BDY" => V2TableRow { value: "BDY", display_name: "Whole body", definition: "", comment_usage_note: "Body submitted for autopsy / carcass submitted", status: "" },
        "BIFL" => V2TableRow { value: "BIFL", display_name: "Bile Fluid", definition: "", comment_usage_note: "", status: "" },
        "BITE" => V2TableRow { value: "BITE", display_name: "Bite", definition: "", comment_usage_note: "Conditions", status: "" },
        "BLD" => V2TableRow { value: "BLD", display_name: "Whole blood", definition: "", comment_usage_note: "", status: "" },
        "BLDA" => V2TableRow { value: "BLDA", display_name: "Blood arterial", definition: "", comment_usage_note: "", status: "" },
        "BLDCO" => V2TableRow { value: "BLDCO", display_name: "Cord blood", definition: "", comment_usage_note: "", status: "" },
        "BLDV" => V2TableRow { value: "BLDV", display_name: "Blood venous", definition: "", comment_usage_note: "", status: "" },
        "BLEB" => V2TableRow { value: "BLEB", display_name: "Bleb", definition: "", comment_usage_note: "Condition, Fluid/Tissue", status: "" },
        "BLIST" => V2TableRow { value: "BLIST", display_name: "Blister", definition: "", comment_usage_note: "Condition, Fluid/Tissue", status: "" },
        "BOIL" => V2TableRow { value: "BOIL", display_name: "Boil", definition: "", comment_usage_note: "Condition", status: "" },
        "BON" => V2TableRow { value: "BON", display_name: "Bone", definition: "", comment_usage_note: "", status: "" },
        "BOWL" => V2TableRow { value: "BOWL", display_name: "Bowel contents", definition: "", comment_usage_note: "Condition", status: "" },
        "BPH" => V2TableRow { value: "BPH", display_name: "Basophils", definition: "", comment_usage_note: "", status: "" },
        "BPU" => V2TableRow { value: "BPU", display_name: "Blood product unit", definition: "", comment_usage_note: "Blood", status: "" },
        "BRN" => V2TableRow { value: "BRN", display_name: "Burn", definition: "", comment_usage_note: "", status: "" },
        "BRSH" => V2TableRow { value: "BRSH", display_name: "Brush", definition: "", comment_usage_note: "Product; Brush or brushing (these may be 2 separate entries as in a physical brush or a portion thereof vs the substance obtained after a surface has been brushed)", status: "" },
        "BRTH" => V2TableRow { value: "BRTH", display_name: "Breath (use EXHLD)", definition: "", comment_usage_note: "", status: "" },
        "BRU" => V2TableRow { value: "BRU", display_name: "S Brushing", definition: "Produc", comment_usage_note: "t", status: "" },
        "BUB" => V2TableRow { value: "BUB", display_name: "Bubo", definition: "Condit", comment_usage_note: "ion", status: "" },
        "BULLA" => V2TableRow { value: "BULLA", display_name: "Bulla/Bullae", definition: "Condit", comment_usage_note: "ion", status: "" },
        "BX" => V2TableRow { value: "BX", display_name: "Biopsy", definition: "Tissue", comment_usage_note: "", status: "" },
        "CALC" => V2TableRow { value: "CALC", display_name: "Calculus (=Stone)", definition: "", comment_usage_note: "", status: "" },
        "CARBU" => V2TableRow { value: "CARBU", display_name: "Carbuncle", definition: "Condit", comment_usage_note: "ion", status: "" },
        "CAT" => V2TableRow { value: "CAT", display_name: "Catheter", definition: "Device", comment_usage_note: "", status: "" },
        "CBITE" => V2TableRow { value: "CBITE", display_name: "Bite, Cat", definition: "Condit", comment_usage_note: "ions", status: "" },
        "CDM" => V2TableRow { value: "CDM", display_name: "Cardiac muscle", definition: "", comment_usage_note: "", status: "" },
        "CLIPP" => V2TableRow { value: "CLIPP", display_name: "Clippings", definition: "Condit", comment_usage_note: "ion", status: "" },
        "CNJT" => V2TableRow { value: "CNJT", display_name: "Conjunctiva", definition: "", comment_usage_note: "", status: "" },
        "CNL" => V2TableRow { value: "CNL", display_name: "Cannula", definition: "", comment_usage_note: "", status: "" },
        "COL" => V2TableRow { value: "COL", display_name: "Colostrum", definition: "", comment_usage_note: "", status: "" },
        "CONE" => V2TableRow { value: "CONE", display_name: "Biospy, Cone", definition: "Tissue", comment_usage_note: "", status: "" },
        "CS" => V2TableRow { value: "CS", display_name: "CR Scratch, Cat", definition: "Condit", comment_usage_note: "ion", status: "" },
        "CSERU" => V2TableRow { value: "CSERU", display_name: "Serum, Convalescent", definition: "Blood", comment_usage_note: "", status: "" },
        "CSF" => V2TableRow { value: "CSF", display_name: "Cerebral spinal fluid", definition: "", comment_usage_note: "", status: "" },
        "CSITE" => V2TableRow { value: "CSITE", display_name: "Catheter Insertion Site", definition: "Device", comment_usage_note: "", status: "" },
        "CSMY" => V2TableRow { value: "CSMY", display_name: "Fluid, Cystostomy Tube", definition: "Fluid", comment_usage_note: "", status: "" },
        "CST" => V2TableRow { value: "CST", display_name: "Fluid, Cyst", definition: "Fluid", comment_usage_note: "", status: "" },
        "CSV" => V2TableRow { value: "CSV", display_name: "R Blood, Cell Saver", definition: "Transf", comment_usage_note: "usion", status: "" },
        "CTP" => V2TableRow { value: "CTP", display_name: "Catheter tip", definition: "Device", comment_usage_note: "", status: "" },
        "CUR" => V2TableRow { value: "CUR", display_name: "Curretage", definition: "Uterin curett", comment_usage_note: "e specimen obtained by age = Currettings", status: "" },
        "CVM" => V2TableRow { value: "CVM", display_name: "Cervical Mucus", definition: "", comment_usage_note: "", status: "" },
        "CVPS" => V2TableRow { value: "CVPS", display_name: "Site, CVP", definition: "Site", comment_usage_note: "", status: "" },
        "CVPT" => V2TableRow { value: "CVPT", display_name: "Catheter Tip, CVP", definition: "Device", comment_usage_note: "", status: "" },
        "CYN" => V2TableRow { value: "CYN", display_name: "Nodule, Cystic", definition: "Condit", comment_usage_note: "ion", status: "" },
        "CYST" => V2TableRow { value: "CYST", display_name: "Cyst", definition: "", comment_usage_note: "", status: "" },
        "DBITE" => V2TableRow { value: "DBITE", display_name: "Bite, Dog", definition: "Condit", comment_usage_note: "ions", status: "" },
        "DCS" => V2TableRow { value: "DCS", display_name: "Sputum, Deep Cough", definition: "Condit", comment_usage_note: "ion", status: "" },
        "DEC" => V2TableRow { value: "DEC", display_name: "Ulcer, Decubitus", definition: "Condit", comment_usage_note: "ion", status: "" },
        "DEION" => V2TableRow { value: "DEION", display_name: "Environmental, Water (Deionized)", definition: "Enviro", comment_usage_note: "nment", status: "" },
        "DIA" => V2TableRow { value: "DIA", display_name: "Dialysate", definition: "Condit", comment_usage_note: "ion", status: "" },
        "DIAF" => V2TableRow { value: "DIAF", display_name: "Dialysis Fluid", definition: "Fluid", comment_usage_note: "used for dialysis - is a product", status: "" },
        "DISCHG" => V2TableRow { value: "DISCHG", display_name: "Discharge", definition: "Condit", comment_usage_note: "ion", status: "" },
        "DIV" => V2TableRow { value: "DIV", display_name: "Diverticulum", definition: "Condit", comment_usage_note: "ion", status: "" },
        "DRN" => V2TableRow { value: "DRN", display_name: "Drain", definition: "Device", comment_usage_note: "", status: "" },
        "DRNG" => V2TableRow { value: "DRNG", display_name: "Drainage, Tube", definition: "Device", comment_usage_note: "", status: "" },
        "DRNGP" => V2TableRow { value: "DRNGP", display_name: "Drainage, Penrose", definition: "Condit", comment_usage_note: "ion", status: "" },
        "DUFL" => V2TableRow { value: "DUFL", display_name: "Duodenal fluid", definition: "", comment_usage_note: "", status: "" },
        "EARW" => V2TableRow { value: "EARW", display_name: "Ear wax (cerumen)", definition: "", comment_usage_note: "", status: "" },
        "EBRU" => V2TableRow { value: "EBRU", display_name: "SH Brush, Esophageal", definition: "Produc", comment_usage_note: "t", status: "" },
        "EEYE" => V2TableRow { value: "EEYE", display_name: "Environmental, Eye Wash", definition: "Enviro", comment_usage_note: "nment", status: "" },
        "EFF" => V2TableRow { value: "EFF", display_name: "Environmental, Effluent", definition: "Enviro", comment_usage_note: "nment", status: "" },
        "EFFUS" => V2TableRow { value: "EFFUS", display_name: "Effusion", definition: "Condit", comment_usage_note: "ion", status: "" },
        "EFOD" => V2TableRow { value: "EFOD", display_name: "Environmental, Food", definition: "Enviro", comment_usage_note: "nment", status: "" },
        "EISO" => V2TableRow { value: "EISO", display_name: "Environmental, Isolette", definition: "Enviro", comment_usage_note: "nment", status: "" },
        "ELT" => V2TableRow { value: "ELT", display_name: "Electrode", definition: "", comment_usage_note: "", status: "" },
        "ENVIR" => V2TableRow { value: "ENVIR", display_name: "Environmental, Unidentified Substance", definition: "Enviro", comment_usage_note: "nment", status: "" },
        "EOS" => V2TableRow { value: "EOS", display_name: "Eosinophils", definition: "", comment_usage_note: "", status: "" },
        "EOTH" => V2TableRow { value: "EOTH", display_name: "Environmental, Other Substance", definition: "Environment but not in", comment_usage_note: "; (Substance is Known code Table)", status: "" },
        "ESOI" => V2TableRow { value: "ESOI", display_name: "Environmental, Soil", definition: "Environment", comment_usage_note: "", status: "" },
        "ESOS" => V2TableRow { value: "ESOS", display_name: "Environmental, Solution (Sterile)", definition: "Environment", comment_usage_note: "", status: "" },
        "ETA" => V2TableRow { value: "ETA", display_name: "Aspirate, Endotrach", definition: "Aspirate", comment_usage_note: "", status: "" },
        "ETTP" => V2TableRow { value: "ETTP", display_name: "Catheter Tip, Endotracheal", definition: "Device", comment_usage_note: "", status: "" },
        "ETTUB" => V2TableRow { value: "ETTUB", display_name: "Tube, Endotracheal", definition: "Device", comment_usage_note: "", status: "" },
        "EWHI" => V2TableRow { value: "EWHI", display_name: "Environmental, Whirlpool", definition: "Environment", comment_usage_note: "", status: "" },
        "EXG" => V2TableRow { value: "EXG", display_name: "Gas, exhaled (=breath)", definition: "", comment_usage_note: "", status: "" },
        "EXS" => V2TableRow { value: "EXS", display_name: "Shunt, External", definition: "Condition", comment_usage_note: "", status: "" },
        "EXUDTE" => V2TableRow { value: "EXUDTE", display_name: "Exudate", definition: "Condition", comment_usage_note: "", status: "" },
        "FAW" => V2TableRow { value: "FAW", display_name: "Environmental, Water (Well)", definition: "Environment", comment_usage_note: "", status: "" },
        "FBLOOD" => V2TableRow { value: "FBLOOD", display_name: "Blood, Fetal", definition: "Blood", comment_usage_note: "", status: "" },
        "FGA" => V2TableRow { value: "FGA", display_name: "Fluid, Abdomen", definition: "Fluid", comment_usage_note: "", status: "" },
        "FIB" => V2TableRow { value: "FIB", display_name: "Fibroblasts", definition: "", comment_usage_note: "", status: "" },
        "FIST" => V2TableRow { value: "FIST", display_name: "Fistula", definition: "", comment_usage_note: "", status: "" },
        "FLD" => V2TableRow { value: "FLD", display_name: "Fluid, Other", definition: "Fluid", comment_usage_note: "", status: "" },
        "FLT" => V2TableRow { value: "FLT", display_name: "Filter", definition: "", comment_usage_note: "", status: "" },
        "FLU" => V2TableRow { value: "FLU", display_name: "Fluid, Body unsp", definition: "", comment_usage_note: "", status: "" },
        "FLUID" => V2TableRow { value: "FLUID", display_name: "Fluid", definition: "Fluid", comment_usage_note: "", status: "" },
        "FOLEY" => V2TableRow { value: "FOLEY", display_name: "Catheter Tip, Foley", definition: "Device", comment_usage_note: "", status: "" },
        "FRS" => V2TableRow { value: "FRS", display_name: "Fluid, Respiratory", definition: "Fluid", comment_usage_note: "", status: "" },
        "FSCL" => V2TableRow { value: "FSCL", display_name: "P Scalp, Fetal", definition: "Condition", comment_usage_note: "", status: "" },
        "FUR" => V2TableRow { value: "FUR", display_name: "Furuncle", definition: "Condition", comment_usage_note: "", status: "" },
        "GAS" => V2TableRow { value: "GAS", display_name: "Gas", definition: "", comment_usage_note: "", status: "" },
        "GASA" => V2TableRow { value: "GASA", display_name: "Aspirate, Gastric", definition: "Aspirate", comment_usage_note: "", status: "" },
        "GASAN" => V2TableRow { value: "GASAN", display_name: "Antrum, Gastric", definition: "Tissue", comment_usage_note: "", status: "" },
        "GASBR" => V2TableRow { value: "GASBR", display_name: "Brushing, Gastric", definition: "Product", comment_usage_note: "", status: "" },
        "GASD" => V2TableRow { value: "GASD", display_name: "Drainage, Gastric", definition: "Condition", comment_usage_note: "", status: "" },
        "GAST" => V2TableRow { value: "GAST", display_name: "Fluid/contents, Gastric", definition: "", comment_usage_note: "", status: "" },
        "GENL" => V2TableRow { value: "GENL", display_name: "Genital lochia", definition: "", comment_usage_note: "", status: "" },
        "GENV" => V2TableRow { value: "GENV", display_name: "Genital vaginal", definition: "", comment_usage_note: "", status: "" },
        "GRAFT" => V2TableRow { value: "GRAFT", display_name: "Graft", definition: "Condition", comment_usage_note: "", status: "" },
        "GRANU" => V2TableRow { value: "GRANU", display_name: "Granuloma", definition: "Condition", comment_usage_note: "", status: "" },
        "GROSH" => V2TableRow { value: "GROSH", display_name: "Catheter, Groshong", definition: "Device", comment_usage_note: "", status: "" },
        "GSOL" => V2TableRow { value: "GSOL", display_name: "Solution, Gastrostomy", definition: "Product", comment_usage_note: "", status: "" },
        "GSPEC" => V2TableRow { value: "GSPEC", display_name: "Biopsy, Gastric", definition: "Tissue", comment_usage_note: "", status: "" },
        "GT" => V2TableRow { value: "GT", display_name: "Tube, Gastric", definition: "Device", comment_usage_note: "", status: "" },
        "GTUBE" => V2TableRow { value: "GTUBE", display_name: "Drainage Tube, Drainage (Gastrostomy)", definition: "Condition", comment_usage_note: "", status: "" },
        "HAR" => V2TableRow { value: "HAR", display_name: "Hair", definition: "", comment_usage_note: "", status: "" },
        "HBITE" => V2TableRow { value: "HBITE", display_name: "Bite, Human", definition: "Conditions", comment_usage_note: "", status: "" },
        "HBLUD" => V2TableRow { value: "HBLUD", display_name: "Blood, Autopsy", definition: "Blood", comment_usage_note: "", status: "" },
        "HEMAQ" => V2TableRow { value: "HEMAQ", display_name: "Catheter Tip, Hemaquit", definition: "Device", comment_usage_note: "", status: "" },
        "HEMO" => V2TableRow { value: "HEMO", display_name: "Catheter Tip, Hemovac", definition: "Device", comment_usage_note: "", status: "" },
        "HERNI" => V2TableRow { value: "HERNI", display_name: "Tissue, Herniated", definition: "Tissue", comment_usage_note: "", status: "" },
        "HEV" => V2TableRow { value: "HEV", display_name: "Drain, Hemovac", definition: "Device", comment_usage_note: "", status: "" },
        "HIC" => V2TableRow { value: "HIC", display_name: "Catheter, Hickman", definition: "Device", comment_usage_note: "", status: "" },
        "HYDC" => V2TableRow { value: "HYDC", display_name: "Fluid, Hydrocele", definition: "Fluid", comment_usage_note: "", status: "" },
        "IBITE" => V2TableRow { value: "IBITE", display_name: "Bite, Insect", definition: "Conditions", comment_usage_note: "", status: "" },
        "ICYST" => V2TableRow { value: "ICYST", display_name: "Cyst, Inclusion", definition: "Condition", comment_usage_note: "", status: "" },
        "IDC" => V2TableRow { value: "IDC", display_name: "Catheter Tip, Indwelling", definition: "Device", comment_usage_note: "", status: "" },
        "IHG" => V2TableRow { value: "IHG", display_name: "Gas, Inhaled", definition: "", comment_usage_note: "", status: "" },
        "ILEO" => V2TableRow { value: "ILEO", display_name: "Drainage, Ileostomy", definition: "Conditio", comment_usage_note: "n", status: "" },
        "ILLEG" => V2TableRow { value: "ILLEG", display_name: "Source of Specimen Is Illegible", definition: "", comment_usage_note: "", status: "" },
        "IMP" => V2TableRow { value: "IMP", display_name: "Implant", definition: "Device", comment_usage_note: "", status: "" },
        "INCI" => V2TableRow { value: "INCI", display_name: "Site, Incision/Surgical", definition: "Site", comment_usage_note: "", status: "" },
        "INFIL" => V2TableRow { value: "INFIL", display_name: "Infiltrate", definition: "Conditio", comment_usage_note: "n", status: "" },
        "INS" => V2TableRow { value: "INS", display_name: "Insect", definition: "Object", comment_usage_note: "", status: "" },
        "INTRD" => V2TableRow { value: "INTRD", display_name: "Catheter Tip, Introducer", definition: "Device", comment_usage_note: "", status: "" },
        "ISLT" => V2TableRow { value: "ISLT", display_name: "Isolate", definition: "", comment_usage_note: "", status: "" },
        "IT" => V2TableRow { value: "IT", display_name: "Intubation tube", definition: "", comment_usage_note: "", status: "" },
        "IUD" => V2TableRow { value: "IUD", display_name: "Intrauterine Device", definition: "Device (", comment_usage_note: "Common Usage)", status: "" },
        "IVCAT" => V2TableRow { value: "IVCAT", display_name: "Catheter Tip, IV", definition: "Device", comment_usage_note: "", status: "" },
        "IVFLD" => V2TableRow { value: "IVFLD", display_name: "Fluid, IV", definition: "Fluid", comment_usage_note: "", status: "" },
        "IVTIP" => V2TableRow { value: "IVTIP", display_name: "Tubing Tip, IV", definition: "Device", comment_usage_note: "", status: "" },
        "JEJU" => V2TableRow { value: "JEJU", display_name: "Drainage, Jejunal", definition: "Conditio", comment_usage_note: "n", status: "" },
        "JNTFLD" => V2TableRow { value: "JNTFLD", display_name: "Fluid, Joint", definition: "Fluid", comment_usage_note: "", status: "" },
        "JP" => V2TableRow { value: "JP", display_name: "Drainage, Jackson Pratt", definition: "Conditio", comment_usage_note: "n", status: "" },
        "KELOI" => V2TableRow { value: "KELOI", display_name: "Lavage", definition: "Product", comment_usage_note: "", status: "" },
        "KIDFLD" => V2TableRow { value: "KIDFLD", display_name: "Fluid, Kidney", definition: "Fluid", comment_usage_note: "", status: "" },
        "LAVG" => V2TableRow { value: "LAVG", display_name: "Lavage, Bronhial", definition: "Product", comment_usage_note: "", status: "" },
        "LAVGG" => V2TableRow { value: "LAVGG", display_name: "Lavage, Gastric", definition: "Product", comment_usage_note: "", status: "" },
        "LAVGP" => V2TableRow { value: "LAVGP", display_name: "Lavage, Peritoneal", definition: "Product", comment_usage_note: "", status: "" },
        "LAVPG" => V2TableRow { value: "LAVPG", display_name: "Lavage, Pre-Bronch", definition: "Product", comment_usage_note: "", status: "" },
        "LENS1" => V2TableRow { value: "LENS1", display_name: "Contact Lens", definition: "Device", comment_usage_note: "", status: "" },
        "LENS2" => V2TableRow { value: "LENS2", display_name: "Contact Lens Case", definition: "Device", comment_usage_note: "", status: "" },
        "LESN" => V2TableRow { value: "LESN", display_name: "Lesion", definition: "Conditio", comment_usage_note: "n", status: "" },
        "LIQ" => V2TableRow { value: "LIQ", display_name: "Liquid, Unspecified", definition: "", comment_usage_note: "", status: "" },
        "LIQO" => V2TableRow { value: "LIQO", display_name: "Liquid, Other", definition: "", comment_usage_note: "", status: "" },
        "LNA" => V2TableRow { value: "LNA", display_name: "Line arterial", definition: "Arterial line", comment_usage_note: "blood collected via arterial", status: "" },
        "LNV" => V2TableRow { value: "LNV", display_name: "Line venous", definition: "Venous b line", comment_usage_note: "lood collected via venous", status: "" },
        "LSAC" => V2TableRow { value: "LSAC", display_name: "Fluid, Lumbar Sac", definition: "Fluid", comment_usage_note: "", status: "" },
        "LYM" => V2TableRow { value: "LYM", display_name: "Lymphocytes", definition: "", comment_usage_note: "", status: "" },
        "MAC" => V2TableRow { value: "MAC", display_name: "Macrophages", definition: "", comment_usage_note: "", status: "" },
        "MAHUR" => V2TableRow { value: "MAHUR", display_name: "Catheter Tip, Makurkour", definition: "Device", comment_usage_note: "", status: "" },
        "MAR" => V2TableRow { value: "MAR", display_name: "Marrow", definition: "Bone mar", comment_usage_note: "row", status: "" },
        "MASS" => V2TableRow { value: "MASS", display_name: "Mass", definition: "Conditio", comment_usage_note: "n", status: "" },
        "MBLD" => V2TableRow { value: "MBLD", display_name: "Blood, Menstrual", definition: "Blood", comment_usage_note: "", status: "" },
        "MEC" => V2TableRow { value: "MEC", display_name: "Meconium", definition: "", comment_usage_note: "", status: "" },
        "MILK" => V2TableRow { value: "MILK", display_name: "Breast milk", definition: "Mother's", comment_usage_note: "milk specimen", status: "" },
        "MLK" => V2TableRow { value: "MLK", display_name: "Milk", definition: "Food spe", comment_usage_note: "cimen", status: "" },
        "MUCOS" => V2TableRow { value: "MUCOS", display_name: "Mucosa", definition: "Conditio", comment_usage_note: "n", status: "" },
        "MUCUS" => V2TableRow { value: "MUCUS", display_name: "Mucus", definition: "Conditio", comment_usage_note: "n", status: "" },
        "NAIL" => V2TableRow { value: "NAIL", display_name: "Nail", definition: "Finger o", comment_usage_note: "r toe nail sample", status: "" },
        "NASDR" => V2TableRow { value: "NASDR", display_name: "Drainage, Nasal", definition: "Conditio", comment_usage_note: "n", status: "" },
        "NEDL" => V2TableRow { value: "NEDL", display_name: "Needle", definition: "Device", comment_usage_note: "", status: "" },
        "NEPH" => V2TableRow { value: "NEPH", display_name: "Site, Nephrostomy", definition: "Site", comment_usage_note: "", status: "" },
        "NGASP" => V2TableRow { value: "NGASP", display_name: "Aspirate, Nasogastric", definition: "Aspirate", comment_usage_note: "", status: "" },
        "NGAST" => V2TableRow { value: "NGAST", display_name: "Drainage, Nasogastric", definition: "Conditio", comment_usage_note: "n", status: "" },
        "NGS" => V2TableRow { value: "NGS", display_name: "Site, Naso/Gastric", definition: "Site", comment_usage_note: "", status: "" },
        "NODUL" => V2TableRow { value: "NODUL", display_name: "Nodule(s)", definition: "Conditio", comment_usage_note: "n", status: "" },
        "NSECR" => V2TableRow { value: "NSECR", display_name: "Secretion, Nasal", definition: "Condition", comment_usage_note: "", status: "" },
        "ORH" => V2TableRow { value: "ORH", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "ORL" => V2TableRow { value: "ORL", display_name: "Lesion, Oral", definition: "Condition", comment_usage_note: "(Common Usage)", status: "" },
        "OTH" => V2TableRow { value: "OTH", display_name: "Source, Other", definition: "", comment_usage_note: "", status: "" },
        "PACEM" => V2TableRow { value: "PACEM", display_name: "Pacemaker", definition: "Device", comment_usage_note: "", status: "" },
        "PAFL" => V2TableRow { value: "PAFL", display_name: "Pancreatic fluid", definition: "", comment_usage_note: "", status: "" },
        "PCFL" => V2TableRow { value: "PCFL", display_name: "Fluid, Pericardial", definition: "", comment_usage_note: "", status: "" },
        "PDSIT" => V2TableRow { value: "PDSIT", display_name: "Site, Peritoneal Dialysis", definition: "Site", comment_usage_note: "", status: "" },
        "PDTS" => V2TableRow { value: "PDTS", display_name: "Site, Peritoneal Dialysis Tunnel", definition: "Site", comment_usage_note: "", status: "" },
        "PELVA" => V2TableRow { value: "PELVA", display_name: "Abscess, Pelvic", definition: "Condition", comment_usage_note: "", status: "" },
        "PENIL" => V2TableRow { value: "PENIL", display_name: "Lesion, Penile", definition: "Condition", comment_usage_note: "(Common Usage)", status: "" },
        "PERIA" => V2TableRow { value: "PERIA", display_name: "Abscess, Perianal", definition: "Condition", comment_usage_note: ", Abscess & Body Part", status: "" },
        "PILOC" => V2TableRow { value: "PILOC", display_name: "Cyst, Pilonidal", definition: "Condition", comment_usage_note: "", status: "" },
        "PINS" => V2TableRow { value: "PINS", display_name: "Site, Pin", definition: "Site", comment_usage_note: "", status: "" },
        "PIS" => V2TableRow { value: "PIS", display_name: "Site, Pacemaker Insetion", definition: "Site", comment_usage_note: "", status: "" },
        "PLAN" => V2TableRow { value: "PLAN", display_name: "Plant Material", definition: "Object", comment_usage_note: "", status: "" },
        "PLAS" => V2TableRow { value: "PLAS", display_name: "Plasma", definition: "Blood", comment_usage_note: "", status: "" },
        "PLB" => V2TableRow { value: "PLB", display_name: "Plasma bag", definition: "Blood", comment_usage_note: "", status: "" },
        "PLC" => V2TableRow { value: "PLC", display_name: "Placenta", definition: "", comment_usage_note: "", status: "" },
        "PLEVS" => V2TableRow { value: "PLEVS", display_name: "Serum, Peak Level", definition: "Blood", comment_usage_note: "", status: "" },
        "PLR" => V2TableRow { value: "PLR", display_name: "Pleural fluid (thoracentesis fluid)", definition: "", comment_usage_note: "", status: "" },
        "PMN" => V2TableRow { value: "PMN", display_name: "Polymorphonuclear neutrophils", definition: "", comment_usage_note: "", status: "" },
        "PND" => V2TableRow { value: "PND", display_name: "Drainage, Penile", definition: "Condition", comment_usage_note: "", status: "" },
        "POL" => V2TableRow { value: "POL", display_name: "Polyps", definition: "Condition", comment_usage_note: "", status: "" },
        "POPGS" => V2TableRow { value: "POPGS", display_name: "Graft Site, Popliteal", definition: "Condition", comment_usage_note: "", status: "" },
        "POPLG" => V2TableRow { value: "POPLG", display_name: "Graft, Popliteal", definition: "Condition", comment_usage_note: "", status: "" },
        "POPLV" => V2TableRow { value: "POPLV", display_name: "Site, Popliteal Vein", definition: "Site", comment_usage_note: "", status: "" },
        "PORTA" => V2TableRow { value: "PORTA", display_name: "Catheter, Porta", definition: "Device", comment_usage_note: "", status: "" },
        "PPP" => V2TableRow { value: "PPP", display_name: "Plasma, Platelet poor", definition: "Blood", comment_usage_note: "", status: "" },
        "PROST" => V2TableRow { value: "PROST", display_name: "Prosthetic Device", definition: "Device", comment_usage_note: "", status: "" },
        "PRP" => V2TableRow { value: "PRP", display_name: "Plasma, Platelet rich", definition: "Blood", comment_usage_note: "", status: "" },
        "PSC" => V2TableRow { value: "PSC", display_name: "Pseudocyst", definition: "Condition", comment_usage_note: "", status: "" },
        "PUNCT" => V2TableRow { value: "PUNCT", display_name: "Wound, Puncture", definition: "Condition", comment_usage_note: "", status: "" },
        "PUS" => V2TableRow { value: "PUS", display_name: "Pus", definition: "", comment_usage_note: "", status: "" },
        "PUSFR" => V2TableRow { value: "PUSFR", display_name: "Pustule", definition: "Condition", comment_usage_note: "", status: "" },
        "PUST" => V2TableRow { value: "PUST", display_name: "Pus", definition: "Condition", comment_usage_note: "", status: "" },
        "QC3" => V2TableRow { value: "QC3", display_name: "Quality Control", definition: "Environme", comment_usage_note: "nt", status: "" },
        "RANDU" => V2TableRow { value: "RANDU", display_name: "Urine, Random", definition: "Condition", comment_usage_note: "", status: "" },
        "RBC" => V2TableRow { value: "RBC", display_name: "Erythrocytes", definition: "", comment_usage_note: "", status: "" },
        "RBITE" => V2TableRow { value: "RBITE", display_name: "Bite, Reptile", definition: "Condition", comment_usage_note: "s", status: "" },
        "RECT" => V2TableRow { value: "RECT", display_name: "Drainage, Rectal", definition: "Condition", comment_usage_note: "", status: "" },
        "RECTA" => V2TableRow { value: "RECTA", display_name: "Abscess, Rec tal", definition: "Condition", comment_usage_note: "", status: "" },
        "RENALC" => V2TableRow { value: "RENALC", display_name: "Cyst, Renal", definition: "Condition", comment_usage_note: "", status: "" },
        "RENC" => V2TableRow { value: "RENC", display_name: "Fluid, Renal Cyst", definition: "Fluid", comment_usage_note: "", status: "" },
        "RES" => V2TableRow { value: "RES", display_name: "Respiratory", definition: "Condition", comment_usage_note: "(Ambiguous)", status: "" },
        "SAL" => V2TableRow { value: "SAL", display_name: "Saliva", definition: "", comment_usage_note: "", status: "" },
        "SCA" => V2TableRow { value: "SCA", display_name: "R Tissue, Keloid (Scar)", definition: "Tissue", comment_usage_note: "", status: "" },
        "SCLV" => V2TableRow { value: "SCLV", display_name: "Catheter Tip, Subclavian", definition: "Device", comment_usage_note: "", status: "" },
        "SCROA" => V2TableRow { value: "SCROA", display_name: "Abscess, Scrotal", definition: "Condition", comment_usage_note: "", status: "" },
        "SECRE" => V2TableRow { value: "SECRE", display_name: "Secretion(s)", definition: "Fluid/Sec", comment_usage_note: "retion", status: "" },
        "SER" => V2TableRow { value: "SER", display_name: "Serum", definition: "", comment_usage_note: "", status: "" },
        "SHU" => V2TableRow { value: "SHU", display_name: "Site, Shunt", definition: "Site", comment_usage_note: "", status: "" },
        "SHUNF" => V2TableRow { value: "SHUNF", display_name: "Fluid, Shunt", definition: "Fluid", comment_usage_note: "", status: "" },
        "SHUNT" => V2TableRow { value: "SHUNT", display_name: "Shunt", definition: "Condition", comment_usage_note: "", status: "" },
        "SITE" => V2TableRow { value: "SITE", display_name: "Site", definition: "Site", comment_usage_note: "", status: "" },
        "SKBP" => V2TableRow { value: "SKBP", display_name: "Biopsy, Skin", definition: "Tissue", comment_usage_note: "", status: "" },
        "SKN" => V2TableRow { value: "SKN", display_name: "Skin", definition: "", comment_usage_note: "", status: "" },
        "SMM" => V2TableRow { value: "SMM", display_name: "Mass, Sub-Mandibular", definition: "Condition", comment_usage_note: "", status: "" },
        "SMN" => V2TableRow { value: "SMN", display_name: "Seminal fluid", definition: "", comment_usage_note: "", status: "" },
        "SNV" => V2TableRow { value: "SNV", display_name: "Fluid, synovial (Joint fluid)", definition: "", comment_usage_note: "", status: "" },
        "SPRM" => V2TableRow { value: "SPRM", display_name: "Spermatozoa", definition: "", comment_usage_note: "", status: "" },
        "SPRP" => V2TableRow { value: "SPRP", display_name: "Catheter Tip, Suprapubic", definition: "Device", comment_usage_note: "", status: "" },
        "SPRPB" => V2TableRow { value: "SPRPB", display_name: "Cathether Tip, Suprapubic", definition: "Device", comment_usage_note: "", status: "" },
        "SPS" => V2TableRow { value: "SPS", display_name: "Environmental, Spore Strip", definition: "Environme", comment_usage_note: "nt", status: "" },
        "SPT" => V2TableRow { value: "SPT", display_name: "Sputum", definition: "", comment_usage_note: "", status: "" },
        "SPTC" => V2TableRow { value: "SPTC", display_name: "Sputum - coughed", definition: "", comment_usage_note: "", status: "" },
        "SPTT" => V2TableRow { value: "SPTT", display_name: "Sputum - tracheal aspirate", definition: "", comment_usage_note: "", status: "" },
        "SPUT1" => V2TableRow { value: "SPUT1", display_name: "Sputum, Simulated", definition: "Condition", comment_usage_note: "", status: "" },
        "SPUTIN" => V2TableRow { value: "SPUTIN", display_name: "Sputum, Inducted", definition: "Condition", comment_usage_note: "", status: "" },
        "SPUTSP" => V2TableRow { value: "SPUTSP", display_name: "Sputum, Spontaneous", definition: "Condition", comment_usage_note: "", status: "" },
        "STER" => V2TableRow { value: "STER", display_name: "Environmental, Sterrad", definition: "Environme", comment_usage_note: "nt", status: "" },
        "STL" => V2TableRow { value: "STL", display_name: "Stool = Fecal", definition: "", comment_usage_note: "", status: "" },
        "STONE" => V2TableRow { value: "STONE", display_name: "Stone, Kidney", definition: "Condition", comment_usage_note: "", status: "" },
        "SUBMA" => V2TableRow { value: "SUBMA", display_name: "Abscess, Submandibular", definition: "Condition", comment_usage_note: "", status: "" },
        "SUBMX" => V2TableRow { value: "SUBMX", display_name: "Abscess, Submaxillary", definition: "Condition", comment_usage_note: "", status: "" },
        "SUMP" => V2TableRow { value: "SUMP", display_name: "Drainage, Sump", definition: "Condition", comment_usage_note: "", status: "" },
        "SUP" => V2TableRow { value: "SUP", display_name: "Suprapubic Tap", definition: "Product", comment_usage_note: "", status: "" },
        "SUTUR" => V2TableRow { value: "SUTUR", display_name: "Suture", definition: "Object", comment_usage_note: "", status: "" },
        "SWGZ" => V2TableRow { value: "SWGZ", display_name: "Catheter Tip, Swan Gantz", definition: "Device", comment_usage_note: "", status: "" },
        "SWT" => V2TableRow { value: "SWT", display_name: "Sweat", definition: "", comment_usage_note: "", status: "" },
        "TASP" => V2TableRow { value: "TASP", display_name: "Aspirate, Tracheal", definition: "Aspirate", comment_usage_note: "", status: "" },
        "TEAR" => V2TableRow { value: "TEAR", display_name: "Tears", definition: "", comment_usage_note: "", status: "" },
        "THRB" => V2TableRow { value: "THRB", display_name: "Thrombocyte (platelet)", definition: "", comment_usage_note: "", status: "" },
        "TISS" => V2TableRow { value: "TISS", display_name: "Tissue", definition: "", comment_usage_note: "", status: "" },
        "TISU" => V2TableRow { value: "TISU", display_name: "Tissue ulcer", definition: "", comment_usage_note: "", status: "" },
        "TLC" => V2TableRow { value: "TLC", display_name: "Cathether Tip, Triple Lumen", definition: "Device", comment_usage_note: "", status: "" },
        "TRAC" => V2TableRow { value: "TRAC", display_name: "Site, Tracheostomy", definition: "Site", comment_usage_note: "", status: "" },
        "TRANS" => V2TableRow { value: "TRANS", display_name: "Transudate", definition: "Condition", comment_usage_note: "", status: "" },
        "TSERU" => V2TableRow { value: "TSERU", display_name: "Serum, Trough", definition: "Blood", comment_usage_note: "", status: "" },
        "TSTES" => V2TableRow { value: "TSTES", display_name: "Abscess, Testicular", definition: "Condition", comment_usage_note: "", status: "" },
        "TTRA" => V2TableRow { value: "TTRA", display_name: "Aspirate, Transtracheal", definition: "Aspirate", comment_usage_note: "", status: "" },
        "TUBES" => V2TableRow { value: "TUBES", display_name: "Tubes", definition: "Device", comment_usage_note: "", status: "" },
        "TUMOR" => V2TableRow { value: "TUMOR", display_name: "Tumor", definition: "Condition", comment_usage_note: "", status: "" },
        "TZANC" => V2TableRow { value: "TZANC", display_name: "Smear, Tzanck", definition: "", comment_usage_note: "", status: "" },
        "UDENT" => V2TableRow { value: "UDENT", display_name: "Source, Unidentified", definition: "", comment_usage_note: "", status: "" },
        "UMED" => V2TableRow { value: "UMED", display_name: "Unknown Medicine", definition: "for foren testing", comment_usage_note: "sic and possibly chemistry", status: "" },
        "UR" => V2TableRow { value: "UR", display_name: "Urine", definition: "", comment_usage_note: "", status: "" },
        "URC" => V2TableRow { value: "URC", display_name: "Urine clean catch", definition: "", comment_usage_note: "", status: "" },
        "URINB" => V2TableRow { value: "URINB", display_name: "Urine, Bladder Washings", definition: "Condition", comment_usage_note: "", status: "" },
        "URINC" => V2TableRow { value: "URINC", display_name: "Urine, Catheterized", definition: "Condition", comment_usage_note: "", status: "" },
        "URINM" => V2TableRow { value: "URINM", display_name: "Urine, Midstream", definition: "Condition", comment_usage_note: "", status: "" },
        "URINN" => V2TableRow { value: "URINN", display_name: "Urine, Nephrostomy", definition: "Condition", comment_usage_note: "", status: "" },
        "URINP" => V2TableRow { value: "URINP", display_name: "Urine, Pedibag", definition: "Device", comment_usage_note: "", status: "" },
        "URNS" => V2TableRow { value: "URNS", display_name: "Urine sediment", definition: "", comment_usage_note: "", status: "" },
        "URT" => V2TableRow { value: "URT", display_name: "Urine catheter", definition: "", comment_usage_note: "", status: "" },
        "USCO" => V2TableRow { value: "USCO", display_name: "P Urine, Cystoscopy", definition: "C", comment_usage_note: "ondition", status: "" },
        "USPEC" => V2TableRow { value: "USPEC", display_name: "Source, Unspecified", definition: "", comment_usage_note: "", status: "" },
        "USUB" => V2TableRow { value: "USUB", display_name: "Unkown substance", definition: "", comment_usage_note: "", status: "" },
        "VASTIP" => V2TableRow { value: "VASTIP", display_name: "Catheter Tip, Vas", definition: "D", comment_usage_note: "evice", status: "" },
        "VENT" => V2TableRow { value: "VENT", display_name: "Catheter Tip, Ventricular", definition: "D", comment_usage_note: "evice", status: "" },
        "VITF" => V2TableRow { value: "VITF", display_name: "Vitreous Fluid", definition: "", comment_usage_note: "", status: "" },
        "VOM" => V2TableRow { value: "VOM", display_name: "Vomitus", definition: "", comment_usage_note: "", status: "" },
        "WASH" => V2TableRow { value: "WASH", display_name: "Wash", definition: "P", comment_usage_note: "roduct", status: "" },
        "WASI" => V2TableRow { value: "WASI", display_name: "Washing, e.g. bronchial washing", definition: "P", comment_usage_note: "roduct", status: "" },
        "WAT" => V2TableRow { value: "WAT", display_name: "Water", definition: "", comment_usage_note: "", status: "" },
        "WB" => V2TableRow { value: "WB", display_name: "Blood, Whole", definition: "B", comment_usage_note: "lood", status: "" },
        "WBC" => V2TableRow { value: "WBC", display_name: "Leukocytes", definition: "", comment_usage_note: "", status: "" },
        "WEN" => V2TableRow { value: "WEN", display_name: "Wen", definition: "T", comment_usage_note: "issue", status: "" },
        "WICK" => V2TableRow { value: "WICK", display_name: "Wick", definition: "", comment_usage_note: "", status: "" },
        "WND" => V2TableRow { value: "WND", display_name: "Wound", definition: "", comment_usage_note: "", status: "" },
        "WNDA" => V2TableRow { value: "WNDA", display_name: "Wound abscess", definition: "", comment_usage_note: "", status: "" },
        "WNDD" => V2TableRow { value: "WNDD", display_name: "Wound drainage", definition: "", comment_usage_note: "", status: "" },
        "WNDE" => V2TableRow { value: "WNDE", display_name: "Wound exudate", definition: "", comment_usage_note: "", status: "" },
        "WORM" => V2TableRow { value: "WORM", display_name: "Worm", definition: "O", comment_usage_note: "bject", status: "" },
        "WRT" => V2TableRow { value: "WRT", display_name: "Wart", definition: "T", comment_usage_note: "issue", status: "" },
        "WWA" => V2TableRow { value: "WWA", display_name: "Environmental, Water", definition: "E", comment_usage_note: "nvironment", status: "" },
        "WWO" => V2TableRow { value: "WWO", display_name: "Environmental, Water (Ocean)", definition: "", comment_usage_note: "", status: "" },
        "WWT" => V2TableRow { value: "WWT", display_name: "Environmental, Water (Tap)", definition: "E", comment_usage_note: "nvironment", status: "" },
    },
};

pub static TABLE_0488: V2Table = V2Table {
    number: 488,
    metadata: &super::metadata::TABLE_0488_METADATA,
    rows: phf_map! {
        "FNA" => V2TableRow { value: "FNA", display_name: "Aspiration, Fine Needle", definition: "", comment_usage_note: "", status: "" },
        "PNA" => V2TableRow { value: "PNA", display_name: "Arterial puncture", definition: "", comment_usage_note: "", status: "" },
        "BIO" => V2TableRow { value: "BIO", display_name: "Biopsy", definition: "", comment_usage_note: "", status: "" },
        "BCAE" => V2TableRow { value: "BCAE", display_name: "Blood Culture, Aerobic Bottle", definition: "", comment_usage_note: "", status: "" },
        "BCA" => V2TableRow { value: "BCA", display_name: "N Blood Culture, Anaerobic Bottle", definition: "", comment_usage_note: "", status: "" },
        "BCPD" => V2TableRow { value: "BCPD", display_name: "Blood Culture, Pediatric Bottle", definition: "", comment_usage_note: "", status: "" },
        "CAP" => V2TableRow { value: "CAP", display_name: "Capillary Specimen", definition: "", comment_usage_note: "", status: "" },
        "CATH" => V2TableRow { value: "CATH", display_name: "Catheterized", definition: "", comment_usage_note: "", status: "" },
        "EPLA" => V2TableRow { value: "EPLA", display_name: "Environmental, Plate", definition: "", comment_usage_note: "", status: "" },
        "ESWA" => V2TableRow { value: "ESWA", display_name: "Environmental, Swab", definition: "", comment_usage_note: "", status: "" },
        "LNA" => V2TableRow { value: "LNA", display_name: "Line, Arterial", definition: "", comment_usage_note: "", status: "" },
        "CVP" => V2TableRow { value: "CVP", display_name: "Line, CVP", definition: "", comment_usage_note: "", status: "" },
        "LNV" => V2TableRow { value: "LNV", display_name: "Line, Venous", definition: "", comment_usage_note: "", status: "" },
        "MARTL" => V2TableRow { value: "MARTL", display_name: "Martin-Lewis Agar", definition: "", comment_usage_note: "", status: "" },
        "ML11" => V2TableRow { value: "ML11", display_name: "Mod. Martin-Lewis Agar", definition: "", comment_usage_note: "", status: "" },
        "PACE" => V2TableRow { value: "PACE", display_name: "Pace, Gen-Probe", definition: "", comment_usage_note: "", status: "" },
        "PIN" => V2TableRow { value: "PIN", display_name: "Pinworm Prep", definition: "", comment_usage_note: "", status: "" },
        "KOFFP" => V2TableRow { value: "KOFFP", display_name: "Plate, Cough", definition: "", comment_usage_note: "", status: "" },
        "MLP" => V2TableRow { value: "MLP", display_name: "Plate, Martin-Lewis", definition: "", comment_usage_note: "", status: "" },
        "NYP" => V2TableRow { value: "NYP", display_name: "Plate, New York City", definition: "", comment_usage_note: "", status: "" },
        "TMP" => V2TableRow { value: "TMP", display_name: "Plate, Thayer-Martin", definition: "", comment_usage_note: "", status: "" },
        "ANP" => V2TableRow { value: "ANP", display_name: "Plates, Anaerobic", definition: "", comment_usage_note: "", status: "" },
        "BAP" => V2TableRow { value: "BAP", display_name: "Plates, Blood Agar", definition: "", comment_usage_note: "", status: "" },
        "PRIME" => V2TableRow { value: "PRIME", display_name: "Pump Prime", definition: "", comment_usage_note: "", status: "" },
        "PUMP" => V2TableRow { value: "PUMP", display_name: "Pump Specimen", definition: "", comment_usage_note: "", status: "" },
        "QC5" => V2TableRow { value: "QC5", display_name: "Quality Control For Micro", definition: "", comment_usage_note: "", status: "" },
        "SCLP" => V2TableRow { value: "SCLP", display_name: "Scalp, Fetal Vein", definition: "", comment_usage_note: "", status: "" },
        "SCRAPS" => V2TableRow { value: "SCRAPS", display_name: "Scrapings", definition: "", comment_usage_note: "", status: "" },
        "SHA" => V2TableRow { value: "SHA", display_name: "Shaving", definition: "", comment_usage_note: "", status: "" },
        "SWA" => V2TableRow { value: "SWA", display_name: "Swab", definition: "", comment_usage_note: "", status: "" },
        "SWD" => V2TableRow { value: "SWD", display_name: "Swab, Dacron tipped", definition: "", comment_usage_note: "", status: "" },
        "WOOD" => V2TableRow { value: "WOOD", display_name: "Swab, Wooden Shaft", definition: "", comment_usage_note: "", status: "" },
        "TMOT" => V2TableRow { value: "TMOT", display_name: "Transport Media,", definition: "", comment_usage_note: "", status: "" },
        "TMAN" => V2TableRow { value: "TMAN", display_name: "Transport Media, Anaerobic", definition: "", comment_usage_note: "", status: "" },
        "TMCH" => V2TableRow { value: "TMCH", display_name: "Transport Media, Chalamydia", definition: "", comment_usage_note: "", status: "" },
        "TMM4" => V2TableRow { value: "TMM4", display_name: "Transport Media, M4", definition: "", comment_usage_note: "", status: "" },
        "TMMY" => V2TableRow { value: "TMMY", display_name: "Transport Media, Mycoplasma", definition: "", comment_usage_note: "", status: "" },
        "TMPV" => V2TableRow { value: "TMPV", display_name: "Transport Media, PVA", definition: "", comment_usage_note: "", status: "" },
        "TMSC" => V2TableRow { value: "TMSC", display_name: "Transport Media, Stool Culture", definition: "", comment_usage_note: "", status: "" },
        "TMUP" => V2TableRow { value: "TMUP", display_name: "Transport Media, Ureaplasma", definition: "", comment_usage_note: "", status: "" },
        "TMVI" => V2TableRow { value: "TMVI", display_name: "Transport Media, Viral", definition: "", comment_usage_note: "", status: "" },
        "VENIP" => V2TableRow { value: "VENIP", display_name: "Venipuncture", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0489: V2Table = V2Table {
    number: 489,
    metadata: &super::metadata::TABLE_0489_METADATA,
    rows: phf_map! {
        "BIO" => V2TableRow { value: "BIO", display_name: "Biological", definition: "", comment_usage_note: "The dangers associated with normal biological materials. I.e. potential risk of unknown infections. Routine biological materials from living subjects.", status: "" },
        "COR" => V2TableRow { value: "COR", display_name: "Corrosive", definition: "", comment_usage_note: "Material is corrosive and may cause severe injury to skin, mucous membranes and eyes. Avoid any unprotected contact.", status: "" },
        "ESC" => V2TableRow { value: "ESC", display_name: "Escape Risk", definition: "", comment_usage_note: "The entity is at risk for escaping from containment or control.", status: "" },
        "AGG" => V2TableRow { value: "AGG", display_name: "Aggressive", definition: "", comment_usage_note: "A danger that can be associated with certain living subjects, including humans.", status: "" },
        "IFL" => V2TableRow { value: "IFL", display_name: "MaterialDangerIn flammable", definition: "", comment_usage_note: "Material is highly inflammable and in certain mixtures (with air) may lead to explosions. Keep away from fire, sparks and excessive heat.", status: "" },
        "EXP" => V2TableRow { value: "EXP", display_name: "Explosive", definition: "", comment_usage_note: "Material is an explosive mixture. Keep away from fire, sparks, and heat.", status: "" },
        "INF" => V2TableRow { value: "INF", display_name: "MaterialDangerIn fectious", definition: "", comment_usage_note: "Material known to be infectious with human pathogenic microorganisms. Those who handle this material must take precautions for their protection.", status: "" },
        "BHZ" => V2TableRow { value: "BHZ", display_name: "Biohazard", definition: "", comment_usage_note: "Material contains microorganisms that are an environmental hazard. Must be handled with special care.", status: "" },
        "INJ" => V2TableRow { value: "INJ", display_name: "Injury Hazard", definition: "", comment_usage_note: "Material is solid and sharp (e.g., cannulas.) Dispose in hard container.", status: "" },
        "POI" => V2TableRow { value: "POI", display_name: "Poison", definition: "", comment_usage_note: "Material is poisonous to humans and/or animals. Special care must be taken to avoid incorporation, even of small amounts.", status: "" },
        "RAD" => V2TableRow { value: "RAD", display_name: "Radioactive", definition: "", comment_usage_note: "Material is a source for ionizing radiation and must be handled with special care to avoid injury of those who handle it and to avoid environmental hazards.", status: "" },
    },
};

pub static TABLE_0490: V2Table = V2Table {
    number: 490,
    metadata: &super::metadata::TABLE_0490_METADATA,
    rows: phf_map! {
        "EX" => V2TableRow { value: "EX", display_name: "Expired", definition: "", comment_usage_note: "", status: "" },
        "QS" => V2TableRow { value: "QS", display_name: "Quantity not sufficient", definition: "", comment_usage_note: "", status: "" },
        "RB" => V2TableRow { value: "RB", display_name: "Broken container", definition: "", comment_usage_note: "", status: "" },
        "RC" => V2TableRow { value: "RC", display_name: "Clotting", definition: "", comment_usage_note: "", status: "" },
        "RD" => V2TableRow { value: "RD", display_name: "Missing collection date", definition: "", comment_usage_note: "", status: "" },
        "RA" => V2TableRow { value: "RA", display_name: "Missing patient ID number", definition: "", comment_usage_note: "", status: "" },
        "RE" => V2TableRow { value: "RE", display_name: "Missing patient name", definition: "", comment_usage_note: "", status: "" },
        "RH" => V2TableRow { value: "RH", display_name: "Hemolysis", definition: "", comment_usage_note: "", status: "" },
        "RI" => V2TableRow { value: "RI", display_name: "Identification problem", definition: "", comment_usage_note: "", status: "" },
        "RM" => V2TableRow { value: "RM", display_name: "Labeling", definition: "", comment_usage_note: "", status: "" },
        "RN" => V2TableRow { value: "RN", display_name: "Contamination", definition: "", comment_usage_note: "", status: "" },
        "RP" => V2TableRow { value: "RP", display_name: "Missing phlebotomist ID", definition: "", comment_usage_note: "", status: "" },
        "RR" => V2TableRow { value: "RR", display_name: "Improper storage", definition: "", comment_usage_note: "", status: "" },
        "RS" => V2TableRow { value: "RS", display_name: "Name misspelling", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0491: V2Table = V2Table {
    number: 491,
    metadata: &super::metadata::TABLE_0491_METADATA,
    rows: phf_map! {
        "E" => V2TableRow { value: "E", display_name: "Excellent", definition: "", comment_usage_note: "", status: "" },
        "G" => V2TableRow { value: "G", display_name: "Good", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Fair", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Poor", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0492: V2Table = V2Table {
    number: 492,
    metadata: &super::metadata::TABLE_0492_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Preferred", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Appropriate", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Inappropriate", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0493: V2Table = V2Table {
    number: 493,
    metadata: &super::metadata::TABLE_0493_METADATA,
    rows: phf_map! {
        "AUT" => V2TableRow { value: "AUT", display_name: "Autolyzed", definition: "", comment_usage_note: "", status: "" },
        "CLOT" => V2TableRow { value: "CLOT", display_name: "Clotted", definition: "", comment_usage_note: "", status: "" },
        "CON" => V2TableRow { value: "CON", display_name: "Contaminated", definition: "", comment_usage_note: "", status: "" },
        "COOL" => V2TableRow { value: "COOL", display_name: "Cool", definition: "", comment_usage_note: "", status: "" },
        "FROZ" => V2TableRow { value: "FROZ", display_name: "Frozen", definition: "", comment_usage_note: "", status: "" },
        "HEM" => V2TableRow { value: "HEM", display_name: "Hemolyzed", definition: "", comment_usage_note: "", status: "" },
        "LIVE" => V2TableRow { value: "LIVE", display_name: "Live", definition: "", comment_usage_note: "", status: "" },
        "ROOM" => V2TableRow { value: "ROOM", display_name: "Room temperature", definition: "", comment_usage_note: "", status: "" },
        "SNR" => V2TableRow { value: "SNR", display_name: "Sample not received", definition: "", comment_usage_note: "Deprecated in v 2.8", status: "" },
        "CFU" => V2TableRow { value: "CFU", display_name: "Centrifuged", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0494: V2Table = V2Table {
    number: 494,
    metadata: &super::metadata::TABLE_0494_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Aliquot", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Component", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Modified from original specimen", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0495: V2Table = V2Table {
    number: 495,
    metadata: &super::metadata::TABLE_0495_METADATA,
    rows: phf_map! {
        "ANT" => V2TableRow { value: "ANT", display_name: "Anterior", definition: "", comment_usage_note: "", status: "" },
        "BIL" => V2TableRow { value: "BIL", display_name: "Bilateral", definition: "", comment_usage_note: "", status: "" },
        "DIS" => V2TableRow { value: "DIS", display_name: "Distal", definition: "", comment_usage_note: "", status: "" },
        "EXT" => V2TableRow { value: "EXT", display_name: "External", definition: "", comment_usage_note: "", status: "" },
        "LAT" => V2TableRow { value: "LAT", display_name: "Lateral", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Left", definition: "", comment_usage_note: "", status: "" },
        "LOW" => V2TableRow { value: "LOW", display_name: "Lower", definition: "", comment_usage_note: "", status: "" },
        "MED" => V2TableRow { value: "MED", display_name: "Medial", definition: "", comment_usage_note: "", status: "" },
        "POS" => V2TableRow { value: "POS", display_name: "Posterior", definition: "", comment_usage_note: "", status: "" },
        "PRO" => V2TableRow { value: "PRO", display_name: "Proximal", definition: "", comment_usage_note: "", status: "" },
        "LLQ" => V2TableRow { value: "LLQ", display_name: "Quadrant, Left Lower", definition: "", comment_usage_note: "", status: "" },
        "LUQ" => V2TableRow { value: "LUQ", display_name: "Quadrant, Left Upper", definition: "", comment_usage_note: "", status: "" },
        "RLQ" => V2TableRow { value: "RLQ", display_name: "Quadrant, Right Lower", definition: "", comment_usage_note: "", status: "" },
        "RUQ" => V2TableRow { value: "RUQ", display_name: "Quadrant, Right Upper", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Right", definition: "", comment_usage_note: "", status: "" },
        "UPP" => V2TableRow { value: "UPP", display_name: "Upper", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0496: V2Table = V2Table {
    number: 496,
    metadata: &super::metadata::TABLE_0496_METADATA,
    rows: phf_map! {
        "001" => V2TableRow { value: "001", display_name: "Release of Information/MR / Authorization to Disclosure Protected Health Information", definition: "", comment_usage_note: "Release of Info/ Disclosure", status: "" },
        "002" => V2TableRow { value: "002", display_name: "Medical Procedure (invasive)", definition: "", comment_usage_note: "Medical Treatment/ Procedure", status: "" },
        "003" => V2TableRow { value: "003", display_name: "Acknowledge Receipt of Privacy Notice", definition: "", comment_usage_note: "Acknowledgement/ Notification", status: "" },
        "004" => V2TableRow { value: "004", display_name: "Abortion", definition: "", comment_usage_note: "Medical Treatment/ Procedure", status: "" },
        "005" => V2TableRow { value: "005", display_name: "Abortion/Laminaria", definition: "", comment_usage_note: "Medical Treatment/ Procedure", status: "" },
        "006" => V2TableRow { value: "006", display_name: "Accutane - Information", definition: "", comment_usage_note: "Medical Treatment/ Procedure", status: "" },
        "007" => V2TableRow { value: "007", display_name: "Accutane - Woman", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "008" => V2TableRow { value: "008", display_name: "Advanced Beneficiary Notice", definition: "Acknowledgement/ Notification", comment_usage_note: "", status: "" },
        "009" => V2TableRow { value: "009", display_name: "AFP (Alpha Fetoprotein) Screening", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "010" => V2TableRow { value: "010", display_name: "Amniocentesis (consent & refusal)", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "011" => V2TableRow { value: "011", display_name: "Anatomical Gift (organ donation)", definition: "Administrative", comment_usage_note: "", status: "" },
        "012" => V2TableRow { value: "012", display_name: "Anesthesia - Complications", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "013" => V2TableRow { value: "013", display_name: "Anesthesia - Questionnaire", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "014" => V2TableRow { value: "014", display_name: "Angiogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "015" => V2TableRow { value: "015", display_name: "Angioplasty", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "016" => V2TableRow { value: "016", display_name: "Anticancer Drugs", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "017" => V2TableRow { value: "017", display_name: "Antipsychotic Medications", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "018" => V2TableRow { value: "018", display_name: "Arthrogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "019" => V2TableRow { value: "019", display_name: "Autopsy", definition: "Administrative", comment_usage_note: "", status: "" },
        "020" => V2TableRow { value: "020", display_name: "AZT Therapy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "021" => V2TableRow { value: "021", display_name: "Biliary Drainage", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "022" => V2TableRow { value: "022", display_name: "Biliary Stone Extraction", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "023" => V2TableRow { value: "023", display_name: "Biopsy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "024" => V2TableRow { value: "024", display_name: "Bleeding Time Test", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "025" => V2TableRow { value: "025", display_name: "Bronchogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "026" => V2TableRow { value: "026", display_name: "Cardiac Catheterization", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "027" => V2TableRow { value: "027", display_name: "Coronary Angiography", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "028" => V2TableRow { value: "028", display_name: "Coronary Angiography w/o \"\" \"\" w/o Surgery Capability Surgery Capability", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "029" => V2TableRow { value: "029", display_name: "Cataract Op/Implant of FDA Aprvd Lens", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "030" => V2TableRow { value: "030", display_name: "Cataract Op/Implant of Investigational Lens", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "031" => V2TableRow { value: "031", display_name: "Cataract Surgery", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "032" => V2TableRow { value: "032", display_name: "Cholera Immunization", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "033" => V2TableRow { value: "033", display_name: "Cholesterol Screening", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "034" => V2TableRow { value: "034", display_name: "Circumcision - Newborn", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "035" => V2TableRow { value: "035", display_name: "Colonoscopy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "036" => V2TableRow { value: "036", display_name: "Contact Lenses", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "037" => V2TableRow { value: "037", display_name: "CT Scan - Cervical & Lumbar", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "038" => V2TableRow { value: "038", display_name: "CT Scan w/ IV Contrast Media into Vein", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "039" => V2TableRow { value: "039", display_name: "CVS (Chorionic Villus) Sampling", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "040" => V2TableRow { value: "040", display_name: "Cystospy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "041" => V2TableRow { value: "041", display_name: "Disclosure of Protected Health Information to Family/Friends", definition: "Release of Info/ Disclosure", comment_usage_note: "", status: "" },
        "042" => V2TableRow { value: "042", display_name: "D & C and Conization", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "043" => V2TableRow { value: "043", display_name: "Dacryocystogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "044" => V2TableRow { value: "044", display_name: "Diagnostic Isotope", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "045" => V2TableRow { value: "045", display_name: "Drainage of an Abscess", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "046" => V2TableRow { value: "046", display_name: "Drug Screening", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "047" => V2TableRow { value: "047", display_name: "Electronic Monitoring of Labor - Refusal", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "048" => V2TableRow { value: "048", display_name: "Endometrial Biopsy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "049" => V2TableRow { value: "049", display_name: "Endoscopy/Sclerosis of Esophageal Varices", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "050" => V2TableRow { value: "050", display_name: "ERCP", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "051" => V2TableRow { value: "051", display_name: "Exposure to reportable Communicable Disease", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "052" => V2TableRow { value: "052", display_name: "External Version", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "053" => V2TableRow { value: "053", display_name: "Fluorescein Angioscopy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "054" => V2TableRow { value: "054", display_name: "Hepatitis B - Consent/Declination", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "055" => V2TableRow { value: "055", display_name: "Herniogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "056" => V2TableRow { value: "056", display_name: "HIV Test - Consent Refusal", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "057" => V2TableRow { value: "057", display_name: "HIV Test - Disclosure", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "058" => V2TableRow { value: "058", display_name: "HIV Test - Prenatal", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "059" => V2TableRow { value: "059", display_name: "Home IV Treatment Program", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "060" => V2TableRow { value: "060", display_name: "Home Parenteral Treatment Program", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "061" => V2TableRow { value: "061", display_name: "Hysterectomy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "062" => V2TableRow { value: "062", display_name: "Hysterosalpingogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "063" => V2TableRow { value: "063", display_name: "Injection Slip/ Consent", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "064" => V2TableRow { value: "064", display_name: "Intrauterine Device", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "065" => V2TableRow { value: "065", display_name: "Intrauterine Device/Sterilization", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "066" => V2TableRow { value: "066", display_name: "Intravascular Infusion of Streptokinase/Urokinase", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "067" => V2TableRow { value: "067", display_name: "Intravenous Cholangiogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "068" => V2TableRow { value: "068", display_name: "Intravenous Digital Angiography", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "069" => V2TableRow { value: "069", display_name: "Iodine Administration", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "070" => V2TableRow { value: "070", display_name: "ISG", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "071" => V2TableRow { value: "071", display_name: "IVP", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "072" => V2TableRow { value: "072", display_name: "Laser Photocoagulation", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "073" => V2TableRow { value: "073", display_name: "Laser treatment", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "074" => V2TableRow { value: "074", display_name: "Lithium Carbonate", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "075" => V2TableRow { value: "075", display_name: "Liver Biopsy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "076" => V2TableRow { value: "076", display_name: "Lumbar Puncture", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "077" => V2TableRow { value: "077", display_name: "Lymphangiogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "078" => V2TableRow { value: "078", display_name: "MAO Inhibitors", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "079" => V2TableRow { value: "079", display_name: "Med, Psych, and/or Drug/Alcohol", definition: "Release of Info/ Disclosure", comment_usage_note: "", status: "" },
        "080" => V2TableRow { value: "080", display_name: "Medical Treatment - Refusal", definition: "Administrative", comment_usage_note: "", status: "" },
        "081" => V2TableRow { value: "081", display_name: "Morning-after Pill", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "082" => V2TableRow { value: "082", display_name: "MRI - Adult", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "083" => V2TableRow { value: "083", display_name: "MRI - Pediatric", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "084" => V2TableRow { value: "084", display_name: "Myelogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "085" => V2TableRow { value: "085", display_name: "Needle Biopsy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "086" => V2TableRow { value: "086", display_name: "Needle Biopsy of Lung", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "087" => V2TableRow { value: "087", display_name: "Newborn Treatment and Release", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "088" => V2TableRow { value: "088", display_name: "Norplant Subdermal Birth Control Implant", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "089" => V2TableRow { value: "089", display_name: "Operations, Anesthesia, Transfusions", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "090" => V2TableRow { value: "090", display_name: "Oral Contraceptives", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "091" => V2TableRow { value: "091", display_name: "Organ Donation", definition: "Administrative", comment_usage_note: "", status: "" },
        "092" => V2TableRow { value: "092", display_name: "Patient Permits, Consents", definition: "Administrative", comment_usage_note: "", status: "" },
        "093" => V2TableRow { value: "093", display_name: "Patient Treatment Permit, Release & Admission", definition: "Administrative", comment_usage_note: "", status: "" },
        "094" => V2TableRow { value: "094", display_name: "Penile Injections", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "095" => V2TableRow { value: "095", display_name: "Percutaneous Nephrostomy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "096" => V2TableRow { value: "096", display_name: "Percutaneous Transhepatic Cholangiogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "097" => V2TableRow { value: "097", display_name: "Photographs", definition: "Release of Info/ Disclosure", comment_usage_note: "", status: "" },
        "098" => V2TableRow { value: "098", display_name: "Photographs - Employee", definition: "Release of Info/ Disclosure", comment_usage_note: "", status: "" },
        "099" => V2TableRow { value: "099", display_name: "Photographs - Medical Research", definition: "Release of Info/ Disclosure", comment_usage_note: "", status: "" },
        "100" => V2TableRow { value: "100", display_name: "Photographs - news Media", definition: "Release of Info/ Disclosure", comment_usage_note: "", status: "" },
        "101" => V2TableRow { value: "101", display_name: "Psychiatric Admission - Next of Kin", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "102" => V2TableRow { value: "102", display_name: "Psychiatric Information During Hospital Stay", definition: "Release of Info/ Disclosure", comment_usage_note: "", status: "" },
        "103" => V2TableRow { value: "103", display_name: "Public Release of Information", definition: "Release of Info/ Disclosure", comment_usage_note: "", status: "" },
        "104" => V2TableRow { value: "104", display_name: "Radiologic Procedure", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "105" => V2TableRow { value: "105", display_name: "Refusal of Treatment", definition: "Administrative", comment_usage_note: "", status: "" },
        "106" => V2TableRow { value: "106", display_name: "Release of Body", definition: "Administrative", comment_usage_note: "", status: "" },
        "107" => V2TableRow { value: "107", display_name: "Release of Limb", definition: "Administrative", comment_usage_note: "", status: "" },
        "108" => V2TableRow { value: "108", display_name: "Rh Immune Globulin", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "109" => V2TableRow { value: "109", display_name: "Rights of Medical Research Participants", definition: "Administrative", comment_usage_note: "", status: "" },
        "110" => V2TableRow { value: "110", display_name: "Request to Restrict Access/Disclosure to Medical Record/Protected Health Information", definition: "Release of Info/ Disclosure", comment_usage_note: "", status: "" },
        "111" => V2TableRow { value: "111", display_name: "Request for Remain Anonymous", definition: "Release of Info/ Disclosure", comment_usage_note: "", status: "" },
        "112" => V2TableRow { value: "112", display_name: "Seat Belt Exemption", definition: "Administrative", comment_usage_note: "", status: "" },
        "113" => V2TableRow { value: "113", display_name: "Sialogram", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "114" => V2TableRow { value: "114", display_name: "Sigmoidoscopy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "115" => V2TableRow { value: "115", display_name: "Sterilization - Anesthesia & Medical Services", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "116" => V2TableRow { value: "116", display_name: "Sterilization -Federally Funded", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "117" => V2TableRow { value: "117", display_name: "Sterilization - Female", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "118" => V2TableRow { value: "118", display_name: "Sterilization - Laparoscopy/Pomeroy", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "119" => V2TableRow { value: "119", display_name: "Sterilization - Non-Federally Funded", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "120" => V2TableRow { value: "120", display_name: "Sterilization - Secondary", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "121" => V2TableRow { value: "121", display_name: "Tranquilizers", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "122" => V2TableRow { value: "122", display_name: "Transfer - Acknowledgement", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "123" => V2TableRow { value: "123", display_name: "Transfer - Authorization", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "124" => V2TableRow { value: "124", display_name: "Transfer Certification - Physician", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "125" => V2TableRow { value: "125", display_name: "Transfer/Discharge Request", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "126" => V2TableRow { value: "126", display_name: "Transfer for Non-Medical Reasons", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "127" => V2TableRow { value: "127", display_name: "Transfer - Interfaculty Neonatal", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "128" => V2TableRow { value: "128", display_name: "Transfer Refusal", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "129" => V2TableRow { value: "129", display_name: "Transfer Refusal of Further Treatment", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "130" => V2TableRow { value: "130", display_name: "Treadmill & EKG", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "131" => V2TableRow { value: "131", display_name: "Treadmill, Thallium-201", definition: "Medical Treatment/ Procedure", comment_usage_note: "", status: "" },
        "132" => V2TableRow { value: "132", display_name: "Typhoid", definition: "", comment_usage_note: "Medical Treatment/ Procedure", status: "" },
        "133" => V2TableRow { value: "133", display_name: "Use of Investigational Device", definition: "", comment_usage_note: "Medical Treatment/ Procedure", status: "" },
        "134" => V2TableRow { value: "134", display_name: "Use of Investigational Drug", definition: "", comment_usage_note: "Medical Treatment/ Procedure", status: "" },
        "135" => V2TableRow { value: "135", display_name: "Venogram", definition: "", comment_usage_note: "Medical Treatment/ Procedure", status: "" },
        "136" => V2TableRow { value: "136", display_name: "Videotape", definition: "", comment_usage_note: "Release of Info/ Disclosure", status: "" },
        "1137" => V2TableRow { value: "1137", display_name: "Voiding Cystogram", definition: "", comment_usage_note: "Medical Treatment/ Procedure", status: "" },
    },
};

pub static TABLE_0497: V2Table = V2Table {
    number: 497,
    metadata: &super::metadata::TABLE_0497_METADATA,
    rows: phf_map! {
        "V" => V2TableRow { value: "V", display_name: "Verbal", definition: "", comment_usage_note: "", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Written", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Telephone", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0498: V2Table = V2Table {
    number: 498,
    metadata: &super::metadata::TABLE_0498_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "A ct iv e - Consent has been granted", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Limited - Consent has been granted with limitations", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Refused - Consent has been refused", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Pending - Consent has not yet been sought", definition: "", comment_usage_note: "", status: "" },
        "X" => V2TableRow { value: "X", display_name: "Rescinded - Consent was initially granted, but was subsequently revoked or ended.", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Bypassed (Consent not sought)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0499: V2Table = V2Table {
    number: 499,
    metadata: &super::metadata::TABLE_0499_METADATA,
    rows: phf_map! {
        "E" => V2TableRow { value: "E", display_name: "Emergency", definition: "", comment_usage_note: "", status: "" },
        "PJ" => V2TableRow { value: "PJ", display_name: "Professional Judgment", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0500: V2Table = V2Table {
    number: 500,
    metadata: &super::metadata::TABLE_0500_METADATA,
    rows: phf_map! {
        "F" => V2TableRow { value: "F", display_name: "Full Disclosure", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Partial Disclosure", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No Disclosure", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0501: V2Table = V2Table {
    number: 501,
    metadata: &super::metadata::TABLE_0501_METADATA,
    rows: phf_map! {
        "E" => V2TableRow { value: "E", display_name: "Emergency", definition: "", comment_usage_note: "", status: "" },
        "RX" => V2TableRow { value: "RX", display_name: "Rx Private", definition: "", comment_usage_note: "", status: "" },
        "PR" => V2TableRow { value: "PR", display_name: "Patient Request", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0502: V2Table = V2Table {
    number: 502,
    metadata: &super::metadata::TABLE_0502_METADATA,
    rows: phf_map! {
        "MIN" => V2TableRow { value: "MIN", display_name: "Subject is a minor", definition: "", comment_usage_note: "", status: "" },
        "NC" => V2TableRow { value: "NC", display_name: "Subject is not competent to consent", definition: "", comment_usage_note: "", status: "" },
        "LM" => V2TableRow { value: "LM", display_name: "Legally mandated", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0503: V2Table = V2Table {
    number: 503,
    metadata: &super::metadata::TABLE_0503_METADATA,
    rows: phf_map! {
        "S" => V2TableRow { value: "S", display_name: "Sequential", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Cyclical", definition: "", comment_usage_note: "Used for indicating a repeating cycle of service requests; for example, individual intravenous solutions used in a cyclical sequence (a.k.a. \"Alternating IVs\"). This value would be compatible with linking separate service requests or with having all cyclical service request components in a single service request. Likewise, the value would be compatible with either Parent-Child messages or a single service request message to communicate the service requests' sequencing", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Reserved for future use", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0504: V2Table = V2Table {
    number: 504,
    metadata: &super::metadata::TABLE_0504_METADATA,
    rows: phf_map! {
        "EE" => V2TableRow { value: "EE", display_name: "End related service request(s), end current service request.", definition: "", comment_usage_note: "", status: "" },
        "ES" => V2TableRow { value: "ES", display_name: "End related service request(s), start current service request.", definition: "", comment_usage_note: "", status: "" },
        "SS" => V2TableRow { value: "SS", display_name: "Start related service request(s), start current service request.", definition: "", comment_usage_note: "", status: "" },
        "SE" => V2TableRow { value: "SE", display_name: "Start related service request(s), end current service request.", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0505: V2Table = V2Table {
    number: 505,
    metadata: &super::metadata::TABLE_0505_METADATA,
    rows: phf_map! {
        "F" => V2TableRow { value: "F", display_name: "First service", definition: "The first service request in a cyclic group", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Last service", definition: "The last service request in a cyclic group", comment_usage_note: "", status: "" },
        "*" => V2TableRow { value: "*", display_name: "The first service request in a cyclic group", definition: "", comment_usage_note: "Usage Note: Retired March 2016. Use code F instead", status: "R" },
        "#" => V2TableRow { value: "#", display_name: "The last service request in a cyclic group.", definition: "", comment_usage_note: "Usage Note: Retired March 2016. Use code L instead", status: "R" },
    },
};

pub static TABLE_0506: V2Table = V2Table {
    number: 506,
    metadata: &super::metadata::TABLE_0506_METADATA,
    rows: phf_map! {
        "N" => V2TableRow { value: "N", display_name: "Nurse prerogative", definition: "", comment_usage_note: "Where a set of two or more orders exist and the Nurse, or other caregiver, has the prerogative to choose which order will be administered at a particular point in time. For example, Milk of Magnesia PO 30 ml q Dulcolax Supp R @ hs prn Colace 100 mg capsule PO bi The nurse would be administering MOM, but may add the Colace and may also give the Dulcolax Supp as needed to promote and maintain regularity.", status: "hs (at bedtime) d" },
        "C" => V2TableRow { value: "C", display_name: "Compound", definition: "", comment_usage_note: "A compound is an extempo order which may be made up of multiple drugs. For example, many hospitals have a standard item called \"Magic Mouthwash\". The item is ordered that way by the physician. The extempo items will contain multiple products, such as Maalox, Benadryl, Xylocaine, etc. They will all be mixed together and will be dispensed in a single container.", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Tapering", definition: "", comment_usage_note: "A tapering order is one in which the same drug is used, but it has a declining dosage over a number of days. For example, Decadron 0.5 mg is often ordered this way. The order would look like this: Decadron 0.5 m for 2 days, then Decadron 0.5 m for 2 days, then Decadro days, then Decadron 0.5 m then stop.", status: "g qid (four times a day) g tid (three times a day) n 0.5 mg bid (twice a day) for 2 g qd (daily) for 2 days," },
        "E" => V2TableRow { value: "E", display_name: "Exclusive", definition: "", comment_usage_note: "An exclusive order is an order where only one of the multiple items should be administered at any one dosage time. The nurse may chose between the alternatives, but should only give ONE of them. An example would be: Phenergan 25 mg PO, IM or R q6h prn (orally, intramuscularly, or rectally every 6 hours as needed).", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Simultaneous", definition: "Ph ne", comment_usage_note: "A simultaneous order is 2 or more drugs which are ordered to be given at the same time. A common example of this would be Demerol and Phenergan (Phenergan is given with the Demerol to control the nausea that Demerol can cause). The order could be: Demerol 50 mg IM with energan 25 mg IM q4h prn (every 4 hours as eded).", status: "" },
    },
};

pub static TABLE_0507: V2Table = V2Table {
    number: 507,
    metadata: &super::metadata::TABLE_0507_METADATA,
    rows: phf_map! {
        "F" => V2TableRow { value: "F", display_name: "Film-with-patient", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Notify provider when ready", definition: "", comment_usage_note: "", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Alert provider when abnormal", definition: "", comment_usage_note: "", status: "" },
        "CC" => V2TableRow { value: "CC", display_name: "Copies requested", definition: "", comment_usage_note: "", status: "" },
        "BCC" => V2TableRow { value: "BCC", display_name: "Blind copy", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0508: V2Table = V2Table {
    number: 508,
    metadata: &super::metadata::TABLE_0508_METADATA,
    rows: phf_map! {
        "LR" => V2TableRow { value: "LR", display_name: "Leukoreduced", definition: "", comment_usage_note: "", status: "" },
        "IR" => V2TableRow { value: "IR", display_name: "Irradiated", definition: "", comment_usage_note: "", status: "" },
        "CS" => V2TableRow { value: "CS", display_name: "CMV Safe", definition: "", comment_usage_note: "", status: "" },
        "FR" => V2TableRow { value: "FR", display_name: "Fresh unit", definition: "", comment_usage_note: "", status: "" },
        "AU" => V2TableRow { value: "AU", display_name: "Autologous Unit", definition: "", comment_usage_note: "", status: "" },
        "DI" => V2TableRow { value: "DI", display_name: "Directed Unit", definition: "", comment_usage_note: "", status: "" },
        "HL" => V2TableRow { value: "HL", display_name: "HLA Matched", definition: "", comment_usage_note: "", status: "" },
        "CM" => V2TableRow { value: "CM", display_name: "CMV Negative", definition: "", comment_usage_note: "", status: "" },
        "HB" => V2TableRow { value: "HB", display_name: "Hemoglobin S Negative", definition: "", comment_usage_note: "", status: "" },
        "WA" => V2TableRow { value: "WA", display_name: "Washed", definition: "", comment_usage_note: "", status: "" },
        "IG" => V2TableRow { value: "IG", display_name: "IgA Deficient", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0510: V2Table = V2Table {
    number: 510,
    metadata: &super::metadata::TABLE_0510_METADATA,
    rows: phf_map! {
        "RI" => V2TableRow { value: "RI", display_name: "Received into inventory (for specified patient)", definition: "", comment_usage_note: "Status determined by Filler", status: "" },
        "RD" => V2TableRow { value: "RD", display_name: "Reserved and ready to dispense", definition: "", comment_usage_note: "Status determined by Filler", status: "" },
        "RS" => V2TableRow { value: "RS", display_name: "Reserved (ordered and product allocated for the patient)", definition: "", comment_usage_note: "Status determined by Filler", status: "" },
        "RE" => V2TableRow { value: "RE", display_name: "Released (no longer allocated for the patient)", definition: "", comment_usage_note: "Status determined by Placer or Filler", status: "" },
        "DS" => V2TableRow { value: "DS", display_name: "Dispensed to patient location", definition: "", comment_usage_note: "Status determined by Filler", status: "" },
        "RA" => V2TableRow { value: "RA", display_name: "Returned unused/no longer needed", definition: "", comment_usage_note: "Status determined by Filler", status: "" },
        "RL" => V2TableRow { value: "RL", display_name: "Returned unused/keep linked to patient for possible use later", definition: "", comment_usage_note: "Status determined by Filler", status: "" },
        "WA" => V2TableRow { value: "WA", display_name: "Wasted (product no longer viable)", definition: "", comment_usage_note: "Status determined by Filler", status: "" },
        "PT" => V2TableRow { value: "PT", display_name: "Presumed transfused (dispensed and not returned)", definition: "", comment_usage_note: "Status determined by Filler", status: "" },
        "CR" => V2TableRow { value: "CR", display_name: "Released into inventory for general availability", definition: "", comment_usage_note: "Status determined by Filler", status: "" },
        "RQ" => V2TableRow { value: "RQ", display_name: "Request to dispense blood product", definition: "", comment_usage_note: "Status determined by Placer", status: "" },
    },
};

pub static TABLE_0511: V2Table = V2Table {
    number: 511,
    metadata: &super::metadata::TABLE_0511_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "Record coming over is a correction and thus replaces a final status", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Deletes the BPX record", definition: "", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Final status; Can only be changed with a corrected status", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Order detail description only (no status)", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Preliminary status", definition: "", comment_usage_note: "", status: "" },
        "W" => V2TableRow { value: "W", display_name: "Post original as wrong, e.g., transmitted for wrong patient", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0513: V2Table = V2Table {
    number: 513,
    metadata: &super::metadata::TABLE_0513_METADATA,
    rows: phf_map! {
        "RA" => V2TableRow { value: "RA", display_name: "Returned unused and unlinked", definition: "Blood product was returned unused, because it is no longer needed.", comment_usage_note: "", status: "" },
        "RL" => V2TableRow { value: "RL", display_name: "Returned unused but linked", definition: "Blood product was returned unused, because it is not curren needed, but should remain linked to the patient for futu use.", comment_usage_note: "tly re", status: "" },
        "WA" => V2TableRow { value: "WA", display_name: "Wasted", definition: "The blood product is no longer viable.", comment_usage_note: "", status: "" },
        "TI" => V2TableRow { value: "TI", display_name: "Transfusion Interrupted", definition: "Transfusion of the blood produc was interrupted and considered ended; a reason for interruption usually also reported.", comment_usage_note: "t is", status: "This is not N expected to be an end state; transfusion will either be ended or restarted." },
        "TR" => V2TableRow { value: "TR", display_name: "Transfusion Ended with Reactions", definition: "The blood product has been transfused and i caused an advers reaction.", comment_usage_note: "t e", status: "" },
        "TS" => V2TableRow { value: "TS", display_name: "Transfusion Started", definition: "Transfusion of the blood produc has been started and is in progre", comment_usage_note: "t ss", status: "This code N may be used for restart if interrupted as well." },
        "TX" => V2TableRow { value: "TX", display_name: "Transfusion Ended", definition: "The blood product has been transfused with normal end to th transfusion.", comment_usage_note: "a e", status: "" },
    },
};

pub static TABLE_0514: V2Table = V2Table {
    number: 514,
    metadata: &super::metadata::TABLE_0514_METADATA,
    rows: phf_map! {
        "ABOINC" => V2TableRow { value: "ABOINC", display_name: "ABO Incompatible Transfusion Reaction", definition: "", comment_usage_note: "", status: "" },
        "ACUTHEH" => V2TableRow { value: "ACUTHEH", display_name: "Acute Hemolytic Transfusion Reaction", definition: "", comment_usage_note: "", status: "" },
        "TR" => V2TableRow { value: "TR", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ALLERGIC" => V2TableRow { value: "ALLERGIC", display_name: "Allergic Reaction - First", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "ANAPHYL" => V2TableRow { value: "ANAPHYL", display_name: "Anaphylactic Reaction", definition: "", comment_usage_note: "", status: "" },
        "AC" => V2TableRow { value: "AC", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "BACT" => V2TableRow { value: "BACT", display_name: "CON Reaction to Bacterial Contamination", definition: "", comment_usage_note: "", status: "" },
        "TAM" => V2TableRow { value: "TAM", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "DELAYED" => V2TableRow { value: "DELAYED", display_name: "Delayed Hemolytic Transfusion Reaction", definition: "", comment_usage_note: "", status: "" },
        "HTR" => V2TableRow { value: "HTR", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "DELAYEDS" => V2TableRow { value: "DELAYEDS", display_name: "Delayed Serological Transfusion Reaction", definition: "", comment_usage_note: "", status: "" },
        "GVHD" => V2TableRow { value: "GVHD", display_name: "Graft vs Host Disease - Transfusion - Associated", definition: "", comment_usage_note: "", status: "" },
        "HYPOTENS" => V2TableRow { value: "HYPOTENS", display_name: "Non-hemolytic Hypotensive Reaction", definition: "", comment_usage_note: "", status: "" },
        "NONHTR1" => V2TableRow { value: "NONHTR1", display_name: "Non-Hemolytic Fever Chill Transfusion Reaction - First", definition: "", comment_usage_note: "", status: "" },
        "NONHTR2" => V2TableRow { value: "NONHTR2", display_name: "Non-Hemolytic Fever Chill Transfusion Reaction - Recurrent", definition: "", comment_usage_note: "", status: "" },
        "NONHTRR" => V2TableRow { value: "NONHTRR", display_name: "Non-Hemolytic Fever Chill Transfusion Reaction -", definition: "", comment_usage_note: "", status: "" },
        "EC" => V2TableRow { value: "EC", display_name: "Repeating", definition: "", comment_usage_note: "", status: "" },
        "NONIMMU" => V2TableRow { value: "NONIMMU", display_name: "Non-Immune Hemolysis", definition: "", comment_usage_note: "", status: "" },
        "NE" => V2TableRow { value: "NE", display_name: "", definition: "", comment_usage_note: "", status: "" },
        "NONSPE" => V2TableRow { value: "NONSPE", display_name: "C Non-Specific, Non-Hemolytic Transfusion Reaction", definition: "", comment_usage_note: "", status: "" },
        "NORXN" => V2TableRow { value: "NORXN", display_name: "No Evidence of Transfusion Reaction", definition: "", comment_usage_note: "", status: "" },
        "PTP" => V2TableRow { value: "PTP", display_name: "Posttransfusion Purpura", definition: "", comment_usage_note: "", status: "" },
        "VOLOVER" => V2TableRow { value: "VOLOVER", display_name: "Symptoms most likely due to volume overload", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0516: V2Table = V2Table {
    number: 516,
    metadata: &super::metadata::TABLE_0516_METADATA,
    rows: phf_map! {
        "W" => V2TableRow { value: "W", display_name: "Warning", definition: "Transaction successful, but there may be issues", comment_usage_note: "Use this severity when parts of the message may not have been stored.", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Information", definition: "Transaction was successful but includes information", comment_usage_note: "e.g., inform patient", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Error", definition: "Transaction was unsuccessful", comment_usage_note: "", status: "" },
        "F" => V2TableRow { value: "F", display_name: "Fatal Error", definition: "Message not processed due to application or network failure condition", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0517: V2Table = V2Table {
    number: 517,
    metadata: &super::metadata::TABLE_0517_METADATA,
    rows: phf_map! {
        "PAT" => V2TableRow { value: "PAT", display_name: "Inform patient", definition: "", comment_usage_note: "", status: "" },
        "NPAT" => V2TableRow { value: "NPAT", display_name: "Do NOT inform patient", definition: "", comment_usage_note: "", status: "" },
        "USR" => V2TableRow { value: "USR", display_name: "Inform User", definition: "", comment_usage_note: "", status: "" },
        "HD" => V2TableRow { value: "HD", display_name: "Inform help desk", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0518: V2Table = V2Table {
    number: 518,
    metadata: &super::metadata::TABLE_0518_METADATA,
    rows: phf_map! {
        "EXTN" => V2TableRow { value: "EXTN", display_name: "Extension Override", definition: "", comment_usage_note: "Identifies an override where a service is being performed for longer than the ordered period of time.", status: "" },
        "INLV" => V2TableRow { value: "INLV", display_name: "Interval Override", definition: "", comment_usage_note: "Identifies an override where a repetition of service is being performed sooner than the ordered frequency.", status: "" },
        "EQV" => V2TableRow { value: "EQV", display_name: "Equivalence Override", definition: "", comment_usage_note: "Identifies an override where a service is being performed against an order that the system does not recognize as equivalent to the ordered service.", status: "" },
    },
};

pub static TABLE_0520: V2Table = V2Table {
    number: 520,
    metadata: &super::metadata::TABLE_0520_METADATA,
    rows: phf_map! {
        "H" => V2TableRow { value: "H", display_name: "High", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Medium", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Low", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0523: V2Table = V2Table {
    number: 523,
    metadata: &super::metadata::TABLE_0523_METADATA,
    rows: phf_map! {
        "%" => V2TableRow { value: "%", display_name: "Indicates a percent change", definition: "", comment_usage_note: "", status: "" },
        "a" => V2TableRow { value: "a", display_name: "Absolute Change", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0527: V2Table = V2Table {
    number: 527,
    metadata: &super::metadata::TABLE_0527_METADATA,
    rows: phf_map! {
        "MY" => V2TableRow { value: "MY", display_name: "month of the year", definition: "", comment_usage_note: "", status: "" },
        "WY" => V2TableRow { value: "WY", display_name: "week of the year", definition: "", comment_usage_note: "", status: "" },
        "DM" => V2TableRow { value: "DM", display_name: "day of the month", definition: "", comment_usage_note: "", status: "" },
        "DY" => V2TableRow { value: "DY", display_name: "day of the year", definition: "", comment_usage_note: "", status: "" },
        "DW" => V2TableRow { value: "DW", display_name: "day of the week (begins with Monday)", definition: "", comment_usage_note: "", status: "" },
        "HD" => V2TableRow { value: "HD", display_name: "hour of the day", definition: "", comment_usage_note: "", status: "" },
        "NH" => V2TableRow { value: "NH", display_name: "minute of the hour", definition: "", comment_usage_note: "", status: "" },
        "SN" => V2TableRow { value: "SN", display_name: "second of the minute", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0528: V2Table = V2Table {
    number: 528,
    metadata: &super::metadata::TABLE_0528_METADATA,
    rows: phf_map! {
        "HS" => V2TableRow { value: "HS", display_name: "the hour of sleep (e.g., H18-22)", definition: "", comment_usage_note: "", status: "" },
        "AC" => V2TableRow { value: "AC", display_name: "before meal (from lat. ante cibus)", definition: "", comment_usage_note: "", status: "" },
        "PC" => V2TableRow { value: "PC", display_name: "after meal (from lat. post cibus)", definition: "", comment_usage_note: "", status: "" },
        "IC" => V2TableRow { value: "IC", display_name: "between meals (from lat. inter cibus)", definition: "", comment_usage_note: "", status: "" },
        "ACM" => V2TableRow { value: "ACM", display_name: "before breakfast (from lat. ante cibus matutinus)", definition: "", comment_usage_note: "", status: "" },
        "ACD" => V2TableRow { value: "ACD", display_name: "before lunch (from lat. ante cibus diurnus)", definition: "", comment_usage_note: "", status: "" },
        "ACV" => V2TableRow { value: "ACV", display_name: "before dinner (from lat. ante cibus vespertinus)", definition: "", comment_usage_note: "", status: "" },
        "PCM" => V2TableRow { value: "PCM", display_name: "after breakfast (from lat. post cibus matutinus)", definition: "", comment_usage_note: "", status: "" },
        "PCD" => V2TableRow { value: "PCD", display_name: "after lunch (from lat. post cibus diurnus)", definition: "", comment_usage_note: "", status: "" },
        "PCV" => V2TableRow { value: "PCV", display_name: "after dinner (from lat. post cibus vespertinus)", definition: "", comment_usage_note: "", status: "" },
        "ICM" => V2TableRow { value: "ICM", display_name: "between breakfast and lunch", definition: "", comment_usage_note: "", status: "" },
        "ICD" => V2TableRow { value: "ICD", display_name: "between lunch and dinner", definition: "", comment_usage_note: "", status: "" },
        "ICV" => V2TableRow { value: "ICV", display_name: "between dinner and the hour of sleep", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0530: V2Table = V2Table {
    number: 530,
    metadata: &super::metadata::TABLE_0530_METADATA,
    rows: phf_map! {
        "AE" => V2TableRow { value: "AE", display_name: "American Express", definition: "", comment_usage_note: "", status: "" },
        "DEA" => V2TableRow { value: "DEA", display_name: "Drug Enforcement Agency", definition: "DEA nu DEA nu hospit Thus, necess Type.", comment_usage_note: "The US Drug Enforcement Administration does not solely assign DEA numbers in the United States. Hospitals have the authority to issue DEA numbers to their medical residents. These mbers are based upon the hospital’s mber, but the authority rests with the al on the assignment to the residents. DEA as an Assigning Authority is ary in addition to DEA as an Identifier", status: "" },
        "DOD" => V2TableRow { value: "DOD", display_name: "Department of Defense", definition: "In som depart Hence, Author", comment_usage_note: "e countries e.g., the US, more than one ment may issue a military identifier. US is not sufficient as the Assigning ity.", status: "" },
        "MC" => V2TableRow { value: "MC", display_name: "Master Card", definition: "", comment_usage_note: "", status: "" },
        "VA" => V2TableRow { value: "VA", display_name: "Veterans Affairs", definition: "", comment_usage_note: "", status: "" },
        "VI" => V2TableRow { value: "VI", display_name: "Visa", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0532: V2Table = V2Table {
    number: 532,
    metadata: &super::metadata::TABLE_0532_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Yes", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No", definition: "", comment_usage_note: "", status: "" },
        "NI" => V2TableRow { value: "NI", display_name: "No Information", definition: "", comment_usage_note: "No information whatsoever can be inferred from this exceptional value. This is the most general exceptional value. It is also the default exceptional value", status: "" },
        "NA" => V2TableRow { value: "NA", display_name: "not applicable", definition: "", comment_usage_note: "No proper value is applicable in this context (e.g., last menstrual period for a male)", status: "" },
        "UNK" => V2TableRow { value: "UNK", display_name: "unknown", definition: "", comment_usage_note: "A proper value is applicable, but not known", status: "" },
        "NASK" => V2TableRow { value: "NASK", display_name: "not asked", definition: "", comment_usage_note: "This information has not been sought (e.g., patient was not asked", status: "" },
        "ASKU" => V2TableRow { value: "ASKU", display_name: "asked but unknown", definition: "", comment_usage_note: "Information was sought but not found (e.g., patient was asked but didn't know", status: "" },
        "NAV" => V2TableRow { value: "NAV", display_name: "temporarily unavailable", definition: "", comment_usage_note: "Information is not available at this time but it is expected that it will be available later", status: "" },
        "NP" => V2TableRow { value: "NP", display_name: "not present", definition: "", comment_usage_note: "Obsolete as of v 2.7.", status: "" },
    },
};

pub static TABLE_0534: V2Table = V2Table {
    number: 534,
    metadata: &super::metadata::TABLE_0534_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "Yes", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Last Rite s only", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Other", definition: "", comment_usage_note: "", status: "" },
        "U" => V2TableRow { value: "U", display_name: "Unknown", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0535: V2Table = V2Table {
    number: 535,
    metadata: &super::metadata::TABLE_0535_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "Signed CMS-1500 claim form on file, e.g., authorization for release of any medical or other information necessary to process this claim and assignment of benefits.", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Signed authorization for release of any medical or other information necessary to process this claim on file.", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Signed authorization for assignment of benefits on file.", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Signature generated by provider because the patient was not physically present for services.", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0536: V2Table = V2Table {
    number: 536,
    metadata: &super::metadata::TABLE_0536_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Provisional", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Revoked", definition: "", comment_usage_note: "", status: "" },
        "V" => V2TableRow { value: "V", display_name: "Active/Valid", definition: "", comment_usage_note: "", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Expired", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Inactive", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0538: V2Table = V2Table {
    number: 538,
    metadata: &super::metadata::TABLE_0538_METADATA,
    rows: phf_map! {
        "EMP" => V2TableRow { value: "EMP", display_name: "Employee", definition: "", comment_usage_note: "", status: "" },
        "VOL" => V2TableRow { value: "VOL", display_name: "Volunteer", definition: "", comment_usage_note: "", status: "" },
        "CON" => V2TableRow { value: "CON", display_name: "Contractor", definition: "", comment_usage_note: "", status: "" },
        "CST" => V2TableRow { value: "CST", display_name: "Consultant", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0540: V2Table = V2Table {
    number: 540,
    metadata: &super::metadata::TABLE_0540_METADATA,
    rows: phf_map! {
        "L" => V2TableRow { value: "L", display_name: "Leave of Absence", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Termination", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Retired", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0544: V2Table = V2Table {
    number: 544,
    metadata: &super::metadata::TABLE_0544_METADATA,
    rows: phf_map! {
        "XC37" => V2TableRow { value: "XC37", display_name: "Not Body temperature", definition: "", comment_usage_note: "Failed to keep at body temperature: 36 - 38 degrees C.", status: "" },
        "XAMB" => V2TableRow { value: "XAMB", display_name: "Not Ambient temperature", definition: "", comment_usage_note: "Failed to keep at ambient (room) temperature, approximately 22 +/- 2 degrees C. Accidental refrigeration or freezing is of little consequence", status: "" },
        "XCAM" => V2TableRow { value: "XCAM", display_name: "B Not Critical ambient temperature", definition: "", comment_usage_note: "Failed to keep critical ambient.", status: "" },
        "XREF" => V2TableRow { value: "XREF", display_name: "Not Refrigerated temperature", definition: "", comment_usage_note: "Failed to keep at refrigerated temperature: 4-8 degrees C.", status: "" },
        "XCRE" => V2TableRow { value: "XCRE", display_name: "F Not Critical refrigerated temperature", definition: "", comment_usage_note: "Failed to keep critical refrigerated.", status: "" },
        "XFRZ" => V2TableRow { value: "XFRZ", display_name: "Not Frozen temperature", definition: "", comment_usage_note: "Failed to keep at frozen temperature: -4 degrees C.", status: "" },
        "XCF" => V2TableRow { value: "XCF", display_name: "RZ Not Critical frozen temperature", definition: "", comment_usage_note: "Failed to keep critical frozen", status: "" },
        "XDFRZ" => V2TableRow { value: "XDFRZ", display_name: "Not Deep frozen", definition: "", comment_usage_note: "Failed to keep deep frozen: -16 to -20 degree C.", status: "" },
        "XUFRZ" => V2TableRow { value: "XUFRZ", display_name: "Not Ultra frozen", definition: "", comment_usage_note: "Failed to keep ultra cold frozen: ~ -75 to -85 degree C. (ultra cold freezer is typically at temperature of dry ice).", status: "" },
        "XNTR" => V2TableRow { value: "XNTR", display_name: "Not Liquid nitrogen", definition: "", comment_usage_note: "Failed to keep in liquid nitrogen.", status: "" },
        "XPRTL" => V2TableRow { value: "XPRTL", display_name: "Not Protected from light", definition: "", comment_usage_note: "Failed to protect from light.", status: "" },
        "XCATM" => V2TableRow { value: "XCATM", display_name: "Exposed to Air", definition: "", comment_usage_note: "Exposed to atmosphere.", status: "" },
        "XDRY" => V2TableRow { value: "XDRY", display_name: "Not Dry", definition: "", comment_usage_note: "Failed to keep in a dry environment.", status: "" },
        "XPSO" => V2TableRow { value: "XPSO", display_name: "Exposed to shock", definition: "", comment_usage_note: "Failed to protect from shock.", status: "" },
        "XPSA" => V2TableRow { value: "XPSA", display_name: "Shaken", definition: "", comment_usage_note: "Shaken.", status: "" },
        "XUPR" => V2TableRow { value: "XUPR", display_name: "Not Upright", definition: "", comment_usage_note: "Failed to keep upright.", status: "" },
        "XMTLF" => V2TableRow { value: "XMTLF", display_name: "Metal Exposed", definition: "", comment_usage_note: "Failed to keep container is free of heavy metals.", status: "" },
        "SB" => V2TableRow { value: "SB", display_name: "Seal Broken", definition: "", comment_usage_note: "Container seal is broken.", status: "" },
        "CC" => V2TableRow { value: "CC", display_name: "Container Cracked", definition: "", comment_usage_note: "Container is cracked.", status: "" },
        "CT" => V2TableRow { value: "CT", display_name: "Container Torn", definition: "", comment_usage_note: "Container is torn", status: "" },
        "CL" => V2TableRow { value: "CL", display_name: "Container Leaking", definition: "", comment_usage_note: "Container is leaking", status: "" },
    },
};

pub static TABLE_0547: V2Table = V2Table {
    number: 547,
    metadata: &super::metadata::TABLE_0547_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "County/Parish", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "State/Province", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Country", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0548: V2Table = V2Table {
    number: 548,
    metadata: &super::metadata::TABLE_0548_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Self", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Parent", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Next of Kin", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Durable Power of Attorney in Healthcare Affairs", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Conservator", definition: "", comment_usage_note: "", status: "" },
        "6" => V2TableRow { value: "6", display_name: "Emergent Practitioner (practitioner judging case as emergency requiring care without a consent)", definition: "", comment_usage_note: "", status: "" },
        "7" => V2TableRow { value: "7", display_name: "Non-Emergent Practitioner (i.e. medical ethics committee)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0550: V2Table = V2Table {
    number: 550,
    metadata: &super::metadata::TABLE_0550_METADATA,
    rows: phf_map! {
        "JUGE" => V2TableRow { value: "JUGE", display_name: "Jugular, External", definition: "", comment_usage_note: "", status: "" },
        "ADB" => V2TableRow { value: "ADB", display_name: "Abdomen", definition: "", comment_usage_note: "", status: "" },
        "ACET" => V2TableRow { value: "ACET", display_name: "Acetabulum", definition: "", comment_usage_note: "", status: "" },
        "ACHIL" => V2TableRow { value: "ACHIL", display_name: "Achilles", definition: "", comment_usage_note: "", status: "" },
        "ADE" => V2TableRow { value: "ADE", display_name: "Adenoids", definition: "", comment_usage_note: "", status: "" },
        "ADR" => V2TableRow { value: "ADR", display_name: "Adrenal", definition: "", comment_usage_note: "", status: "" },
        "AMN" => V2TableRow { value: "AMN", display_name: "Amniotic fluid", definition: "", comment_usage_note: "", status: "" },
        "AMS" => V2TableRow { value: "AMS", display_name: "Amniotic Sac", definition: "", comment_usage_note: "", status: "" },
        "ANAL" => V2TableRow { value: "ANAL", display_name: "Anal", definition: "", comment_usage_note: "", status: "" },
        "ANKL" => V2TableRow { value: "ANKL", display_name: "Ankle", definition: "", comment_usage_note: "", status: "" },
        "ANTEC" => V2TableRow { value: "ANTEC", display_name: "Antecubital", definition: "", comment_usage_note: "", status: "" },
        "ANTECF" => V2TableRow { value: "ANTECF", display_name: "Antecubital Fossa", definition: "", comment_usage_note: "", status: "" },
        "ANTR" => V2TableRow { value: "ANTR", display_name: "Antrum", definition: "", comment_usage_note: "", status: "" },
        "ANUS" => V2TableRow { value: "ANUS", display_name: "Anus", definition: "", comment_usage_note: "", status: "" },
        "AORTA" => V2TableRow { value: "AORTA", display_name: "Aorta", definition: "", comment_usage_note: "", status: "" },
        "AR" => V2TableRow { value: "AR", display_name: "Aortic Rim", definition: "", comment_usage_note: "", status: "" },
        "AV" => V2TableRow { value: "AV", display_name: "Aortic Valve", definition: "", comment_usage_note: "", status: "" },
        "APDX" => V2TableRow { value: "APDX", display_name: "Appendix", definition: "", comment_usage_note: "", status: "" },
        "AREO" => V2TableRow { value: "AREO", display_name: "Areola", definition: "", comment_usage_note: "", status: "" },
        "ARM" => V2TableRow { value: "ARM", display_name: "Arm", definition: "", comment_usage_note: "", status: "" },
        "ARTE" => V2TableRow { value: "ARTE", display_name: "Artery", definition: "", comment_usage_note: "", status: "" },
        "ASCIT" => V2TableRow { value: "ASCIT", display_name: "Ascites", definition: "", comment_usage_note: "", status: "" },
        "ASCT" => V2TableRow { value: "ASCT", display_name: "Ascitic Fluid", definition: "", comment_usage_note: "", status: "" },
        "ATR" => V2TableRow { value: "ATR", display_name: "Atrium", definition: "", comment_usage_note: "", status: "" },
        "AURI" => V2TableRow { value: "AURI", display_name: "Auricular", definition: "", comment_usage_note: "", status: "" },
        "AXI" => V2TableRow { value: "AXI", display_name: "Axilla", definition: "", comment_usage_note: "", status: "" },
        "BACK" => V2TableRow { value: "BACK", display_name: "Back", definition: "", comment_usage_note: "", status: "" },
        "BARTD" => V2TableRow { value: "BARTD", display_name: "Bartholin Duct", definition: "", comment_usage_note: "", status: "" },
        "BARTG" => V2TableRow { value: "BARTG", display_name: "Bartholin Gland", definition: "", comment_usage_note: "", status: "" },
        "BRTG" => V2TableRow { value: "BRTG", display_name: "F Bartholin Gland Fluid", definition: "", comment_usage_note: "", status: "" },
        "BPH" => V2TableRow { value: "BPH", display_name: "Basophils", definition: "", comment_usage_note: "", status: "" },
        "BID" => V2TableRow { value: "BID", display_name: "Bile Duct", definition: "", comment_usage_note: "", status: "" },
        "BIFL" => V2TableRow { value: "BIFL", display_name: "Bile fluid", definition: "", comment_usage_note: "", status: "" },
        "BLAD" => V2TableRow { value: "BLAD", display_name: "Bladder", definition: "", comment_usage_note: "", status: "" },
        "BLOOD" => V2TableRow { value: "BLOOD", display_name: "Blood", definition: "", comment_usage_note: "", status: "" },
        "BLDA" => V2TableRow { value: "BLDA", display_name: "Blood, Arterial", definition: "", comment_usage_note: "", status: "" },
        "BLDC" => V2TableRow { value: "BLDC", display_name: "Blood, Capillary", definition: "", comment_usage_note: "", status: "" },
        "BLDV" => V2TableRow { value: "BLDV", display_name: "Blood, Venous", definition: "", comment_usage_note: "", status: "" },
        "CBLD" => V2TableRow { value: "CBLD", display_name: "Blood, Cord", definition: "", comment_usage_note: "", status: "" },
        "BLD" => V2TableRow { value: "BLD", display_name: "Blood, Whole", definition: "", comment_usage_note: "", status: "" },
        "BDY" => V2TableRow { value: "BDY", display_name: "Body, Whole", definition: "", comment_usage_note: "", status: "" },
        "BON" => V2TableRow { value: "BON", display_name: "Bone", definition: "", comment_usage_note: "", status: "" },
        "BMAR" => V2TableRow { value: "BMAR", display_name: "Bone marrow", definition: "", comment_usage_note: "", status: "" },
        "BOWEL" => V2TableRow { value: "BOWEL", display_name: "Bowel", definition: "", comment_usage_note: "", status: "" },
        "BOWL" => V2TableRow { value: "BOWL", display_name: "A Bowel, Large", definition: "", comment_usage_note: "", status: "" },
        "BOWSM" => V2TableRow { value: "BOWSM", display_name: "Bowel, Small", definition: "", comment_usage_note: "", status: "" },
        "BRA" => V2TableRow { value: "BRA", display_name: "Brachial", definition: "", comment_usage_note: "", status: "" },
        "BRAIN" => V2TableRow { value: "BRAIN", display_name: "Brain", definition: "", comment_usage_note: "", status: "" },
        "BCY" => V2TableRow { value: "BCY", display_name: "S Brain Cyst Fluid", definition: "", comment_usage_note: "", status: "" },
        "BRST" => V2TableRow { value: "BRST", display_name: "Breast", definition: "", comment_usage_note: "", status: "" },
        "BRSTFL" => V2TableRow { value: "BRSTFL", display_name: "Breast fluid", definition: "", comment_usage_note: "", status: "" },
        "BRO" => V2TableRow { value: "BRO", display_name: "Bronchial", definition: "", comment_usage_note: "", status: "" },
        "BRV" => V2TableRow { value: "BRV", display_name: "Broviac", definition: "", comment_usage_note: "", status: "" },
        "BUCCA" => V2TableRow { value: "BUCCA", display_name: "Buccal", definition: "", comment_usage_note: "", status: "" },
        "BURSA" => V2TableRow { value: "BURSA", display_name: "Bursa", definition: "", comment_usage_note: "", status: "" },
        "BURSF" => V2TableRow { value: "BURSF", display_name: "Bursa Fluid", definition: "", comment_usage_note: "", status: "" },
        "BUTT" => V2TableRow { value: "BUTT", display_name: "Buttocks", definition: "", comment_usage_note: "", status: "" },
        "CALF" => V2TableRow { value: "CALF", display_name: "Calf", definition: "", comment_usage_note: "", status: "" },
        "CANAL" => V2TableRow { value: "CANAL", display_name: "Canal", definition: "", comment_usage_note: "", status: "" },
        "CANLI" => V2TableRow { value: "CANLI", display_name: "Canaliculis", definition: "", comment_usage_note: "", status: "" },
        "CNL" => V2TableRow { value: "CNL", display_name: "Cannula", definition: "", comment_usage_note: "", status: "" },
        "CANTH" => V2TableRow { value: "CANTH", display_name: "Canthus", definition: "", comment_usage_note: "", status: "" },
        "CDM" => V2TableRow { value: "CDM", display_name: "Cardiac Muscle", definition: "", comment_usage_note: "", status: "" },
        "CARO" => V2TableRow { value: "CARO", display_name: "Carotid", definition: "", comment_usage_note: "", status: "" },
        "CARP" => V2TableRow { value: "CARP", display_name: "Carpal", definition: "", comment_usage_note: "", status: "" },
        "CAVIT" => V2TableRow { value: "CAVIT", display_name: "Cavity", definition: "", comment_usage_note: "", status: "" },
        "CHE" => V2TableRow { value: "CHE", display_name: "Cavity, Chest", definition: "", comment_usage_note: "", status: "" },
        "CECUM" => V2TableRow { value: "CECUM", display_name: "Cecum/Cecal", definition: "", comment_usage_note: "", status: "" },
        "CSF" => V2TableRow { value: "CSF", display_name: "Cerebral Spinal Fluid", definition: "", comment_usage_note: "", status: "" },
        "CVX" => V2TableRow { value: "CVX", display_name: "Cervix", definition: "", comment_usage_note: "", status: "" },
        "CERVUT" => V2TableRow { value: "CERVUT", display_name: "Cervix/Uterus", definition: "", comment_usage_note: "", status: "" },
        "CHEEK" => V2TableRow { value: "CHEEK", display_name: "Cheek", definition: "", comment_usage_note: "", status: "" },
        "CHES" => V2TableRow { value: "CHES", display_name: "Chest", definition: "", comment_usage_note: "", status: "" },
        "CHESTÂ" => V2TableRow { value: "CHESTÂ", display_name: "Chest Tube", definition: "", comment_usage_note: "", status: "" },
        "CHIN" => V2TableRow { value: "CHIN", display_name: "Chin", definition: "", comment_usage_note: "", status: "" },
        "CIRCU" => V2TableRow { value: "CIRCU", display_name: "Circumcision Site", definition: "", comment_usage_note: "", status: "" },
        "CLAVI" => V2TableRow { value: "CLAVI", display_name: "Clavicle/Clavicul ar", definition: "", comment_usage_note: "", status: "" },
        "CLITO" => V2TableRow { value: "CLITO", display_name: "Clitoral", definition: "", comment_usage_note: "", status: "" },
        "CLIT" => V2TableRow { value: "CLIT", display_name: "Clitoris", definition: "", comment_usage_note: "", status: "" },
        "COCCG" => V2TableRow { value: "COCCG", display_name: "Coccygeal", definition: "", comment_usage_note: "", status: "" },
        "COCCY" => V2TableRow { value: "COCCY", display_name: "Coccyx", definition: "", comment_usage_note: "", status: "" },
        "COLON" => V2TableRow { value: "COLON", display_name: "Colon", definition: "", comment_usage_note: "", status: "" },
        "COLOS" => V2TableRow { value: "COLOS", display_name: "Colostomy", definition: "", comment_usage_note: "", status: "" },
        "COS" => V2TableRow { value: "COS", display_name: "Colostomy Stoma", definition: "", comment_usage_note: "", status: "" },
        "CDU" => V2TableRow { value: "CDU", display_name: "CT Common Duct", definition: "", comment_usage_note: "", status: "" },
        "CONJ" => V2TableRow { value: "CONJ", display_name: "Conjunctiva", definition: "", comment_usage_note: "", status: "" },
        "CORAL" => V2TableRow { value: "CORAL", display_name: "Coral", definition: "", comment_usage_note: "", status: "" },
        "COR" => V2TableRow { value: "COR", display_name: "Cord", definition: "", comment_usage_note: "", status: "" },
        "CORD" => V2TableRow { value: "CORD", display_name: "Cord Blood", definition: "", comment_usage_note: "", status: "" },
        "CORN" => V2TableRow { value: "CORN", display_name: "Cornea", definition: "", comment_usage_note: "", status: "" },
        "CRA" => V2TableRow { value: "CRA", display_name: "NE Cranium, ethmoid", definition: "", comment_usage_note: "", status: "" },
        "CUBIT" => V2TableRow { value: "CUBIT", display_name: "Cubitus", definition: "", comment_usage_note: "", status: "" },
        "CUFF" => V2TableRow { value: "CUFF", display_name: "Cuff", definition: "", comment_usage_note: "", status: "" },
        "CULD" => V2TableRow { value: "CULD", display_name: "Cul De Sac", definition: "", comment_usage_note: "", status: "" },
        "CULDO" => V2TableRow { value: "CULDO", display_name: "Culdocentesis", definition: "", comment_usage_note: "", status: "" },
        "DELT" => V2TableRow { value: "DELT", display_name: "Deltoid", definition: "", comment_usage_note: "", status: "" },
        "DENTA" => V2TableRow { value: "DENTA", display_name: "Dental", definition: "", comment_usage_note: "", status: "" },
        "DEN" => V2TableRow { value: "DEN", display_name: "Dental Gingiva", definition: "", comment_usage_note: "", status: "" },
        "DIAF" => V2TableRow { value: "DIAF", display_name: "Dialysis Fluid", definition: "", comment_usage_note: "", status: "" },
        "DPH" => V2TableRow { value: "DPH", display_name: "Diaphragm", definition: "", comment_usage_note: "", status: "" },
        "DIGIT" => V2TableRow { value: "DIGIT", display_name: "Digit", definition: "", comment_usage_note: "", status: "" },
        "DISC" => V2TableRow { value: "DISC", display_name: "Disc", definition: "", comment_usage_note: "", status: "" },
        "DORS" => V2TableRow { value: "DORS", display_name: "Dorsum/Dorsal", definition: "", comment_usage_note: "", status: "" },
        "DUFL" => V2TableRow { value: "DUFL", display_name: "Duodenal Fluid", definition: "", comment_usage_note: "", status: "" },
        "DUODE" => V2TableRow { value: "DUODE", display_name: "Duodenum/Duod enal", definition: "", comment_usage_note: "", status: "" },
        "DUR" => V2TableRow { value: "DUR", display_name: "Dura", definition: "", comment_usage_note: "", status: "" },
        "EAR" => V2TableRow { value: "EAR", display_name: "Ear", definition: "", comment_usage_note: "", status: "" },
        "EARBI" => V2TableRow { value: "EARBI", display_name: "Ear bone, incus", definition: "", comment_usage_note: "", status: "" },
        "EARBM" => V2TableRow { value: "EARBM", display_name: "Ear bone, malleus", definition: "", comment_usage_note: "", status: "" },
        "EARBS" => V2TableRow { value: "EARBS", display_name: "Ear bone,stapes", definition: "", comment_usage_note: "", status: "" },
        "EARLO" => V2TableRow { value: "EARLO", display_name: "Ear Lobe", definition: "", comment_usage_note: "", status: "" },
        "ELBOW" => V2TableRow { value: "ELBOW", display_name: "Elbow", definition: "", comment_usage_note: "", status: "" },
        "ELBOWJ" => V2TableRow { value: "ELBOWJ", display_name: "Elbow Joint", definition: "", comment_usage_note: "", status: "" },
        "ENDC" => V2TableRow { value: "ENDC", display_name: "Endocardium", definition: "", comment_usage_note: "", status: "" },
        "EC" => V2TableRow { value: "EC", display_name: "Endocervical", definition: "", comment_usage_note: "", status: "" },
        "EOLPH" => V2TableRow { value: "EOLPH", display_name: "endolpthamitis", definition: "", comment_usage_note: "", status: "" },
        "ENDM" => V2TableRow { value: "ENDM", display_name: "Endometrium", definition: "", comment_usage_note: "", status: "" },
        "ET" => V2TableRow { value: "ET", display_name: "Endotracheal", definition: "", comment_usage_note: "", status: "" },
        "EUR" => V2TableRow { value: "EUR", display_name: "Endourethral", definition: "", comment_usage_note: "", status: "" },
        "EOS" => V2TableRow { value: "EOS", display_name: "Eosinophils", definition: "", comment_usage_note: "", status: "" },
        "EPICA" => V2TableRow { value: "EPICA", display_name: "Epicardial", definition: "", comment_usage_note: "", status: "" },
        "EPICM" => V2TableRow { value: "EPICM", display_name: "Epicardium", definition: "", comment_usage_note: "", status: "" },
        "EPD" => V2TableRow { value: "EPD", display_name: "Epididymis", definition: "", comment_usage_note: "", status: "" },
        "EPIDU" => V2TableRow { value: "EPIDU", display_name: "Epidural", definition: "", comment_usage_note: "", status: "" },
        "EPIGL" => V2TableRow { value: "EPIGL", display_name: "Epiglottis", definition: "", comment_usage_note: "", status: "" },
        "ESOPG" => V2TableRow { value: "ESOPG", display_name: "Esophageal", definition: "", comment_usage_note: "", status: "" },
        "ESO" => V2TableRow { value: "ESO", display_name: "Esophagus", definition: "", comment_usage_note: "", status: "" },
        "ETHMO" => V2TableRow { value: "ETHMO", display_name: "Ethmoid", definition: "", comment_usage_note: "", status: "" },
        "Â" => V2TableRow { value: "Â", display_name: "External Jugular", definition: "", comment_usage_note: "", status: "" },
        "EYE" => V2TableRow { value: "EYE", display_name: "Eye", definition: "", comment_usage_note: "", status: "" },
        "EYELI" => V2TableRow { value: "EYELI", display_name: "Eyelid", definition: "", comment_usage_note: "", status: "" },
        "FACE" => V2TableRow { value: "FACE", display_name: "Face", definition: "", comment_usage_note: "", status: "" },
        "FBINC" => V2TableRow { value: "FBINC", display_name: "Facial bone, inferior nasal concha", definition: "", comment_usage_note: "", status: "" },
        "FBLA" => V2TableRow { value: "FBLA", display_name: "C Facial bone, lacrimal", definition: "", comment_usage_note: "", status: "" },
        "FBMAX" => V2TableRow { value: "FBMAX", display_name: "Facial bone, maxilla", definition: "", comment_usage_note: "", status: "" },
        "FBNA" => V2TableRow { value: "FBNA", display_name: "S Facial bone, nasal", definition: "", comment_usage_note: "", status: "" },
        "FBPAL" => V2TableRow { value: "FBPAL", display_name: "Facial bone, palatine", definition: "", comment_usage_note: "", status: "" },
        "FBVOM" => V2TableRow { value: "FBVOM", display_name: "Facial bone, vomer", definition: "", comment_usage_note: "", status: "" },
        "FBZYG" => V2TableRow { value: "FBZYG", display_name: "Facial bone, zygomatic", definition: "", comment_usage_note: "", status: "" },
        "FALLT" => V2TableRow { value: "FALLT", display_name: "Fallopian Tube", definition: "", comment_usage_note: "", status: "" },
        "FEMOR" => V2TableRow { value: "FEMOR", display_name: "Femoral", definition: "", comment_usage_note: "", status: "" },
        "FMH" => V2TableRow { value: "FMH", display_name: "Femoral Head", definition: "", comment_usage_note: "", status: "" },
        "FEMUR" => V2TableRow { value: "FEMUR", display_name: "Femur", definition: "", comment_usage_note: "", status: "" },
        "FET" => V2TableRow { value: "FET", display_name: "Fetus", definition: "", comment_usage_note: "", status: "" },
        "FIBU" => V2TableRow { value: "FIBU", display_name: "Fibula", definition: "", comment_usage_note: "", status: "" },
        "FING" => V2TableRow { value: "FING", display_name: "Finger", definition: "", comment_usage_note: "", status: "" },
        "FINGN" => V2TableRow { value: "FINGN", display_name: "Finger Nail", definition: "", comment_usage_note: "", status: "" },
        "FOL" => V2TableRow { value: "FOL", display_name: "Follicle", definition: "", comment_usage_note: "", status: "" },
        "FOOT" => V2TableRow { value: "FOOT", display_name: "Foot", definition: "", comment_usage_note: "", status: "" },
        "FOREA" => V2TableRow { value: "FOREA", display_name: "Forearm", definition: "", comment_usage_note: "", status: "" },
        "FOREH" => V2TableRow { value: "FOREH", display_name: "Forehead", definition: "", comment_usage_note: "", status: "" },
        "FORES" => V2TableRow { value: "FORES", display_name: "Foreskin", definition: "", comment_usage_note: "", status: "" },
        "FOURC" => V2TableRow { value: "FOURC", display_name: "Fourchette", definition: "", comment_usage_note: "", status: "" },
        "GB" => V2TableRow { value: "GB", display_name: "Gall Bladder", definition: "", comment_usage_note: "", status: "" },
        "GEN" => V2TableRow { value: "GEN", display_name: "Genital", definition: "", comment_usage_note: "", status: "" },
        "GVU" => V2TableRow { value: "GVU", display_name: "Genital - Vulva", definition: "", comment_usage_note: "", status: "" },
        "GENC" => V2TableRow { value: "GENC", display_name: "Genital Cervix", definition: "", comment_usage_note: "", status: "" },
        "GL" => V2TableRow { value: "GL", display_name: "Genital Lesion", definition: "", comment_usage_note: "", status: "" },
        "GENL" => V2TableRow { value: "GENL", display_name: "Genital Lochia", definition: "", comment_usage_note: "", status: "" },
        "GLAND" => V2TableRow { value: "GLAND", display_name: "Gland", definition: "", comment_usage_note: "", status: "" },
        "GLANS" => V2TableRow { value: "GLANS", display_name: "Glans", definition: "", comment_usage_note: "", status: "" },
        "GLUTE" => V2TableRow { value: "GLUTE", display_name: "Gluteal", definition: "", comment_usage_note: "", status: "" },
        "GLUT" => V2TableRow { value: "GLUT", display_name: "Gluteus", definition: "", comment_usage_note: "", status: "" },
        "GLUTM" => V2TableRow { value: "GLUTM", display_name: "Gluteus Medius", definition: "", comment_usage_note: "", status: "" },
        "GROIN" => V2TableRow { value: "GROIN", display_name: "Groin", definition: "", comment_usage_note: "", status: "" },
        "GUM" => V2TableRow { value: "GUM", display_name: "Gum", definition: "", comment_usage_note: "", status: "" },
        "HAR" => V2TableRow { value: "HAR", display_name: "Hair", definition: "", comment_usage_note: "", status: "" },
        "HAL" => V2TableRow { value: "HAL", display_name: "Hallux", definition: "", comment_usage_note: "", status: "" },
        "HAND" => V2TableRow { value: "HAND", display_name: "Hand", definition: "", comment_usage_note: "", status: "" },
        "HEAD" => V2TableRow { value: "HEAD", display_name: "Head", definition: "", comment_usage_note: "", status: "" },
        "HART" => V2TableRow { value: "HART", display_name: "Heart", definition: "", comment_usage_note: "", status: "" },
        "HV" => V2TableRow { value: "HV", display_name: "Heart Valve", definition: "", comment_usage_note: "", status: "" },
        "HVB" => V2TableRow { value: "HVB", display_name: "Heart Valve, Bicuspid", definition: "", comment_usage_note: "", status: "" },
        "HVT" => V2TableRow { value: "HVT", display_name: "Heart Valve, Tricuspid", definition: "", comment_usage_note: "", status: "" },
        "HEEL" => V2TableRow { value: "HEEL", display_name: "Heel", definition: "", comment_usage_note: "", status: "" },
        "HEM" => V2TableRow { value: "HEM", display_name: "Hemorrhoid", definition: "", comment_usage_note: "", status: "" },
        "HIP" => V2TableRow { value: "HIP", display_name: "Hip", definition: "", comment_usage_note: "", status: "" },
        "HIPJ" => V2TableRow { value: "HIPJ", display_name: "Hip Joint", definition: "", comment_usage_note: "", status: "" },
        "HUMER" => V2TableRow { value: "HUMER", display_name: "Humerus", definition: "", comment_usage_note: "", status: "" },
        "HYMEN" => V2TableRow { value: "HYMEN", display_name: "Hymen", definition: "", comment_usage_note: "", status: "" },
        "ILC" => V2TableRow { value: "ILC", display_name: "Ileal Conduit", definition: "", comment_usage_note: "", status: "" },
        "ILE" => V2TableRow { value: "ILE", display_name: "Ileal Loop", definition: "", comment_usage_note: "", status: "" },
        "ILEOS" => V2TableRow { value: "ILEOS", display_name: "Ileostomy", definition: "", comment_usage_note: "", status: "" },
        "ILEUM" => V2TableRow { value: "ILEUM", display_name: "Ileum", definition: "", comment_usage_note: "", status: "" },
        "ILIAC" => V2TableRow { value: "ILIAC", display_name: "Iliac", definition: "", comment_usage_note: "", status: "" },
        "ILCR" => V2TableRow { value: "ILCR", display_name: "Iliac Crest", definition: "", comment_usage_note: "", status: "" },
        "ILCON" => V2TableRow { value: "ILCON", display_name: "Ilical Conduit", definition: "", comment_usage_note: "", status: "" },
        "INGUI" => V2TableRow { value: "INGUI", display_name: "Inguinal", definition: "", comment_usage_note: "", status: "" },
        "JUGI" => V2TableRow { value: "JUGI", display_name: "Jugular, Internal", definition: "", comment_usage_note: "", status: "" },
        "INT" => V2TableRow { value: "INT", display_name: "Intestine", definition: "", comment_usage_note: "", status: "" },
        "ICX" => V2TableRow { value: "ICX", display_name: "Intracervical", definition: "", comment_usage_note: "", status: "" },
        "INASA" => V2TableRow { value: "INASA", display_name: "Intranasal", definition: "", comment_usage_note: "", status: "" },
        "INTRU" => V2TableRow { value: "INTRU", display_name: "Intrauterine", definition: "", comment_usage_note: "", status: "" },
        "INTRO" => V2TableRow { value: "INTRO", display_name: "Introitus", definition: "", comment_usage_note: "", status: "" },
        "ISCHI" => V2TableRow { value: "ISCHI", display_name: "Ischium", definition: "", comment_usage_note: "", status: "" },
        "JAW" => V2TableRow { value: "JAW", display_name: "Jaw", definition: "", comment_usage_note: "", status: "" },
        "KIDNÂ" => V2TableRow { value: "KIDNÂ", display_name: "Kidney", definition: "", comment_usage_note: "", status: "" },
        "KNEE" => V2TableRow { value: "KNEE", display_name: "Knee", definition: "", comment_usage_note: "", status: "" },
        "KNEEF" => V2TableRow { value: "KNEEF", display_name: "Knee Fluid", definition: "", comment_usage_note: "", status: "" },
        "KNEEJ" => V2TableRow { value: "KNEEJ", display_name: "Knee Joint", definition: "", comment_usage_note: "", status: "" },
        "LABIA" => V2TableRow { value: "LABIA", display_name: "Labia", definition: "", comment_usage_note: "", status: "" },
        "LABMA" => V2TableRow { value: "LABMA", display_name: "Labia Majora", definition: "", comment_usage_note: "", status: "" },
        "LABMI" => V2TableRow { value: "LABMI", display_name: "Labia Minora", definition: "", comment_usage_note: "", status: "" },
        "LACRI" => V2TableRow { value: "LACRI", display_name: "Lacrimal", definition: "", comment_usage_note: "", status: "" },
        "LAM" => V2TableRow { value: "LAM", display_name: "Lamella", definition: "", comment_usage_note: "", status: "" },
        "INSTL" => V2TableRow { value: "INSTL", display_name: "Intestine, Large", definition: "", comment_usage_note: "", status: "" },
        "LARYN" => V2TableRow { value: "LARYN", display_name: "Larynx", definition: "", comment_usage_note: "", status: "" },
        "LEG" => V2TableRow { value: "LEG", display_name: "Leg", definition: "", comment_usage_note: "", status: "" },
        "LENS" => V2TableRow { value: "LENS", display_name: "Lens", definition: "", comment_usage_note: "", status: "" },
        "WBC" => V2TableRow { value: "WBC", display_name: "Leukocytes", definition: "", comment_usage_note: "", status: "" },
        "LING" => V2TableRow { value: "LING", display_name: "Lingual", definition: "", comment_usage_note: "", status: "" },
        "LINGU" => V2TableRow { value: "LINGU", display_name: "Lingula", definition: "", comment_usage_note: "", status: "" },
        "LIP" => V2TableRow { value: "LIP", display_name: "Lip", definition: "", comment_usage_note: "", status: "" },
        "STOOLL" => V2TableRow { value: "STOOLL", display_name: "Liquid Stool", definition: "", comment_usage_note: "", status: "" },
        "LIVER" => V2TableRow { value: "LIVER", display_name: "Liver", definition: "", comment_usage_note: "", status: "" },
        "LOBE" => V2TableRow { value: "LOBE", display_name: "Lobe", definition: "", comment_usage_note: "", status: "" },
        "LOCH" => V2TableRow { value: "LOCH", display_name: "Lochia", definition: "", comment_usage_note: "", status: "" },
        "ISH" => V2TableRow { value: "ISH", display_name: "Loop, Ishial", definition: "", comment_usage_note: "", status: "" },
        "LUMBA" => V2TableRow { value: "LUMBA", display_name: "Lumbar", definition: "", comment_usage_note: "", status: "" },
        "LMN" => V2TableRow { value: "LMN", display_name: "Lumen", definition: "", comment_usage_note: "", status: "" },
        "LUNG" => V2TableRow { value: "LUNG", display_name: "Lung", definition: "", comment_usage_note: "", status: "" },
        "LN" => V2TableRow { value: "LN", display_name: "Lymph Node", definition: "", comment_usage_note: "", status: "" },
        "LNG" => V2TableRow { value: "LNG", display_name: "Lymph Node, Groin", definition: "", comment_usage_note: "", status: "" },
        "LYM" => V2TableRow { value: "LYM", display_name: "Lymphocytes", definition: "", comment_usage_note: "", status: "" },
        "MAC" => V2TableRow { value: "MAC", display_name: "Macrophages", definition: "", comment_usage_note: "", status: "" },
        "MALLE" => V2TableRow { value: "MALLE", display_name: "Malleolus", definition: "", comment_usage_note: "", status: "" },
        "MANDI" => V2TableRow { value: "MANDI", display_name: "Mandible/Mandib ular", definition: "", comment_usage_note: "", status: "" },
        "MAR" => V2TableRow { value: "MAR", display_name: "Marrow", definition: "", comment_usage_note: "", status: "" },
        "MAST" => V2TableRow { value: "MAST", display_name: "Mastoid", definition: "", comment_usage_note: "", status: "" },
        "MAXIL" => V2TableRow { value: "MAXIL", display_name: "Maxilla/Maxillar y", definition: "", comment_usage_note: "", status: "" },
        "MAXS" => V2TableRow { value: "MAXS", display_name: "Maxillary Sinus", definition: "", comment_usage_note: "", status: "" },
        "MEATU" => V2TableRow { value: "MEATU", display_name: "Meatus", definition: "", comment_usage_note: "", status: "" },
        "MEC" => V2TableRow { value: "MEC", display_name: "Meconium", definition: "", comment_usage_note: "", status: "" },
        "MEDST" => V2TableRow { value: "MEDST", display_name: "Mediastinum", definition: "", comment_usage_note: "", status: "" },
        "MEDU" => V2TableRow { value: "MEDU", display_name: "Medullary", definition: "", comment_usage_note: "", status: "" },
        "MOU" => V2TableRow { value: "MOU", display_name: "Membrane", definition: "", comment_usage_note: "", status: "" },
        "MPB" => V2TableRow { value: "MPB", display_name: "Meninges", definition: "", comment_usage_note: "", status: "" },
        "METAC" => V2TableRow { value: "METAC", display_name: "Metacarpal", definition: "", comment_usage_note: "", status: "" },
        "METAT" => V2TableRow { value: "METAT", display_name: "Metatarsal", definition: "", comment_usage_note: "", status: "" },
        "MILK" => V2TableRow { value: "MILK", display_name: "Milk, Breast", definition: "", comment_usage_note: "", status: "" },
        "MITRL" => V2TableRow { value: "MITRL", display_name: "Mitral Valve", definition: "", comment_usage_note: "", status: "" },
        "MOLAR" => V2TableRow { value: "MOLAR", display_name: "Molar", definition: "", comment_usage_note: "", status: "" },
        "MP" => V2TableRow { value: "MP", display_name: "Mons Pubis", definition: "", comment_usage_note: "", status: "" },
        "MONSU" => V2TableRow { value: "MONSU", display_name: "Mons Ureteris", definition: "", comment_usage_note: "", status: "" },
        "MONSV" => V2TableRow { value: "MONSV", display_name: "Mons Veneris(Mons Pubis)", definition: "", comment_usage_note: "", status: "" },
        "MOUTH" => V2TableRow { value: "MOUTH", display_name: "Mouth", definition: "", comment_usage_note: "", status: "" },
        "MRSA2" => V2TableRow { value: "MRSA2", display_name: "Mrsa:", definition: "", comment_usage_note: "", status: "" },
        "MYO" => V2TableRow { value: "MYO", display_name: "Myocardium", definition: "", comment_usage_note: "", status: "" },
        "NAIL" => V2TableRow { value: "NAIL", display_name: "Nail", definition: "", comment_usage_note: "", status: "" },
        "NAILB" => V2TableRow { value: "NAILB", display_name: "Nail Bed", definition: "", comment_usage_note: "", status: "" },
        "NAILF" => V2TableRow { value: "NAILF", display_name: "Nail, Finger", definition: "", comment_usage_note: "", status: "" },
        "NAILT" => V2TableRow { value: "NAILT", display_name: "Nail, Toe", definition: "", comment_usage_note: "", status: "" },
        "NARES" => V2TableRow { value: "NARES", display_name: "Nares", definition: "", comment_usage_note: "", status: "" },
        "NASL" => V2TableRow { value: "NASL", display_name: "Nasal", definition: "", comment_usage_note: "", status: "" },
        "NSS" => V2TableRow { value: "NSS", display_name: "Nasal Septum", definition: "", comment_usage_note: "", status: "" },
        "NLACR" => V2TableRow { value: "NLACR", display_name: "Nasolacrimal", definition: "", comment_usage_note: "", status: "" },
        "NP" => V2TableRow { value: "NP", display_name: "Nasopharyngeal/ Nasopharynx", definition: "", comment_usage_note: "", status: "" },
        "NTRAC" => V2TableRow { value: "NTRAC", display_name: "Nasotracheal", definition: "", comment_usage_note: "", status: "" },
        "NAVEL" => V2TableRow { value: "NAVEL", display_name: "Navel", definition: "", comment_usage_note: "", status: "" },
        "NECK" => V2TableRow { value: "NECK", display_name: "Neck", definition: "", comment_usage_note: "", status: "" },
        "NERVE" => V2TableRow { value: "NERVE", display_name: "Nerve", definition: "", comment_usage_note: "", status: "" },
        "NIPPL" => V2TableRow { value: "NIPPL", display_name: "Nipple", definition: "", comment_usage_note: "", status: "" },
        "NOS" => V2TableRow { value: "NOS", display_name: "Nose (Nasal Passage)", definition: "", comment_usage_note: "", status: "" },
        "NOSE" => V2TableRow { value: "NOSE", display_name: "Nose/Nose(outsid e)", definition: "", comment_usage_note: "", status: "" },
        "NOSTR" => V2TableRow { value: "NOSTR", display_name: "Nostril", definition: "", comment_usage_note: "", status: "" },
        "OCCIP" => V2TableRow { value: "OCCIP", display_name: "Occipital", definition: "", comment_usage_note: "", status: "" },
        "OLECR" => V2TableRow { value: "OLECR", display_name: "Olecranon", definition: "", comment_usage_note: "", status: "" },
        "OMEN" => V2TableRow { value: "OMEN", display_name: "Omentum", definition: "", comment_usage_note: "", status: "" },
        "ORBIT" => V2TableRow { value: "ORBIT", display_name: "Orbit/Orbital", definition: "", comment_usage_note: "", status: "" },
        "ORO" => V2TableRow { value: "ORO", display_name: "Oropharynx", definition: "", comment_usage_note: "", status: "" },
        "OSCOX" => V2TableRow { value: "OSCOX", display_name: "Os coxa (pelvic girdle)", definition: "", comment_usage_note: "", status: "" },
        "OVARY" => V2TableRow { value: "OVARY", display_name: "Ovary", definition: "", comment_usage_note: "", status: "" },
        "PALAT" => V2TableRow { value: "PALAT", display_name: "Palate", definition: "", comment_usage_note: "", status: "" },
        "PLATH" => V2TableRow { value: "PLATH", display_name: "Palate, Hard", definition: "", comment_usage_note: "", status: "" },
        "PLATS" => V2TableRow { value: "PLATS", display_name: "Palate, Soft", definition: "", comment_usage_note: "", status: "" },
        "PALM" => V2TableRow { value: "PALM", display_name: "Palm", definition: "", comment_usage_note: "", status: "" },
        "PANCR" => V2TableRow { value: "PANCR", display_name: "Pancreas", definition: "", comment_usage_note: "", status: "" },
        "PAFL" => V2TableRow { value: "PAFL", display_name: "Pancreatic Fluid", definition: "", comment_usage_note: "", status: "" },
        "PAS" => V2TableRow { value: "PAS", display_name: "Parasternal", definition: "", comment_usage_note: "", status: "" },
        "PARAT" => V2TableRow { value: "PARAT", display_name: "Paratracheal", definition: "", comment_usage_note: "", status: "" },
        "PARIE" => V2TableRow { value: "PARIE", display_name: "Parietal", definition: "", comment_usage_note: "", status: "" },
        "PARON" => V2TableRow { value: "PARON", display_name: "Paronychia", definition: "", comment_usage_note: "", status: "" },
        "PAROT" => V2TableRow { value: "PAROT", display_name: "Parotid/Parotid Gland", definition: "", comment_usage_note: "", status: "" },
        "PATEL" => V2TableRow { value: "PATEL", display_name: "Patella", definition: "", comment_usage_note: "", status: "" },
        "PELV" => V2TableRow { value: "PELV", display_name: "Pelvis", definition: "", comment_usage_note: "", status: "" },
        "PENSH" => V2TableRow { value: "PENSH", display_name: "Penile Shaft", definition: "", comment_usage_note: "", status: "" },
        "PENIS" => V2TableRow { value: "PENIS", display_name: "Penis", definition: "", comment_usage_note: "", status: "" },
        "PANAL" => V2TableRow { value: "PANAL", display_name: "Perianal/Perirecta l", definition: "", comment_usage_note: "", status: "" },
        "PERI" => V2TableRow { value: "PERI", display_name: "Pericardial Fluid", definition: "", comment_usage_note: "", status: "" },
        "PCA" => V2TableRow { value: "PCA", display_name: "RD Pericardium", definition: "", comment_usage_note: "", status: "" },
        "PCLIT" => V2TableRow { value: "PCLIT", display_name: "Periclitoral", definition: "", comment_usage_note: "", status: "" },
        "PERIH" => V2TableRow { value: "PERIH", display_name: "Perihepatic", definition: "", comment_usage_note: "", status: "" },
        "PNEAL" => V2TableRow { value: "PNEAL", display_name: "Perineal", definition: "", comment_usage_note: "", status: "" },
        "PERIN" => V2TableRow { value: "PERIN", display_name: "Perineal Abscess", definition: "", comment_usage_note: "", status: "" },
        "PNEPH" => V2TableRow { value: "PNEPH", display_name: "Perinephric", definition: "", comment_usage_note: "", status: "" },
        "PNM" => V2TableRow { value: "PNM", display_name: "Perineum", definition: "", comment_usage_note: "", status: "" },
        "PORBI" => V2TableRow { value: "PORBI", display_name: "Periorbital", definition: "", comment_usage_note: "", status: "" },
        "PERRA" => V2TableRow { value: "PERRA", display_name: "Perirectal", definition: "", comment_usage_note: "", status: "" },
        "PERIS" => V2TableRow { value: "PERIS", display_name: "Perisplenic", definition: "", comment_usage_note: "", status: "" },
        "PER" => V2TableRow { value: "PER", display_name: "Peritoneal", definition: "", comment_usage_note: "", status: "" },
        "PERT" => V2TableRow { value: "PERT", display_name: "Peritoneal Fluid", definition: "", comment_usage_note: "", status: "" },
        "PERIT" => V2TableRow { value: "PERIT", display_name: "Peritoneum", definition: "", comment_usage_note: "", status: "" },
        "PTONS" => V2TableRow { value: "PTONS", display_name: "Peritonsillar", definition: "", comment_usage_note: "", status: "" },
        "PERIU" => V2TableRow { value: "PERIU", display_name: "Periurethal", definition: "", comment_usage_note: "", status: "" },
        "PERIV" => V2TableRow { value: "PERIV", display_name: "Perivesicular", definition: "", comment_usage_note: "", status: "" },
        "PHALA" => V2TableRow { value: "PHALA", display_name: "Phalanyx", definition: "", comment_usage_note: "", status: "" },
        "PILO" => V2TableRow { value: "PILO", display_name: "Pilonidal", definition: "", comment_usage_note: "", status: "" },
        "PINNA" => V2TableRow { value: "PINNA", display_name: "Pinna", definition: "", comment_usage_note: "", status: "" },
        "PLC" => V2TableRow { value: "PLC", display_name: "Placenta", definition: "", comment_usage_note: "", status: "" },
        "PLACF" => V2TableRow { value: "PLACF", display_name: "Placenta (Fetal Side)", definition: "", comment_usage_note: "", status: "" },
        "PLACM" => V2TableRow { value: "PLACM", display_name: "Placenta (Maternal Side)", definition: "", comment_usage_note: "", status: "" },
        "PLANT" => V2TableRow { value: "PLANT", display_name: "Plantar", definition: "", comment_usage_note: "", status: "" },
        "PLEUR" => V2TableRow { value: "PLEUR", display_name: "Pleura", definition: "", comment_usage_note: "", status: "" },
        "PLEU" => V2TableRow { value: "PLEU", display_name: "Pleural Fluid", definition: "", comment_usage_note: "", status: "" },
        "PLR" => V2TableRow { value: "PLR", display_name: "Pleural Fluid (Thoracentesis Fld)", definition: "", comment_usage_note: "", status: "" },
        "POPLI" => V2TableRow { value: "POPLI", display_name: "Popliteal", definition: "", comment_usage_note: "", status: "" },
        "PREAU" => V2TableRow { value: "PREAU", display_name: "Preauricular", definition: "", comment_usage_note: "", status: "" },
        "PRERE" => V2TableRow { value: "PRERE", display_name: "Prerenal", definition: "", comment_usage_note: "", status: "" },
        "PRST" => V2TableRow { value: "PRST", display_name: "Prostate Gland", definition: "", comment_usage_note: "", status: "" },
        "PROS" => V2TableRow { value: "PROS", display_name: "Prostatic Fluid", definition: "", comment_usage_note: "", status: "" },
        "PUBIC" => V2TableRow { value: "PUBIC", display_name: "Pubic", definition: "", comment_usage_note: "", status: "" },
        "PUL" => V2TableRow { value: "PUL", display_name: "Pulmonary Artery", definition: "", comment_usage_note: "", status: "" },
        "RADI" => V2TableRow { value: "RADI", display_name: "Radial", definition: "", comment_usage_note: "", status: "" },
        "RADIUS" => V2TableRow { value: "RADIUS", display_name: "Radius", definition: "", comment_usage_note: "", status: "" },
        "RECTL" => V2TableRow { value: "RECTL", display_name: "Rectal", definition: "", comment_usage_note: "", status: "" },
        "RECTU" => V2TableRow { value: "RECTU", display_name: "Rectum", definition: "", comment_usage_note: "", status: "" },
        "RBC" => V2TableRow { value: "RBC", display_name: "Red Blood Cells", definition: "", comment_usage_note: "", status: "" },
        "RENL" => V2TableRow { value: "RENL", display_name: "Renal", definition: "", comment_usage_note: "", status: "" },
        "RNP" => V2TableRow { value: "RNP", display_name: "Renal Pelvis", definition: "", comment_usage_note: "", status: "" },
        "RPERI" => V2TableRow { value: "RPERI", display_name: "Retroperitoneal", definition: "", comment_usage_note: "", status: "" },
        "RIB" => V2TableRow { value: "RIB", display_name: "Rib", definition: "", comment_usage_note: "", status: "" },
        "SACRA" => V2TableRow { value: "SACRA", display_name: "Sacral", definition: "", comment_usage_note: "", status: "" },
        "SACRO" => V2TableRow { value: "SACRO", display_name: "Sacrococcygeal", definition: "", comment_usage_note: "", status: "" },
        "SACIL" => V2TableRow { value: "SACIL", display_name: "Sacroiliac", definition: "", comment_usage_note: "", status: "" },
        "SACRU" => V2TableRow { value: "SACRU", display_name: "Sacrum", definition: "", comment_usage_note: "", status: "" },
        "SALGL" => V2TableRow { value: "SALGL", display_name: "Salivary Gland", definition: "", comment_usage_note: "", status: "" },
        "SCALP" => V2TableRow { value: "SCALP", display_name: "Scalp", definition: "", comment_usage_note: "", status: "" },
        "SCAP" => V2TableRow { value: "SCAP", display_name: "U Scapula/Scapular", definition: "", comment_usage_note: "", status: "" },
        "SCLER" => V2TableRow { value: "SCLER", display_name: "Sclera", definition: "", comment_usage_note: "", status: "" },
        "SCROT" => V2TableRow { value: "SCROT", display_name: "Scrotum/Scrotal", definition: "", comment_usage_note: "", status: "" },
        "SEMN" => V2TableRow { value: "SEMN", display_name: "Semen", definition: "", comment_usage_note: "", status: "" },
        "SEM" => V2TableRow { value: "SEM", display_name: "Seminal Fluid", definition: "", comment_usage_note: "", status: "" },
        "SEPTU" => V2TableRow { value: "SEPTU", display_name: "Septum/Septal", definition: "", comment_usage_note: "", status: "" },
        "SEROM" => V2TableRow { value: "SEROM", display_name: "Seroma", definition: "", comment_usage_note: "", status: "" },
        "SHIN" => V2TableRow { value: "SHIN", display_name: "Shin", definition: "", comment_usage_note: "", status: "" },
        "SHOLJ" => V2TableRow { value: "SHOLJ", display_name: "Sholder Joint", definition: "", comment_usage_note: "", status: "" },
        "SHOL" => V2TableRow { value: "SHOL", display_name: "Shoulder", definition: "", comment_usage_note: "", status: "" },
        "SIGMO" => V2TableRow { value: "SIGMO", display_name: "Sigmoid", definition: "", comment_usage_note: "", status: "" },
        "SINUS" => V2TableRow { value: "SINUS", display_name: "Sinus", definition: "", comment_usage_note: "", status: "" },
        "SKM" => V2TableRow { value: "SKM", display_name: "Skeletal Muscle", definition: "", comment_usage_note: "", status: "" },
        "SKENE" => V2TableRow { value: "SKENE", display_name: "Skene's Gland", definition: "", comment_usage_note: "", status: "" },
        "SKULL" => V2TableRow { value: "SKULL", display_name: "Skull", definition: "", comment_usage_note: "", status: "" },
        "INSTS" => V2TableRow { value: "INSTS", display_name: "Intestine, Small", definition: "", comment_usage_note: "", status: "" },
        "SOLE" => V2TableRow { value: "SOLE", display_name: "Sole", definition: "", comment_usage_note: "", status: "" },
        "SPRM" => V2TableRow { value: "SPRM", display_name: "Spermatozoa", definition: "", comment_usage_note: "", status: "" },
        "SPHEN" => V2TableRow { value: "SPHEN", display_name: "Sphenoid", definition: "", comment_usage_note: "", status: "" },
        "SPCO" => V2TableRow { value: "SPCO", display_name: "R Spinal Cord", definition: "", comment_usage_note: "", status: "" },
        "SPLN" => V2TableRow { value: "SPLN", display_name: "Spleen", definition: "", comment_usage_note: "", status: "" },
        "STER" => V2TableRow { value: "STER", display_name: "Sternum/Sternal", definition: "", comment_usage_note: "", status: "" },
        "STOM" => V2TableRow { value: "STOM", display_name: "Stoma", definition: "", comment_usage_note: "", status: "" },
        "USTOM" => V2TableRow { value: "USTOM", display_name: "Stoma, Urinary", definition: "", comment_usage_note: "", status: "" },
        "STOMA" => V2TableRow { value: "STOMA", display_name: "Stomach", definition: "", comment_usage_note: "", status: "" },
        "STUMP" => V2TableRow { value: "STUMP", display_name: "Stump", definition: "", comment_usage_note: "", status: "" },
        "SCLV" => V2TableRow { value: "SCLV", display_name: "Sub Clavian", definition: "", comment_usage_note: "", status: "" },
        "SDP" => V2TableRow { value: "SDP", display_name: "Subdiaphramatic", definition: "", comment_usage_note: "", status: "" },
        "SUB" => V2TableRow { value: "SUB", display_name: "Subdural", definition: "", comment_usage_note: "", status: "" },
        "SUBD" => V2TableRow { value: "SUBD", display_name: "Subdural Fluid", definition: "", comment_usage_note: "", status: "" },
        "SGF" => V2TableRow { value: "SGF", display_name: "Subgaleal Fluid", definition: "", comment_usage_note: "", status: "" },
        "SUBM" => V2TableRow { value: "SUBM", display_name: "Submandibular", definition: "", comment_usage_note: "", status: "" },
        "SUBX" => V2TableRow { value: "SUBX", display_name: "Submaxillary", definition: "", comment_usage_note: "", status: "" },
        "SUBME" => V2TableRow { value: "SUBME", display_name: "Submental", definition: "", comment_usage_note: "", status: "" },
        "SUBP" => V2TableRow { value: "SUBP", display_name: "H Subphrenic", definition: "", comment_usage_note: "", status: "" },
        "SPX" => V2TableRow { value: "SPX", display_name: "Supra Cervical", definition: "", comment_usage_note: "", status: "" },
        "SCLAV" => V2TableRow { value: "SCLAV", display_name: "Supraclavicle/Sup raclavicular", definition: "", comment_usage_note: "", status: "" },
        "SUPRA" => V2TableRow { value: "SUPRA", display_name: "Suprapubic", definition: "", comment_usage_note: "", status: "" },
        "SUPB" => V2TableRow { value: "SUPB", display_name: "Suprapubic Specimen", definition: "", comment_usage_note: "", status: "" },
        "SWT" => V2TableRow { value: "SWT", display_name: "Sweat", definition: "", comment_usage_note: "", status: "" },
        "SWTG" => V2TableRow { value: "SWTG", display_name: "Sweat Gland", definition: "", comment_usage_note: "", status: "" },
        "SYNOL" => V2TableRow { value: "SYNOL", display_name: "Synovial", definition: "", comment_usage_note: "", status: "" },
        "SYN" => V2TableRow { value: "SYN", display_name: "Synovial Fluid", definition: "", comment_usage_note: "", status: "" },
        "SYNOV" => V2TableRow { value: "SYNOV", display_name: "Synovium", definition: "", comment_usage_note: "", status: "" },
        "TARS" => V2TableRow { value: "TARS", display_name: "Tarsal", definition: "", comment_usage_note: "", status: "" },
        "TDUCT" => V2TableRow { value: "TDUCT", display_name: "Tear Duct", definition: "", comment_usage_note: "", status: "" },
        "TEAR" => V2TableRow { value: "TEAR", display_name: "Tears", definition: "", comment_usage_note: "", status: "" },
        "TEMPL" => V2TableRow { value: "TEMPL", display_name: "Temple", definition: "", comment_usage_note: "", status: "" },
        "TEMPO" => V2TableRow { value: "TEMPO", display_name: "Temporal", definition: "", comment_usage_note: "", status: "" },
        "TML" => V2TableRow { value: "TML", display_name: "Temporal Lobe", definition: "", comment_usage_note: "", status: "" },
        "TESTI" => V2TableRow { value: "TESTI", display_name: "Testicle(Testis)", definition: "", comment_usage_note: "", status: "" },
        "THIGH" => V2TableRow { value: "THIGH", display_name: "Thigh", definition: "", comment_usage_note: "", status: "" },
        "THORA" => V2TableRow { value: "THORA", display_name: "Thorax/Thoracic/ Thoracentesis", definition: "", comment_usage_note: "", status: "" },
        "THRB" => V2TableRow { value: "THRB", display_name: "Throat", definition: "", comment_usage_note: "", status: "" },
        "THUMB" => V2TableRow { value: "THUMB", display_name: "Thumb", definition: "", comment_usage_note: "", status: "" },
        "TNL" => V2TableRow { value: "TNL", display_name: "Thumbnail", definition: "", comment_usage_note: "", status: "" },
        "THM" => V2TableRow { value: "THM", display_name: "Thymus", definition: "", comment_usage_note: "", status: "" },
        "THYRD" => V2TableRow { value: "THYRD", display_name: "Thyroid", definition: "", comment_usage_note: "", status: "" },
        "TIBIA" => V2TableRow { value: "TIBIA", display_name: "Tibia", definition: "", comment_usage_note: "", status: "" },
        "TOE" => V2TableRow { value: "TOE", display_name: "Toe", definition: "", comment_usage_note: "", status: "" },
        "TOEN" => V2TableRow { value: "TOEN", display_name: "Toe Nail", definition: "", comment_usage_note: "", status: "" },
        "TONG" => V2TableRow { value: "TONG", display_name: "Tongue", definition: "", comment_usage_note: "", status: "" },
        "TONS" => V2TableRow { value: "TONS", display_name: "Tonsil", definition: "", comment_usage_note: "", status: "" },
        "TOOTH" => V2TableRow { value: "TOOTH", display_name: "Tooth", definition: "", comment_usage_note: "", status: "" },
        "TSK" => V2TableRow { value: "TSK", display_name: "Tooth Socket", definition: "", comment_usage_note: "", status: "" },
        "TRCHE" => V2TableRow { value: "TRCHE", display_name: "Trachea/Tracheal", definition: "", comment_usage_note: "", status: "" },
        "TBRON" => V2TableRow { value: "TBRON", display_name: "Transbronchial", definition: "", comment_usage_note: "", status: "" },
        "TCN" => V2TableRow { value: "TCN", display_name: "Transcarina Asp", definition: "", comment_usage_note: "", status: "" },
        "ULNA" => V2TableRow { value: "ULNA", display_name: "Ulna/Ulnar", definition: "", comment_usage_note: "", status: "" },
        "UMB" => V2TableRow { value: "UMB", display_name: "Umbilical Blood", definition: "", comment_usage_note: "", status: "" },
        "UMBL" => V2TableRow { value: "UMBL", display_name: "Umbilicus/Umbili cal", definition: "", comment_usage_note: "", status: "" },
        "URET" => V2TableRow { value: "URET", display_name: "Ureter", definition: "", comment_usage_note: "", status: "" },
        "URTH" => V2TableRow { value: "URTH", display_name: "Urethra", definition: "", comment_usage_note: "", status: "" },
        "UTERI" => V2TableRow { value: "UTERI", display_name: "Uterine", definition: "", comment_usage_note: "", status: "" },
        "SAC" => V2TableRow { value: "SAC", display_name: "Uterine Cul/De/Sac", definition: "", comment_usage_note: "", status: "" },
        "UTER" => V2TableRow { value: "UTER", display_name: "Uterus", definition: "", comment_usage_note: "", status: "" },
        "VAGIN" => V2TableRow { value: "VAGIN", display_name: "Vagina/Vaginal", definition: "", comment_usage_note: "", status: "" },
        "VCUFF" => V2TableRow { value: "VCUFF", display_name: "Vaginal Cuff", definition: "", comment_usage_note: "", status: "" },
        "VGV" => V2TableRow { value: "VGV", display_name: "Vaginal Vault", definition: "", comment_usage_note: "", status: "" },
        "VAL" => V2TableRow { value: "VAL", display_name: "Valve", definition: "", comment_usage_note: "", status: "" },
        "VAS" => V2TableRow { value: "VAS", display_name: "Vas Deferens", definition: "", comment_usage_note: "", status: "" },
        "VASTL" => V2TableRow { value: "VASTL", display_name: "Vastus Lateralis", definition: "", comment_usage_note: "", status: "" },
        "VAULT" => V2TableRow { value: "VAULT", display_name: "Vault", definition: "", comment_usage_note: "", status: "" },
        "VEIN" => V2TableRow { value: "VEIN", display_name: "Vein", definition: "", comment_usage_note: "", status: "" },
        "VENTG" => V2TableRow { value: "VENTG", display_name: "Ventragluteal", definition: "", comment_usage_note: "", status: "" },
        "VCSF" => V2TableRow { value: "VCSF", display_name: "Ventricular CSF", definition: "", comment_usage_note: "", status: "" },
        "VERMI" => V2TableRow { value: "VERMI", display_name: "Vermis Cerebelli", definition: "", comment_usage_note: "", status: "" },
        "VERTC" => V2TableRow { value: "VERTC", display_name: "Vertebra, cervical", definition: "", comment_usage_note: "", status: "" },
        "VERTL" => V2TableRow { value: "VERTL", display_name: "Vertebra, lumbar", definition: "", comment_usage_note: "", status: "" },
        "VERTT" => V2TableRow { value: "VERTT", display_name: "Vertebra, thoracic", definition: "", comment_usage_note: "", status: "" },
        "VESI" => V2TableRow { value: "VESI", display_name: "Vesicle", definition: "", comment_usage_note: "", status: "" },
        "VESCL" => V2TableRow { value: "VESCL", display_name: "Vesicular", definition: "", comment_usage_note: "", status: "" },
        "VESFLD" => V2TableRow { value: "VESFLD", display_name: "Vesicular Fluid", definition: "", comment_usage_note: "", status: "" },
        "VESTI" => V2TableRow { value: "VESTI", display_name: "Vestibule(Genital )", definition: "", comment_usage_note: "", status: "" },
        "VITR" => V2TableRow { value: "VITR", display_name: "Vitreous Fluid", definition: "", comment_usage_note: "", status: "" },
        "VOC" => V2TableRow { value: "VOC", display_name: "Vocal Cord", definition: "", comment_usage_note: "", status: "" },
        "VULVA" => V2TableRow { value: "VULVA", display_name: "Vulva", definition: "", comment_usage_note: "", status: "" },
        "WRIST" => V2TableRow { value: "WRIST", display_name: "Wrist", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0553: V2Table = V2Table {
    number: 553,
    metadata: &super::metadata::TABLE_0553_METADATA,
    rows: phf_map! {
        "OR" => V2TableRow { value: "OR", display_name: "Original Invoice", definition: "", comment_usage_note: "", status: "" },
        "CN" => V2TableRow { value: "CN", display_name: "Cancel Invoice", definition: "", comment_usage_note: "Can be used to reverse or cancel an invoice in progress or reverse a paid invoice. Receiver may only mark Invoice as cancelled, not purge records", status: "" },
        "CG" => V2TableRow { value: "CG", display_name: "Cancel Invoice Product/Service Group", definition: "", comment_usage_note: "Cancel a specific Product/Service Group in an Invoice", status: "" },
        "CL" => V2TableRow { value: "CL", display_name: "Cancel Invoice Product/Service Line Item", definition: "", comment_usage_note: "Cancel a specific Product/Service Line Item in an Invoice", status: "" },
        "PD" => V2TableRow { value: "PD", display_name: "Pre- Determination Invoice", definition: "", comment_usage_note: "Can be used to submit an invoice through a Payer's edit and adjudication engine to determine if the invoice will be paid - does not result in payment by Payer", status: "" },
        "RA" => V2TableRow { value: "RA", display_name: "Re-Asse ssment", definition: "", comment_usage_note: "Used on EHC^E04 only", status: "" },
        "OA" => V2TableRow { value: "OA", display_name: "Original Authorization", definition: "", comment_usage_note: "", status: "" },
        "SA" => V2TableRow { value: "SA", display_name: "Special Authorization", definition: "", comment_usage_note: "", status: "" },
        "AI" => V2TableRow { value: "AI", display_name: "Combined Authorization and Adjudication request", definition: "", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Pre-Authorization", definition: "", comment_usage_note: "", status: "" },
        "AA" => V2TableRow { value: "AA", display_name: "Authorization request for inpatient admission", definition: "", comment_usage_note: "", status: "" },
        "EA" => V2TableRow { value: "EA", display_name: "Authorization request for inpatient stay extension", definition: "", comment_usage_note: "", status: "" },
        "RC" => V2TableRow { value: "RC", display_name: "Referral Pre- Authorization", definition: "", comment_usage_note: "", status: "" },
        "CA" => V2TableRow { value: "CA", display_name: "Cancel Authorization request", definition: "", comment_usage_note: "", status: "" },
        "CP" => V2TableRow { value: "CP", display_name: "Copy of Invoice", definition: "", comment_usage_note: "", status: "" },
        "CQ" => V2TableRow { value: "CQ", display_name: "Coverage Register Query", definition: "", comment_usage_note: "", status: "" },
        "RU" => V2TableRow { value: "RU", display_name: "Referral authorization", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0554: V2Table = V2Table {
    number: 554,
    metadata: &super::metadata::TABLE_0554_METADATA,
    rows: phf_map! {
        "LATE" => V2TableRow { value: "LATE", display_name: "Late Invoice", definition: "", comment_usage_note: "Over the Payer's published time limit for this invoice", status: "" },
        "NORM" => V2TableRow { value: "NORM", display_name: "Normal submission", definition: "", comment_usage_note: "", status: "" },
        "SUB" => V2TableRow { value: "SUB", display_name: "Subscriber coverage problem", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0555: V2Table = V2Table {
    number: 555,
    metadata: &super::metadata::TABLE_0555_METADATA,
    rows: phf_map! {
        "FS" => V2TableRow { value: "FS", display_name: "Fee for Service", definition: "", comment_usage_note: "", status: "" },
        "SS" => V2TableRow { value: "SS", display_name: "By Session", definition: "", comment_usage_note: "", status: "" },
        "GP" => V2TableRow { value: "GP", display_name: "Group", definition: "", comment_usage_note: "", status: "" },
        "BK" => V2TableRow { value: "BK", display_name: "Block", definition: "", comment_usage_note: "", status: "" },
        "SL" => V2TableRow { value: "SL", display_name: "Salary", definition: "", comment_usage_note: "", status: "" },
        "IN" => V2TableRow { value: "IN", display_name: "Information Only", definition: "", comment_usage_note: "Payee information not required for this Invoice Type", status: "" },
        "NP" => V2TableRow { value: "NP", display_name: "Non Patient", definition: "bulk invoicing f Pharmacy for a care facility.", comment_usage_note: "Invoice without a patient. E.g. or", status: "" },
        "FN" => V2TableRow { value: "FN", display_name: "Final", definition: "Final Invoice", comment_usage_note: "", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Partial", definition: "Partial Invoice", comment_usage_note: "", status: "" },
        "SU" => V2TableRow { value: "SU", display_name: "Supplemental", definition: "Supplementa l Invoice", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0556: V2Table = V2Table {
    number: 556,
    metadata: &super::metadata::TABLE_0556_METADATA,
    rows: phf_map! {
        "AMB" => V2TableRow { value: "AMB", display_name: "AMBULATORY CARE", definition: "", comment_usage_note: "", status: "" },
        "DENT" => V2TableRow { value: "DENT", display_name: "DENTAL", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0557: V2Table = V2Table {
    number: 557,
    metadata: &super::metadata::TABLE_0557_METADATA,
    rows: phf_map! {
        "ORG" => V2TableRow { value: "ORG", display_name: "Payee Organization", definition: "", comment_usage_note: "The payee is not a person/individual/being, but an entity commonly identified as an organization. Examples could be a country healthcare payer, or an insurance company responsible for payment.", status: "" },
        "PERS" => V2TableRow { value: "PERS", display_name: "Person", definition: "", comment_usage_note: "A person/individual/being.", status: "" },
        "PPER" => V2TableRow { value: "PPER", display_name: "Pay Person", definition: "", comment_usage_note: "Person/individual/being responsible for payment.", status: "" },
        "EMPL" => V2TableRow { value: "EMPL", display_name: "Employer", definition: "", comment_usage_note: "A legal entity that controls and directs a worker under an express or implied contract of employment and a salary or wages in compensation. In worker’s compensation cases, the Employer may be the “organization” responsible for paying the healthcare charges for employment related illness or injury.", status: "" },
    },
};

pub static TABLE_0558: V2Table = V2Table {
    number: 558,
    metadata: &super::metadata::TABLE_0558_METADATA,
    rows: phf_map! {
        "PT" => V2TableRow { value: "PT", display_name: "Patient", definition: "", comment_usage_note: "", status: "" },
        "FM" => V2TableRow { value: "FM", display_name: "Family Member", definition: "", comment_usage_note: "", status: "" },
        "SB" => V2TableRow { value: "SB", display_name: "Subscriber", definition: "", comment_usage_note: "", status: "" },
        "GT" => V2TableRow { value: "GT", display_name: "Guarantor", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0559: V2Table = V2Table {
    number: 559,
    metadata: &super::metadata::TABLE_0559_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Processed", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Denied", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Rejected", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0560: V2Table = V2Table {
    number: 560,
    metadata: &super::metadata::TABLE_0560_METADATA,
    rows: phf_map! {
        "FL" => V2TableRow { value: "FL", display_name: "Units", definition: "", comment_usage_note: "", status: "" },
        "HS" => V2TableRow { value: "HS", display_name: "Hours", definition: "", comment_usage_note: "", status: "" },
        "DY" => V2TableRow { value: "DY", display_name: "Days", definition: "", comment_usage_note: "", status: "" },
        "MN" => V2TableRow { value: "MN", display_name: "Month", definition: "", comment_usage_note: "", status: "" },
        "YY" => V2TableRow { value: "YY", display_name: "Years", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0561: V2Table = V2Table {
    number: 561,
    metadata: &super::metadata::TABLE_0561_METADATA,
    rows: phf_map! {
        "DTCTR" => V2TableRow { value: "DTCTR", display_name: "Data Center Number", definition: "", comment_usage_note: "MSP: Data Center Number associated with the Provider's Network Application ID", status: "" },
        "SEQ" => V2TableRow { value: "SEQ", display_name: "Sequence Number", definition: "", comment_usage_note: "MSP: Must be sequential by Data Center Number", status: "" },
        "DGAPP" => V2TableRow { value: "DGAPP", display_name: "Diagnostic Approval Number", definition: "", comment_usage_note: "MSP: assigned by MSP", status: "" },
        "CLCT" => V2TableRow { value: "CLCT", display_name: "R Claim Center", definition: "", comment_usage_note: "", status: "" },
        "ENC" => V2TableRow { value: "ENC", display_name: "Encounter Number", definition: "", comment_usage_note: "", status: "" },
        "OOP" => V2TableRow { value: "OOP", display_name: "Out of Province Indicator", definition: "", comment_usage_note: "", status: "" },
        "GFTH" => V2TableRow { value: "GFTH", display_name: "Good Faith Indicator", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0562: V2Table = V2Table {
    number: 562,
    metadata: &super::metadata::TABLE_0562_METADATA,
    rows: phf_map! {
        "PAPER" => V2TableRow { value: "PAPER", display_name: "Paper documentation to follow", definition: "", comment_usage_note: "", status: "" },
        "EFORM" => V2TableRow { value: "EFORM", display_name: "Electronic form to follow", definition: "", comment_usage_note: "E30 transaction to follow", status: "" },
        "FAX" => V2TableRow { value: "FAX", display_name: "Fax to follow", definition: "", comment_usage_note: "", status: "" },
        "RTADJ" => V2TableRow { value: "RTADJ", display_name: "Real Time Adjudication Processing", definition: "", comment_usage_note: "If permitted by Payer", status: "" },
        "DFADJ" => V2TableRow { value: "DFADJ", display_name: "Deferred Adjudication Processing", definition: "", comment_usage_note: "If permitted by Payer", status: "" },
        "PYRDELA" => V2TableRow { value: "PYRDELA", display_name: "Delayed by a Previous", definition: "", comment_usage_note: "Allows Provider to explain lateness of", status: "" },
        "Y" => V2TableRow { value: "Y", display_name: "Payer", definition: "", comment_usage_note: "Invoice to a subsequent Payer", status: "" },
    },
};

pub static TABLE_0564: V2Table = V2Table {
    number: 564,
    metadata: &super::metadata::TABLE_0564_METADATA,
    rows: phf_map! {
        "EA" => V2TableRow { value: "EA", display_name: "Edit/Adjudication Response", definition: "", comment_usage_note: "Payer adjustment", status: "" },
        "IN" => V2TableRow { value: "IN", display_name: "Information", definition: "", comment_usage_note: "Payer adjustment", status: "" },
        "PA" => V2TableRow { value: "PA", display_name: "Provider Adjustment", definition: "", comment_usage_note: "Provider adjustment", status: "" },
        "PR" => V2TableRow { value: "PR", display_name: "Processing Result", definition: "", comment_usage_note: "Payer adjustment", status: "" },
    },
};

pub static TABLE_0565: V2Table = V2Table {
    number: 565,
    metadata: &super::metadata::TABLE_0565_METADATA,
    rows: phf_map! {
        "PST" => V2TableRow { value: "PST", display_name: "Provincial Sales Tax", definition: "", comment_usage_note: "", status: "" },
        "GST" => V2TableRow { value: "GST", display_name: "Goods and Services Tax", definition: "", comment_usage_note: "", status: "" },
        "HST" => V2TableRow { value: "HST", display_name: "Harmonized Sales Tax", definition: "", comment_usage_note: "", status: "" },
        "DISP" => V2TableRow { value: "DISP", display_name: "Dispensing Fee", definition: "", comment_usage_note: "", status: "" },
        "MKUP" => V2TableRow { value: "MKUP", display_name: "Mark up Fee", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0566: V2Table = V2Table {
    number: 566,
    metadata: &super::metadata::TABLE_0566_METADATA,
    rows: phf_map! {
        "WBL" => V2TableRow { value: "WBL", display_name: "Whole Blood", definition: "", comment_usage_note: "", status: "" },
        "RBC" => V2TableRow { value: "RBC", display_name: "Red Blood Cells", definition: "", comment_usage_note: "", status: "" },
        "PLS" => V2TableRow { value: "PLS", display_name: "Plasma", definition: "", comment_usage_note: "", status: "" },
        "PLT" => V2TableRow { value: "PLT", display_name: "Platelets", definition: "", comment_usage_note: "", status: "" },
        "GRN" => V2TableRow { value: "GRN", display_name: "Granulocytes", definition: "", comment_usage_note: "", status: "" },
        "PSC" => V2TableRow { value: "PSC", display_name: "Peripheral Stem Cells", definition: "", comment_usage_note: "", status: "" },
        "LYM" => V2TableRow { value: "LYM", display_name: "Lymphocytes", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0567: V2Table = V2Table {
    number: 567,
    metadata: &super::metadata::TABLE_0567_METADATA,
    rows: phf_map! {
        "[lb_av]" => V2TableRow { value: "[lb_av]", display_name: "Pound", definition: "", comment_usage_note: "", status: "" },
        "[oz_av]" => V2TableRow { value: "[oz_av]", display_name: "Ounce", definition: "", comment_usage_note: "", status: "" },
        "kg" => V2TableRow { value: "kg", display_name: "Kilogram", definition: "", comment_usage_note: "", status: "" },
        "g" => V2TableRow { value: "g", display_name: "Gram", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0568: V2Table = V2Table {
    number: 568,
    metadata: &super::metadata::TABLE_0568_METADATA,
    rows: phf_map! {
        "l" => V2TableRow { value: "l", display_name: "Liter", definition: "", comment_usage_note: "", status: "" },
        "[pt_us]" => V2TableRow { value: "[pt_us]", display_name: "Pint", definition: "", comment_usage_note: "", status: "" },
        "ml" => V2TableRow { value: "ml", display_name: "Milliliters", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0569: V2Table = V2Table {
    number: 569,
    metadata: &super::metadata::TABLE_0569_METADATA,
    rows: phf_map! {
        "EOB" => V2TableRow { value: "EOB", display_name: "Print on EOB", definition: "to adj the (Ex of han pat", comment_usage_note: "Instructs party print this ustment on EOB planation Benefits) ded to the ient", status: "" },
        "PAT" => V2TableRow { value: "PAT", display_name: "Inform Patient", definition: "", comment_usage_note: "", status: "" },
        "PRO" => V2TableRow { value: "PRO", display_name: "Inform Provider", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0570: V2Table = V2Table {
    number: 570,
    metadata: &super::metadata::TABLE_0570_METADATA,
    rows: phf_map! {
        "CASH" => V2TableRow { value: "CASH", display_name: "Cash", definition: "", comment_usage_note: "", status: "" },
        "CCCA" => V2TableRow { value: "CCCA", display_name: "Credit Card", definition: "", comment_usage_note: "", status: "" },
        "CCH" => V2TableRow { value: "CCH", display_name: "K Cashier 's Check", definition: "", comment_usage_note: "", status: "" },
        "CDA" => V2TableRow { value: "CDA", display_name: "C Credit/Debit Account", definition: "", comment_usage_note: "", status: "" },
        "CHCK" => V2TableRow { value: "CHCK", display_name: "Check", definition: "", comment_usage_note: "", status: "" },
        "DDPO" => V2TableRow { value: "DDPO", display_name: "Direct Deposit", definition: "", comment_usage_note: "EFT", status: "" },
        "DEBC" => V2TableRow { value: "DEBC", display_name: "Debit Card", definition: "", comment_usage_note: "", status: "" },
        "SWFT" => V2TableRow { value: "SWFT", display_name: "Society for Worldwide Interbank Financial Telecommunicati ons (S.W.I.F.T.)", definition: "", comment_usage_note: "", status: "" },
        "TRAC" => V2TableRow { value: "TRAC", display_name: "Traveler's Check", definition: "", comment_usage_note: "", status: "" },
        "VISN" => V2TableRow { value: "VISN", display_name: "VISA Special Electronic Funds Transfer Network", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0571: V2Table = V2Table {
    number: 571,
    metadata: &super::metadata::TABLE_0571_METADATA,
    rows: phf_map! {
        "ACK" => V2TableRow { value: "ACK", display_name: "Acknowledge", definition: "", comment_usage_note: "", status: "" },
        "REJECT" => V2TableRow { value: "REJECT", display_name: "Reject", definition: "", comment_usage_note: "", status: "" },
        "PEND" => V2TableRow { value: "PEND", display_name: "Pending", definition: "", comment_usage_note: "", status: "" },
        "ADJZER" => V2TableRow { value: "ADJZER", display_name: "Adjudicated to Zero", definition: "", comment_usage_note: "", status: "" },
        "ADJSUB" => V2TableRow { value: "ADJSUB", display_name: "Adjudicated as Submitted", definition: "", comment_usage_note: "", status: "" },
        "ADJ" => V2TableRow { value: "ADJ", display_name: "Adjudicated with Adjustments", definition: "", comment_usage_note: "", status: "" },
        "PAID" => V2TableRow { value: "PAID", display_name: "Paid", definition: "", comment_usage_note: "", status: "" },
        "PRED" => V2TableRow { value: "PRED", display_name: "Pre- Determination", definition: "", comment_usage_note: "Indicates that the IPR has been adjudicated but will not be paid. Equivalent to ADJUD (Adjudicate)", status: "" },
    },
};

pub static TABLE_0572: V2Table = V2Table {
    number: 572,
    metadata: &super::metadata::TABLE_0572_METADATA,
    rows: phf_map! {
        "RVAT" => V2TableRow { value: "RVAT", display_name: "Registered in VAT register", definition: "", comment_usage_note: "", status: "" },
        "UVAT" => V2TableRow { value: "UVAT", display_name: "Unregistered in VAT register", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0615: V2Table = V2Table {
    number: 615,
    metadata: &super::metadata::TABLE_0615_METADATA,
    rows: phf_map! {
        "KERB" => V2TableRow { value: "KERB", display_name: "Kerberos Service Ticket", definition: "", comment_usage_note: "Structure defined by RFC 1510", status: "" },
        "SAML" => V2TableRow { value: "SAML", display_name: "Authenticated User Identity Assertion", definition: "", comment_usage_note: "XML structure defined by the OASIS Security Assertion Markup Language (SAML) specification", status: "" },
    },
};

pub static TABLE_0616: V2Table = V2Table {
    number: 616,
    metadata: &super::metadata::TABLE_0616_METADATA,
    rows: phf_map! {
        "M" => V2TableRow { value: "M", display_name: "Moved", definition: "", comment_usage_note: "The individual associated with the address has moved and is no longer reachable at the address", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Added in error", definition: "w i", comment_usage_note: "The address was incorrect and should never have been associated ith the ndividual", status: "" },
        "R" => V2TableRow { value: "R", display_name: "On request", definition: "T a i r t a r f r", comment_usage_note: "he ssociated ndividual equested hat the ddress be emoved rom their ecord", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Corrected", definition: "T h r w c v", comment_usage_note: "he address as been eplaced ith a orrected ersion", status: "" },
    },
};

pub static TABLE_0617: V2Table = V2Table {
    number: 617,
    metadata: &super::metadata::TABLE_0617_METADATA,
    rows: phf_map! {
        "M" => V2TableRow { value: "M", display_name: "Mailing", definition: "", comment_usage_note: "Identifies an address for mail correspondence from a healthcare provider as stipulated by the subject. For example, under the tenets of certain privacy regulations, it is exclusive to the patient and is typically maintained at the encounter or visit level versus the person level as it only has relevance to the specifics of a given encounter. This is an exception category of address in that the patient has stipulated that they want all correspondence relevant to a given encounter sent to this address in lieu of any other address on file. Providers are required to accommodate such requests under HIPAA promulgated regulation. Note that mailing and legal address may be mutually exclusive as defined below. (Implementors are reminded that although the privacy regulation requires the provider to honor such requests, it does require the provider to enquire whether the patient has a preferred address other than the one volunteered.)", status: "" },
        "V" => V2TableRow { value: "V", display_name: "Visit", definition: "", comment_usage_note: "Identifies an address at which the individual is physically located and may be visited.", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Classification", definition: "", comment_usage_note: "Identifies an address used for the purpose of demographic classification or searching. Such addresses frequently contain insufficient information to be used as mailing or visit addresses. For example, they may only indicate country and postal code, without providing a street address.", status: "" },
    },
};

pub static TABLE_0618: V2Table = V2Table {
    number: 618,
    metadata: &super::metadata::TABLE_0618_METADATA,
    rows: phf_map! {
        "LI" => V2TableRow { value: "LI", display_name: "Listed", definition: "", comment_usage_note: "", status: "" },
        "UL" => V2TableRow { value: "UL", display_name: "Unlisted (Should not appear in directories)", definition: "", comment_usage_note: "", status: "" },
        "UP" => V2TableRow { value: "UP", display_name: "Unpublished", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0625: V2Table = V2Table {
    number: 625,
    metadata: &super::metadata::TABLE_0625_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "A ct iv e", definition: "", comment_usage_note: "Item is available to be purchased or issued.", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Pending Inactive", definition: "", comment_usage_note: "Item is not available to be purchased, but is available to be issued.", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Inactive", definition: "", comment_usage_note: "Item is not available to be purchased or issued.", status: "" },
    },
};

pub static TABLE_0634: V2Table = V2Table {
    number: 634,
    metadata: &super::metadata::TABLE_0634_METADATA,
    rows: phf_map! {
        "CRT" => V2TableRow { value: "CRT", display_name: "Critical", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0642: V2Table = V2Table {
    number: 642,
    metadata: &super::metadata::TABLE_0642_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "DOP/DOQ", definition: "", comment_usage_note: "Corresponds to the theory that calculates the appropriate order point and recommends the quantity to order based on system parameters and historical trends. DOP stands for Dynamic Order Point, and DOQ stands for Dynamic Order Quantity.", status: "" },
        "M" => V2TableRow { value: "M", display_name: "MIN/MAX", definition: "", comment_usage_note: "Corresponds to theory - the quantity recommended is the Order Quantity, less the On Hand Quantity, and less the On Order Quantity. The Order Amount is the desired Maximum On Hand Quantity.", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Override", definition: "", comment_usage_note: "The quantity recommended is the Order Quantity, less the On Order Quantity. The Order Amount is the amount to order when the On Hand reaches the Order Point.", status: "" },
    },
};

pub static TABLE_0651: V2Table = V2Table {
    number: 651,
    metadata: &super::metadata::TABLE_0651_METADATA,
    rows: phf_map! {
        "TME" => V2TableRow { value: "TME", display_name: "Time", definition: "", comment_usage_note: "", status: "" },
        "CST" => V2TableRow { value: "CST", display_name: "Cost", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0653: V2Table = V2Table {
    number: 653,
    metadata: &super::metadata::TABLE_0653_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "mm/dd/yy", definition: "", comment_usage_note: "USA standard", status: "" },
        "2" => V2TableRow { value: "2", display_name: "yy.mm.dd", definition: "", comment_usage_note: "ANSI standard", status: "" },
        "3" => V2TableRow { value: "3", display_name: "dd/mm/yy", definition: "", comment_usage_note: "Britain/France standard", status: "" },
        "4" => V2TableRow { value: "4", display_name: "dd.mm.yy", definition: "", comment_usage_note: "Germany standard", status: "" },
        "5" => V2TableRow { value: "5", display_name: "yy/mm/dd", definition: "", comment_usage_note: "Japan standard", status: "" },
        "6" => V2TableRow { value: "6", display_name: "Yymmdd", definition: "", comment_usage_note: "ISO standard", status: "" },
    },
};

pub static TABLE_0657: V2Table = V2Table {
    number: 657,
    metadata: &super::metadata::TABLE_0657_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "EO Gas Sterilizer", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Steam Sterilizer", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Peracetic Acid", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0659: V2Table = V2Table {
    number: 659,
    metadata: &super::metadata::TABLE_0659_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "OR Mode Without Operator", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "OR Mode with Operator", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "CPD Mode Without Operator", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "CPD Mode With Operator", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Offline Mode", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0667: V2Table = V2Table {
    number: 667,
    metadata: &super::metadata::TABLE_0667_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Real Time Values", definition: "", comment_usage_note: "Display data", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Historic Values", definition: "", comment_usage_note: "Print data", status: "" },
    },
};

pub static TABLE_0669: V2Table = V2Table {
    number: 669,
    metadata: &super::metadata::TABLE_0669_METADATA,
    rows: phf_map! {
        "LLD" => V2TableRow { value: "LLD", display_name: "Building a Load", definition: "", comment_usage_note: "A load is being built", status: "" },
        "LCP" => V2TableRow { value: "LCP", display_name: "Load In Process", definition: "", comment_usage_note: "The load is running", status: "" },
        "LCC" => V2TableRow { value: "LCC", display_name: "Load is Complete", definition: "", comment_usage_note: "The load is complete", status: "" },
        "LCN" => V2TableRow { value: "LCN", display_name: "Load Canceled", definition: "", comment_usage_note: "The load is canceled", status: "" },
    },
};

pub static TABLE_0682: V2Table = V2Table {
    number: 682,
    metadata: &super::metadata::TABLE_0682_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Ready", definition: "", comment_usage_note: "Door Locked", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Not Ready", definition: "", comment_usage_note: "Door Unlocked", status: "" },
    },
};

pub static TABLE_0702: V2Table = V2Table {
    number: 702,
    metadata: &super::metadata::TABLE_0702_METADATA,
    rows: phf_map! {
        "FLS" => V2TableRow { value: "FLS", display_name: "Flash", definition: "", comment_usage_note: "Used to quickly sterilize instruments that were dropped during surgery.", status: "" },
        "PRV" => V2TableRow { value: "PRV", display_name: "Prevac", definition: "", comment_usage_note: "A prevac cycle is vacuum assisted.", status: "" },
        "GRV" => V2TableRow { value: "GRV", display_name: "Gravity", definition: "", comment_usage_note: "A gravity cycle is executed at atmospheric pressure.", status: "" },
        "LQD" => V2TableRow { value: "LQD", display_name: "Liquid", definition: "", comment_usage_note: "A cycle specific to sterilizing liquids.", status: "" },
        "EXP" => V2TableRow { value: "EXP", display_name: "Express", definition: "", comment_usage_note: "An express cycle is similar to a flash cycle but the supply item is wrapped.", status: "" },
        "DRT" => V2TableRow { value: "DRT", display_name: "Dart", definition: "", comment_usage_note: "A dart cycle is a special cycle used to test the integrity of the sterilizer chamber to hold a vacuum.", status: "" },
        "DRW" => V2TableRow { value: "DRW", display_name: "Dart Warm-up Cycle", definition: "", comment_usage_note: "A dart warm-up cycle is used to bring the sterilizer chamber up to operating temperature in order to run a dart test cycle.", status: "" },
        "THR" => V2TableRow { value: "THR", display_name: "Thermal", definition: "", comment_usage_note: "", status: "" },
        "ISO" => V2TableRow { value: "ISO", display_name: "Isothermal", definition: "", comment_usage_note: "", status: "" },
        "BWD" => V2TableRow { value: "BWD", display_name: "Bowie-Dick Test", definition: "", comment_usage_note: "A Bowie-Dick test cycle is a special cycle used to test the integrity of the sterilizer chamber to hold a vacuum.", status: "" },
        "LKT" => V2TableRow { value: "LKT", display_name: "Leak Test", definition: "", comment_usage_note: "A leak test cycle tests the integrity of the sterilizer chamber to hold a vacuum over a specific period of time.", status: "" },
        "WFP" => V2TableRow { value: "WFP", display_name: "Wrap/Steam Flush Pressure Pulse (Wrap/SFPP)", definition: "", comment_usage_note: "A Wrap/SFPP cycle uses steam pulses instead of a vacuum during the conditioning phase of the cycle when the supply item is unwrapped.", status: "" },
        "SFP" => V2TableRow { value: "SFP", display_name: "Steam Flush Pressure Pulse", definition: "", comment_usage_note: "An SFPP cycle uses steam pulses instead of a vacuum during the conditioning phase of the cycle when the supply item is wrapped.", status: "" },
        "CMW" => V2TableRow { value: "CMW", display_name: "Chemical Wash", definition: "", comment_usage_note: "A chemical wash cycle.", status: "" },
        "PEA" => V2TableRow { value: "PEA", display_name: "Peracetic Acid", definition: "", comment_usage_note: "A peracetic acid cycle.", status: "" },
        "EOH" => V2TableRow { value: "EOH", display_name: "EO High Temperature", definition: "", comment_usage_note: "", status: "" },
        "EOL" => V2TableRow { value: "EOL", display_name: "EO Low Temperature", definition: "", comment_usage_note: "", status: "" },
        "CRT" => V2TableRow { value: "CRT", display_name: "Cart Wash", definition: "", comment_usage_note: "", status: "" },
        "UTL" => V2TableRow { value: "UTL", display_name: "Utensil Wa sh", definition: "", comment_usage_note: "", status: "" },
        "IST" => V2TableRow { value: "IST", display_name: "Instrument Wash", definition: "", comment_usage_note: "", status: "" },
        "GLS" => V2TableRow { value: "GLS", display_name: "Glassware", definition: "", comment_usage_note: "", status: "" },
        "PLA" => V2TableRow { value: "PLA", display_name: "Plastic Goods Wash", definition: "", comment_usage_note: "", status: "" },
        "ANR" => V2TableRow { value: "ANR", display_name: "Anesthesia/Re spir atory", definition: "", comment_usage_note: "Special Wash cycle", status: "" },
        "GTL" => V2TableRow { value: "GTL", display_name: "Gentle", definition: "", comment_usage_note: "", status: "" },
        "OPW" => V2TableRow { value: "OPW", display_name: "Optional Wash", definition: "", comment_usage_note: "Any Optional Wash cycle", status: "" },
        "BDP" => V2TableRow { value: "BDP", display_name: "Bedpans", definition: "", comment_usage_note: "", status: "" },
        "TRB" => V2TableRow { value: "TRB", display_name: "Tray/Basin", definition: "", comment_usage_note: "", status: "" },
        "GNP" => V2TableRow { value: "GNP", display_name: "Gen. Purpose", definition: "", comment_usage_note: "", status: "" },
        "COD" => V2TableRow { value: "COD", display_name: "Code", definition: "", comment_usage_note: "", status: "" },
        "RNS" => V2TableRow { value: "RNS", display_name: "Rinse", definition: "", comment_usage_note: "", status: "" },
        "2RS" => V2TableRow { value: "2RS", display_name: "Second Rinse", definition: "", comment_usage_note: "", status: "" },
        "DEC" => V2TableRow { value: "DEC", display_name: "Decontamination", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0717: V2Table = V2Table {
    number: 717,
    metadata: &super::metadata::TABLE_0717_METADATA,
    rows: phf_map! {
        "Pers" => V2TableRow { value: "Pers", display_name: "DEID personal de- identified information policy", definition: "Personal policy on collection, access, use, or disclosure of de- identified information as defined by the information subject or by applicable jurisdictional law.", comment_usage_note: "", status: "" },
        "ALL" => V2TableRow { value: "ALL", display_name: "All", definition: "", comment_usage_note: "This code is for backwards compatibility only as of v2.9. If any of 1..* ARV-4 sensitivity codes (Table 0179) apply to the entire message, then ERL is not populated. This emulates the current Table 0717 code “ALL”.", status: "D" },
        "DEM" => V2TableRow { value: "DEM", display_name: "All demographic data", definition: "", comment_usage_note: "This code has been replaced by the v3 concept “DEMO” as of v2.9. Since this is describing a sensitivity, it will be sent in ARV-4.", status: "D" },
        "LOC" => V2TableRow { value: "LOC", display_name: "Patient Location", definition: "", comment_usage_note: "This code has been replaced by the v3 concept “PATLOC” as of v2.9. Since this is describing a sensitivity, it will be sent in ARV-4.", status: "D" },
        "PID-7" => V2TableRow { value: "PID-7", display_name: "Date of Birth", definition: "", comment_usage_note: "This code has been replaced by the v3 concept “DOB” as of v2.9. Since this is describing a sensitivity, it will be sent in ARV-4.", status: "D" },
        "PID-17" => V2TableRow { value: "PID-17", display_name: "Religion", definition: "", comment_usage_note: "This code has been replaced by the v3 concept “REL” as of v2.9. Since this is describing a sensitivity, it will be sent in ARV-4.", status: "D" },
        "HIV" => V2TableRow { value: "HIV", display_name: "HIV status and results", definition: "", comment_usage_note: "This code has been replaced by the v3 concept “HIV” as of v2.9. Since this is describing a sensitivity, it will be sent in ARV-4.", status: "D" },
        "STD" => V2TableRow { value: "STD", display_name: "Sexually transmitted diseases", definition: "", comment_usage_note: "This code has been replaced by the v3 concept “STD” as of v2.9. Since this is describing a sensitivity, it will be sent in ARV-4.", status: "D" },
        "PSY" => V2TableRow { value: "PSY", display_name: "Psychiatric Mental health", definition: "", comment_usage_note: "This code has been replaced by the v3 concept “SPI” as of v2.9. Since this is describing a sensitivity, it will be sent in ARV-4.", status: "D" },
        "DRG" => V2TableRow { value: "DRG", display_name: "Drug", definition: "", comment_usage_note: "This code has been replaced by the v3 concept “DRGIS” as of v2.9. Since this is describing a sensitivity, it will be sent in ARV-4.", status: "D" },
        "SMD" => V2TableRow { value: "SMD", display_name: "Sensitive medical data", definition: "", comment_usage_note: "This code has been replaced by the several concepts that are more granular v3 ActCode_ActPrivacyPolicy_Informat ionSensitivityPolicy code, e.g., DIA (diagnosis information sensitivity) and PRS (patient requested information sensitivity) as of v2.9. Since this is describing a sensitivity, it will be sent in ARV-4.", status: "D" },
        "NO" => V2TableRow { value: "NO", display_name: "None", definition: "", comment_usage_note: "This code is for backwards compatibility only as of v2.9. If no restrictions, don’t send an ARV segment altogether. If sent, ARV-3 is a required element however. To emulate 0717 “NONE”, populate ARV-3 with code from new ActCode_ActPolicyType__ActInfor mationPolicy such as OrgNSI (organizational non- sensitive information policy) or PersNSI (personal non- sensitive information policy). Don’t populate ARV-7. E.g., Device record that is not sensitive. If you have a situation where an ARV-4 is valued, as required by, e.g., an organizational policy related to disclosure of a VIP’s health status or location at a facility, and the VIP has authorized disclosure as public information, one may use OrgPI (organizational public information policy - Organizational policy on collection, access, use, or disclosure of public information as defined by the organization or governing jurisdiction) to value ARV-3 indicating that the policy permits the disclosure of VIP sensitive information as coded in ARV-4.", status: "D" },
        "OO" => V2TableRow { value: "OO", display_name: "Opt out all registries (HIPAA)", definition: "", comment_usage_note: "Map to similar code in new ActCode_ActPolicyType_ActConsen t_ActPrivacyConsentDirective_Regis tryConsentDirective, which are any of these: OPTOUT, OPTOUT E, OOC, OOS", status: "D" },
        "OI" => V2TableRow { value: "OI", display_name: "Opt in all registries (HIPAA)", definition: "", comment_usage_note: "Map to similar code in new ActCode_ActPolicyType_ActConsen t_ActPrivacyConsentDirective_Regis tryConsentDirective, which are any of these: OPTIN, OPTINR, OIC, OIS", status: "D" },
        "JurisIP" => V2TableRow { value: "JurisIP", display_name: "jurisdictional information policy", definition: "Jurisdictional policy on collection, access, use, or disclosure of information as defined by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "JurisCUI" => V2TableRow { value: "JurisCUI", display_name: "jurisdictional controlled unclassified information policy", definition: "Jurisdictional policy on collection, access, .use, or disclosure of controlled unclassified information as defined by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "JurisDEID" => V2TableRow { value: "JurisDEID", display_name: "jurisdictional de- identified information policy", definition: "Jurisdictional policy on collection, access, use, or disclosure of de-identified information as defined by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "JurisLDS" => V2TableRow { value: "JurisLDS", display_name: "jurisdictional limited data set", definition: "Jurisdictional policy on collection, access, use, or disclosure of information in a limited data set as defined by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "JurisNSI" => V2TableRow { value: "JurisNSI", display_name: "jurisdictional non- information policy", definition: "Jurisdictional policy on sensitive collection, access, use, or disclosure of information deemed non-sensitive by applicable jurisdiction law.", comment_usage_note: "", status: "N" },
        "JurisPI" => V2TableRow { value: "JurisPI", display_name: "jurisdictional public information policy", definition: "Jurisdictional policy on collection, access, use, or disclosure of information deemed public by applicable jurisdiction law.", comment_usage_note: "", status: "N" },
        "JurisSP-CUI" => V2TableRow { value: "JurisSP-CUI", display_name: "jurisdictional specified controlled unclassified information policy", definition: "Jurisdictional policy on collection, access, use, or disclosure of specified controlled unclassified information as defined by applicable jurisdictional policy.", comment_usage_note: "", status: "N" },
        "JurisUUI" => V2TableRow { value: "JurisUUI", display_name: "jurisdictional uncontrolled unclassified information policy", definition: "Jurisdictional policy on collection, access, use, or disclosure of uncontrolled unclassified information as defined by applicable jurisdictional policy.", comment_usage_note: "", status: "N" },
        "OrgIP" => V2TableRow { value: "OrgIP", display_name: "organizational information policy", definition: "Organizational policy on collection, access, use, or disclosure of information, which does not conflict with jurisdictional law", comment_usage_note: "", status: "N" },
        "OrgCUI" => V2TableRow { value: "OrgCUI", display_name: "organizational basic controlled unclassified information policy", definition: "Organizational policy on collection, access, use, or disclosure of basic controlled unclassified information as defined by the organization or by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "OrgDEID" => V2TableRow { value: "OrgDEID", display_name: "organizational de- identified information policy", definition: "Organizational policy on collection, access, use, or disclosure of de-identified information as defined by the organization or by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "OrgLDS" => V2TableRow { value: "OrgLDS", display_name: "organizational limited data set information policy", definition: "Organizational policy on collection, access, use, or disclosure of information in a limited data set as defined by the organization or by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "OrgNSI" => V2TableRow { value: "OrgNSI", display_name: "organizational non- information policy", definition: "Organizational policy on sensitive collection, access, use, or disclosure of information deemed non-sensitive by the organization by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "OrgPI" => V2TableRow { value: "OrgPI", display_name: "organizational public information policy", definition: "Organizational policy on collection, access, use, or disclosure of public information as defined by the organization or by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "OrgSP-CUI" => V2TableRow { value: "OrgSP-CUI", display_name: "organizational specified controlled unclassified information policy", definition: "Organizational policy on collection, access, use, or disclosure of specified controlled unclassified information as defined by the organization or by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "OrgUUI" => V2TableRow { value: "OrgUUI", display_name: "organizational uncontrolled unclassified information policy", definition: "Organizational policy on collection, access, use, or disclosure of uncontrolled unclassified information as defined by the organization or by applicable jurisdictional law.", comment_usage_note: "", status: "N" },
        "PersIP" => V2TableRow { value: "PersIP", display_name: "personal information policy", definition: "Personal policy on collection, access, use, or disclosure of information.", comment_usage_note: "", status: "N" },
        "PersNSI" => V2TableRow { value: "PersNSI", display_name: "personal non- sensitive information policy", definition: "Personal policy on collection, access, use, or disclosure of information deemed non-sensitive by the information subject.", comment_usage_note: "", status: "N" },
        "PersLDS" => V2TableRow { value: "PersLDS", display_name: "personal limited data set information policy", definition: "Personal policy personal policy on collection, access, use, or disclosure of information in a limited data set by the information subject.", comment_usage_note: "", status: "N" },
        "PersPI" => V2TableRow { value: "PersPI", display_name: "personal public Personal poli information collection, a policy disclosure of deemed public information s", definition: "cy on ccess, use, or information by the ubject.", comment_usage_note: "N", status: "" },
        "GRANTOR" => V2TableRow { value: "GRANTOR", display_name: "grantor choice A grantor's t", definition: "erms of If the grantor's term of", comment_usage_note: "agreement N", status: "" },
        "CHOICE" => V2TableRow { value: "CHOICE", display_name: "agreement to grantee may a dissent, and include an op grantee to re restrictions Comment: A gr typically is preferred ter agreement whe has control o the agreement grantee must or may be off opportunity t restrict cert", definition: "which a must be accepted in full, ssent or considered \"\"basic consen which may grantee is offered an opp portunity for a extend or restrict certai quest the agreement is consider or extensions. \"\"granular consent\"\". Examples: (1) Healthcare: antor account holder [grantor] able to stipulate any PHR user [grantee] to ms of terms of agreement in ful n the grantor permit a PHR user to exte ver the topic of restrict terms selected b , which a holder or requested by th accept in full (2) Non-healthcare: The o ered an resource server [grantor] o extend or any authorization server ain terms. meet authorization requir stipulated in the grantor agreement.", comment_usage_note: "then this is t\"\". If a ortunity to n terms, then ed A PHR may require accept the l, or may nd or y the account e PHR user. wner of a may require [grantee] to ements 's terms of", status: "" },
        "IMPLIED" => V2TableRow { value: "IMPLIED", display_name: "implied consent A grantor's p to the grante agreement is grantor's beh may result fr expressly ass consent direc from having n assent or dis the grantee. Comment: Impl \"\"implicit\"\" when the beha grantor is un reasonable pe agreement to terms.", definition: "resumed assent Implied consent with no o e's terms of to assent or dissent to c based on the considered \"\"basic consen avior, which Examples: (1) Healthcare: om not patient schedules an appo enting to the with a provider, and eith tive offered, or take the opportunity to e o right to assent or dissent to the sent offered by consent directive, does opportunity to do so, as where emergency care is r ied or simply behaves as though consent occurs [grantor] agrees to the r vior of the to the provider [grantee] derstood by a implicit consent directiv rson to signal injured and unconscious p the grantee's deemed to have assented t emergency treatment by th permitted to do so under jurisdictional laws, e.g. Samaritan laws. (2) Non-h (a) Upon receiving a driv the driver is deemed to h without explicitly consen undergoing field sobriety corporation that does bus foreign nation is deemed deemed to have assented w explicitly consenting to nation's laws.", comment_usage_note: "pportunity N ertain terms is t\"\". (a) A intment er does not xpressly provider's not have an in the case equired, or the patient ights granted in an e. (b) An atient is o ose , Good ealthcare: er's license, ave assented ting to tests. (b) A iness in a to have ithout abide by that", status: "" },
        "IMPLIEDD" => V2TableRow { value: "IMPLIEDD", display_name: "implied consent A grantor's with opportunity to the grant to dissent agreement, w on the grant and includes dissent to c Comment: A g assenting to terms of agr may not exer dissent to g terms or to selected ter grantor may", definition: "presumed assent Implied or \"\"implicit\" ee's terms of an \"\"opportunity to di hich is based when the grantor's beh or's behavior, understood by a reason a right to signal assent to the g ertain terms. agreement whether the requests or the grante rantor further restrictions, the grantee's \"\"granular consent\"\". eement may or Examples: (1) Healthc cise a right to healthcare provider de rantor selected assent to disclosure o grantee's information to family ms to which a friends, but offers an dissent. permits the patient to disclosures.(b) A heal exchanges deems a pati assented to disclosure information for treatm but offers the patient to dissents to disclos provider organizations healthcare: A bank dee customer's assent to s collection, access, us of financial informati requirement of holding account, but provides opportunity to limit t collection, access, us of that information fo purposes.", comment_usage_note: "\" consent with N ssent\"\" occurs avior is able person to rantee's terms of grantor e approves is considered are: (a) A ems a patient's f health members and opportunity or dissent to such th information ent to have of health ent purposes, an opportunity ure to particular . (2) Non- ms a banking pecified e, or disclosure on as a a bank the user an hird-pa r ty e or disclosure r marketing", status: "" },
        "NOCONSE" => V2TableRow { value: "NOCONSE", display_name: "no consent No notificat", definition: "ion or The grantee's terms of", comment_usage_note: "agreement, N", status: "" },
        "NT" => V2TableRow { value: "NT", display_name: "opportunity a grantor to dissent to a of agreement Comment: A \" policy schem opportunity accommodatio individual's and may not Fair Informa Principles [ enabling the object, acce information, or have acco disclosures.", definition: "is provided for may be available to th assent or reviewing the grantee' grantee's terms policies, but there is . which a grantor is app policy directly or abl No Consent\" acknowledge. e provides no Examples: (1) Healthca for Without notification o n of an opportunity to assent preferences, patient's health infor comply with automatically included tion Practice available (often accor FIPP] by rules) through a healt data subject to exchange. Note that th ss collected implied consent, where correct errors, assumed to have consen unting of Without notification o opportunity to assent patient's health infor collected, accessed, u for research, public h fraud prevention, cour enforcement. (2) Non-he Without notification or opportunity to assent o consumer's healthcare o healthcare internet sea aggregated for secondar behavioral tracking and Without notification or opportunity to assent o consumer's location and shopping mall are track tags on purchased items", comment_usage_note: "e grantor by s privacy no notice by rised of the e to re: (a) r an or dissent, a mation is in and ding to certain h information is differs from the patient is ted. (b) r an or dissent, a mation is sed, or disclosed ealth, security, t order, or law althcare: (a) an r dissent, a r non- rches are y uses such as profiling. (b ) an r dissent, a activities in a ed by RFID", status: "" },
        "OPTIN" => V2TableRow { value: "OPTIN", display_name: "opt-in A grantor's terms of an offered by a without an o to dissent t Comment: Acc grantee's te for example, activities, handling cav date, and re policies.", definition: "assent to the Opt-in with no opportun agreement grantor to restrict cer grantee sought by the grantee i pportunity for \"\"basic consent\"\". o any terms. Examples: (1) Healthca [grantor] signs a provi eptance of a consent directive form, rms pertaining, permissible collection, to permissible disclosure activities, purposes of use, handling caveats, and r eats, expiry policies. (2) Non-healt vocation employee [grantor] sign employer's [grantee's] and non-compete agreeme", comment_usage_note: "ity for a N tain permissions s considered re: A patient der's [grantee's] which lists access, use, or purposes of use, evocation hcare: An s an non- disclosure nt.", status: "" },
        "OPTINR" => V2TableRow { value: "OPTINR", display_name: "opt-in with A grantor's restrictions grantee's te agreement wi opportunity certain gran selected ter Comment: A g dissenting t terms of agr may not exer assent to gr approved res grantee's se which a gran dissent.", definition: "assent to the Opt-in with restriction rms of an \"\"granular consent\"\" be th an grantor has an opportun for to dissent to the permissions sought tor or grantee grantee. ms. Examples: (1) Healthcar assent to grantee's con rantor terms for collection, a o the grantee's disclosure of health in eement may or dissents to disclosure cise a right to recipients as allowed b antor's pre- provider's pre-approved trictions or to list. (2) Non-healthcar lected terms to user assents to the cel tor may privacy practices and t but dissents from locat turning off the cell ph capability.", comment_usage_note: "s is considered N cause the ity to narrow by the e: A patient sent directive ccess, use, or formation, and to certain y the restriction e: A cell phone l phone's erms of use, ion tracking by one's tracking", status: "" },
        "OPTOUT" => V2TableRow { value: "OPTOUT", display_name: "opt-out A grantor's terms of agr by a grantee opportunity any terms. Comment: Rej grantee's te agreement pe example, to activities, handling cav date, and re policies.", definition: "dissent to the Opt-out with no opportu eement offered grantor to permit certa without an sought by the grantee i for to assent to \"\"basic consent\"\". Examples: (1) Healthcar [grantor] declines to s ection of a [grantee's] consent dir rms of which lists permissible rtaining, for access, use, or disclos permissible purposes of use, handli purposes of use, revocation policies, an eats, expiry of not assenting. (2) N vocation (a) A patient [grantor sign a provider's [gran directive form, which l permissible collection, disclosure activities, handling caveats, revoc and consequences of not (b) A citizen [grantor] enroll in mandatory gov [grantee] health insura religious beliefs, whic exemption.", comment_usage_note: "nity for a N in permissions s considered e: A patient ign a provider's ective form, collection, ure activities, ng caveats, d consequences on-healthcare: ] declines to tee's] consent ists access, use, or purposes of use, ation policies, assenting. refuses to ernment nce based on h is an", status: "" },
        "OPTOUTE" => V2TableRow { value: "OPTOUTE", display_name: "opt-out with A grantor's exceptions grantee's te agreement ex certain gran selected ter Comment: A r grantee's te agreement wh to certain p sought by th requesting a additional g", definition: "dissent to the Opt-out with exceptions rms of a \"\"granular consent\"\" cept for grantor has an opportun tor or grantee certain permissions sou ms. grantee or request addi terms, while rejecting ejection of a terms. rms of Examples: (1) Healthcar ile assenting [grantor] dissents to a ermissions information exchange co e grantee or directive with the exce pproval of disclosure based on a l rantor terms. to live\"\" shared secret or password], which the give to a provider when (2) Non-healthcare: A s user [grantor] dissents access to their account access to a circle of f", comment_usage_note: "is considered N because the ity to accept ght by the tional grantor other grantee e: A patient health nsent ption of imited \"\"time [e.g., a token patient can seeking care. ocial media from public , but assents to riends.", status: "" },
        "EMRGONL" => V2TableRow { value: "EMRGONL", display_name: "opt-in emergency Privacy cons", definition: "ent directive To specify the scope of", comment_usage_note: "an N", status: "" },
        "Y" => V2TableRow { value: "Y", display_name: "only restricting access, use, personal inf including de information, effects, suc biospecimen material, wh used to iden individual i repository f except for e treatment ge may include during a dis in an emerge department a the glass pu specified by domain polic", definition: "or prohibiting “EMRGONLY” consent dire or disclosure of within a policy domain, ormation, more of the following P -identified codes in the ActReason and personal OID: 2.16.840.1.113883. h as biometrics, ETREAT (Emergency Treat or genetic Description: ich may be To perform one or more tify an on information for prov n a registry or immediately needed heal or all purposes emergent condition. mergency BTG (break the glass) nerally, which Description: treatment To perform policy overr aster, a threat, operations on informati ncy provision of immediatel nd for break health care for an emer rposes of use as affecting potential har applicable patient safety by end u y. not provisioned for thi use. Includes override organizational provisio a o a E t D T o i e r b p p t T D T o o b D D T o i p i M d 1 5", comment_usage_note: "ctive use one or urpose of Use code system 5.8. ment) operations ision of th care for an ide on for y needed gent condition m, death or sers who are s purpose of of ning policies nd may include override of subject f care consent directive restricting ccess. RTREAT (emergency room reatment) escription: o perform one or more operations n information for provision of mmediately needed health care for an mergent condition in an emergency oom or similar emergent care context y end users provisioned for this urpose, which does not constitute as olicy override such as in a \"\"Break he Glass\"\" purpose of use. HREAT (threat) escription: o perform one or more operations n information used to prevent injury r disease to living subjects who may e the target of violence. ISASTER (disaster) escription: o perform one or more operations n information used for provision of mmediately needed health care to a opulation of living subjects located n a disaster zone. ap: An “emergency only” consent irective maps to ISO/TS 7975:2015(E) .13 Exceptional access.\"", status: "" },
        "NOPP" => V2TableRow { value: "NOPP", display_name: "notice of privacy practices payment, operat research, infor exchange, publi disaster, quali reporting; as r law including c law enforcement security, milit authorities; an analytics, mark profiling.", definition: "An implied privacy consent M directive or notification, m which the data subject may d or may not acknowledge. C The notification specifies h permitted actions, which a may include access, use, or a disclosure of any and all w personal information. The notification specifies the scope of personal information, which may include de-identified information, and personal effects, such as biometrics, biospecimen or genetic material, that may be used to identify an individual in a registry or repository. The notification specifies the purposes for which personal information may be used such as treatment, ions, mation c health, ty and safety equired by ourt order, , national ary d for data eting, and", comment_usage_note: "ap: An “implied” consent directive N aps to ISO/TS 17975:2015(E) efinition for “Implied: Consent to ollect, Use and Disclose personal ealth information is implied by the ctions or inactions of the individual nd the circumstances under which it as implied", status: "" },
        "OOC" => V2TableRow { value: "OOC", display_name: "opt-out of An expressed pr personal consent directi information or or prohibiting effect collection personal inform in a registry or including de-id repository information, an effects, such a biospecimen or material, which used to identif individual in a repository for such as treatme operations, res information exc public health, analytics, mark profiling.", definition: "ivacy Useful when a more specific ve restricting jurisdictional or organizati collection of consent directive policy or ation, specified, available, or kno entified example, where an individual d personal to opt-out of access, use, o s biometrics, of some or all of the indivi genetic information by multiple regi may be repositories. y an Map: An “expressed” opt-out registry or collection consent directive purposes ISO/TS 17975:2015(E) definit nt, payment, for “Express or Expressed: earch, to Collect, Use and Disclose hange, health information is expres data by the subject of care” and eting, and or Expressed (and Informed)", comment_usage_note: "N onal form is not wn, for wishes r disclosure dual’s stries and to maps to ions Consent personal sly given “Express Denial”.", status: "" },
        "OOS" => V2TableRow { value: "OOS", display_name: "opt-out of An expressed pr personal consent directi information or or prohibiting effect sharing via or disclosure o a registry or information, in repository identified info personal effect biometrics, bio genetic materia may be used to individual in a repository for such as treatme operations, res information exc public health, analytics, mark profiling", definition: "ivacy Useful when a more specific ve restricting jurisdictional or organizati access, use, consent directive policy or f personal specified, available, or kno cluding de- example, where an individual rmation, and to opt-out of access, use, o s, such as of some or all of the indivi specimen or information by multiple regi l, which repositories. identify an Map: An “expressed” opt-out registry or sharing consent directive ma purposes ISO/TS 17975:2015(E) definit nt, payment, for “Express or Expressed: C earch , to Collect, Use and Disclose hange, health information is expres data by the subject of care” and eting, and or Expressed (and Informed)", comment_usage_note: "N onal form is not wn, for wishes r disclosure dual’s stries and to ps to ions onsent personal sly given “Express Denial”.", status: "" },
        "OIC" => V2TableRow { value: "OIC", display_name: "opt-in to personal An expressed pr information or consent directi effect collection the collection in a registry or all personal in repository including de-id information, an effects, such a biospecimen or material, whi used to ident individual in repository fo such as treat operations, r information e public health analytics, ma profiling.", definition: "ivacy Useful when a more specific ve permitting jurisdictional or organizati of a some or consent directive policy formation, specified, available, or kno entified example, where an individual d personal to opt-in to collection of s s biometrics, of the individual’s informat genetic multiple registries and repo ch may be Map: An “expressed” cons ify an directive maps to ISO/TS a registry or 17975:2015(E) definition r purposes “Express or Expressed: C ment, payment, Collect, Use and Disclos esearch, health information is ex xchange, by the subject of care” , data rketing, and", comment_usage_note: "N onal or form is not wn, for wishes ome or all ion by sitories. ent s for onsent to e personal pressly given and “Opt-in”.", status: "" },
        "OIS" => V2TableRow { value: "OIS", display_name: "opt-in to personal An expressed information or consent direc effect sharing via access, use, a registry or a some or all repository information, identified in personal effe biometrics, b genetic mater may be used individual in repository fo such as treat operations, r information e public health analytics, ma profiling", definition: "privacy Useful when a more speci tive permitting jurisdictional or organi or disclosure of consent directive policy personal specified, available, or including de- example, where an indivi formation, and to opt-in to access, use cts, such as of some or all of the in iospecimen or information by multiple ial, which repositories. to identify an Map: An “expressed” cons a registry or directive maps to ISO/TS r purposes 17975:2015(E) Express or ment, payment, Consent to Collect, Use esearch, personal health informat xchange, expressly given by the s , data and “Opt-in”. rketing, and", comment_usage_note: "fic N zational or form is not known, for dual wishes , or disclosure dividual’s registries and ent Expressed: and Disclose ion is ubject of care", status: "" },
        "42CFRPart2" => V2TableRow { value: "42CFRPart2", display_name: "42 CFR Part 2 A code repres", definition: "enting Used to indicate the leg", comment_usage_note: "al authority N", status: "" },
        "CD" => V2TableRow { value: "CD", display_name: "consent directive consent direc complies with Consent requi https://www.g pkg/CFR-2017- vol1/pdf/CFR- vol1- a US Federal stipulating t elements cont written conse disclosure un regulations i 42 CFR Part 2 (a)Required e written conse consent to a under the reg part may be p electronic an include: (1) The name (2) The speci general desig the part 2 pr entity(ies),", definition: "tive that for assigning security l Section 2.31 governed information. In rements where collection, access po.gov/fdsys/ disclosure of healthcare 2017- sec2-31.pdf, which is “42CFRPart2CD” as the se law label policy code. he policy ent of a Since information govern nt to a individual’s 42 CFR Part der the consent directive has a n Part 2. confidentiality protecti .31 stringent than the norma lements for protection under HIPAA 4 nt. A written Section 164.506 Uses and disclosure to carry out treatment, ulations in this health care operations aper or https://www.gpo.gov/fdsy d must 2017-title45-vol1/pdf/CF title45-vol1- of the patient. the HL7 Confidentiality fic name(s) or (restricted). nation(s) of ogram(s), or individual(s) permitted to make the disclosure. (3) How much and what kind of information is to be disclosed, including an explicit description of the substance use disorder information that may be disclosed. (4) (i) The name(s) of the individual(s) to whom a disclosure is to be made; or (ii)Entities with a treating provider relationship with the patient. If the recipient entity has a treating provider relationship with the patient whose information is being disclosed, such as a hospital, a health care clinic, or a private practice, the name of that entity; or (iii)Entities without a treating provider relationship with the patient. (A) If the recipient entity does not have a treating provider relationship with the patient whose information is being disclosed and is a third- party payer, the name of the entity; or (B) If the recipient entity does not have a treating provider relationship with the patient whose information is being disclosed and is not covered by paragraph (a)(4)(iii)(A) of this section, such as an entity that facilitates the exchange of health information or a research institution, the name(s) of the entity(-ies); and (1) The name(s) of an individual participant(s); or (2) The name(s) of an entity participant(s) that has a treating provider relationship with the patient whose information is being disclosed; or (3) A general designation of an individual or entity participant(s) or class of participants that must be limited to a participant(s) who has a treating provider relationship with the patient whose information is being disclosed. (i) When using a general designation, a statement must be included on the consent form that the patient (or other individual authorized to sign in lieu of the patient), confirms their understanding that, upon their request and consistent with this part, they must be provided a list of entities to which their information has been disclosed pursuant to the general designation (see Section 2.13(d)). (ii) [Reserved] (5) The purpose of the disclosure. In accordance with Section 2.13(a), the disclosure must be limited to that information which is necessary to carry out the stated purpose. (6) A statement that the consent is subject to revocation at any time except to the extent that the part 2 program or other lawful holder of patient identifying information that is permitted to make the disclosure has already acted in reliance on it. Acting in reliance includes the provision of treatment services in reliance on a valid consent to disclose information to a third-party payer (7) The date, event, or condition upon which the consent will expire if not revoked before. This date, event, or condition must ensure that the consent will last no longer than reasonably necessary to serve the purpose for which it is provided. (8) The signature of the patient and, when required for a patient who is a minor, the signature of an individual authorized to give consent under Section 2.14; or, when required for a patient who is incompetent or deceased, the signature of an individual authorized to sign under Section 2.15. Electronic signatures are permitted to the extent that they are not prohibited by any applicable law. (9) The date on which the consent is signed.", comment_usage_note: "abels to this case, , use, or information title42-is governed by an individual’s 42 title42-CFR Part 2.31 consent directive, curity ed by an 2.31 level of on that is more l level of 5 CFR disclosures payment, or s/pkg/CFR- R-2017- sec164-506.pdf, assign code “R”", status: "" },
        "HIPAAAuth" => V2TableRow { value: "HIPAAAuth", display_name: "HIPAA", definition: "A code representing an U", comment_usage_note: "sed to indicate the legal authority", status: "N" },
        "HIPAACons" => V2TableRow { value: "HIPAACons", display_name: "HIPAA Consent A co", definition: "de representing U.S. Used to", comment_usage_note: "indicate the legal authority N", status: "" },
        "entCD" => V2TableRow { value: "entCD", display_name: "Directive Publ Insu Acco (HIP CFR Righ prot heal http pkg/ vol1 vol1 stip whic seek indi will indi info paym oper \"con", definition: "ic Law 104- rance Portability and governe untability Act where c AA) Privacy Rule 45 disclos Section 164.522 is gove ts to request privacy consent ection for protected Section th information “HIPAAC s://www.gpo.gov/fdsys/ label p CFR-2017- /pdf/CFR-2017- - sec164-522.pdf, which CFR Sec ulates the process by confide h a covered entity stringe s agreement from an protect vidual regarding how it Section use and disclose the to carr vidual's protected health health rmation for treatment, https:/ ent, and health care 2017-ti ations is termed a title45 sent. the HL7 (restri", comment_usage_note: "191 Health for assigning security labels to d information. In this case, ollection, access, use, or ure of healthcare information rned by an individual’s directive under 45 CFR 164.522 use onsentCD” as the security olicy code. title45- title45-Since information governed by a 45 tion 164.522 has a level of ntiality protection that is more nt than the normal level of ion under HIPAA 45 CFR 164.506 Uses and disclosures y out treatment, payment, or care operations /www.gpo.gov/fdsys/pkg/CFR- tle45-vol1/pdf/CFR-2017- -vol1- sec164-506.pdf, assign Confidentiality code “R” cted).", status: "" },
        "HIPAAROA" => V2TableRow { value: "HIPAAROA", display_name: "HIPAA Right of A co", definition: "de representing U.S. \"Used t", comment_usage_note: "o indicate the legal authority N", status: "" },
        "HIPAARe" => V2TableRow { value: "HIPAARe", display_name: "se HIPAA A cod", definition: "e representing an Used to ind", comment_usage_note: "icate the legal authority N", status: "" },
        "archAuthCD" => V2TableRow { value: "archAuthCD", display_name: "Authorization for indiv Disclosure for direc Research Consent HIPAA Directive CFR S and d an au https pkg/C vol1/ vol1- is a stipu eleme autho Secti discl resea", definition: "idual’s consent for assigni tive that complies with governed in Privacy rule 45 where colle ection 164.508 Uses disclosure isclosures for which is governed thorization is required HIPAA Autho ://www.gpo.gov/fdsys/ for Researc FR-2017- pdf/CFR-2017- sec164-508.pdf, which security la US Federal law lating the policy Information nts of a valid individual’ rization under this for Disclos on specific to protected b osures for purposes of Rule. If pr rch. such as con under the C HL7 Confide (moderate). See ActCode._Ac yPolicy._Ac acyLaw.HIPA Authorizati HIPAAAuth a Authorizati Uses and Di Identifiabl Covered Hea https://pri v/authoriza", comment_usage_note: "ng security labels to formation. In this case, ction, access, use, or of healthcare information by an individual’s rization for Disclosure h under 45 CFR Se ction title45-164.508 use title45-“HIPAAResearchAuthCD” as the bel policy code. disclosed under an s HIPAA Authorization ure for Research are not y the HIPAA Privacy otected under other laws fidentiality provisions ommon Rule, assign the ntiality code “M” tPolicyType._ActPrivac tPrivacyLaw._ActUSPriv AAuth (HIPAA on for Disclosure). See: nd NIH Sample on Language for Research sclosures of Individually e Health Information by a lth Care Provider vacyruleandresearch.nih.go tion.asp", status: "" },
        "CompoundR" => V2TableRow { value: "CompoundR", display_name: "Compound A cod", definition: "e representing an The Agency", comment_usage_note: "for Healthcare Research N", status: "" },
        "esearchCD" => V2TableRow { value: "esearchCD", display_name: "HIPAA Re search indiv Authorization and direc Informed Consent HIPAA P for Research CFR Sec and dis an auth https:/ pkg/CFR vol1/pd vol1- is a US stipula element authori Section disclos researc with a Federal Adminis partici known a authori", definition: "idual’s consent and Quality tive that complies with the Informe rivacy rule 45 Authorization tion 164.508 Uses Risk Research closures for which of obtaining i orization is required Health Insuran /www.gpo.gov/fdsys/ Accountability -2017- f/CFR-2017- sec164-508.pdf, which information fo Federal law ensuring that ting the policy subjects are i s of a valid that is consis zation under this and regulatory specific to https://www.ah ures for purposes of es/publication h when combined Common Rule or Used to indica Drug for assigning tration consent to governed infor pate in research also where collecti s a compound disclosure of zation. is governed by of access dire Section 164.50 “CompoundResea security label Information or under the Comm protected by t Rule. If prote such as confid under the Comm HL7 Confidenti (moderate). See ActCode._ActPo yPolicy._ActPr acyLaw.HIPAAAu Authorization HIPAAAuth and Authorization Uses and Discl Identifiable H Covered Health https://privac v/authorizatio", comment_usage_note: "(AHRQ) has developed d Consent and Toolkit for Minimal to facilitate the process nformed consent and ce Portability and Act (HIPAA) title45 -authorization from potential research title45-subjects. This toolkit contains r people responsible for potential research nformed in a manner tent with medical ethics guidelines. From rq.gov/sites/default/fil s/files/ictoolkit.pdf. te the legal authority security labels to mation. In this case, on, access, use, or healthcare information an individual’s right ctive under 45 CFR 8 use rchCD” as the policy code. biospecimen disclosed on Rule are not he HIPAA Privacy cted under other laws entiality provisions on Rule, assign the ality code “M” licyType._ActPrivac ivacyLaw._ActUSPriv th (HIPAA for Disclosure). See: NIH Sample Language for Research osures of Individually ealth Information by a Care Provider yruleandresearch.nih.go n.asp", status: "" },
        "MDHHS-" => V2TableRow { value: "MDHHS-", display_name: "Michigan Michiga", definition: "n’s standard For legislativ", comment_usage_note: "e background, current N", status: "" },
        "5515" => V2TableRow { value: "5515", display_name: "Consent to Share consent Behavioral sharing Health informa Information for behavio Care substan Coordination accorda Purposes 129 of while providers required to use standard form (M 5515), they are accept it.", definition: "form for the MDHHS-5515 con of health form, and prov tion specific to see ral health and http://www.mic ce use treatment in 85,7- nce with Public Act 343686-- 2014. In Michigan, are not this new DHHS- required to", comment_usage_note: "sent directive ider and patient FAQs higan.gov/mdhhs/0,58 339-71550_2941_58005- ,00.html", status: "" },
        "GDPRCD" => V2TableRow { value: "GDPRCD", display_name: "GDPR Consent A consent direct Directive compliant with t European Union G Data Protection (GDPR) definitio of the data subj any freely given informed and una indication of th subject’s wishes he or she, by a by a clear affir action, signifie to the processin personal data re him or her. Where processing on consent, the shall be able to that the data su consented to pro his or her perso If the data subj is given in the written declarat also concerns ot the request for be presented in which is clearly distinguishable other matters, i intelligible and accessible form, and plain langua part of such a d which constitute infringement of Regulation shall binding. The data subject the right to wit her consent at a The withdrawal o shall not affect lawfulness of pr based on consent withdrawal. Pri shou them If t is t requ mean clea unne the whic", definition: "ive Article 4.11 GDPR Definitions he https://gdpr-info.eu/art-4-gd eneral 11) ‘consent’ of the data su Regulation means any freely given, speci n: Consent informed and unambiguous indi ect means of the data subject’s wishes , specific, he or she, by a statement or mbiguous affirmative action, signifies e data agreement to the processing o by which personal data relating to him statement or Article 7 GDPR Conditions for mative consent https://gdpr-info.eu/ s agreement gdpr g of Recital 32 Conditions for con lating to https://gdpr-info.eu/recitals Recital 42 Burden of proof an requirements for consent* is based https://gdpr-info.eu/recitals controller Recital 43 Freely given conse demonstrate https://gdpr-info.eu/recitals bject has GDPR Consent Brief https://gd cessing of info.eu/issues/consent/ nal data. Art. 4 GDPR Definitions Art. GDPR Lawfulness of processing ect’s consent 7 GDPR Conditions for consent context of a 8 GDPR Conditions applicable ion which child's consent in relation t her matters, information society services consent shall GDPR Processing of special a manner categories of personal data A GDPR Automated individual from the decision-making, including pr n an Art. 49 GDPR Derogations for easily specific situations using clear Relevant GDPR Recitals: ge. Any (32) Conditions for consent ( eclaration Consent to certain areas of s s an research (38) Special protect this children's personal data (40) not be Lawfulness of data processing Burden of proof and requireme consent (43) Freely given con shall have (50) Further processing of pe hdraw his or data (51) Protecting sensitiv ny time. personal data (54) Processing f consent sensitive data in public heal the (71) Profiling (111) Exceptio ocessing certain cases of internationa before its (155) Processing in the emplo or to giving context (161) Consenting to t consent, the data subject shall be informed thereof. It shall be as easy to withdraw as to give consent. When assessing whether consent is freely given, utmost account shall be taken of whether, inter alia, the performance of a contract, including the provision of a service, is conditional on consent to the processing of personal data that is not necessary for the performance of that contract. Consent should be given by a clear affirmative act establishing a freely given, specific, informed and unambiguous indication of the data subject’s agreement to the processing of personal data relating to him or her, such as by a written statement, including by electronic means, or an oral statement. This could include ticking a box when visiting an internet website, choosing technical settings for information society services or another statement or conduct which clearly indicates in this context the data subject’s acceptance of the proposed processing of his or her personal data. Silence, pre- ticked boxes or inactivity should not therefore constitute consent. Consent should cover all processing activities carried out for the same purpose or purposes. When the processing has multiple purposes, consent ld be given for all of . he data subject’s consent o be given following a est by electronic s, the request must be r, concise and not cessarily disruptive to use of the service for h it is provided.", comment_usage_note: "N pr/ bject fic, cation by which by a clear f or her. art-7- sent* /no-32 d /no-42/> nt* /no-43 pr- 6 Art. Art. to o Art. 9 rt. 22 ofiling 33) cientific ion of (42) nts for sent rsonal e of th sector ns for l transfers yment he participation in clinical trials (171) Repeal of Directive 95/46/EC and transitional provisions", status: "" },
        "GDPRResea" => V2TableRow { value: "GDPRResea", display_name: "GDPR Research A co", definition: "nsent directive that HL7 Purp", comment_usage_note: "ose of Use codes include N", status: "" },
        "rchCD" => V2TableRow { value: "rchCD", display_name: "Consent Directive comp requ dire Euro Data (GDP of t any info indi subj he o by a acti to t pers him GDPR dire cave poss purp proc rese time Ther shou thei area when reco stan rese shou to g cert part the inte", definition: "lies with regulatory speciali irements for a consent which co ctive compliant with the subject’ pean Union General related Protection Regulation research R) definition: Consent See cita he data subject means and belo freely given, specific, Recital rmed and unambiguous scientif cation of the data info.eu/ ect’s wishes by which Recital r she, by a statement or registri clear affirmative https:// on, signifies agreement Recital he processing of research onal data relating to info.eu/ or her. research consent ctive has the additional at that it is often not ible to fully identify the ose of personal data essing for scientific arch purposes at the of data collection. efore, data subjects ld be allowed to give r consent to certain s of scientific research in keeping with gnized ethical dards for scientific arch. Data subjects ld have the opportunity ive their consent only to ain areas of research or s of research projects to extent allowed by the nded purpose.", comment_usage_note: "ze research purposes of use, uld be used to convey a data s purpose of use restrictions to areas of research or parts of projects. tions for GDPRRe searchCD w: 33 Consent to certain areas of ic research https://gdpr- recitals/no-33/> 157 Information from es and scientific research gdpr-info.eu/recitals/no-157 159 Processing for scientific purposes* https://gdpr- recitals/no-159/", status: "" },
        "COMMONR" => V2TableRow { value: "COMMONR", display_name: "Common Rule A code repr", definition: "esenting U.S. Used to indicate the l", comment_usage_note: "egal authority N", status: "" },
        "ULE" => V2TableRow { value: "ULE", display_name: "Federal law research-re policies kn “Common Rul Common Rule Federal reg governing t human subje (codified a CFR part 46 been adopte Federal dep agencies in promote uni understandi compliance subject pro Existing re governing t human subje Drug Admini (FDA)-regul (21 CFR par and 812) ar the Common include sim requirement", definition: "s governing for assigning security lated privacy governed information. own as the where collection, acce e”. The disclosure of healthca is the U.S. or biospecimen is gove ulations Common Rule use he protection of “COMMONRULE” as the se cts in research label policy code. In t Subpart A of 45 biospecimen disclosed ), which has Common Rule are not pr d by 15 U.S. the HIPAA Privacy Rule artments and under other laws such an effort to confidentiality provis formity, Common Rule, assign th ng, and Confidentiality code “ with human See tections. ActReason_ActInformati gulations entReason_ActHealthInf he protection of nagementReason.Purpose cts in Food and HRESCH for applicable stration label purpose of use c ated research ts 50, 56, 312, e separate from Rule but ilar s.", comment_usage_note: "labels to In this case, ss, use, or re information rned by the curity formation or under the otected by . If protected as ions under the e HL7 M” (moderate). onManagem ormationMa OfUse. security odes.\"", status: "" },
        "HIPAANOP" => V2TableRow { value: "HIPAANOP", display_name: "HIPAA notice of A code repr", definition: "esenting U.S. Used to indicate the l", comment_usage_note: "egal authority N", status: "" },
        "P" => V2TableRow { value: "P", display_name: "privacy practices Public Law Insurance P Accountabi (HIPAA) Pr CFR Sectio which stip individual adequate n and disclo health inf be made by entity, an individual covered en with respe health inf Relevant H Rule provi Section 16 Standard: practices. https://ww pkg/CFR-20 vol1/pdf/C vol1-sec16", definition: "104- 191 ortability and HIPAA governed informa lity Act case, where collec ivacy Rule (45 disclosure of heal n 164.520), is governed by a c ulates an HIPAA Notice of Pr ’s right to use “HIPAANOPP” as otice of the uses label policy code. sures of protected ormation that may Information gover the covered HIPAA Notice of Pr d of the has the level of c 's rights and the protection afforde tity's legal duties Section 164.506 - ct to protected disclosures to car ormation. payment, or health IPAA Privacy https://www.gpo.go sions are at 2017-title45-vol1/ 4.520 (a) title45-vol1- Notice of privacy is considered the HL7 Confidentialit w.gpo.gov/fdsys/ (normal). 17- FR-2017- 4-520.pdf", comment_usage_note: "Health for assigning security labels to tion. In this tion, access, use, or thcare information overed entity’s ivacy Practices, the security ned under a ivacy Practices onfidentiality d under the 45 CFR Uses and ry out treatment, care operations v/fdsys/pkg/CFR- pdf/CFR-2017- sec164-506.pdf , which “norm”, assign the y code “N” title45- title45-", status: "" },
        "HIPAAPsyN" => V2TableRow { value: "HIPAAPsyN", display_name: "HIPAA A code rep", definition: "resenting U.S. Used to indicate t", comment_usage_note: "he legal authority N", status: "" },
        "otes" => V2TableRow { value: "otes", display_name: "psychotherapy Public Law notes Insurance Accountabi (HIPAA) Pr CFR Sectio which stip rights of is the sub psychother requires a certain us of that in Definition Psychother CF R Sect https://ww pkg/CFR-20 vol1/pdf/C vol1- Psychother notes reco medium) by provider w health pro documentin the conten during a p session or family cou and that a", definition: "104- 19 Portability and HIPAA governed inf lity Act case, where collec ivacy Rule (45 disclosure of heal n 164.508), is governed by HIP ulates the privacy 164.508 (2) Author an individual who Psychotherapy note ject of https://www.gpo.go apy notes, and 2017-title45-vol1/ uthorization for title45-vol1- es and disclosure “HIPAAPsyNotes” as formation. label policy code. of Since information apy notes 45 HIPAA 45 CFR 164.5 ion 164.501 level of confident w.gpo.gov/fdsys/ is more stringent 17- FR-2017- sec164-501.pdf: to carry out treat apy notes means health care operat rded (in any https://www.gpo.go a health care 2017-title45-vol1/ ho is a mental title45-vol1- fessional the HL7 Confidenti g or analyzing (restricted). ts of conversation rivate counseling a group, joint, or nseling session re separated from the rest of the individual's medical record. Psychotherapy notes excludes medication prescription and monitoring, counseling session start and stop times, the modalities and frequencies of treatment furnished, results of clinical tests, and any summary of the following items: Diagnosis, functional status, the treatment plan, symptoms, prognosis, and progress to date. See Section 164.508 Uses and disclosures for which an authorization is required. (2)Authorization required: Psychotherapy notes https://www.gpo.gov/fdsys/ pkg/CFR-2017- vol1/pdf/CFR-2017- vol1- sec164-508.pdf: Notwithstanding any provision of this subp ar t, other than the transition provisions in Section 164.532, a covered entity must obtain an authorization for any use or disclosure of psychotherapy notes, except: (i) To carry out the following treatment, payment, or health care operations: (A) Use by the originator of the psychotherapy notes for treatment; (B) Use or disclosure by the covered entity for its own training programs in which students, trainees, or practitioners in mental health learn under supervision to practice or improve their skills in group, joint, family, or individual counseling; or (C) Use or disclosure by the covered entity to defend itself in a legal action or other proceeding brought by the individual; and (ii) A use or disclosure that is required by Section 164.502(a)(2)(ii) o r permitted by Section 164.512(a); Section 164.512(d) with respect to the oversight of the originator of the psychotherapy notes; Section 164.512(g)(1); Section 164.512(j)(1)(i).", comment_usage_note: "1 Health for assigning security labels to ormation. In this tion, access, use, or thcare information AA 45 CFR ization required: s v/fdsys/pkg/CFR- pdf/CFR-2017- sec164-506.pdf , use the security governed by a 08 (2) has a iality protection that than the normal title45-level of protection under 45 CFR title45-Section 164.506 Uses and disclosures ment, payment, or ions v/fdsys/pkg/CFR- pdf/CFR-2017- sec164-506.pdf, assign ality code “R” title45- title45-", status: "" },
        "HIPAASelfP" => V2TableRow { value: "HIPAASelfP", display_name: "HIPAA self-pay", definition: "A code representing 45 U", comment_usage_note: "sed to indicate the legal authority N", status: "" },
        "ay" => V2TableRow { value: "ay", display_name: "", definition: "CFR 164.522 Rights to f request privacy protection H for protected health w information, which is a US d Federal law stipulating the i privacy rights of an 1 individual to restrict h disclosure of information 2 related to health care items t or services for which the “ individual pays out of p pocket in full to a health plan or payer. S H See 45 CFR 164.522 o https://www.gpo.gov/fdsys/ m pkg/CFR-2017- vol1/pdf/CFR-2017- vol1- sec164-522.pdf. (vi) A o covered entity must agree o to the request of an h individual to restrict 2 disclosure of protected t health information about t the individual to a health ( plan if: (A) The disclosure is for the purpose of carrying out payment or health care operations and is not otherwise required by law; and (B) The protected health information pertains solely to a health care item or service for which the individual, or person other than the health plan on behalf of the individual, has paid the covered entity in full.", comment_usage_note: "or assigning security labels to IPAA governed information. In this here collection, access, use, or isclosure of healthcare information s governed by HIPAA 45 CFR 64.522 ttps://www.gpo.gov/fdsys/pkg/CFR- 017-title45-vol1/pdf/CFR-2017- itle45- vol1- sec164-522.pdf use HIPAASelfPay” as the security label olicy code. ince information governed by a IPAA 45 CFR 164.522 has a level f confidentiality protection that is ore stringent than the normal level title45-of protection under 45 CFR Section title45-164.506 Uses and disclosures to carry ut treatment, payment, or health care perations ttps://www.gpo.gov/fdsys/pkg/CFR- 017-title45-vol1/pdf/CFR-2017- itle45-vol1- sec164-506.pdf, assign he HL7 Confidentiality code “R” restricted).", status: "" },
        "Title38Secti" => V2TableRow { value: "Title38Secti", display_name: "Title 38 Section A cod", definition: "e representing Title Used to in", comment_usage_note: "dicate the legal authority N", status: "" },
        "on7332" => V2TableRow { value: "on7332", display_name: "7332 38 Se US Fe the p veter treat", definition: "ction 7332, which is a for assign deral law stipulating governed i rivacy rights of where coll ans diagnosed and disclosure ed for substance use is governe", comment_usage_note: "ing security labels to nformation. In this case, ection, access, use, or of healthcare information d by 38 U.S. Code Section", status: "" },
    },
};

pub static TABLE_0725: V2Table = V2Table {
    number: 725,
    metadata: &super::metadata::TABLE_0725_METADATA,
    rows: phf_map! {
        "INT" => V2TableRow { value: "INT", display_name: "Intent", definition: "", comment_usage_note: "Plan to perform a service", status: "" },
        "APT" => V2TableRow { value: "APT", display_name: "Appointment", definition: "", comment_usage_note: "Planned act for specific time and place", status: "" },
        "ARQ" => V2TableRow { value: "ARQ", display_name: "Appointment Request", definition: "", comment_usage_note: "Request for Booking of an Appointment", status: "" },
        "PRMS" => V2TableRow { value: "PRMS", display_name: "Promise", definition: "An intent to pe", comment_usage_note: "rform a service", status: "" },
        "PRP" => V2TableRow { value: "PRP", display_name: "Proposal", definition: "Non-mandated in", comment_usage_note: "tent to perform an act", status: "" },
        "RQO" => V2TableRow { value: "RQO", display_name: "Request-Order", definition: "Request or Orde", comment_usage_note: "r for a service", status: "" },
        "EVN" => V2TableRow { value: "EVN", display_name: "Event", definition: "Service actuall ongoing.", comment_usage_note: "y happens or happened or is", status: "" },
        "EVN.CRT" => V2TableRow { value: "EVN.CRT", display_name: "Event Criterion", definition: "Criterion apply Service to be a This is an Act Similar uses of defined. E.g. Use in Car", comment_usage_note: "ing to Events for another pplied. Mood Predicate. above moods may be e Plans,", status: "" },
        "EXP" => V2TableRow { value: "EXP", display_name: "Expectation", definition: "Expecting that independently o expect a patien", comment_usage_note: "something will occur f deliberate intent. E.g. t will discard medications.", status: "" },
    },
};

pub static TABLE_0728: V2Table = V2Table {
    number: 728,
    metadata: &super::metadata::TABLE_0728_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Nothing obvious", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Low", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Moderate", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "High", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Very high", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0731: V2Table = V2Table {
    number: 731,
    metadata: &super::metadata::TABLE_0731_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Valid code", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Invalid code", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Two primary diagnosis codes", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Invalid for this gender", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Invalid for this age", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0734: V2Table = V2Table {
    number: 734,
    metadata: &super::metadata::TABLE_0734_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Normal grouping", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Invalid or missing primary diagnosis", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Diagnosis is not allowed to be primary", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Data does not fulfill DRG criteria", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Invalid age, admission date, date of birth or discharge date", definition: "", comment_usage_note: "", status: "" },
        "5" => V2TableRow { value: "5", display_name: "Invalid gender", definition: "", comment_usage_note: "", status: "" },
        "6" => V2TableRow { value: "6", display_name: "Invalid discharge status", definition: "", comment_usage_note: "", status: "" },
        "7" => V2TableRow { value: "7", display_name: "Invalid weight ad admission", definition: "", comment_usage_note: "", status: "" },
        "8" => V2TableRow { value: "8", display_name: "Invalid length of stay", definition: "", comment_usage_note: "", status: "" },
        "9" => V2TableRow { value: "9", display_name: "Invalid field \"same day\"", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0739: V2Table = V2Table {
    number: 739,
    metadata: &super::metadata::TABLE_0739_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Normal length of stay", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Short length of stay", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Long length of stay", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0742: V2Table = V2Table {
    number: 742,
    metadata: &super::metadata::TABLE_0742_METADATA,
    rows: phf_map! {
        "00" => V2TableRow { value: "00", display_name: "Effective weight calculated", definition: "", comment_usage_note: "", status: "" },
        "01" => V2TableRow { value: "01", display_name: "Hospital specific contract", definition: "", comment_usage_note: "", status: "" },
        "03" => V2TableRow { value: "03", display_name: "Eeffective weight for transfer/referral calculated", definition: "", comment_usage_note: "", status: "" },
        "04" => V2TableRow { value: "04", display_name: "Referral from other hospital based on a cooperation (no DRG reimbursement)", definition: "", comment_usage_note: "", status: "" },
        "05" => V2TableRow { value: "05", display_name: "Invalid length of stay", definition: "", comment_usage_note: "", status: "" },
        "10" => V2TableRow { value: "10", display_name: "No information/entry in cost data for this DRG", definition: "", comment_usage_note: "", status: "" },
        "11" => V2TableRow { value: "11", display_name: "No relative weight found for department (type)", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0749: V2Table = V2Table {
    number: 749,
    metadata: &super::metadata::TABLE_0749_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Valid code; not used for grouping", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Valid code; used for grouping", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Invalid code; not used for grouping", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Invalid code; code is relevant for grouping", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0755: V2Table = V2Table {
    number: 755,
    metadata: &super::metadata::TABLE_0755_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "No weight reported at admission used for grouping", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Weight reported at admission used for grouping", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Default weight (>2499g) used for grouping", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0757: V2Table = V2Table {
    number: 757,
    metadata: &super::metadata::TABLE_0757_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Respiration minutes not used for grouping", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Listed respiration minutes used for grouping", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "OPS code value used for grouping", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0759: V2Table = V2Table {
    number: 759,
    metadata: &super::metadata::TABLE_0759_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Admission status is valid; used for grouping", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Admission status is valid; not used for grouping", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Admission status is invalid; not used for grouping", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Admission status is invalid; default value used for grouping", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0761: V2Table = V2Table {
    number: 761,
    metadata: &super::metadata::TABLE_0761_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Valid code", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Invalid code", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Not used", definition: "", comment_usage_note: "", status: "" },
        "3" => V2TableRow { value: "3", display_name: "Invalid for this gender", definition: "", comment_usage_note: "", status: "" },
        "4" => V2TableRow { value: "4", display_name: "Invalid for this age", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0763: V2Table = V2Table {
    number: 763,
    metadata: &super::metadata::TABLE_0763_METADATA,
    rows: phf_map! {
        "0" => V2TableRow { value: "0", display_name: "Neither operation relevant nor non-operation relevant procedure", definition: "", comment_usage_note: "", status: "" },
        "1" => V2TableRow { value: "1", display_name: "Operation relevant procedure", definition: "", comment_usage_note: "", status: "" },
        "2" => V2TableRow { value: "2", display_name: "Non-operation relevant procedure", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0776: V2Table = V2Table {
    number: 776,
    metadata: &super::metadata::TABLE_0776_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "A ct iv e", definition: "", comment_usage_note: "Item is available to be purchased or issued.", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Pending Inactive", definition: "", comment_usage_note: "Item is not available to be purchased, but is available to be issued.", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Inactive", definition: "", comment_usage_note: "Item is not available to be purchased or issued.", status: "" },
    },
};

pub static TABLE_0778: V2Table = V2Table {
    number: 778,
    metadata: &super::metadata::TABLE_0778_METADATA,
    rows: phf_map! {
        "EQP" => V2TableRow { value: "EQP", display_name: "Equipment", definition: "", comment_usage_note: "", status: "" },
        "SUP" => V2TableRow { value: "SUP", display_name: "Supply", definition: "", comment_usage_note: "", status: "" },
        "IMP" => V2TableRow { value: "IMP", display_name: "Implant", definition: "", comment_usage_note: "", status: "" },
        "MED" => V2TableRow { value: "MED", display_name: "Medication", definition: "", comment_usage_note: "", status: "" },
        "TDC" => V2TableRow { value: "TDC", display_name: "Tubes, Drains, and Catheters", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0790: V2Table = V2Table {
    number: 790,
    metadata: &super::metadata::TABLE_0790_METADATA,
    rows: phf_map! {
        "FDA" => V2TableRow { value: "FDA", display_name: "Food and Drug Administration", definition: "", comment_usage_note: "", status: "" },
        "AMA" => V2TableRow { value: "AMA", display_name: "American Medical Association", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0793: V2Table = V2Table {
    number: 793,
    metadata: &super::metadata::TABLE_0793_METADATA,
    rows: phf_map! {
        "SMDA" => V2TableRow { value: "SMDA", display_name: "Safe Medical Devices Act", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0806: V2Table = V2Table {
    number: 806,
    metadata: &super::metadata::TABLE_0806_METADATA,
    rows: phf_map! {
        "EOG" => V2TableRow { value: "EOG", display_name: "Ethylene Oxide Gas", definition: "", comment_usage_note: "", status: "" },
        "PCA" => V2TableRow { value: "PCA", display_name: "Peracetic acid", definition: "", comment_usage_note: "", status: "" },
        "STM" => V2TableRow { value: "STM", display_name: "Steam", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0818: V2Table = V2Table {
    number: 818,
    metadata: &super::metadata::TABLE_0818_METADATA,
    rows: phf_map! {
        "CS" => V2TableRow { value: "CS", display_name: "Case", definition: "", comment_usage_note: "", status: "" },
        "BX" => V2TableRow { value: "BX", display_name: "Box", definition: "", comment_usage_note: "", status: "" },
        "EA" => V2TableRow { value: "EA", display_name: "Each", definition: "", comment_usage_note: "", status: "" },
        "SET" => V2TableRow { value: "SET", display_name: "Set", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0834: V2Table = V2Table {
    number: 834,
    metadata: &super::metadata::TABLE_0834_METADATA,
    rows: phf_map! {
        "application" => V2TableRow { value: "application", display_name: "Application data", definition: "", comment_usage_note: "", status: "" },
        "audio" => V2TableRow { value: "audio", display_name: "Audio data", definition: "", comment_usage_note: "", status: "" },
        "image" => V2TableRow { value: "image", display_name: "Image data", definition: "", comment_usage_note: "", status: "" },
        "model" => V2TableRow { value: "model", display_name: "Model data", definition: "", comment_usage_note: "RFC 2077", status: "" },
        "text" => V2TableRow { value: "text", display_name: "Text data", definition: "", comment_usage_note: "", status: "" },
        "video" => V2TableRow { value: "video", display_name: "Video data", definition: "", comment_usage_note: "", status: "" },
        "multipart" => V2TableRow { value: "multipart", display_name: "MIME multipart package", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0868: V2Table = V2Table {
    number: 868,
    metadata: &super::metadata::TABLE_0868_METADATA,
    rows: phf_map! {
        "M" => V2TableRow { value: "M", display_name: "Moved", definition: "", comment_usage_note: "The individual associated with the telecommunication address has moved and is no longer reachable at the address.", status: "" },
        "E" => V2TableRow { value: "E", display_name: "Added in error", definition: "", comment_usage_note: "The telecommunication address was incorrect and should never have been associated with the individual.", status: "" },
        "R" => V2TableRow { value: "R", display_name: "On request", definition: "", comment_usage_note: "The associated individual requested that the telecommunication address be removed from their record (though it may still be correct).", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Corrected", definition: "", comment_usage_note: "The telecommunication address has been replaced with a corrected version.", status: "" },
        "N" => V2TableRow { value: "N", display_name: "No longer in service", definition: "", comment_usage_note: "The telecommunication address is no longer connected or available", status: "" },
    },
};

pub static TABLE_0871: V2Table = V2Table {
    number: 871,
    metadata: &super::metadata::TABLE_0871_METADATA,
    rows: phf_map! {
        "COR" => V2TableRow { value: "COR", display_name: "Corrosive", definition: "", comment_usage_note: "Material is corrosive and may cause severe injury to skin, mucous membranes and eyes. Avoid any unprotected contact.", status: "" },
        "FLA" => V2TableRow { value: "FLA", display_name: "Flammable", definition: "", comment_usage_note: "Material is highly flammable and in certain mixtures (with air) may lead to explosions. Keep away from fire, sparks and excessive heat.", status: "" },
        "EXP" => V2TableRow { value: "EXP", display_name: "Explosive", definition: "", comment_usage_note: "Material is an explosive mixture. Keep away from fire, sparks, and heat.", status: "" },
        "INJ" => V2TableRow { value: "INJ", display_name: "Injury Hazard", definition: "", comment_usage_note: "Material is solid and sharp (e.g., cannulas.) Dispose in hard container.", status: "" },
        "TOX" => V2TableRow { value: "TOX", display_name: "Toxic", definition: "", comment_usage_note: "Material is toxic to humans and/or animals. Special care must be taken to avoid incorporation, even of small amounts.", status: "" },
        "RAD" => V2TableRow { value: "RAD", display_name: "Radioactive", definition: "", comment_usage_note: "Material is a source for ionizing radiation and must be handled with special care to avoid injury of those who handle it and to avoid environmental hazards.", status: "" },
        "UNK" => V2TableRow { value: "UNK", display_name: "Unknown", definition: "", comment_usage_note: "Material hazard level is unknown.", status: "" },
    },
};

pub static TABLE_0881: V2Table = V2Table {
    number: 881,
    metadata: &super::metadata::TABLE_0881_METADATA,
    rows: phf_map! {
        "T" => V2TableRow { value: "T", display_name: "Technical Part", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Professional Part", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Both", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0882: V2Table = V2Table {
    number: 882,
    metadata: &super::metadata::TABLE_0882_METADATA,
    rows: phf_map! {
        "E" => V2TableRow { value: "E", display_name: "Employed", definition: "", comment_usage_note: "", status: "" },
        "SE" => V2TableRow { value: "SE", display_name: "Self-employed", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0894: V2Table = V2Table {
    number: 894,
    metadata: &super::metadata::TABLE_0894_METADATA,
    rows: phf_map! {
        "L" => V2TableRow { value: "L", display_name: "Left", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Right", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0904: V2Table = V2Table {
    number: 904,
    metadata: &super::metadata::TABLE_0904_METADATA,
    rows: phf_map! {
        "BCV" => V2TableRow { value: "BCV", display_name: "Bank Card Validation Number", definition: "", comment_usage_note: "A non-embossed number included on bank cards and used to validate authenticity of the card and the person presenting the card", status: "" },
        "CCS" => V2TableRow { value: "CCS", display_name: "Credit Card Security code", definition: "", comment_usage_note: "", status: "" },
        "VID" => V2TableRow { value: "VID", display_name: "Version ID", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0905: V2Table = V2Table {
    number: 905,
    metadata: &super::metadata::TABLE_0905_METADATA,
    rows: phf_map! {
        "ONH" => V2TableRow { value: "ONH", display_name: "On Hold", definition: "", comment_usage_note: "", status: "" },
        "INV" => V2TableRow { value: "INV", display_name: "Inventoried", definition: "", comment_usage_note: "", status: "" },
        "PRC" => V2TableRow { value: "PRC", display_name: "Processing", definition: "", comment_usage_note: "", status: "" },
        "REJ" => V2TableRow { value: "REJ", display_name: "Rejected", definition: "", comment_usage_note: "", status: "" },
        "TTL" => V2TableRow { value: "TTL", display_name: "Triaged to Lab", definition: "", comment_usage_note: "", status: "" },
        "TRN" => V2TableRow { value: "TRN", display_name: "In Transit", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0906: V2Table = V2Table {
    number: 906,
    metadata: &super::metadata::TABLE_0906_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "ASAP - As soon as possible, next highest priority after stat", definition: "", comment_usage_note: "", status: "" },
        "CR" => V2TableRow { value: "CR", display_name: "Callback results - filler should contact the placer as soon as results are available, even for preliminary results", definition: "", comment_usage_note: "", status: "" },
        "CS" => V2TableRow { value: "CS", display_name: "Callback for scheduling - Filler should contact the placer (or target) to schedule the service.", definition: "", comment_usage_note: "", status: "" },
        "CSP" => V2TableRow { value: "CSP", display_name: "Callback placer for scheduling - filler should contact the placer to schedule the service", definition: "", comment_usage_note: "", status: "" },
        "EL" => V2TableRow { value: "EL", display_name: "Elective - Beneficial to the patient but not essential for survival.", definition: "", comment_usage_note: "", status: "" },
        "EM" => V2TableRow { value: "EM", display_name: "Emergency - An unforeseen combination of circumstances or the resulting state that calls for immediate action", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "Preop - Used to indicate that a service is to Preop – Used to indicate that a be performed prior to a scheduled surgery. service is to be performed When ordering a service and using the pre- prior to a scheduled surgery. op priority, a check is done to see the When ordering a service and amount of time that must be allowed for using the pre-op priority, a performance of the service. When the check is done to see the order amount of time that must be allowed for performance of the service. When the order is placed, a message can be generated indicating the time needed for the service so that it is not ordered in conflict with a scheduled operation.", definition: "", comment_usage_note: "", status: "" },
        "PRN" => V2TableRow { value: "PRN", display_name: "As needed - An \"as needed\" order should be accompanied by a description of what constitutes a need. This description is represented by an observation service predicate as a precondition.", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Routine - Routine service, do at usual work hours", definition: "", comment_usage_note: "", status: "" },
        "RR" => V2TableRow { value: "RR", display_name: "Rush reporting - A report should be prepared and sent as quickly as possible", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Stat - With highest priority (e.g. emergency).", definition: "", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Timing critical - It is critical to come as close as possible to the requested time (e.g. for a through antimicrobial level).", definition: "", comment_usage_note: "", status: "" },
        "UD" => V2TableRow { value: "UD", display_name: "Use as directed - Drug is to be used as directed by the prescriber.", definition: "", comment_usage_note: "", status: "" },
        "UR" => V2TableRow { value: "UR", display_name: "Urgent - Calls for prompt action", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0907: V2Table = V2Table {
    number: 907,
    metadata: &super::metadata::TABLE_0907_METADATA,
    rows: phf_map! {
        "B" => V2TableRow { value: "B", display_name: "Business - Since the service class can represent knowledge structures that may be considered a trade or business secret, there is sometimes (though rarely) the need to flag those items as of business level confidentiality. However, no patient related inf", definition: "Business – Since the service class can represent knowledge structures that may be considered a trade or business secret, there is sometimes (though rarely) the need to flag those items as of business level confidentiality. However, no patient related information may ever be of this confidentiality level.", comment_usage_note: "", status: "" },
        "D can no permis" => V2TableRow { value: "D can no permis", display_name: "Clinician - Only clinicians may see this item, billing and administration persons t access this item without special sion.", definition: "", comment_usage_note: "", status: "" },
        "I" => V2TableRow { value: "I", display_name: "Individual - Access only to individual persons who are mentioned explicitly as actors of this service and whose actor type warrants that access (cf. to actor typed code).", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Low - No patient record item can be of low confidentiality. However, some service objects are not patient related and therefore may have low confidentiality.", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "Normal - Normal confidentiality rules (according to good health care practice) apply, that is, only authorized individuals with a legitimate medical or business need may access this item.", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Restricted - Restricted access, e.g. only to providers having a current care relationship to the patient.", definition: "", comment_usage_note: "", status: "" },
        "V" => V2TableRow { value: "V", display_name: "Very restricted - Very restricted access as declared by the Privacy Officer of the record holder.", definition: "", comment_usage_note: "", status: "" },
        "ETH" => V2TableRow { value: "ETH", display_name: "Substance abuse related - Alcohol/drug- abuse related item", definition: "", comment_usage_note: "", status: "" },
        "HIV" => V2TableRow { value: "HIV", display_name: "HIV Related - HIV and AIDS related item", definition: "", comment_usage_note: "", status: "" },
        "PSY" => V2TableRow { value: "PSY", display_name: "Psychiatry related - Psychiatry related item", definition: "", comment_usage_note: "", status: "" },
        "SDV" => V2TableRow { value: "SDV", display_name: "Sexual and domestic violence related - Sexual assault / domestic violence related item", definition: "", comment_usage_note: "", status: "" },
        "C" => V2TableRow { value: "C", display_name: "Celebrity - Celebrities are people of public interest (VIP) including employees, whose information require special protection.", definition: "", comment_usage_note: "", status: "" },
        "S" => V2TableRow { value: "S", display_name: "Sensitive - Information for which the Sensitive – Informatio patient seeks heightened confidentiality. which the patient seek Sensitive information is not to be shared heightened confidentia with family members. Information Sensitive information reported by the patient about family be shared with family members is sensitive by default. Flag members. Information can be set or cleared reported by the patien family members is sens by default. Flag can cleared on patient's", definition: "n for s lity. is not to t about itive be set or request.", comment_usage_note: "", status: "" },
        "T" => V2TableRow { value: "T", display_name: "Taboo - Information not to be disclosed or discussed with patient except through physician assigned to patient in this case. This is usually a temporary constraint only; example use is a new fatal diagnosis or finding, such as malignancy or HIV.", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0909: V2Table = V2Table {
    number: 909,
    metadata: &super::metadata::TABLE_0909_METADATA,
    rows: phf_map! {
        "STBD" => V2TableRow { value: "STBD", display_name: "Share To Be Determined - Category to be determined", definition: "", comment_usage_note: "NA", status: "" },
        "SIMM" => V2TableRow { value: "SIMM", display_name: "Share Immediately - Share result with patient immediately", definition: "", comment_usage_note: "Immediate", status: "" },
        "SWNL" => V2TableRow { value: "SWNL", display_name: "Share Within Normal Limits - Share result in reference/therapeutic range with patient immediately Share result out of reference/therapeutic ranges with patient after 1 or more business day as agreed to by the systems in play.", definition: "", comment_usage_note: "immediate 1 day", status: "" },
        "SID" => V2TableRow { value: "SID", display_name: "Share In1 Day - Share result regardless of reference/therapeutic range after 1 or more business day as agreed to by the systems in play.", definition: "", comment_usage_note: "1 day", status: "" },
        "SIDC" => V2TableRow { value: "SIDC", display_name: "Share in 1 Day Conditionally - Share result in reference ranges/therapeutic with patient after 1 or more business day as agreed to by the systems in play. Withhold result out of reference/therapeutic range until physician release", definition: "", comment_usage_note: "1 day Withhold", status: "" },
        "SWTH" => V2TableRow { value: "SWTH", display_name: "Share Withhold - Withhold result regardless of reference/therapeutic ranges", definition: "", comment_usage_note: "Withhold", status: "" },
    },
};

pub static TABLE_0912: V2Table = V2Table {
    number: 912,
    metadata: &super::metadata::TABLE_0912_METADATA,
    rows: phf_map! {
        "AAP" => V2TableRow { value: "AAP", display_name: "Alert Acknowledging Provider", definition: "", comment_usage_note: "", status: "" },
        "AC" => V2TableRow { value: "AC", display_name: "Administration Cosigner", definition: "Person that cosigned the prescription", comment_usage_note: "", status: "" },
        "AD" => V2TableRow { value: "AD", display_name: "Admitting Provider", definition: "", comment_usage_note: "This role is used in the PRT associated with the PV1 segment, when more information about PV1-17 Admitting doctor is desired.", status: "" },
        "AHP" => V2TableRow { value: "AHP", display_name: "Authorized Performing Health Professional", definition: "The specific Health Professional who has been approved to perform the ordered services for the patient.", comment_usage_note: "This role is used in the PRT associated with the AUT segment, which represents an authorization or a pre- authorization for a referred procedure or requested service by the payor covering the patient's health care. This code is used when the authorization is for a person.", status: "N" },
        "AI" => V2TableRow { value: "AI", display_name: "Assistant/Alternate Interpreter", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0914: V2Table = V2Table {
    number: 914,
    metadata: &super::metadata::TABLE_0914_METADATA,
    rows: phf_map! {
        "AP" => V2TableRow { value: "AP", display_name: "Analysis Process", definition: "• Equipment or instrument", comment_usage_note: "Enter (AP) when ANALYSIS PROCESS is the reason due to: • Product or supply failure (reagents, calibrators, QC material) • Equipment or instrumentation failure Usage Note: Enter (AP) when ANALYSIS PROCESS is the reason due to: • Product or supply failure (reagents, calibrators, QC material) ation failure", status: "" },
        "IM" => V2TableRow { value: "IM", display_name: "Information Management", definition: "Enter (IM) when INFORMAT is the reason due to: • Database or Programmin • Overriding of test res • Inaccurate calculation • Inaccurate flagging, r measure • Incomplete/inaccurate • Other Usage Note: Enter (IM) w MANAGEMENT is the reaso • Database or Programmin • Overriding of test res • Inaccurate calculation • Inaccurate flagging, r measure • Incomplete/inaccurate • Other", comment_usage_note: "ION MANAGEMENT g issues ults s eference ranges, units of transmission of test results hen INFORMATION n due to: g issues ults s eference ranges, units of transmission of test results", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Laboratory", definition: "Enter (L) when LABORATOR • Data Entry error • Testing/Technical erro • Repeat testing causing Usage Note: Enter (L) wh reason due to: • Data Entry error • Testing/Technical erro • Repeat testing causing", comment_usage_note: "Y is the reason due to: r change to test result en LABORATORY is the r change to test result", status: "" },
        "NA" => V2TableRow { value: "NA", display_name: "Not Applicable", definition: "Enter (NA) when NOT-APPL due to: • If no revisions perfor • unable to determine re Note: Do not use NA if corrected (revised) or i with a correction (revis Usage Note: Enter (NA) w the reason due to: • If no revisions perfor • unable to determine re Note: Do not use NA if r corrected (revised) or i with a correction (revis", comment_usage_note: "ICABLE is the reason med or ason for revision result code status is not f a preliminary release of results ion) hen NOT-APPLICABLE is med or ason for revision esult code status is not f a preliminary release of results ion)", status: "" },
        "PD" => V2TableRow { value: "PD", display_name: "Placer Data", definition: "Enter (PD) when new or c information is the reaso • Changed patient demogr • Result code data provi requisition or specimen during order entry (i.e. Information, Source, etc Usage Note: Enter (PD) w PLACER DATA information • Changed patie • Result code d requisition or during order en Information, So", comment_usage_note: "hanged PLACER DATA n due to: aphics or ded by the client on the manifest that will be entered Previous Biopsy Date, Clinical ...) hen new or changed is the reason due to: nt demographics or ata provided by the client on the specimen manifest that will be entered try (i.e. Previous Biopsy Date, Clinical urce, etc...)", status: "" },
    },
};

pub static TABLE_0916: V2Table = V2Table {
    number: 916,
    metadata: &super::metadata::TABLE_0916_METADATA,
    rows: phf_map! {
        "F" => V2TableRow { value: "F", display_name: "Patient was fasting prior to the procedure.", definition: "", comment_usage_note: "", status: "" },
        "NF" => V2TableRow { value: "NF", display_name: "The patient indicated they did not fast prior to the procedure.", definition: "", comment_usage_note: "", status: "" },
        "NG" => V2TableRow { value: "NG", display_name: "Not Given - Patient was not asked at the time of the procedure.", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0917: V2Table = V2Table {
    number: 917,
    metadata: &super::metadata::TABLE_0917_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "Supplemental", definition: "", comment_usage_note: "", status: "" },
        "L" => V2TableRow { value: "L", display_name: "Loading", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0918: V2Table = V2Table {
    number: 918,
    metadata: &super::metadata::TABLE_0918_METADATA,
    rows: phf_map! {
        "C" => V2TableRow { value: "C", display_name: "Continuous", definition: "", comment_usage_note: "", status: "" },
        "P" => V2TableRow { value: "P", display_name: "PCA Only", definition: "", comment_usage_note: "", status: "" },
        "PC" => V2TableRow { value: "PC", display_name: "PCA + Continuous", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0919: V2Table = V2Table {
    number: 919,
    metadata: &super::metadata::TABLE_0919_METADATA,
    rows: phf_map! {
        "Y" => V2TableRow { value: "Y", display_name: "This test should be exclusive", definition: "", comment_usage_note: "", status: "" },
        "N" => V2TableRow { value: "N", display_name: "This test can be included with any number of other tests", definition: "", comment_usage_note: "Default -.will be assumed when this field is empty", status: "" },
        "D" => V2TableRow { value: "D", display_name: "In some cases, this test should be only exclusively with like tests (examples are cyto or pathology)", definition: "", comment_usage_note: "When D is specified for this field, using field OM1-49 determines how tests must be grouped together. Tests within the same Diagnostic Service Sector may be on the same requisition, and therefore in the same message", status: "" },
    },
};

pub static TABLE_0920: V2Table = V2Table {
    number: 920,
    metadata: &super::metadata::TABLE_0920_METADATA,
    rows: phf_map! {
        "P" => V2TableRow { value: "P", display_name: "Preferred", definition: "", comment_usage_note: "This specimen is Preferred for all attributes (Container and Additive) identified in the OM4 segment.", status: "" },
        "A" => V2TableRow { value: "A", display_name: "Alternate", definition: "", comment_usage_note: "This is a specimen that is acceptable as a replacement for a preferred specimen. In the following field (OM4-17), the sequence number of the preferred specimen must be messaged.", status: "" },
    },
};

pub static TABLE_0921: V2Table = V2Table {
    number: 921,
    metadata: &super::metadata::TABLE_0921_METADATA,
    rows: phf_map! {
        "ADM" => V2TableRow { value: "ADM", display_name: "Admitting", definition: "", comment_usage_note: "", status: "" },
        "SERV" => V2TableRow { value: "SERV", display_name: "Service", definition: "", comment_usage_note: "", status: "" },
        "PRO" => V2TableRow { value: "PRO", display_name: "C Procedure", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0922: V2Table = V2Table {
    number: 922,
    metadata: &super::metadata::TABLE_0922_METADATA,
    rows: phf_map! {
        "IR" => V2TableRow { value: "IR", display_name: "Initial Request", definition: "", comment_usage_note: "", status: "" },
        "RA" => V2TableRow { value: "RA", display_name: "Request for Appeal", definition: "", comment_usage_note: "", status: "" },
        "RE" => V2TableRow { value: "RE", display_name: "Request for Extension", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0923: V2Table = V2Table {
    number: 923,
    metadata: &super::metadata::TABLE_0923_METADATA,
    rows: phf_map! {
        "NIN" => V2TableRow { value: "NIN", display_name: "Process was not interrupted", definition: "", comment_usage_note: "", status: "" },
        "WOT" => V2TableRow { value: "WOT", display_name: "Walk Out: Process interrupted before the Phlebotomist inserts the needle in the Donor's arm", definition: "", comment_usage_note: "", status: "" },
        "ABR" => V2TableRow { value: "ABR", display_name: "Aborted Run: Process interrupted after the Phlebotomist inserts the needle in the Donor's arm", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0924: V2Table = V2Table {
    number: 924,
    metadata: &super::metadata::TABLE_0924_METADATA,
    rows: phf_map! {
        "A" => V2TableRow { value: "A", display_name: "Annual", definition: "", comment_usage_note: "", status: "" },
        "D" => V2TableRow { value: "D", display_name: "Per Day", definition: "", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Per Month", definition: "", comment_usage_note: "", status: "" },
        "O" => V2TableRow { value: "O", display_name: "Duration of the Order", definition: "", comment_usage_note: "Not from UCUM", status: "" },
        "PL" => V2TableRow { value: "PL", display_name: "Patients Lifetime", definition: "", comment_usage_note: "Not from UCUM", status: "" },
        "WK" => V2TableRow { value: "WK", display_name: "Per Week", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0925: V2Table = V2Table {
    number: 925,
    metadata: &super::metadata::TABLE_0925_METADATA,
    rows: phf_map! {
        "INF" => V2TableRow { value: "INF", display_name: "Infiltration", definition: "", comment_usage_note: "", status: "" },
        "VSM" => V2TableRow { value: "VSM", display_name: "Vein Spasm", definition: "", comment_usage_note: "", status: "" },
        "COL" => V2TableRow { value: "COL", display_name: "Collapse", definition: "", comment_usage_note: "", status: "" },
        "MIS" => V2TableRow { value: "MIS", display_name: "Missed / in tissue", definition: "", comment_usage_note: "", status: "" },
        "NAD" => V2TableRow { value: "NAD", display_name: "Needle adjustment (this may not end a procedure, if successful will impact component production)", definition: "", comment_usage_note: "", status: "" },
        "PFL" => V2TableRow { value: "PFL", display_name: "Poor flow", definition: "", comment_usage_note: "", status: "" },
        "CLT" => V2TableRow { value: "CLT", display_name: "Clotted", definition: "", comment_usage_note: "", status: "" },
        "DND" => V2TableRow { value: "DND", display_name: "Defective Needle", definition: "", comment_usage_note: "", status: "" },
        "DBG" => V2TableRow { value: "DBG", display_name: "Defective Bag", definition: "", comment_usage_note: "", status: "" },
        "DAK" => V2TableRow { value: "DAK", display_name: "Defective Apheresis Kit", definition: "", comment_usage_note: "", status: "" },
        "DMT" => V2TableRow { value: "DMT", display_name: "Defective Instrument", definition: "", comment_usage_note: "", status: "" },
        "IPF" => V2TableRow { value: "IPF", display_name: "Instrument Power Failure", definition: "", comment_usage_note: "", status: "" },
        "ACN" => V2TableRow { value: "ACN", display_name: "Air Contamination", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0926: V2Table = V2Table {
    number: 926,
    metadata: &super::metadata::TABLE_0926_METADATA,
    rows: phf_map! {
        "SUC" => V2TableRow { value: "SUC", display_name: "Successful", definition: "", comment_usage_note: "Successful means a complete component was drawn", status: "" },
        "NDR" => V2TableRow { value: "NDR", display_name: "Not Drawn", definition: "", comment_usage_note: "", status: "" },
        "UL5" => V2TableRow { value: "UL5", display_name: "Unsuccessful Less than 50 ml drawn", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0927: V2Table = V2Table {
    number: 927,
    metadata: &super::metadata::TABLE_0927_METADATA,
    rows: phf_map! {
        "L" => V2TableRow { value: "L", display_name: "Left Arm", definition: "", comment_usage_note: "", status: "" },
        "R" => V2TableRow { value: "R", display_name: "Right Arm", definition: "", comment_usage_note: "", status: "" },
        "B" => V2TableRow { value: "B", display_name: "Both Arms", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0929: V2Table = V2Table {
    number: 929,
    metadata: &super::metadata::TABLE_0929_METADATA,
    rows: phf_map! {
        "[lb_av]" => V2TableRow { value: "[lb_av]", display_name: "Pound", definition: "", comment_usage_note: "", status: "" },
        "[oz_av]" => V2TableRow { value: "[oz_av]", display_name: "Ounce", definition: "", comment_usage_note: "", status: "" },
        "kg" => V2TableRow { value: "kg", display_name: "Kilogram", definition: "", comment_usage_note: "", status: "" },
        "g" => V2TableRow { value: "g", display_name: "Gram", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0930: V2Table = V2Table {
    number: 930,
    metadata: &super::metadata::TABLE_0930_METADATA,
    rows: phf_map! {
        "l" => V2TableRow { value: "l", display_name: "Liter", definition: "", comment_usage_note: "", status: "" },
        "[pt_us]" => V2TableRow { value: "[pt_us]", display_name: "Pint", definition: "", comment_usage_note: "", status: "" },
        "ml" => V2TableRow { value: "ml", display_name: "Milliliters", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0931: V2Table = V2Table {
    number: 931,
    metadata: &super::metadata::TABLE_0931_METADATA,
    rows: phf_map! {
        "degF" => V2TableRow { value: "degF", display_name: "Degrees Fahrenheit", definition: "", comment_usage_note: "", status: "" },
        "Cel" => V2TableRow { value: "Cel", display_name: "Degrees Celsius", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0932: V2Table = V2Table {
    number: 932,
    metadata: &super::metadata::TABLE_0932_METADATA,
    rows: phf_map! {
        "min" => V2TableRow { value: "min", display_name: "Minutes", definition: "", comment_usage_note: "", status: "" },
        "s" => V2TableRow { value: "s", display_name: "Seconds", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0933: V2Table = V2Table {
    number: 933,
    metadata: &super::metadata::TABLE_0933_METADATA,
    rows: phf_map! {
        "WBL" => V2TableRow { value: "WBL", display_name: "Whole Blood", definition: "", comment_usage_note: "", status: "" },
        "2RC" => V2TableRow { value: "2RC", display_name: "Double Red Cells", definition: "", comment_usage_note: "", status: "" },
        "PLS" => V2TableRow { value: "PLS", display_name: "Plasma", definition: "", comment_usage_note: "", status: "" },
        "PLT" => V2TableRow { value: "PLT", display_name: "Platelets", definition: "", comment_usage_note: "", status: "" },
        "PNP" => V2TableRow { value: "PNP", display_name: "Platelets and Plasma", definition: "", comment_usage_note: "", status: "" },
        "PNR" => V2TableRow { value: "PNR", display_name: "Platelets and Red Cells", definition: "", comment_usage_note: "", status: "" },
        "PPR" => V2TableRow { value: "PPR", display_name: "Platelets, Plasma, and Red Cells", definition: "", comment_usage_note: "", status: "" },
        "GRN" => V2TableRow { value: "GRN", display_name: "Granulocytes", definition: "", comment_usage_note: "", status: "" },
        "HEM" => V2TableRow { value: "HEM", display_name: "Hemachromatosis", definition: "", comment_usage_note: "", status: "" },
        "HPC" => V2TableRow { value: "HPC", display_name: "Hematopoietic Progenitor Cells", definition: "", comment_usage_note: "Stem Cells and other cells classified as Hematopoietic", status: "" },
        "LYM" => V2TableRow { value: "LYM", display_name: "Lymphocytes", definition: "", comment_usage_note: "", status: "" },
        "THA" => V2TableRow { value: "THA", display_name: "Therapeutic Apheresis", definition: "", comment_usage_note: "", status: "" },
        "THW" => V2TableRow { value: "THW", display_name: "Therapeutic Whole Blood", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0935: V2Table = V2Table {
    number: 935,
    metadata: &super::metadata::TABLE_0935_METADATA,
    rows: phf_map! {
        "NRG" => V2TableRow { value: "NRG", display_name: "No reason given, donor decided to stop without giving a reason", definition: "", comment_usage_note: "", status: "" },
        "PCD" => V2TableRow { value: "PCD", display_name: "Phone Call-Donor", definition: "", comment_usage_note: "", status: "" },
        "DCW" => V2TableRow { value: "DCW", display_name: "Couldn't wait", definition: "", comment_usage_note: "", status: "" },
        "CFT" => V2TableRow { value: "CFT", display_name: "Couldn't follow through with donation (scared)", definition: "", comment_usage_note: "", status: "" },
        "DBB" => V2TableRow { value: "DBB", display_name: "Bathroom", definition: "", comment_usage_note: "", status: "" },
        "DNI" => V2TableRow { value: "DNI", display_name: "Phlebotomy Issue", definition: "", comment_usage_note: "", status: "" },
        "ASC" => V2TableRow { value: "ASC", display_name: "Apheresis Software Crash", definition: "", comment_usage_note: "", status: "" },
        "BSC" => V2TableRow { value: "BS", display_name: "Manufacturing Software Crash", definition: "", comment_usage_note: "", status: "" },
        "GFE" => V2TableRow { value: "GFE", display_name: "General Facility Emergency", definition: "", comment_usage_note: "Power outage, natural disaster (tornado, flood, hurricane, etc.), air conditioning failure, etc.", status: "" },
    },
};

pub static TABLE_0936: V2Table = V2Table {
    number: 936,
    metadata: &super::metadata::TABLE_0936_METADATA,
    rows: phf_map! {
        "QST" => V2TableRow { value: "QST", display_name: "Question", definition: "", comment_usage_note: "Limited to expected responses to questions by the filler", status: "" },
        "RSLT" => V2TableRow { value: "RSLT", display_name: "Result", definition: "", comment_usage_note: "", status: "" },
        "SCI" => V2TableRow { value: "SCI", display_name: "Supporting Clinical Information", definition: "", comment_usage_note: "Placer observations not explicitly requested by the filler to provide context or supporting information", status: "" },
    },
};

pub static TABLE_0937: V2Table = V2Table {
    number: 937,
    metadata: &super::metadata::TABLE_0937_METADATA,
    rows: phf_map! {
        "AOE" => V2TableRow { value: "AOE", display_name: "Ask at Order Entry", definition: "", comment_usage_note: "Sub-type of QST (Question) - OBX-5 value is answer to an Ask at Order Entry question", status: "" },
        "ASC" => V2TableRow { value: "ASC", display_name: "Ask at Specimen Collection", definition: "", comment_usage_note: "Sub-type of QST (Question) - OBX-5 value is answer to an Ask at Specimen Collection question", status: "" },
        "MCS" => V2TableRow { value: "MCS", display_name: "Micro Culture Status", definition: "Sub-type of RSLT (Result) This term identifies observations that give overall culture outcome, when no specific organism is being named.", comment_usage_note: "Examples are: no growth, normal flora", status: "N" },
        "MID" => V2TableRow { value: "MID", display_name: "Micro Isolate Descriptor", definition: "Sub-type of RSLT (Result) This term identifies any other observations about the isolate.", comment_usage_note: "Examples: catalase positive, hemolytic, etc.", status: "N" },
        "MIG" => V2TableRow { value: "MIG", display_name: "Micro Isolate Growth Quantity", definition: "Sub-type of RSLT (Result) This term identifies growth levels of an isolate – can be numeric or categorical.", comment_usage_note: "Examples: <10,000 CFU, >100,00 CFU, Few, Moderate, Many, etc.", status: "N" },
        "MIN" => V2TableRow { value: "MIN", display_name: "Micro Isolate Name", definition: "Sub-type of RSLT (Result) This term identifies observations that name a specific organism either at the species or the genus level.", comment_usage_note: "Examples are: Genus Serratia, Neisseria meningitidis, Escherichia coli O157:H7, but also identification on a culture plate like presumptive Proteus species.", status: "N" },
        "MIR" => V2TableRow { value: "MIR", display_name: "Micro Isolate Related", definition: "", comment_usage_note: "Sub-type of RSLT (Result) - qualifies the result as an isolate.", status: "D" },
        "MIRM" => V2TableRow { value: "MIRM", display_name: "Micro Isolate Related Modifier", definition: "", comment_usage_note: "Sub-type of RSLT (Result) - result is a modifier of the isolate (e.g., Few, Some, etc.)", status: "D" },
        "MNIR" => V2TableRow { value: "MNIR", display_name: "Micro Non- Isolate Related", definition: "", comment_usage_note: "Sub-type of RSLT (Result) - qualifies result as microbiology not related to any isolate (e.g., gram stain observations)", status: "D" },
        "MOD" => V2TableRow { value: "MOD", display_name: "Micro Other Descriptor", definition: "Sub-type of RSLT (Result) This term identifies observations that do not fit the culture status, but is not at the isolate level.", comment_usage_note: "", status: "N" },
        "MSS" => V2TableRow { value: "MSS", display_name: "Micro Sample Stain", definition: "Sub-type of RSLT (Result) This term identifies observations on gram stains (and other stains or smears) on the clinical sample.", comment_usage_note: "Examples are: many gram positive rods, acid fast bacteria seen, etc.", status: "" },
        "SUP" => V2TableRow { value: "SUP", display_name: "Supplemental Result", definition: "", comment_usage_note: "Sub-type of RSLT (Result) - result coming over is additional material, for example points on a graph, an image, raw instrument data, links to related observations, etc.", status: "" },
        "SUR" => V2TableRow { value: "SUR", display_name: "Susceptibility Related", definition: "", comment_usage_note: "Sub-type of RSLT (Result) - result value is for micro susceptibility/sensitivity", status: "" },
        "UNSP" => V2TableRow { value: "UNSP", display_name: "Unspecified", definition: "", comment_usage_note: "Sub-type of RSLT (Result) - result falls outside of the other sub-types.", status: "" },
    },
};

pub static TABLE_0938: V2Table = V2Table {
    number: 938,
    metadata: &super::metadata::TABLE_0938_METADATA,
    rows: phf_map! {
        "ORD" => V2TableRow { value: "ORD", display_name: "Placing the order", definition: "", comment_usage_note: "At time of placing the order", status: "" },
        "DRW" => V2TableRow { value: "DRW", display_name: "Collecting the specimen", definition: "", comment_usage_note: "When the specimen is collected (e.g. fasting status)", status: "" },
    },
};

pub static TABLE_0939: V2Table = V2Table {
    number: 939,
    metadata: &super::metadata::TABLE_0939_METADATA,
    rows: phf_map! {
        "OBR-OBX" => V2TableRow { value: "OBR-OBX", display_name: "OBX segment following an OBR segment", definition: "", comment_usage_note: "For responses related to the test order", status: "" },
        "SPM-OBX" => V2TableRow { value: "SPM-OBX", display_name: "OBX segment following an SPM segment", definition: "", comment_usage_note: "For responses related to the specimen", status: "" },
        "DG1-3" => V2TableRow { value: "DG1-3", display_name: "Diagnosis Code", definition: "", comment_usage_note: "", status: "" },
        "NK1-11" => V2TableRow { value: "NK1-11", display_name: "Next of Kin / Associated Parties Job Code/Class", definition: "", comment_usage_note: "Used to convey patient's or next of kin's employment job class code", status: "" },
        "NK1-13" => V2TableRow { value: "NK1-13", display_name: "Organization Name - NK1", definition: "", comment_usage_note: "Next of kin's organization name", status: "" },
        "NK1-28" => V2TableRow { value: "NK1-28", display_name: "Ethnic Group", definition: "", comment_usage_note: "Next of kin's ethnicity", status: "" },
        "NK1-35" => V2TableRow { value: "NK1-35", display_name: "Race", definition: "", comment_usage_note: "Next of kin's race", status: "" },
        "OBR-16" => V2TableRow { value: "OBR-16", display_name: "Ordering Provider", definition: "", comment_usage_note: "", status: "" },
        "OBR-13" => V2TableRow { value: "OBR-13", display_name: "Relevant Clinical Information", definition: "", comment_usage_note: "The purpose varies based on the AOE referencing this field.", status: "" },
        "OBR-49" => V2TableRow { value: "OBR-49", display_name: "Result Handling", definition: "", comment_usage_note: "This was used for call or fax results back", status: "" },
        "PID-11" => V2TableRow { value: "PID-11", display_name: "Patient Address", definition: "", comment_usage_note: "Applies to patient", status: "" },
        "PID-3" => V2TableRow { value: "PID-3", display_name: "Patient Identifier List", definition: "", comment_usage_note: "Applies to patient", status: "" },
        "PID-5" => V2TableRow { value: "PID-5", display_name: "Patient Name", definition: "", comment_usage_note: "Applies to patient", status: "" },
        "PID-6" => V2TableRow { value: "PID-6", display_name: "Mother's Maiden Name", definition: "", comment_usage_note: "Applies to patient", status: "" },
        "PID-7" => V2TableRow { value: "PID-7", display_name: "Date/Time of Birth", definition: "", comment_usage_note: "Applies to patient", status: "" },
        "PID-13" => V2TableRow { value: "PID-13", display_name: "Phone Number - Home", definition: "", comment_usage_note: "deprecated field as of v2.7", status: "" },
        "PID-14" => V2TableRow { value: "PID-14", display_name: "Phone Number - Business", definition: "", comment_usage_note: "deprecated field as of v2.7", status: "" },
        "PID-40" => V2TableRow { value: "PID-40", display_name: "Phone Number", definition: "", comment_usage_note: "New field in V2.7", status: "" },
        "PRT-5" => V2TableRow { value: "PRT-5", display_name: "Participation Person", definition: "", comment_usage_note: "", status: "" },
        "SPM-4" => V2TableRow { value: "SPM-4", display_name: "Specimen Type", definition: "", comment_usage_note: "", status: "" },
        "SPM-8" => V2TableRow { value: "SPM-8", display_name: "Specimen Source Site", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0940: V2Table = V2Table {
    number: 940,
    metadata: &super::metadata::TABLE_0940_METADATA,
    rows: phf_map! {
        "LCP" => V2TableRow { value: "LCP", display_name: "Limited Coverage Policy", definition: "", comment_usage_note: "", status: "" },
        "NFDA" => V2TableRow { value: "NFDA", display_name: "Non-FDA Approved Diagnositic Procedure", definition: "", comment_usage_note: "", status: "" },
        "FLDP" => V2TableRow { value: "FLDP", display_name: "Frequency Limited Diagnostics Procedure", definition: "", comment_usage_note: "", status: "" },
        "NT" => V2TableRow { value: "NT", display_name: "New Test - Limited Diagnostic History", definition: "", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0942: V2Table = V2Table {
    number: 942,
    metadata: &super::metadata::TABLE_0942_METADATA,
    rows: phf_map! {
        "TA" => V2TableRow { value: "TA", display_name: "Current test availability", definition: "", comment_usage_note: "", status: "" },
        "OB" => V2TableRow { value: "OB", display_name: "Output buffer current capacity", definition: "Current capacity of an output specimen buffer", comment_usage_note: "", status: "" },
        "IC" => V2TableRow { value: "IC", display_name: "Instrument current processing capacity", definition: "Current processing capacity of the instrument", comment_usage_note: "", status: "" },
        "IB" => V2TableRow { value: "IB", display_name: "Input buffer current capacity", definition: "Current capacity of a regular input specimen buffer", comment_usage_note: "", status: "" },
        "EB" => V2TableRow { value: "EB", display_name: "Emergency input buffer current capacity", definition: "Current capacity of an emergency input specimen buffer", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0945: V2Table = V2Table {
    number: 945,
    metadata: &super::metadata::TABLE_0945_METADATA,
    rows: phf_map! {
        "X2" => V2TableRow { value: "X2", display_name: "Dilution factor 2", definition: "Specimen pre-diluted by factor 2 applied on the instrument", comment_usage_note: "Usage Note: corresponds to ^1^:^2 in TCD-2", status: "" },
        "X5" => V2TableRow { value: "X5", display_name: "Dilution factor 5", definition: "Specimen pre-diluted by factor 5 applied on the instrument", comment_usage_note: "Usage Note: corresponds to ^1^:^5 in TCD-2", status: "" },
        "D1" => V2TableRow { value: "D1", display_name: "Dilution factor 1.23", definition: "Specimen pre-diluted by factor 1.23 applied on the instrument", comment_usage_note: "Usage Note: corresponds to ^1^:^1.23 in TCD-2", status: "" },
        "D2" => V2TableRow { value: "D2", display_name: "Dilution factor 1.45", definition: "Specimen pre-diluted by factor 1.45 applied on the instrument", comment_usage_note: "Usage Note: corresponds to ^1^:^1.45 in TCD-2", status: "" },
    },
};

pub static TABLE_0946: V2Table = V2Table {
    number: 946,
    metadata: &super::metadata::TABLE_0946_METADATA,
    rows: phf_map! {
        "D" => V2TableRow { value: "D", display_name: "Distributor", definition: "When a vendor supplies the medical supplies on this contract", comment_usage_note: "", status: "" },
        "M" => V2TableRow { value: "M", display_name: "Manufacturer", definition: "When the Manufacturer supplies the medical supplies on this contract", comment_usage_note: "", status: "" },
    },
};

pub static TABLE_0948: V2Table = V2Table {
    number: 948,
    metadata: &super::metadata::TABLE_0948_METADATA,
    rows: phf_map! {
        "CAUS" => V2TableRow { value: "CAUS", display_name: "Causes/caused", definition: "Source universal service identifier causes/caused the outcome(s) on the target", comment_usage_note: "Example: a procedure caused complication(s)", status: "N" },
        "COMP" => V2TableRow { value: "COMP", display_name: "Component of", definition: "Target universal service identifier is a component of source universal identifier", comment_usage_note: "Example: one care plan activity (target) is a component of another care plan activity (source)", status: "N" },
        "CON" => V2TableRow { value: "CONCR", display_name: "Concurrently Source and ta service ident start concurr", definition: "Source and target universal service identifier should start concurrently", comment_usage_note: "Example: two care plan activities should start concurrently", status: "N" },
        "EVID" => V2TableRow { value: "EVID", display_name: "Evidence", definition: "Source universal service identifier provides evidence for target universal service identifier", comment_usage_note: "Example: observation result provides evidence for certain care plan activity or treatment action", status: "N" },
        "INTF" => V2TableRow { value: "INTF", display_name: "Interferes / interfered", definition: "Source universal service interfered identifier interferes / interfered with fulfilment of target universal service identifier", comment_usage_note: "Example: patient financial or physical constraints interferes / interfered with fulfilment of goal", status: "N" },
        "LIMIT" => V2TableRow { value: "LIMIT", display_name: "Limits/limited", definition: "Source universal service identifier limits/limited the fulfillment of target universal service identifier", comment_usage_note: "Example: patient condition limits the extend that planned treatment can be implemented", status: "N" },
        "SUCCD" => V2TableRow { value: "SUCCD", display_name: "Succeeds", definition: "Target universal service identifier should succeed (starts after end of) source universal service identifier", comment_usage_note: "Example: one care plan activity (target) should start after completion of another (source)", status: "N" },
        "SVTGT" => V2TableRow { value: "SVTGT", display_name: "Service target", definition: "Target universal service identifier is the object of the service identified by the source universal service identifier", comment_usage_note: "Example: An order requests clarification or interpretation of a previous clinical laboratory test result", status: "N" },
        "TRIG" => V2TableRow { value: "TRIG", display_name: "Triggers/triggered", definition: "Source universal service identifier triggers action of target universal service identifier", comment_usage_note: "Example: a bleeding complication triggers the review and change in anti-coagulant dosage", status: "N" },
    },
};

pub static TABLE_0949: V2Table = V2Table {
    number: 949,
    metadata: &super::metadata::TABLE_0949_METADATA,
    rows: phf_map! {
        "CO" => V2TableRow { value: "CO", display_name: "Cost", definition: "Order changed based on cost", comment_usage_note: "In an order replacement context, this would accompany proposal of a similar but lower cost order", status: "N" },
        "ST" => V2TableRow { value: "ST", display_name: "Specimen Type", definition: "Incorrect specimen type submitted for the requested test", comment_usage_note: "Order placer may accept, cancel and replace, or choose other testing. Usage Note: Recommend testing that can use the submitted specimen type", status: "N" },
        "SV" => V2TableRow { value: "SV", display_name: "Specimen Volume", definition: "Provided specimen volume inadequate for testing", comment_usage_note: "The question is how to use the available specimen. The Order Placer may choose a different subset of tests. Usage Note: Recommend a subset of ordered tests appropriate for volume", status: "N" },
        "UN" => V2TableRow { value: "UN", display_name: "Unavailable test", definition: "Requested test not available", comment_usage_note: "In an order replacement context, an alternative might be proposed.", status: "N" },
    },
};

pub static TABLE_0950: V2Table = V2Table {
    number: 950,
    metadata: &super::metadata::TABLE_0950_METADATA,
    rows: phf_map! {
        "EOE" => V2TableRow { value: "EOE", display_name: "Expiration on event", definition: "The order status auto- expires when a specified event occurs", comment_usage_note: "", status: "N" },
        "EOT" => V2TableRow { value: "EOT", display_name: "Expiration on time", definition: "The order status is timed and will auto-expire once the prescribed time interval has passed In an order recommendati (OML), where hold for a r timed.", comment_usage_note: "For example this code would be used to indicate that the order is not currently being worked on but has been placed on a time limited hold awaiting a replacement order. If the hold time expires, default processing will resume. Usage Note: Filler Applications: replacement setting, sent in on for order replacement message ORC-5 = HD, indicating that the esponse to the recommendation is", status: "N" },
    },
};

pub static TABLE_0951: V2Table = V2Table {
    number: 951,
    metadata: &super::metadata::TABLE_0951_METADATA,
    rows: phf_map! {
        "BS" => V2TableRow { value: "BS", display_name: "Bank residual specimen", definition: "Requests that the specimen should be stored long term", comment_usage_note: "", status: "N" },
        "CR" => V2TableRow { value: "CR", display_name: "Confirm results value", definition: "Requests verification of previously reported results", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "N" },
        "FP" => V2TableRow { value: "FP", display_name: "Store residual specimen pending follow up", definition: "Requests that the specimen should be saved for a short duration until follow up is completed", comment_usage_note: "Provides instructions for Specimen storage", status: "N" },
        "IN" => V2TableRow { value: "IN", display_name: "Interpret results", definition: "Requests interpretation of previously reported results", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "N" },
        "IR" => V2TableRow { value: "IR", display_name: "Review clinically inconsistent results", definition: "Requests comparison of previously reported results amongst themselves", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "N" },
        "IT" => V2TableRow { value: "IT", display_name: "Incorrect test performed", definition: "For process improvement work this code can be used to identify when an incorrect test was performed for the target order", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "N" },
        "OP" => V2TableRow { value: "OP", display_name: "Test ordering problem", definition: "For process improvement work this code can be used to identify orders and the respective results, where problems occurred during ordering", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "N" },
        "PI" => V2TableRow { value: "PI", display_name: "Patient identificatio n problem", definition: "For process improvement work this code can be used to identify when a patient identification issue has occurred on the target order", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "N" },
        "SI" => V2TableRow { value: "SI", display_name: "Suspected interference", definition: "Requests verification of previously reported results due to suspected interference", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "N" },
        "SP" => V2TableRow { value: "SP", display_name: "Sampling problem", definition: "For process improvement work this code can be used to identify orders, where problems occurred during sample collection", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "R" },
        "TP" => V2TableRow { value: "TP", display_name: "Specimen transport problem", definition: "For process improvement work this code can be used to identify orders, where problems occurred during sample transport", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "N" },
        "TT" => V2TableRow { value: "TT", display_name: "Turnaround time problem", definition: "For process improvement work this code can be used to identify results with excessive reporting delay", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "N" },
        "XR" => V2TableRow { value: "XR", display_name: "Incorrect results", definition: "For process improvement work this code can be used to identify when incorrect result were reported for the target order", comment_usage_note: "Usage Note: Used to indicate why review is requested", status: "N" },
    },
};

pub static TABLE_0970: V2Table = V2Table {
    number: 970,
    metadata: &super::metadata::TABLE_0970_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Update performed", definition: "English: Update of insurance information on card German: Aktualisierung VSD auf eGK durchgeführt", comment_usage_note: "", status: "N" },
        "2" => V2TableRow { value: "2", display_name: "update not necessary", definition: "English: update of insurance information on card not necessary German: Keine Aktualisierung VSD auf eGK erforderlich", comment_usage_note: "", status: "N" },
        "3" => V2TableRow { value: "3", display_name: "Error", definition: "English: An unrecoverable error occurred during the update or verification of the insurance information German: Aktualisierung VSD auf eGK war nicht möglich", comment_usage_note: "", status: "N" },
    },
};

pub static TABLE_0971: V2Table = V2Table {
    number: 971,
    metadata: &super::metadata::TABLE_0971_METADATA,
    rows: phf_map! {
        "1" => V2TableRow { value: "1", display_name: "Update technically not possible", definition: "English: update of insurance date on card technically not possible German: Aktualisierung VSD auf eGK technisch nicht möglich", comment_usage_note: "", status: "N" },
        "2" => V2TableRow { value: "2", display_name: "Invalid Authentification certificate", definition: "English: authentication certificate is invalid German: Authentifizierungszertifikat eGK ungültig", comment_usage_note: "", status: "N" },
        "3" => V2TableRow { value: "3", display_name: "Online verification technically not possible", definition: "English: online verification of authentication certificate is technically not possible German: Onlineprüfung des Authentifizierungszerti- fikats technisch nicht möglich", comment_usage_note: "", status: "N" },
        "4" => V2TableRow { value: "4", display_name: "Update technically not possible due to offline time exceeded", definition: "English: update of insurance date is technically not possible due to maximum offline time exceeded German: Aktualisierung VSD auf eGK technisch nicht möglich weil maximaler Offline-Zeitraum überschritten", comment_usage_note: "", status: "N" },
    },
};
