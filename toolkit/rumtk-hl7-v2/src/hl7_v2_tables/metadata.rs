//! HL7 v2 coded-content table metadata.

use super::v2_tables::V2MetadataTable;

pub static TABLE_0001_METADATA: V2MetadataTable = V2MetadataTable {
    table: 1usize,
    description: "Table of codes specifying a patient's sex.",
    ttype: "User",
    steward: "PA",
    where_used: "PID-8",
    hl7_version: "2.1",
};

pub static TABLE_0002_METADATA: V2MetadataTable = V2MetadataTable {
    table: 2usize,
    description: "Table of codes specifying a person's marital (civil/legal) status.",
    ttype: "User",
    steward: "PA",
    where_used: "PID-16",
    hl7_version: "2.1",
};

pub static TABLE_0003_METADATA: V2MetadataTable = V2MetadataTable {
    table: 3usize,
    description: "HL7-defined table of codes specifying the trigger event for Version 2.x interface messages.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MSH-9.2",
    hl7_version: "2.1",
};

pub static TABLE_0004_METADATA: V2MetadataTable = V2MetadataTable {
    table: 4usize,
    description: "Table of codes used by systems to categorize patients by site in HL7 Version 2.x interfaces.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-2",
    hl7_version: "2.1",
};

pub static TABLE_0005_METADATA: V2MetadataTable = V2MetadataTable {
    table: 5usize,
    description: "Table of codes specifying the patient's race. These values are suggestions only, they are not required for use in HL7 messages.",
    ttype: "User",
    steward: "PA",
    where_used: "PID-10",
    hl7_version: "2.1",
};

pub static TABLE_0006_METADATA: V2MetadataTable = V2MetadataTable {
    table: 6usize,
    description: "Table of codes specifying a person's religion.",
    ttype: "User",
    steward: "InM",
    where_used: "PID-17",
    hl7_version: "2.1",
};

pub static TABLE_0007_METADATA: V2MetadataTable = V2MetadataTable {
    table: 7usize,
    description: "Table of codes specifying the circumstances under which the patient was or will be admitted.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-4",
    hl7_version: "2.1",
};

pub static TABLE_0008_METADATA: V2MetadataTable = V2MetadataTable {
    table: 8usize,
    description: "HL7-defined table of codes specifying acknowledgment codes used in Version 2.x message.  For details of usage, see message processing rules in the published Standard.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MSA-1.1",
    hl7_version: "2.1",
};

pub static TABLE_0009_METADATA: V2MetadataTable = V2MetadataTable {
    table: 9usize,
    description:
        "Table of codes specifying permanent or transient handicapped conditions of a person.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-15",
    hl7_version: "2.1",
};

pub static TABLE_0010_METADATA: V2MetadataTable = V2MetadataTable {
    table: 10usize,
    description:
        "Table of codes specifying the attending physician information.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-7, SCD-34",
    hl7_version: "2.1",
};

pub static TABLE_0017_METADATA: V2MetadataTable = V2MetadataTable {
    table: 17usize,
    description: "Table of codes specifying a type of financial transaction.",
    ttype: "User",
    steward: "FM",
    where_used: "FT1-6",
    hl7_version: "2.1",
};

pub static TABLE_0018_METADATA: V2MetadataTable = V2MetadataTable {
    table: 18usize,
    description: "Table of codes specifying the patient type.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-18",
    hl7_version: "2.1",
};

pub static TABLE_0019_METADATA: V2MetadataTable = V2MetadataTable {
    table: 19usize,
    description:
        "Table of codes specifying the anesthesia used during the procedure.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "PR1-9",
    hl7_version: "2.1",
};

pub static TABLE_0021_METADATA: V2MetadataTable = V2MetadataTable {
    table: 21usize,
    description: "Table of codes specifying the bad debt agency to which the account was transferred.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-31",
    hl7_version: "2.1",
};

pub static TABLE_0022_METADATA: V2MetadataTable = V2MetadataTable {
    table: 22usize,
    description: "Table of codes specifying whether the particular insurance has been billed and, if so, the type of bill.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN1-32",
    hl7_version: "2.1",
};

pub static TABLE_0023_METADATA: V2MetadataTable = V2MetadataTable {
    table: 23usize,
    description: "Table of codes specifying where the patient was admitted.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-14",
    hl7_version: "2.1",
};

pub static TABLE_0024_METADATA: V2MetadataTable = V2MetadataTable {
    table: 24usize,
    description: "Table of codes specifying the appropriate fee schedule to be used for this transaction posting.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "FT1-17",
    hl7_version: "2.1",
};

pub static TABLE_0027_METADATA: V2MetadataTable = V2MetadataTable {
    table: 27usize,
    description:
        "HL7-defined table of codes specifying the allowed priorities for obtaining the specimen.",
    ttype: "HL7",
    steward: "InM/OO",
    where_used: "OM4-13",
    hl7_version: "2.1",
};

pub static TABLE_0032_METADATA: V2MetadataTable = V2MetadataTable {
    table: 32usize,
    description: "Table of codes specifying which price schedule is to be used for room and bed charges.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-21",
    hl7_version: "2.1",
};

pub static TABLE_0038_METADATA: V2MetadataTable = V2MetadataTable {
    table: 38usize,
    description: "HL7-defined table of codes specifying the status of an order. The purpose of these values are to report the status of an order either upon request (solicited), or when the status changes (unsolicited). The values are not intended to initiate action.  It is assumed that the order status value always reflects the status as it is known to the sending application at the time that a message is sent.  Only the filler can originate these values.",
    ttype: "HL7",
    steward: "OO",
    where_used: "ORC-5",
    hl7_version: "2.1",
};

pub static TABLE_0042_METADATA: V2MetadataTable = V2MetadataTable {
    table: 42usize,
    description:
        "Table of codes specifying an insurance company plan uniquely.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN1-35",
    hl7_version: "2.1",
};

pub static TABLE_0043_METADATA: V2MetadataTable = V2MetadataTable {
    table: 43usize,
    description: "Table of codes specifying the condition code.  These codes are defined by CMS or other regulartory agencies.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "UB2-3",
    hl7_version: "2.1",
};

pub static TABLE_0044_METADATA: V2MetadataTable = V2MetadataTable {
    table: 44usize,
    description: "Table of codes specifying the type of contract entered into by the healthcare facility and the guarantor for the purpose of settling outstanding account balances.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-24",
    hl7_version: "2.1",
};

pub static TABLE_0045_METADATA: V2MetadataTable = V2MetadataTable {
    table: 45usize,
    description: "Table of codes specifying whether the patient will be extended certain special courtesies.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-22",
    hl7_version: "2.1",
};

pub static TABLE_0046_METADATA: V2MetadataTable = V2MetadataTable {
    table: 46usize,
    description: "Table of codes specifying past credit experience.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-23",
    hl7_version: "2.1",
};

pub static TABLE_0049_METADATA: V2MetadataTable = V2MetadataTable {
    table: 49usize,
    description: "Table of codes specifying the department code that controls the transaction code.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "FT1-13",
    hl7_version: "2.1",
};

pub static TABLE_0050_METADATA: V2MetadataTable = V2MetadataTable {
    table: 50usize,
    description: "Table of codes specifying the type of accident.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "ACC-2",
    hl7_version: "2.1",
};

pub static TABLE_0051_METADATA: V2MetadataTable = V2MetadataTable {
    table: 51usize,
    description: "Table of codes specifying the primary diagnosis code for billing purposes.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "FT1-19",
    hl7_version: "2.1",
};

pub static TABLE_0052_METADATA: V2MetadataTable = V2MetadataTable {
    table: 52usize,
    description: "Table of codes that specify a type of diagnosis being sent.",
    ttype: "User",
    steward: "FM",
    where_used: "DG1-6",
    hl7_version: "2.1",
};

pub static TABLE_0055_METADATA: V2MetadataTable = V2MetadataTable {
    table: 55usize,
    description: "Table of codes specifying the diagnostic related group (DRG) for the transaction.  No suggested values.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "DMI-1, DRG-1",
    hl7_version: "2.1",
};

pub static TABLE_0056_METADATA: V2MetadataTable = V2MetadataTable {
    table: 56usize,
    description: "Table of codes specifying that the grouper results have been reviewed and approved.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "DRG-4",
    hl7_version: "2.1",
};

pub static TABLE_0059_METADATA: V2MetadataTable = V2MetadataTable {
    table: 59usize,
    description: "Table of codes specifying the type of consent that was obtained for permission to treat the patient.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "PR1-13",
    hl7_version: "2.1",
};

pub static TABLE_0061_METADATA: V2MetadataTable = V2MetadataTable {
    table: 61usize,
    description: "HL7-defined table of codes specifying the check digit scheme employed.",
    ttype: "HL7",
    steward: "InM",
    where_used: "CX.3, PPN.12, XCN.12, XON.5",
    hl7_version: "2.1",
};

pub static TABLE_0062_METADATA: V2MetadataTable = V2MetadataTable {
    table: 62usize,
    description: "Table of codes which specify the reason for an event.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "EVN-4, FC.1",
    hl7_version: "2.1",
};

pub static TABLE_0063_METADATA: V2MetadataTable = V2MetadataTable {
    table: 63usize,
    description: "Table of codes specifying an actual personal relationship that the next of kin/associated party has to a patient.",
    ttype: "User",
    steward: "PA",
    where_used: "NK1-3, IN1-17",
    hl7_version: "2.1",
};

pub static TABLE_0064_METADATA: V2MetadataTable = V2MetadataTable {
    table: 64usize,
    description:
        "Table of codes specifying the financial class assigned to a person.  No suggested values.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "FC.1, PV1-20",
    hl7_version: "2.1",
};

pub static TABLE_0065_METADATA: V2MetadataTable = V2MetadataTable {
    table: 65usize,
    description: "HL7-defined table of codes which specify actions to be taken with respect to the specimens that accompany or precede an order.  The purpose of these are to further qualify (when appropriate) the general action indicated by the order control code ( table 0119).",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBR-11",
    hl7_version: "2.1",
};

pub static TABLE_0066_METADATA: V2MetadataTable = V2MetadataTable {
    table: 66usize,
    description: "Table of codes specifying the guarantor's employment status.",
    ttype: "User",
    steward: "FM",
    where_used: "GT1-20",
    hl7_version: "2.1",
};

pub static TABLE_0068_METADATA: V2MetadataTable = V2MetadataTable {
    table: 68usize,
    description: "Table of codes specifying the type of guarantor, e.g., individual, institution, etc. No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "DRG-33, GT1-10",
    hl7_version: "2.1",
};

pub static TABLE_0069_METADATA: V2MetadataTable = V2MetadataTable {
    table: 69usize,
    description: "Table of codes specifying the treatment or type of surgery the patient is scheduled to receive.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-10",
    hl7_version: "2.1",
};

pub static TABLE_0072_METADATA: V2MetadataTable = V2MetadataTable {
    table: 72usize,
    description: "Table of codes specifying the identifier of an insurance plan with which a transaction should be associated.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "AUT-1, FT1-14, IN1-2",
    hl7_version: "2.1",
};

pub static TABLE_0073_METADATA: V2MetadataTable = V2MetadataTable {
    table: 73usize,
    description: "Table of codes specifying the amount of interest that will be charged the guarantor on any outstanding amounts.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-28",
    hl7_version: "2.1",
};

pub static TABLE_0074_METADATA: V2MetadataTable = V2MetadataTable {
    table: 74usize,
    description: "HL7-defined table of codes which specify a section of a diagnostic service where an observation may be performed.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBR-24, OM4-49",
    hl7_version: "2.1",
};

pub static TABLE_0076_METADATA: V2MetadataTable = V2MetadataTable {
    table: 76usize,
    description: "HL7-defined table of codes which specify message types.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MSH-9.1",
    hl7_version: "2.1",
};

pub static TABLE_0078_METADATA: V2MetadataTable = V2MetadataTable {
    table: 78usize,
    description:
        "HL7-defined table of codes which specify a categorical assessment of an observation value.",
    ttype: "HL7-EXT",
    steward: "OO",
    where_used: "OBX-8",
    hl7_version: "2.1",
};

pub static TABLE_0080_METADATA: V2MetadataTable = V2MetadataTable {
    table: 80usize,
    description: "HL7-defined table of codes specifying the nature of an abnormal test.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBX-10",
    hl7_version: "2.1",
};

pub static TABLE_0083_METADATA: V2MetadataTable = V2MetadataTable {
    table: 83usize,
    description: "Table of codes specifying the type of outlier (i.e. period of care beyond DRG-standard stay in facility) that has been paid.",
    ttype: "User",
    steward: "PA",
    where_used: "DRG-5",
    hl7_version: "2.1",
};

pub static TABLE_0084_METADATA: V2MetadataTable = V2MetadataTable {
    table: 84usize,
    description: "Table of codes specifying a composite number/name of a person/group that performed a test/procedure/transaction, etc.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "FT1-20",
    hl7_version: "2.1",
};

pub static TABLE_0085_METADATA: V2MetadataTable = V2MetadataTable {
    table: 85usize,
    description: "HL7-defined table of codes which specify observation result status. These codes reflect the current completion status of the results for one Observation Identifier.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBX-11",
    hl7_version: "2.1",
};

pub static TABLE_0086_METADATA: V2MetadataTable = V2MetadataTable {
    table: 86usize,
    description: "Table of codes specifying the coding structure that identifies the various plan types (i.e., Medicare, Medicaid, Blue Cross, HMO, etc.).  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "IN1-15",
    hl7_version: "2.1",
};

pub static TABLE_0087_METADATA: V2MetadataTable = V2MetadataTable {
    table: 87usize,
    description: "Table of codes specifying whether the patient must have pre-admission testing done in order to be admitted.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-13",
    hl7_version: "2.1",
};

pub static TABLE_0088_METADATA: V2MetadataTable = V2MetadataTable {
    table: 88usize,
    description: "Table of codes specifying a unique identifier assigned to a procedure, if any, associated with a charge.",
    ttype: "User",
    steward: "FM",
    where_used: "FT1-25, OBR-44, CDM-7,FT1-25",
    hl7_version: "2.1",
};

pub static TABLE_0091_METADATA: V2MetadataTable = V2MetadataTable {
    table: 91usize,
    description: "HL7-defined table of codes which specify a time frame in which a querry response is expected.",
    ttype: "HL7",
    steward: "InM",
    where_used: "RCP.1",
    hl7_version: "2.1",
};

pub static TABLE_0092_METADATA: V2MetadataTable = V2MetadataTable {
    table: 92usize,
    description: "Table of codes which are used to specify that a patient is being re-admitted to a healthcare facility from which they were discharged, and indicates the circumstances around such re-admission.",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.1",
};

pub static TABLE_0093_METADATA: V2MetadataTable = V2MetadataTable {
    table: 93usize,
    description: "Table of codes specifying whether the healthcare provider can release information about a patient and what information can be released.",
    ttype: "User",
    steward: "FM",
    where_used: "IN1-27",
    hl7_version: "2.1",
};

pub static TABLE_0098_METADATA: V2MetadataTable = V2MetadataTable {
    table: 98usize,
    description: "Table of codes which specify codes to further identify an insurance plan.",
    ttype: "User",
    steward: "FM",
    where_used: "IN1-31",
    hl7_version: "2.1",
};

pub static TABLE_0099_METADATA: V2MetadataTable = V2MetadataTable {
    table: 99usize,
    description: "Table of codes specifying a type of VIP.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-16",
    hl7_version: "2.1",
};

pub static TABLE_0100_METADATA: V2MetadataTable = V2MetadataTable {
    table: 100usize,
    description: "HL7-defined table of codes which specify codes for an event precipitating/triggering a charge activity.",
    ttype: "HL7",
    steward: "InM",
    where_used: "CCD.1, BLG-1",
    hl7_version: "2.1",
};

pub static TABLE_0103_METADATA: V2MetadataTable = V2MetadataTable {
    table: 103usize,
    description: "HL7-defined table of codes which specify whether the message is part of a production, training or debugging system.",
    ttype: "HL7",
    steward: "InM",
    where_used: "PT.1",
    hl7_version: "2.1",
};

pub static TABLE_0104_METADATA: V2MetadataTable = V2MetadataTable {
    table: 104usize,
    description: "HL7-defined table of codes which are used to identify an HL7 version in the Version 2.x family of published standards.",
    ttype: "HL7",
    steward: "InM",
    where_used: "VID.1",
    hl7_version: "2.1",
};

pub static TABLE_0105_METADATA: V2MetadataTable = V2MetadataTable {
    table: 105usize,
    description: "HL7-defined table of codes which are used to specify the source of a comment.",
    ttype: "HL7",
    steward: "InM",
    where_used: "NTE-2",
    hl7_version: "2.1",
};

pub static TABLE_0110_METADATA: V2MetadataTable = V2MetadataTable {
    table: 110usize,
    description: "Table of codes specifying that the account was transferred to bad debts and gives the reason.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-29",
    hl7_version: "2.1",
};

pub static TABLE_0111_METADATA: V2MetadataTable = V2MetadataTable {
    table: 111usize,
    description: "Table of codes specifying that the account was deleted from the file and gives the reason.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-34",
    hl7_version: "2.1",
};

pub static TABLE_0112_METADATA: V2MetadataTable = V2MetadataTable {
    table: 112usize,
    description: "Table of codes which specify the disposition of the patient at time of discharge (i.e., discharged to home, expired, etc.).  No suggested values.  In the US, this field should use the Official Uniform Billing (UB) 04 2008 numeric codes found on form locator 17.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-36",
    hl7_version: "2.1",
};

pub static TABLE_0113_METADATA: V2MetadataTable = V2MetadataTable {
    table: 113usize,
    description: "Table of codes specifying the healthcare facility to which the patient was discharged.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "DLD.1. PV1-37",
    hl7_version: "2.1",
};

pub static TABLE_0114_METADATA: V2MetadataTable = V2MetadataTable {
    table: 114usize,
    description:
        "Table of codes specifying a special diet type for a patient.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "DLD.1. PV1-38",
    hl7_version: "2.1",
};

pub static TABLE_0115_METADATA: V2MetadataTable = V2MetadataTable {
    table: 115usize,
    description: "Table of codes specifying the healthcare facility with which this visit is associated in a multiple facility environment.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "DLD.1. PV1-39",
    hl7_version: "2.1",
};

pub static TABLE_0116_METADATA: V2MetadataTable = V2MetadataTable {
    table: 116usize,
    description: "Table of codes which specify the state of a bed in an inpatient setting, and is used to determine if a patient may be assigned to it or not.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "DLD.1. PV1-40",
    hl7_version: "2.1",
};

pub static TABLE_0117_METADATA: V2MetadataTable = V2MetadataTable {
    table: 117usize,
    description: "Table of codes specifying the account status.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "NPU-2, PV1-41",
    hl7_version: "2.1",
};

pub static TABLE_0118_METADATA: V2MetadataTable = V2MetadataTable {
    table: 118usize,
    description: "Table of codes specifying the major diagnostic category.  No suggested values.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "DMI-2",
    hl7_version: "2.1",
};

pub static TABLE_0119_METADATA: V2MetadataTable = V2MetadataTable {
    table: 119usize,
    description: "HL7-defined table of codes which are used to determine the function of the order segment.  Depending on the message, the action specified by one of these control codes may refer to an order or an individual service.",
    ttype: "HL7",
    steward: "",
    where_used: "ORC-1",
    hl7_version: "2.1",
};

pub static TABLE_0121_METADATA: V2MetadataTable = V2MetadataTable {
    table: 121usize,
    description: "HL7-defined table of codes allowing the placer (sending) application to determine the amount of information to be returned from the filler.",
    ttype: "HL7",
    steward: "",
    where_used: "ORC-6",
    hl7_version: "2.1",
};

pub static TABLE_0122_METADATA: V2MetadataTable = V2MetadataTable {
    table: 122usize,
    description: "HL7-defined table of codes which specify someone or something other than the patient to be billed for a service.",
    ttype: "HL7",
    steward: "OO",
    where_used: "BLG-2",
    hl7_version: "2.1",
};

pub static TABLE_0123_METADATA: V2MetadataTable = V2MetadataTable {
    table: 123usize,
    description: "HL7-defined table of codes which specify a status of results for an order.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBR-25",
    hl7_version: "2.1",
};

pub static TABLE_0124_METADATA: V2MetadataTable = V2MetadataTable {
    table: 124usize,
    description: "HL7-defined table of codes which specify how (or whether) to transport a patient, when applicable, for an ordered service.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBR-30",
    hl7_version: "2.1",
};

pub static TABLE_0125_METADATA: V2MetadataTable = V2MetadataTable {
    table: 125usize,
    description: "HL7-defined table of codes specifying the format of the observation value in the Observation Result (OBX).",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBX-2, OM1-3, OM3-7 and OMC-9",
    hl7_version: "2.1",
};

pub static TABLE_0126_METADATA: V2MetadataTable = V2MetadataTable {
    table: 126usize,
    description: "HL7-defined table of codes which specify the maximum length of a query response that can be accepted by a requesting system, and are expressed as units of mesaure of query response objects.",
    ttype: "HL7",
    steward: "InM",
    where_used: "RCP.2",
    hl7_version: "2.1",
};

pub static TABLE_0127_METADATA: V2MetadataTable = V2MetadataTable {
    table: 127usize,
    description: "Table of codes speciying a classification of general allergy categories (drug, food, pollen, etc.).",
    ttype: "User",
    steward: "PA",
    where_used: "AL1-2",
    hl7_version: "2.2",
};

pub static TABLE_0128_METADATA: V2MetadataTable = V2MetadataTable {
    table: 128usize,
    description: "Table of codes which specify the general severity of an allergy.",
    ttype: "User",
    steward: "PA",
    where_used: "AL1-4",
    hl7_version: "2.2",
};

pub static TABLE_0129_METADATA: V2MetadataTable = V2MetadataTable {
    table: 129usize,
    description: "Table of codes specifying the fiancial accommodation type of the bed or room which implies the rate to be used when occupied by a patient under specific medical conditions, which determines how it is billed.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LCC-3",
    hl7_version: "2.2",
};

pub static TABLE_0130_METADATA: V2MetadataTable = V2MetadataTable {
    table: 130usize,
    description: "Table of codes which specify categories of a patient's visit with respect to an individual institution's needs, and is expected to be different on a site-specific basis.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-7",
    hl7_version: "2.2",
};

pub static TABLE_0131_METADATA: V2MetadataTable = V2MetadataTable {
    table: 131usize,
    description: "Table of codes which specify a relationship role that the next of kin/associated parties plays with regard to a patient. Also used in referrals, for example, it may be necessary to identify the contact representative at the clinic that issued a referral.",
    ttype: "User",
    steward: "PA",
    where_used: "NK1-7, CTD-1",
    hl7_version: "2.2",
};

pub static TABLE_0132_METADATA: V2MetadataTable = V2MetadataTable {
    table: 132usize,
    description: "Table of coded codes that are used by an institution for the purpose of uniquely identifying a transaction based on the Transaction Type.  For example, procedure, supply item, or test for charges; or to identify the payment medium for payments.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "FT1-7, ITM-12",
    hl7_version: "2.2",
};

pub static TABLE_0133_METADATA: V2MetadataTable = V2MetadataTable {
    table: 133usize,
    description:
        "Procedure practitioner identifier code type (include definition, but  PA wants to publish)",
    ttype: "undefined",
    steward: "",
    where_used: "",
    hl7_version: "",
};

pub static TABLE_0135_METADATA: V2MetadataTable = V2MetadataTable {
    table: 135usize,
    description: "Table of codes which indicate whether an insured person agreed to assign the insurance benefits to a healthcare provider.  If so, the insurance will pay the provider directly.",
    ttype: "User",
    steward: "FM",
    where_used: "IN1-20",
    hl7_version: "2.2",
};

pub static TABLE_0136_METADATA: V2MetadataTable = V2MetadataTable {
    table: 136usize,
    description: "HL7-defined table of codes specifying either Yes or No used in fields containing binary answers generally user-specified. The actual interpretation of Yes/No is context sensitive. Individual chapters will further refine the meaning of Yes/No in their specific context.",
    ttype: "HL7",
    steward: "InM",
    where_used: "numerous",
    hl7_version: "2.2",
};

pub static TABLE_0137_METADATA: V2MetadataTable = V2MetadataTable {
    table: 137usize,
    description: "Table of codes which specify a party to which a claim should be mailed when claims are sent by mail.",
    ttype: "User",
    steward: "FM",
    where_used: "IN2-5",
    hl7_version: "2.2",
};

pub static TABLE_0139_METADATA: V2MetadataTable = V2MetadataTable {
    table: 139usize,
    description: "Table of codes specifying the required employer information data for UB82 form locator 71.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN1-50, IN2-4",
    hl7_version: "2.2",
};

pub static TABLE_0140_METADATA: V2MetadataTable = V2MetadataTable {
    table: 140usize,
    description: "Table of codes which specify the military branch.  This field is defined by CMS or other regulatory agencies.",
    ttype: "User",
    steward: "PA",
    where_used: "PD1-19",
    hl7_version: "2.2",
};

pub static TABLE_0141_METADATA: V2MetadataTable = V2MetadataTable {
    table: 141usize,
    description: "Table of codes which specify the military rank/grade of the patient. Australia: https://en.wikipedia.org/wiki/Australian_Defence_Force_ranks Canada: http://www.forces.gc.ca/en/honours-history-badges-insignia/rank.page United States: published in the Defense Travel Administrator's Manual, Appendix M: Military Rank/Civilian Pay Grade Abbreviations and Service Agency Names, http://www.defensetravel.dod.mil/Docs/Training/DTA_App_M.pdf",
    ttype: "User",
    steward: "PA",
    where_used: "PD1-20",
    hl7_version: "2.2",
};

pub static TABLE_0142_METADATA: V2MetadataTable = V2MetadataTable {
    table: 142usize,
    description: "Table of codes which specify the military status of the patient.  This field is defined by CMS or other regulatory agencies.",
    ttype: "User",
    steward: "PA",
    where_used: "PD1-21",
    hl7_version: "2.2",
};

pub static TABLE_0143_METADATA: V2MetadataTable = V2MetadataTable {
    table: 143usize,
    description:
        "Table of codes specifying the reason a service is not covered.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN2-24",
    hl7_version: "2.2",
};

pub static TABLE_0144_METADATA: V2MetadataTable = V2MetadataTable {
    table: 144usize,
    description: "Table of codes which specify the source of information about the insured's eligibility for benefits.",
    ttype: "User",
    steward: "FM",
    where_used: "IN2-27",
    hl7_version: "2.2",
};

pub static TABLE_0145_METADATA: V2MetadataTable = V2MetadataTable {
    table: 145usize,
    description: "Table of codes which specify the room type.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "RMC.1",
    hl7_version: "2.1",
};

pub static TABLE_0146_METADATA: V2MetadataTable = V2MetadataTable {
    table: 146usize,
    description: "Table of codes which specify amount quantity type.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "RMC.2",
    hl7_version: "2.2",
};

pub static TABLE_0147_METADATA: V2MetadataTable = V2MetadataTable {
    table: 147usize,
    description: "Table of codes which specify the policy type.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "PTA.1",
    hl7_version: "2.2",
};

pub static TABLE_0148_METADATA: V2MetadataTable = V2MetadataTable {
    table: 148usize,
    description:
        "HL7-defined table of codes which specify whether the amount is currency or a percentage.",
    ttype: "HL7",
    steward: "InM/FM",
    where_used: "MOP.1",
    hl7_version: "2.2",
};

pub static TABLE_0149_METADATA: V2MetadataTable = V2MetadataTable {
    table: 149usize,
    description: "Table of codes which specify whether the days are denied, pending or approved.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "DTN.1",
    hl7_version: "2.2",
};

pub static TABLE_0150_METADATA: V2MetadataTable = V2MetadataTable {
    table: 150usize,
    description: "Table of codes which specify the category or type of patient for which this certification is requested.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "ICD.1",
    hl7_version: "2.2",
};

pub static TABLE_0151_METADATA: V2MetadataTable = V2MetadataTable {
    table: 151usize,
    description:
        "Table of codes specifying the status of the second opinion.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN3-23",
    hl7_version: "2.2",
};

pub static TABLE_0152_METADATA: V2MetadataTable = V2MetadataTable {
    table: 152usize,
    description: "Table of codes specifying if accompanying documentation has been received by the provider.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN3-24",
    hl7_version: "2.2",
};

pub static TABLE_0153_METADATA: V2MetadataTable = V2MetadataTable {
    table: 153usize,
    description: "Note that information for the external code system on which this table is built (NUBC) is still pending from the FM Work Group.  In the US, code system 2.16.840.1.113883.6.301.6 nubc-ValueCode-cs may be used for values.",
    ttype: "External",
    steward: "InM/FM",
    where_used: "UVC.1",
    hl7_version: "2.2",
};

pub static TABLE_0155_METADATA: V2MetadataTable = V2MetadataTable {
    table: 155usize,
    description: "HL7-defined table of codes which identify conditions under which accept acknowledgments are required to be returned in response to a message, and required for enhanced acknowledgment mode.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MSH-15, MSH-16",
    hl7_version: "2.2",
};

pub static TABLE_0159_METADATA: V2MetadataTable = V2MetadataTable {
    table: 159usize,
    description: "HL7-defined table of codes which specify the type of diet.",
    ttype: "HL7",
    steward: "OO",
    where_used: "ODS-1",
    hl7_version: "2.2",
};

pub static TABLE_0160_METADATA: V2MetadataTable = V2MetadataTable {
    table: 160usize,
    description: "HL7-defined table of codes which specify the type of dietary tray.",
    ttype: "HL7",
    steward: "OO",
    where_used: "ODT-1",
    hl7_version: "2.2",
};

pub static TABLE_0161_METADATA: V2MetadataTable = V2MetadataTable {
    table: 161usize,
    description: "HL7-defined table of codes which specify whether substitutions are allowed and, if so, the type of substitutions allowed.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXO-9",
    hl7_version: "2.2",
};

pub static TABLE_0162_METADATA: V2MetadataTable = V2MetadataTable {
    table: 162usize,
    description: "Table of codes which specify the route of administration.",
    ttype: "User",
    steward: "OO",
    where_used: "RXR-1",
    hl7_version: "2.2",
};

pub static TABLE_0163_METADATA: V2MetadataTable = V2MetadataTable {
    table: 163usize,
    description:
        "HL7-defined table of codes that specify a body site from which a specimen is obtained.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBX-20, CH7",
    hl7_version: "2.2",
};

pub static TABLE_0164_METADATA: V2MetadataTable = V2MetadataTable {
    table: 164usize,
    description: "Table of codes which specify the mechanical device used to aid in the administration of the drug or other treatment.  Common examples are IV-sets of different types.",
    ttype: "User",
    steward: "OO",
    where_used: "RXR-3",
    hl7_version: "2.2",
};

pub static TABLE_0165_METADATA: V2MetadataTable = V2MetadataTable {
    table: 165usize,
    description: "Table of codes which specify the specific method requested for the administration of the drug or treatment to the patient.",
    ttype: "User",
    steward: "OO",
    where_used: "RXR-4",
    hl7_version: "2.2",
};

pub static TABLE_0166_METADATA: V2MetadataTable = V2MetadataTable {
    table: 166usize,
    description: "HL7-defined table of codes which specify the RX component type.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXC-1",
    hl7_version: "2.2",
};

pub static TABLE_0167_METADATA: V2MetadataTable = V2MetadataTable {
    table: 167usize,
    description: "HL7-defined table of codes which specify the substitution status.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXE-9, RXD-11,RXG-10",
    hl7_version: "2.2",
};

pub static TABLE_0168_METADATA: V2MetadataTable = V2MetadataTable {
    table: 168usize,
    description: "HL7-defined table of codes which specify one or more available priorities for performing the observation or test.",
    ttype: "HL7",
    steward: "InM",
    where_used: "OM1-25",
    hl7_version: "2.2",
};

pub static TABLE_0169_METADATA: V2MetadataTable = V2MetadataTable {
    table: 169usize,
    description: "HL7-defined table of codes which specify the available priorities reporting the test results when the user is asked to specify the reporting priority independent of the processing priority.",
    ttype: "HL7",
    steward: "InM",
    where_used: "OM1-26",
    hl7_version: "2.2",
};

pub static TABLE_0170_METADATA: V2MetadataTable = V2MetadataTable {
    table: 170usize,
    description: "HL7-defined table of codes which specify the parents and children for diagnostic studies, especially in microbiology, where the initial specimen (e.g., blood) is processed to produce results (e.g., the identity of the bacteria grown out of the culture).  The process also produces new \"specimens\" (e.g., pure culture of staphylococcus, and E. coli), and these are studied by a second order process (bacterial sensitivities). The parents (e.g., blood culture) and children (e.g., penicillin MIC) are identified in such cases.",
    ttype: "HL7",
    steward: "InM/OO",
    where_used: "OM4-2",
    hl7_version: "2.2",
};

pub static TABLE_0171_METADATA: V2MetadataTable = V2MetadataTable {
    table: 171usize,
    description: "Table of codes specifying the information related to a person's country citizenship.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PID-26, PID-39",
    hl7_version: "2.1",
};

pub static TABLE_0172_METADATA: V2MetadataTable = V2MetadataTable {
    table: 172usize,
    description:
        "Table of codes specifying the military status assigned to a veteran.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PID-27",
    hl7_version: "2.2",
};

pub static TABLE_0173_METADATA: V2MetadataTable = V2MetadataTable {
    table: 173usize,
    description: "Table of codes specifying whether this insurance works in conjunction with other insurance plans or if it provides independent coverage and payment of benefits regardless of other insurance that might be available to the patient.",
    ttype: "User",
    steward: "FM",
    where_used: "IN1-21",
    hl7_version: "2.2",
};

pub static TABLE_0174_METADATA: V2MetadataTable = V2MetadataTable {
    table: 174usize,
    description: "Table of codes specifying an identification of a test battery, an entire functional procedure or study, a single test value (observation), multiple test batteries or functional procedures as an orderable unit (profile), or a single test value (observation) calculated from other independent observations, typically used as an indicator for Master Files.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "OM1-18",
    hl7_version: "2.2",
};

pub static TABLE_0175_METADATA: V2MetadataTable = V2MetadataTable {
    table: 175usize,
    description: "HL7-defined table of codes which are represented by codes identifying HL7 Versions 2.x Master Files.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MFI-1",
    hl7_version: "2.2",
};

pub static TABLE_0177_METADATA: V2MetadataTable = V2MetadataTable {
    table: 177usize,
    description: "Table of codes specifying the degree to which special confidentiality protection should be applied to the observation.",
    ttype: "User",
    steward: "InM",
    where_used: "OM1-30, ORC-28",
    hl7_version: "2.2",
};

pub static TABLE_0178_METADATA: V2MetadataTable = V2MetadataTable {
    table: 178usize,
    description: "HL7-defined table of codes specifying file-level events for master files.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MFI-3",
    hl7_version: "2.2",
};

pub static TABLE_0179_METADATA: V2MetadataTable = V2MetadataTable {
    table: 179usize,
    description: "HL7-defined table of codes specifying application response levels defined for a given Master File Message at the MFE segment level, and used for MFN-Master File Notification message.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MFI-6",
    hl7_version: "2.2",
};

pub static TABLE_0180_METADATA: V2MetadataTable = V2MetadataTable {
    table: 180usize,
    description: "HL7-defined table of codes specifying an action for a master file record.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MFE-1",
    hl7_version: "2.2",
};

pub static TABLE_0181_METADATA: V2MetadataTable = V2MetadataTable {
    table: 181usize,
    description: "Table of codes specifying the status of the requested update.  Site-defined table, specific to each master file being updated via this transaction.",
    ttype: "User",
    steward: "InM",
    where_used: "MFA-4",
    hl7_version: "2.2",
};

pub static TABLE_0182_METADATA: V2MetadataTable = V2MetadataTable {
    table: 182usize,
    description: "Table of codes specifying the staff person's sex.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "STF-4",
    hl7_version: "2.2",
};

pub static TABLE_0183_METADATA: V2MetadataTable = V2MetadataTable {
    table: 183usize,
    description:
        "HL7-defined table of codes specifying whether a person is currently a valid staff member.",
    ttype: "HL7",
    steward: "PA",
    where_used: "STF-7",
    hl7_version: "2.2",
};

pub static TABLE_0184_METADATA: V2MetadataTable = V2MetadataTable {
    table: 184usize,
    description: "Table of codes specifying the institution department to which this person reports or belongs.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "STF-8",
    hl7_version: "2.2",
};

pub static TABLE_0185_METADATA: V2MetadataTable = V2MetadataTable {
    table: 185usize,
    description: "HL7-defined table of codes specifying which of a group of multiple phone numbers is the preferred method of contact for this person.",
    ttype: "HL7",
    steward: "PA",
    where_used: "STF-16, PRD-6, PRD-14, CTD-6",
    hl7_version: "2.2",
};

pub static TABLE_0186_METADATA: V2MetadataTable = V2MetadataTable {
    table: 186usize,
    description: "Table of codes specifying the category of practitioner.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PRA-3",
    hl7_version: "2.2",
};

pub static TABLE_0187_METADATA: V2MetadataTable = V2MetadataTable {
    table: 187usize,
    description: "HL7-defined table of codes specifying how provider services are billed.",
    ttype: "HL7",
    steward: "PA",
    where_used: "PRA-4",
    hl7_version: "2.2",
};

pub static TABLE_0188_METADATA: V2MetadataTable = V2MetadataTable {
    table: 188usize,
    description: "Table of codes specifying the individual responsible for triggering the event.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "EVN-5",
    hl7_version: "2.2",
};

pub static TABLE_0189_METADATA: V2MetadataTable = V2MetadataTable {
    table: 189usize,
    description: "Table of codes further defining a patient's ancestry.  In the US, a current use is to use these codes to report ethnicity in line with US federal standards for Hispanic origin.",
    ttype: "User",
    steward: "PA",
    where_used: "PID-22",
    hl7_version: "2.2",
};

pub static TABLE_0190_METADATA: V2MetadataTable = V2MetadataTable {
    table: 190usize,
    description: "HL7-defined table of codes specifying types or kinds of addresses.",
    ttype: "HL7",
    steward: "InM",
    where_used: "XAD.8, AD.7",
    hl7_version: "2.2",
};

pub static TABLE_0191_METADATA: V2MetadataTable = V2MetadataTable {
    table: 191usize,
    description:
        "HL7-defined table of codes declaring the general type of media data that is encoded.",
    ttype: "HL7",
    steward: "InM/FM",
    where_used: "TXA-3",
    hl7_version: "2.2",
};

pub static TABLE_0193_METADATA: V2MetadataTable = V2MetadataTable {
    table: 193usize,
    description: "Table of codes specifying the amount quantity class.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "PTA.2",
    hl7_version: "2.2",
};

pub static TABLE_0200_METADATA: V2MetadataTable = V2MetadataTable {
    table: 200usize,
    description: "HL7-defined table of codes for types of names for persons.",
    ttype: "HL7",
    steward: "InM",
    where_used: "XPN.8, PPN.10, XCN.10, PID-5, MRG-7",
    hl7_version: "2.3",
};

pub static TABLE_0201_METADATA: V2MetadataTable = V2MetadataTable {
    table: 201usize,
    description: "HL7-defined table of codes for specifying a specific use of a telecommunication number.  Best practice is to use this concept whenever a telecommunication number or access string is specified.",
    ttype: "HL7",
    steward: "InM",
    where_used: "XTN.2",
    hl7_version: "2.3",
};

pub static TABLE_0202_METADATA: V2MetadataTable = V2MetadataTable {
    table: 202usize,
    description: "HL7-defined table of codes  for specifying a type of telecommunication equipment.  Best practice is to use this concept whenever a telecommunication number or access string for particular equipment is specified.",
    ttype: "HL7",
    steward: "InM",
    where_used: "XTN.3",
    hl7_version: "2.3",
};

pub static TABLE_0203_METADATA: V2MetadataTable = V2MetadataTable {
    table: 203usize,
    description: "HL7-defined table of codes specifying the type of identififier.",
    ttype: "HL7",
    steward: "InM",
    where_used: "CX.5, PPN.13, XCN.13, XON.7",
    hl7_version: "2.3",
};

pub static TABLE_0204_METADATA: V2MetadataTable = V2MetadataTable {
    table: 204usize,
    description: "Table of codes used to specify the type of name for an organization i.e., legal name,  display name.",
    ttype: "User",
    steward: "InM",
    where_used: "XON.2, PD1-3",
    hl7_version: "2.3",
};

pub static TABLE_0205_METADATA: V2MetadataTable = V2MetadataTable {
    table: 205usize,
    description: "HL7-defined table of codes used to identify the intent for the dollar amount on a pricing transaction.",
    ttype: "HL7",
    steward: "InM",
    where_used: "CP.2",
    hl7_version: "2.3",
};

pub static TABLE_0206_METADATA: V2MetadataTable = V2MetadataTable {
    table: 206usize,
    description: "HL7-defined table of codes specifying actions to be applied for segments when an HL7 version 2 interface is operating in \"action code mode\" (a kind of update mode in the Standard).",
    ttype: "HL7",
    steward: "InM",
    where_used: "CH2, RXA-21, RXV-22, LCH-2, IAM-6,A    RV-2, IN1-55, OH1",
    hl7_version: "2.3",
};

pub static TABLE_0207_METADATA: V2MetadataTable = V2MetadataTable {
    table: 207usize,
    description:
        "HL7-defined table of codes that indicate an archival process or an initial load process.",
    ttype: "HL7",
    steward: "InM",
    where_used: "PT.2",
    hl7_version: "2.3",
};

pub static TABLE_0208_METADATA: V2MetadataTable = V2MetadataTable {
    table: 208usize,
    description: "HL7-defined table of codes defining precise response status concepts in support of HL7 Version 2 query messaging.  It is commonly used to indicate no data is found that matches the query parameters, but no error.",
    ttype: "HL7",
    steward: "InM",
    where_used: "QAK.2",
    hl7_version: "2.3",
};

pub static TABLE_0209_METADATA: V2MetadataTable = V2MetadataTable {
    table: 209usize,
    description: "HL7-defined table of codes used to define the relationship between HL7 segment field names identified in a query construct.",
    ttype: "HL7",
    steward: "InM",
    where_used: "QSC.2",
    hl7_version: "2.3",
};

pub static TABLE_0210_METADATA: V2MetadataTable = V2MetadataTable {
    table: 210usize,
    description: "HL7-defined table of codes used with relational operator values to group more than one segment field name.",
    ttype: "HL7",
    steward: "InM",
    where_used: "QSC.4",
    hl7_version: "2.3",
};

pub static TABLE_0211_METADATA: V2MetadataTable = V2MetadataTable {
    table: 211usize,
    description: "Table of codes that identify one of a number of possible standard alternate character sets for a message, either single-byte or double-byte.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MSH-18",
    hl7_version: "2.3",
};

pub static TABLE_0212_METADATA: V2MetadataTable = V2MetadataTable {
    table: 212usize,
    description: "Table of codes that identify the nation or national grouping to which the person belongs.  This information may be different from a person’s citizenship in countries in which multiple nationalities are recognized (for example, Spain: Basque, Catalan, etc.).  No suggested values.",
    ttype: "User",
    steward: "",
    where_used: "NK1-27, GT1-43. IN1-41,",
    hl7_version: "2.3",
};

pub static TABLE_0213_METADATA: V2MetadataTable = V2MetadataTable {
    table: 213usize,
    description: "Table of codes used to define the state of a visit relative to its place in a purge workflow.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-16",
    hl7_version: "2.3",
};

pub static TABLE_0214_METADATA: V2MetadataTable = V2MetadataTable {
    table: 214usize,
    description: "Table of codes used to record a health insurance program required for healthcare visit reimbursement.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-18",
    hl7_version: "2.3",
};

pub static TABLE_0215_METADATA: V2MetadataTable = V2MetadataTable {
    table: 215usize,
    description: "Table of codes specifying a level of publicity of information about a patient for a specific visit.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-21, PD1-11",
    hl7_version: "2.3",
};

pub static TABLE_0216_METADATA: V2MetadataTable = V2MetadataTable {
    table: 216usize,
    description: "Table of codes used to define the state of a care episode for a patient.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-24",
    hl7_version: "2.3",
};

pub static TABLE_0217_METADATA: V2MetadataTable = V2MetadataTable {
    table: 217usize,
    description:
        "Table of codes used to define a relative level of urgency applied to a patient visit.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-25",
    hl7_version: "2.3",
};

pub static TABLE_0218_METADATA: V2MetadataTable = V2MetadataTable {
    table: 218usize,
    description: "Table of codes used to indicate which adjustments should be made to a guarantor ’s charges.  For example, when a hospital agrees to adjust a guarantor  ’s charges to a sliding scale.  No  suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "GT1-26",
    hl7_version: "2.3",
};

pub static TABLE_0219_METADATA: V2MetadataTable = V2MetadataTable {
    table: 219usize,
    description:
        "Table of codes used to indicate whether a treatment is continuous.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-31",
    hl7_version: "2.3",
};

pub static TABLE_0220_METADATA: V2MetadataTable = V2MetadataTable {
    table: 220usize,
    description: "Table of codes characterizing the situation that patient-associated parties live in at their residential address.",
    ttype: "User",
    steward: "PA",
    where_used: "NK1-21, PD1-2",
    hl7_version: "2.3",
};

pub static TABLE_0222_METADATA: V2MetadataTable = V2MetadataTable {
    table: 222usize,
    description: "Table of codes used to indicate a reason for contacting a guarantor, for example, to phone a guarantor if payments are late.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "GT1-47, NK1-29",
    hl7_version: "2.3",
};

pub static TABLE_0223_METADATA: V2MetadataTable = V2MetadataTable {
    table: 223usize,
    description: "Table of codes identifying specific living conditions (e.g., spouse dependent on patient, walk-up) that are relevant to an evaluation of the patient's healthcare needs.",
    ttype: "User",
    steward: "PA",
    where_used: "NK1-17, PD1-1",
    hl7_version: "2.3",
};

pub static TABLE_0224_METADATA: V2MetadataTable = V2MetadataTable {
    table: 224usize,
    description: "HL7-defined table of codes defining whether patient transportation preparations are in place.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBR-41",
    hl7_version: "2.3",
};

pub static TABLE_0225_METADATA: V2MetadataTable = V2MetadataTable {
    table: 225usize,
    description: "HL7-defined table of codes indicating whether a patient must be accompanied while travelling to a diagnostic service department.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBR-42",
    hl7_version: "2.3",
};

pub static TABLE_0227_METADATA: V2MetadataTable = V2MetadataTable {
    table: 227usize,
    description: "Table of codes specifying the organization that manufactures a vaccine. The values are maintained by the US Centers of Disease Control.  Note that the source of truth for these code values are maintained by the CDC, and the code system may be accessed at URL: https://phinvads.cdc.gov/vads/SearchCodeSystems_search.action?searchOptions .searchText=PH_ManufacturersOfVaccinesMVX_CDC_NIP.  The value set is also maintained by the CDC, and may be accessed at URL: https://phinvads.cdc.gov/vads/SearchValueSets_search.action?searchOptions.sea rchText=PHVS_ManufacturersOfVaccinesMVX_CDC_NIP",
    ttype: "Imported",
    steward: "OO",
    where_used: "RXA-17, RXD-20, RXG-21",
    hl7_version: "2.3",
};

pub static TABLE_0228_METADATA: V2MetadataTable = V2MetadataTable {
    table: 228usize,
    description:
        "Table of codes used to classify whether a patient visit can be related to a diagnosis.",
    ttype: "User",
    steward: "FM",
    where_used: "DG1-17",
    hl7_version: "2.3",
};

pub static TABLE_0229_METADATA: V2MetadataTable = V2MetadataTable {
    table: 229usize,
    description: "Table of codes used to identify a Diagnostic Resource Group Payor. US Realm. No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN3-12",
    hl7_version: "2.3",
};

pub static TABLE_0230_METADATA: V2MetadataTable = V2MetadataTable {
    table: 230usize,
    description: "Table of codes used to classify a procedure.",
    ttype: "User",
    steward: "FM",
    where_used: "PR1-6",
    hl7_version: "2.3",
};

pub static TABLE_0231_METADATA: V2MetadataTable = V2MetadataTable {
    table: 231usize,
    description:
        "Table of codes used to designate whether a guarantor is a full or part time student.",
    ttype: "User",
    steward: "FM",
    where_used: "GT1-40, NK1-24, PD1-5",
    hl7_version: "2.3",
};

pub static TABLE_0232_METADATA: V2MetadataTable = V2MetadataTable {
    table: 232usize,
    description: "Table of codes used to describe why an insurance company has been contacted.",
    ttype: "User",
    steward: "FM",
    where_used: "IN2-57",
    hl7_version: "2.3",
};

pub static TABLE_0233_METADATA: V2MetadataTable = V2MetadataTable {
    table: 233usize,
    description: "Table of codes that specify a non-concur code and description for a denied request, used in insurance claims processing.  No suggested values.",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.3",
};

pub static TABLE_0234_METADATA: V2MetadataTable = V2MetadataTable {
    table: 234usize,
    description: "HL7-defined table of codes used to identify the time span of a report or the reason for a report sent to a regulatory agency.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PES-11",
    hl7_version: "2.3",
};

pub static TABLE_0235_METADATA: V2MetadataTable = V2MetadataTable {
    table: 235usize,
    description:
        "HL7-defined table of codes used to identify where a report sender learned about an event.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PES-12",
    hl7_version: "2.3",
};

pub static TABLE_0236_METADATA: V2MetadataTable = V2MetadataTable {
    table: 236usize,
    description: "HL7-defined table of codes used to identify the type of entity to which the event has been reported.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PES-13",
    hl7_version: "2.3",
};

pub static TABLE_0237_METADATA: V2MetadataTable = V2MetadataTable {
    table: 237usize,
    description:
        "HL7-defined table of codes used to qualify an event related to a product experience.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PEO-8",
    hl7_version: "2.3",
};

pub static TABLE_0238_METADATA: V2MetadataTable = V2MetadataTable {
    table: 238usize,
    description: "HL7-defined table of codes used by a sender to designate an event as serious or significant.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PEO-9",
    hl7_version: "2.3",
};

pub static TABLE_0239_METADATA: V2MetadataTable = V2MetadataTable {
    table: 239usize,
    description: "HL7-defined table of codes used to communicate whether an event has been judged to be expected or unexpected.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PEO-10",
    hl7_version: "2.3",
};

pub static TABLE_0240_METADATA: V2MetadataTable = V2MetadataTable {
    table: 240usize,
    description: "HL7-defined table of codes used to describe the impact of an event on a patient.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PEO-11",
    hl7_version: "2.3",
};

pub static TABLE_0241_METADATA: V2MetadataTable = V2MetadataTable {
    table: 241usize,
    description: "HL7-defined table of codes used to describe the overall state of a patient as a result of patient care.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PEO-12",
    hl7_version: "2.3",
};

pub static TABLE_0242_METADATA: V2MetadataTable = V2MetadataTable {
    table: 242usize,
    description: "HL7-defined table of codes used to provide a general description of the kind of health care professional who provided the primary observation.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PEO-22, PEO-23, PCR-19",
    hl7_version: "2.3",
};

pub static TABLE_0243_METADATA: V2MetadataTable = V2MetadataTable {
    table: 243usize,
    description: "HL7-defined table of codes used to define whether the primary observer has given permission for their identification information to be provided to a product manufacturer.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PEO-25",
    hl7_version: "2.3",
};

pub static TABLE_0244_METADATA: V2MetadataTable = V2MetadataTable {
    table: 244usize,
    description: "Table of codes that indicate whether a product is designed for a single use.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "PCR-9",
    hl7_version: "2.3",
};

pub static TABLE_0245_METADATA: V2MetadataTable = V2MetadataTable {
    table: 245usize,
    description: "Table of codes used to indicate if a product problem would exist if a product malfunction could lead to death or serious injury.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "PCR-11",
    hl7_version: "2.3",
};

pub static TABLE_0246_METADATA: V2MetadataTable = V2MetadataTable {
    table: 246usize,
    description: "Table of codes used to indicate that the product is available for analysis. No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "PCR-13",
    hl7_version: "2.3",
};

pub static TABLE_0247_METADATA: V2MetadataTable = V2MetadataTable {
    table: 247usize,
    description: "HL7-defined table of codes that describes the status of product evaluation.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PCR-15",
    hl7_version: "2.3",
};

pub static TABLE_0248_METADATA: V2MetadataTable = V2MetadataTable {
    table: 248usize,
    description: "HL7-defined table of codes used to describe the evaluation state of a product identified in an incident.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PCR-17",
    hl7_version: "2.3",
};

pub static TABLE_0249_METADATA: V2MetadataTable = V2MetadataTable {
    table: 249usize,
    description: "Table of codes used to indicate whether the product used is a generic or a branded product. No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "PCR-2",
    hl7_version: "2.3",
};

pub static TABLE_0250_METADATA: V2MetadataTable = V2MetadataTable {
    table: 250usize,
    description: "HL7-defined table of codes used to provide an estimate of whether an issue with a  product was the cause of an event.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PCR-20",
    hl7_version: "2.3",
};

pub static TABLE_0251_METADATA: V2MetadataTable = V2MetadataTable {
    table: 251usize,
    description: "HL7-defined table of codes used to define the action taken as a result of an event related to a product issue.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PCR-21",
    hl7_version: "2.3",
};

pub static TABLE_0252_METADATA: V2MetadataTable = V2MetadataTable {
    table: 252usize,
    description: "HL7-defined table of codes used to record event observations regarding what may have caused a product related event.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PCR-22",
    hl7_version: "2.3",
};

pub static TABLE_0253_METADATA: V2MetadataTable = V2MetadataTable {
    table: 253usize,
    description: "HL7-defined table of codes used to identify the mechanism of product transmission when the product has not been directly applied to the patient.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PCR-23",
    hl7_version: "2.3",
};

pub static TABLE_0254_METADATA: V2MetadataTable = V2MetadataTable {
    table: 254usize,
    description: "HL7-defined table of codes that describe the underlying kind of property represented by an observation.  The categories distinguish concentrations from total amounts, molar concentrations from mass concentrations, partial pressures from colors, and so forth.  These are discussed more fully in the LOINC Users' Manual.   They are derived from the approach described in 1995 edition of the IUPAC Silver Book.  These distinctions are used in IUPAC and LOINC standard codes.  The distinctions of true quantities in this table are based primarily on dimensional analyses. The table contains a number of \"families,\" those related to simple counts (number, number concentration, etc.), to mass (mass, mass concentration, etc.), to enzyme activity (catalytic content, catalytic concentration, etc.), and molar or equivalents (substance content, substance concentration).",
    ttype: "HL7",
    steward: "InM",
    where_used: "OM1-42",
    hl7_version: "2.3",
};

pub static TABLE_0255_METADATA: V2MetadataTable = V2MetadataTable {
    table: 255usize,
    description: "Table of codes used to classify an observation definition as intended to measure a patient's state at a point in time.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "OM1-43",
    hl7_version: "2.3",
};

pub static TABLE_0256_METADATA: V2MetadataTable = V2MetadataTable {
    table: 256usize,
    description: "HL7-defined table of codes used to classify an observation definition as being a component of a challenge test.",
    ttype: "HL7",
    steward: "InM/OO",
    where_used: "OM1-44",
    hl7_version: "2.3",
};

pub static TABLE_0257_METADATA: V2MetadataTable = V2MetadataTable {
    table: 257usize,
    description: "HL7-defined table of codes used to further describe an observation definition that is characterized as a challenge observation.",
    ttype: "HL7",
    steward: "InM/OO",
    where_used: "OM1-44",
    hl7_version: "2.3",
};

pub static TABLE_0258_METADATA: V2MetadataTable = V2MetadataTable {
    table: 258usize,
    description: "HL7-defined table of codes used in an observation definition to describe the subject of an observation in relation to a patient.",
    ttype: "HL7",
    steward: "InM/OO",
    where_used: "OM1-45",
    hl7_version: "2.3",
};

pub static TABLE_0259_METADATA: V2MetadataTable = V2MetadataTable {
    table: 259usize,
    description: "Ta   ble of codes used to define the imaging apparatus expected to be used to acquire an observation.  This table has been removed from the standard as of 2.7 in favor of table 0910.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "OM1-47",
    hl7_version: "2.3",
};

pub static TABLE_0260_METADATA: V2MetadataTable = V2MetadataTable {
    table: 260usize,
    description:
        "Table of codes used to identify the kind of location described in the location definition.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LOC-3",
    hl7_version: "2.3",
};

pub static TABLE_0261_METADATA: V2MetadataTable = V2MetadataTable {
    table: 261usize,
    description: "Table of codes used to identify the equipment available in a location definition identified as a room or bed.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LOC-8",
    hl7_version: "2.3",
};

pub static TABLE_0262_METADATA: V2MetadataTable = V2MetadataTable {
    table: 262usize,
    description: "Table of codes used to identify the level of privacy a patient will be afforded when assigned to this location definition.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LCH-5",
    hl7_version: "2.3",
};

pub static TABLE_0263_METADATA: V2MetadataTable = V2MetadataTable {
    table: 263usize,
    description: "Table of codes used to identify the level of care a patient may be afforded when assigned to this location definition.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LCH-5",
    hl7_version: "2.3",
};

pub static TABLE_0264_METADATA: V2MetadataTable = V2MetadataTable {
    table: 264usize,
    description: "Table of codes used to specify the institution’s department to which a location belongs, or its cost center.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LDP-2",
    hl7_version: "2.3",
};

pub static TABLE_0265_METADATA: V2MetadataTable = V2MetadataTable {
    table: 265usize,
    description: "Table of codes used to identify the specialty of the care professional who is supported when using this location definition.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LDP-4",
    hl7_version: "2.3",
};

pub static TABLE_0267_METADATA: V2MetadataTable = V2MetadataTable {
    table: 267usize,
    description: "HL7-defined table of codes used to identify the day(s) of the week when a location may be scheduled for appointments.",
    ttype: "HL7",
    steward: "InM/PA",
    where_used: "UVC.2, LDP.10 UVC.1",
    hl7_version: "2.3",
};

pub static TABLE_0268_METADATA: V2MetadataTable = V2MetadataTable {
    table: 268usize,
    description: "Table of codes used to define whether a Charge Description Master description may be overridden or if it must be overridden.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "CDM-5, PRC-13",
    hl7_version: "2.3",
};

pub static TABLE_0269_METADATA: V2MetadataTable = V2MetadataTable {
    table: 269usize,
    description: "Table of codes used to define the event upon which a charge should be generated.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "PRC-18",
    hl7_version: "2.3",
};

pub static TABLE_0270_METADATA: V2MetadataTable = V2MetadataTable {
    table: 270usize,
    description: "Table of codes used to identify the kind of patient document.",
    ttype: "User",
    steward: "StrucDoc",
    where_used: "TXA-2",
    hl7_version: "2.3",
};

pub static TABLE_0271_METADATA: V2MetadataTable = V2MetadataTable {
    table: 271usize,
    description: "HL7-defined table of codes used to record the state of a document in a workflow.",
    ttype: "HL7",
    steward: "StrucDoc",
    where_used: "TXA-17",
    hl7_version: "2.3",
};

pub static TABLE_0272_METADATA: V2MetadataTable = V2MetadataTable {
    table: 272usize,
    description: "HL7-defined table of codes that specify the degree to which special confidentiality protection should be applied to  information.  The assignment of data elements to these categories is left to the discretion of the healthcare organization.",
    ttype: "HL7",
    steward: "StrucDoc",
    where_used: "TXA-18",
    hl7_version: "2.3",
};

pub static TABLE_0273_METADATA: V2MetadataTable = V2MetadataTable {
    table: 273usize,
    description: "HL7-defined table of codes used to define whether a patient document is appropriate or available for use in patient care.",
    ttype: "HL7",
    steward: "StrucDoc",
    where_used: "TXA-19",
    hl7_version: "2.3",
};

pub static TABLE_0275_METADATA: V2MetadataTable = V2MetadataTable {
    table: 275usize,
    description: "HL7-defined table of codes used to describe the availability of a document in relation to the type of storage.",
    ttype: "HL7",
    steward: "StrucDoc",
    where_used: "TXA-20",
    hl7_version: "2.3",
};

pub static TABLE_0276_METADATA: V2MetadataTable = V2MetadataTable {
    table: 276usize,
    description: "Table of codes used to describe the kind of appointment or the reason why an appointment has been scheduled.",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.3",
};

pub static TABLE_0277_METADATA: V2MetadataTable = V2MetadataTable {
    table: 277usize,
    description:
        "Table of codes used in an appointment request to describe the kind of appointment.",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.3",
};

pub static TABLE_0278_METADATA: V2MetadataTable = V2MetadataTable {
    table: 278usize,
    description: "Table of codes used to describe an appointment status from the perspective of the entity assigned to fulfill the appointment.",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.3",
};

pub static TABLE_0279_METADATA: V2MetadataTable = V2MetadataTable {
    table: 279usize,
    description: "Table of codes used to indicate whether the appointment resource may be substituted for another by the entity assigned to fulfill the appointment.",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.3",
};

pub static TABLE_0280_METADATA: V2MetadataTable = V2MetadataTable {
    table: 280usize,
    description: "Table of codes used to designate the urgency of a  referral.",
    ttype: "User",
    steward: "OO",
    where_used: "RF1-2",
    hl7_version: "2.3",
};

pub static TABLE_0281_METADATA: V2MetadataTable = V2MetadataTable {
    table: 281usize,
    description: "Table of codes used to identify the general category of healthcare professional desired to satisfy a referral.",
    ttype: "User",
    steward: "OO",
    where_used: "RF1-3",
    hl7_version: "2.3",
};

pub static TABLE_0282_METADATA: V2MetadataTable = V2MetadataTable {
    table: 282usize,
    description: "Table of codes used to identify the expected response from the healthcare professional receiving a referral.",
    ttype: "User",
    steward: "OO",
    where_used: "RF1-4",
    hl7_version: "2.3",
};

pub static TABLE_0283_METADATA: V2MetadataTable = V2MetadataTable {
    table: 283usize,
    description: "Table of codes used to define the state of a  referral.",
    ttype: "User",
    steward: "OO",
    where_used: "RF1-1",
    hl7_version: "2.3",
};

pub static TABLE_0284_METADATA: V2MetadataTable = V2MetadataTable {
    table: 284usize,
    description: "Table of codes used to describe the patient care setting where a referral should take place.",
    ttype: "User",
    steward: "OO",
    where_used: "RF1-5",
    hl7_version: "2.3",
};

pub static TABLE_0285_METADATA: V2MetadataTable = V2MetadataTable {
    table: 285usize,
    description: "Table of codes specifying the identification of the insurance company or other entity that administers the authorizing coverage plan.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "AUT-2",
    hl7_version: "2.3",
};

pub static TABLE_0286_METADATA: V2MetadataTable = V2MetadataTable {
    table: 286usize,
    description: "Table of codes used to define the relationship between a referral recipient and a patient or between a referral initiator and a patient.",
    ttype: "User",
    steward: "OO",
    where_used: "PRD-1",
    hl7_version: "2.3",
};

pub static TABLE_0287_METADATA: V2MetadataTable = V2MetadataTable {
    table: 287usize,
    description:
        "HL7-defined table of codes used in Patient Care for the intent of a problem or goal.",
    ttype: "HL7",
    steward: "PA",
    where_used: "ROL-2, GOL-1, PRB-1, PTH-1",
    hl7_version: "2.3",
};

pub static TABLE_0288_METADATA: V2MetadataTable = V2MetadataTable {
    table: 288usize,
    description: "Table of codes specifying the census tract in which the specified address resides. No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "XAD.109",
    hl7_version: "2.3",
};

pub static TABLE_0289_METADATA: V2MetadataTable = V2MetadataTable {
    table: 289usize,
    description: "Table of codes specifying the county or parish in which the specified address resides.  No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "XAD.9, CX.9",
    hl7_version: "2.3",
};

pub static TABLE_0291_METADATA: V2MetadataTable = V2MetadataTable {
    table: 291usize,
    description: "Table of codes specifying a subset of the media subtypes of binary data that are encoded in an ascii structure or stream.",
    ttype: "HL7",
    steward: "",
    where_used: "RP.4",
    hl7_version: "2.3",
};

pub static TABLE_0292_METADATA: V2MetadataTable = V2MetadataTable {
    table: 292usize,
    description: "Table of codes specifying the administered vaccines.   The values are maintained by the US Centers of Disease Control.  The code system is maintained by the CDC, and may be found at URL; https://phinvads.cdc.gov/vads/ViewCodeSystem.action?id=2.16.840.1.113883.1 2.292 The value set is maintained by the CDC and may be found at URL: https://phinvads.cdc.gov/vads/ViewValueSet.action?id=ABDEE003-77C3-48E7-B941-EBF92B6B81FC",
    ttype: "External",
    steward: "OO",
    where_used: "RXA-5, RXE-2, RXD-2, RX G-4",
    hl7_version: "2.3",
};

pub static TABLE_0293_METADATA: V2MetadataTable = V2MetadataTable {
    table: 293usize,
    description: "Table of codes specifying the billing category codes for any classification systems needed, for example, general ledger codes and UB92 categories.  No suggested values.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "PRC-14",
    hl7_version: "2.3",
};

pub static TABLE_0294_METADATA: V2MetadataTable = V2MetadataTable {
    table: 294usize,
    description: "Table of codes used to describe acceptable start and end times, as well as days of the week, for appointment or resource scheduling.",
    ttype: "User",
    steward: "InM/S&L",
    where_used: "SCV.1, APR-1, APR-2",
    hl7_version: "2.3",
};

pub static TABLE_0295_METADATA: V2MetadataTable = V2MetadataTable {
    table: 295usize,
    description:
        "Table of codes specifying an associated party's disability.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "NK1-36, PD1-6",
    hl7_version: "2.3",
};

pub static TABLE_0296_METADATA: V2MetadataTable = V2MetadataTable {
    table: 296usize,
    description: "Table of codes specifying the patient's primary language.  No suggested values.",
    ttype: "User",
    steward: "StrucDoc",
    where_used: "CON:18, PID-15",
    hl7_version: "2.3",
};

pub static TABLE_0297_METADATA: V2MetadataTable = V2MetadataTable {
    table: 297usize,
    description: "Table of codes specifying the CN identification source.  No suggested values.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "CNN.8, PPN.8, XCN.8",
    hl7_version: "2.3",
};

pub static TABLE_0298_METADATA: V2MetadataTable = V2MetadataTable {
    table: 298usize,
    description: "HL7-defined table of codes that specify whether a composite price range is experssed as a flat rate or a percentage.",
    ttype: "HL7",
    steward: "InM",
    where_used: "Data type CP.6",
    hl7_version: "2.3",
};

pub static TABLE_0299_METADATA: V2MetadataTable = V2MetadataTable {
    table: 299usize,
    description: "HL7-defined table of codes identifying the type of encoding used to represent successive octets of binary data as displayable ASCII characters.  These are defined by IETF; more information may be found at https://www.ietf.org/rfc/rfc1521.txt",
    ttype: "HL7",
    steward: "",
    where_used: "",
    hl7_version: "2.3",
};

pub static TABLE_0300_METADATA: V2MetadataTable = V2MetadataTable {
    table: 300usize,
    description: "Table of codes which specify the unique name of the system that stores the data. It was previously named the Application ID.  No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "HD.1",
    hl7_version: "2.3",
};

pub static TABLE_0301_METADATA: V2MetadataTable = V2MetadataTable {
    table: 301usize,
    description: "HL7-defined table of codes specifying the type of UID (Universal Identifier). Open Issue:  Table 0301 has a mix of class and instance identifiers for namespaces, which is improper.  The values for instances. such as CLIA, CLIP, CAP, and NPI. These were added for pragmatic issues related to older datatypes that were universally bound tand should not be here but are needed for implementation pragmatics.   These should be annotated in Comment or Usage Notes that these are not universal ID types really.",
    ttype: "HL7",
    steward: "InM",
    where_used: "HD.3, EI.4",
    hl7_version: "2.3",
};

pub static TABLE_0302_METADATA: V2MetadataTable = V2MetadataTable {
    table: 302usize,
    description: "Table of codes specifying the point where patient care is administered.  It is conditional on Person Location Type (e.g., nursing unit or department or clinic). No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "PL.1, NDL.4",
    hl7_version: "2.3",
};

pub static TABLE_0303_METADATA: V2MetadataTable = V2MetadataTable {
    table: 303usize,
    description: "Table of codes specifying the patient's room.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "PL.2, NDL.5",
    hl7_version: "2.3",
};

pub static TABLE_0304_METADATA: V2MetadataTable = V2MetadataTable {
    table: 304usize,
    description: "Table of codes specifying the patient's bed.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "PL.3, NDL.6",
    hl7_version: "2.3",
};

pub static TABLE_0305_METADATA: V2MetadataTable = V2MetadataTable {
    table: 305usize,
    description: "Table of codes specifying the categorization of the person's location.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "PL.6, NDL.9",
    hl7_version: "2.3",
};

pub static TABLE_0306_METADATA: V2MetadataTable = V2MetadataTable {
    table: 306usize,
    description: "Table of codes specifying the status or availability of the location, such as the bed status.   No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "PL.5, NDL.8",
    hl7_version: "2.3",
};

pub static TABLE_0307_METADATA: V2MetadataTable = V2MetadataTable {
    table: 307usize,
    description:
        "Table of codes specifying the building where the person is located.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "PL.7, NDL.10",
    hl7_version: "2.3",
};

pub static TABLE_0308_METADATA: V2MetadataTable = V2MetadataTable {
    table: 308usize,
    description:
        "Table of codes specifying the floor where the person is located.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "PL.8, NDL.11",
    hl7_version: "2.3",
};

pub static TABLE_0309_METADATA: V2MetadataTable = V2MetadataTable {
    table: 309usize,
    description: "Table of codes specifying the type of insurance coverage or what types of services are covered for the purposes of a billing system.  For example, a physician billing system will only want to receive insurance information for plans that cover physician/professional charges.",
    ttype: "User",
    steward: "FM",
    where_used: "IN1-47",
    hl7_version: "2.3",
};

pub static TABLE_0311_METADATA: V2MetadataTable = V2MetadataTable {
    table: 311usize,
    description: "Table of codes specifying a next of kin/associated party's job status.",
    ttype: "User",
    steward: "PA",
    where_used: "NK1-34",
    hl7_version: "2.3",
};

pub static TABLE_0312_METADATA: V2MetadataTable = V2MetadataTable {
    table: 312usize,
    description: "Table of codes specifying the extent of insurance coverage for a participating member (e.g., single, family, etc.).  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN2-59",
    hl7_version: "2.3",
};

pub static TABLE_0313_METADATA: V2MetadataTable = V2MetadataTable {
    table: 313usize,
    description:
        "Table of codes specifying how an insurance policy is established.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "IN2-60",
    hl7_version: "2.3",
};

pub static TABLE_0315_METADATA: V2MetadataTable = V2MetadataTable {
    table: 315usize,
    description: "Table of codes specifying whether or not the patient has a living will and, if so, whether a copy fo the living will is on file at the healthcare facility.  If the patient does not have a living will, the value of this field indicates whether the patient was provided information on living wills.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-43, PD1-7",
    hl7_version: "2.3",
};

pub static TABLE_0316_METADATA: V2MetadataTable = V2MetadataTable {
    table: 316usize,
    description: "Table of codes specifying whether the patient wants to donate his/her organs and whether an organ donor card or similar documentation is on file with the healthcare organization.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-44, PD1-8",
    hl7_version: "2.3",
};

pub static TABLE_0317_METADATA: V2MetadataTable = V2MetadataTable {
    table: 317usize,
    description: "Table of codes specifying the coded entry associated with a given point in time during the waveform recording.  Note codes beyond 9903 may exist; extensions to this table may be done by incrementing the code value.",
    ttype: "User",
    steward: "OO",
    where_used: "OBX ANO",
    hl7_version: "2.3",
};

pub static TABLE_0319_METADATA: V2MetadataTable = V2MetadataTable {
    table: 319usize,
    description: "Table of codes specifying the accounting code that identifies the department in order to charge for the item.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "RQD-7",
    hl7_version: "2.3",
};

pub static TABLE_0320_METADATA: V2MetadataTable = V2MetadataTable {
    table: 320usize,
    description:
        "Table of codes identifying an item in order to charge for the item.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "RQD-8",
    hl7_version: "2.3",
};

pub static TABLE_0321_METADATA: V2MetadataTable = V2MetadataTable {
    table: 321usize,
    description:
        "HL7-defined table of codes specifying the method by which treatment is dispensed.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXE-30, RXD-24",
    hl7_version: "2.3",
};

pub static TABLE_0322_METADATA: V2MetadataTable = V2MetadataTable {
    table: 322usize,
    description:
        "HL7-defined table of codes specifying the status of the treatment administration event.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXA-20",
    hl7_version: "2.3",
};

pub static TABLE_0324_METADATA: V2MetadataTable = V2MetadataTable {
    table: 324usize,
    description: "Table of codes specifying an identifier code to show which characteristic is being communicated with the segment.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LCH-4",
    hl7_version: "2.3",
};

pub static TABLE_0325_METADATA: V2MetadataTable = V2MetadataTable {
    table: 325usize,
    description: "Table of codes specifying an identifier code to show which relationship is being communicated with the segment.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LRL-4",
    hl7_version: "2.3",
};

pub static TABLE_0326_METADATA: V2MetadataTable = V2MetadataTable {
    table: 326usize,
    description: "Table of codes specifying the level on which data are being sent.  It is the indicator used to send data at two levels, visit and account.  HL7 recommends sending an \"A\" or no value when the data in the message are at the account level or \"V\" to indicate that the data sent in the message are at the visit level.",
    ttype: "User",
    steward: "PA",
    where_used: "PV1-51",
    hl7_version: "2.3",
};

pub static TABLE_0327_METADATA: V2MetadataTable = V2MetadataTable {
    table: 327usize,
    description: "Table of codes that specify a person's job code.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "JCC.1",
    hl7_version: "2.3",
};

pub static TABLE_0328_METADATA: V2MetadataTable = V2MetadataTable {
    table: 328usize,
    description:
        "Table of codes that specify a person's employee classification.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "JCC.2",
    hl7_version: "2.3",
};

pub static TABLE_0329_METADATA: V2MetadataTable = V2MetadataTable {
    table: 329usize,
    description: "HL7-defined table of codes used to specify the method by which the quantity distributed is measured.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PSH-8, PSH-11",
    hl7_version: "2.3",
};

pub static TABLE_0330_METADATA: V2MetadataTable = V2MetadataTable {
    table: 330usize,
    description: "HL7-defined table of codes used to specify the basis for marketing approval.",
    ttype: "HL7",
    steward: "OO",
    where_used: "PDC-10",
    hl7_version: "2.3",
};

pub static TABLE_0331_METADATA: V2MetadataTable = V2MetadataTable {
    table: 331usize,
    description: "HL7-defined table of codes used to specify the type of facility.",
    ttype: "HL7",
    steward: "OO",
    where_used: "FAC-2",
    hl7_version: "2.3",
};

pub static TABLE_0332_METADATA: V2MetadataTable = V2MetadataTable {
    table: 332usize,
    description: "HL7-defined table of codes used to indicate (in certain systems) whether a lower level source identifier is an initiate or accept type.",
    ttype: "HL7",
    steward: "InM",
    where_used: "NST-3",
    hl7_version: "2.3",
};

pub static TABLE_0333_METADATA: V2MetadataTable = V2MetadataTable {
    table: 333usize,
    description:
        "Table of codes specifying the driver's license issuing authority.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "DLN.2",
    hl7_version: "2.3",
};

pub static TABLE_0334_METADATA: V2MetadataTable = V2MetadataTable {
    table: 334usize,
    description: "Table of codes used to specify to which person the disability information relates in the message.  For example, if the value is PT, the disability information relates to the patient.",
    ttype: "User",
    steward: "PA",
    where_used: "DB1-2",
    hl7_version: "2.3",
};

pub static TABLE_0335_METADATA: V2MetadataTable = V2MetadataTable {
    table: 335usize,
    description: "Table of codes used to specify the interval between repeated services. See the Comment/Usage Note in the table below, as the table contains both precoordinated codes that may be used in an HL7 field or component and also explanatory patterns illustrating the syntax used to construct expressions using the codes and other modifiers.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "RI.1, RPT.1",
    hl7_version: "2.3",
};

pub static TABLE_0336_METADATA: V2MetadataTable = V2MetadataTable {
    table: 336usize,
    description:
        "Table of codes used to specify the reason for which the referral will take place.",
    ttype: "User",
    steward: "OO",
    where_used: "RF1-10",
    hl7_version: "2.3",
};

pub static TABLE_0337_METADATA: V2MetadataTable = V2MetadataTable {
    table: 337usize,
    description: "HL7-defined table of codes used to specify the status of the practitioner's speciality certification.",
    ttype: "HL7",
    steward: "InM/PA",
    where_used: "SPD.3, PRA-5",
    hl7_version: "2.3",
};

pub static TABLE_0338_METADATA: V2MetadataTable = V2MetadataTable {
    table: 338usize,
    description:
        "Table of codes specifying the type of number used for the practitioner identification.",
    ttype: "User",
    steward: "InM",
    where_used: "PLN.2, PRD-7, CTD-7",
    hl7_version: "2.3",
};

pub static TABLE_0339_METADATA: V2MetadataTable = V2MetadataTable {
    table: 339usize,
    description: "Table of codes specifying the status of the patient's or the patient's representative's consent for responsibility to pay for potentially uninsured services. This element was introduced to satisfy CMS Medical Necessity requirements for outpatient services in the United States. Includes concepts such as (a) whether the associated diagnosis codes for the service are subject to medical necessity procedures, (b) whether, for this type of service, the patient has been informed that they may be responsible for payment for the service, and (c) whether the patient agrees to be billed for this service.",
    ttype: "User",
    steward: "OO",
    where_used: "ORC-20, FT1-27",
    hl7_version: "2.3.1",
};

pub static TABLE_0340_METADATA: V2MetadataTable = V2MetadataTable {
    table: 340usize,
    description: "Table of codes that specify a procedure code modifier to a procedure code. Procedure code modifiers are defined by regulatory agencies such as CMS and the AMA.  Multiple modifiers may be reported.  The modifiers are sequenced in priority according to user entry.  This is a requirement of the UB and the 1500 claim forms.  Multiple modifiers are allowed and the order placed on the form affects reimbursement.",
    ttype: "External",
    steward: "FM",
    where_used: "FT1-26, OBR-45, IIM-15",
    hl7_version: "2.3.1",
};

pub static TABLE_0341_METADATA: V2MetadataTable = V2MetadataTable {
    table: 341usize,
    description: "Table of codes that specify a guarantor's credit rating.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "GT1-23",
    hl7_version: "2.3.1",
};

pub static TABLE_0342_METADATA: V2MetadataTable = V2MetadataTable {
    table: 342usize,
    description: "Table of codes that specify a designation as a military recipient.  This field is defined by CMS or other regulatory agencies.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN2-11",
    hl7_version: "2.3.1",
};

pub static TABLE_0343_METADATA: V2MetadataTable = V2MetadataTable {
    table: 343usize,
    description: "Table of codes that specify a military program for the handicapped in which a patient is enrolled.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN2-59",
    hl7_version: "2.3.1",
};

pub static TABLE_0344_METADATA: V2MetadataTable = V2MetadataTable {
    table: 344usize,
    description: "Table of codes used to specify the relationship of the patient to the insured, as defined by  CMS or other regulatory agencies.",
    ttype: "User",
    steward: "FM",
    where_used: "IN2-72",
    hl7_version: "2.3.1",
};

pub static TABLE_0345_METADATA: V2MetadataTable = V2MetadataTable {
    table: 345usize,
    description: "Table of codes that specify  reasons an appeal was made on a non-concur for certification.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN3-17",
    hl7_version: "2.3.1",
};

pub static TABLE_0346_METADATA: V2MetadataTable = V2MetadataTable {
    table: 346usize,
    description: "Table of codes that specify a certification agency.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "IN3-18",
    hl7_version: "2.3.1",
};

pub static TABLE_0347_METADATA: V2MetadataTable = V2MetadataTable {
    table: 347usize,
    description: "Table of codes specifying the names of the principal country subdivisions (e.g., provinces or states).  The values in the table are country specific.  For example, in the US, the Federal Information Processing Standard (FIPS) alpha codes may be used by local agreement.",
    ttype: "User",
    steward: "InM",
    where_used: "CX.9, PPN.23, XCN.22, ACC-4",
    hl7_version: "2.3.1",
};

pub static TABLE_0350_METADATA: V2MetadataTable = V2MetadataTable {
    table: 350usize,
    description: "Table of codes that specify a National Uniform Billing Committee (NUBC) code for the event or occurrence relating to a bill that may affect payer processing.  In the US, NUBC codes generally used, see code system 2.16.840.1.113883.6.301.7; more information may be found at http://www.nubc.org/become.html",
    ttype: "External",
    steward: "InM/FM",
    where_used: "OCD.1",
    hl7_version: "2.3.1",
};

pub static TABLE_0351_METADATA: V2MetadataTable = V2MetadataTable {
    table: 351usize,
    description: "Externally defined table of codes specifying a National Uniform Billing Committee (NUBC) code that identifies an event that relates to the payment of a claim.  In the US, NUBC codes generally used, see code system 2.16.840.1.113883.6.301.8; more information may be found at http://www.nubc.org/become.html.  The UB-04 Data Specifications Manual with the codes is available by subscription from NUBC at http://www.nubc.org/become.html.",
    ttype: "External",
    steward: "InM/FM",
    where_used: "OSP.1",
    hl7_version: "2.3.1",
};

pub static TABLE_0353_METADATA: V2MetadataTable = V2MetadataTable {
    table: 353usize,
    description: "HL7-defined table of codes that represent an exception identifier code; that is, a code that is not defined in the value set (either model or site-extended).   These are occationsally referred to a 'flavors of null' although this set of concepts is specific to the CWE datatype used in Version 2 messaging, and the codes may be used in the 'identifier' component of the 'triplets' in that datatype.",
    ttype: "HL7",
    steward: "Vocab",
    where_used: "CWE.1",
    hl7_version: "2.3.1",
};

pub static TABLE_0354_METADATA: V2MetadataTable = V2MetadataTable {
    table: 354usize,
    description: "HL7-defined table of abstract message structure codes. Each code identifies a specific message structure abstract syntax as published in the HL7 Version 2 standard.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MSH-9.3",
    hl7_version: "2.3.1",
};

pub static TABLE_0355_METADATA: V2MetadataTable = V2MetadataTable {
    table: 355usize,
    description:
        "HL7-defined table of codes used to specify the type for the master file record identifier.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MFE-5",
    hl7_version: "2.3.1",
};

pub static TABLE_0356_METADATA: V2MetadataTable = V2MetadataTable {
    table: 356usize,
    description: "HL7-defined table of codes that specify the scheme used when any alternative character sets are specified in the second or later iterations of MSH                 -18 Character Set, and if any special handling scheme is needed.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MSH-20",
    hl7_version: "2.3.1",
};

pub static TABLE_0357_METADATA: V2MetadataTable = V2MetadataTable {
    table: 357usize,
    description: "HL7-defined table of codes specifying the HL7 (communications) error code.",
    ttype: "HL7",
    steward: "InM",
    where_used: "ERR-3",
    hl7_version: "2.3.1",
};

pub static TABLE_0358_METADATA: V2MetadataTable = V2MetadataTable {
    table: 358usize,
    description: "Table of codes specifying the name and/or code of a group of practitioners to which this practitioner belongs.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "DG1-15",
    hl7_version: "2.3.1",
};

pub static TABLE_0359_METADATA: V2MetadataTable = V2MetadataTable {
    table: 359usize,
    description: "Table of codes that identify the significance or priority of the diagnosis code. Note that the codes are numeric, and the number of the code represents the ordinal priority of the associated diagnosis. The predefined codes are the most common, and just a starter set, as the codes are an unbounded list; additional ranked procedures may be signified by incrementing the code value as needed.",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.3.1",
};

pub static TABLE_0360_METADATA: V2MetadataTable = V2MetadataTable {
    table: 360usize,
    description: "Table of codes specifying an educational degree (e.g., MD).  Used in the CNN datatype (names and identifiers of clinicians) in Version 2 messaging.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "CNN.7",
    hl7_version: "2.3.1",
};

pub static TABLE_0361_METADATA: V2MetadataTable = V2MetadataTable {
    table: 361usize,
    description: "Table of codes that identify a sending application among all other applications within the network enterprise.  The network enterprise consists of all those applications that participate in the exchange of HL7 messages within the enterprise.  No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "MSH-3, MSH-5",
    hl7_version: "2.3.1",
};

pub static TABLE_0362_METADATA: V2MetadataTable = V2MetadataTable {
    table: 362usize,
    description: "Table of codes specifying the site-specific name for the facility used by this application.  No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "MSH-4, MSH-6",
    hl7_version: "2.3.1",
};

pub static TABLE_0363_METADATA: V2MetadataTable = V2MetadataTable {
    table: 363usize,
    description: "Table of codes specifying a unique name of the system (or organization or agency or department) that creates the data.  No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "CX.4, EI.2, PL.11, PPN.9, XCN.9, XON.6",
    hl7_version: "2.3.1",
};

pub static TABLE_0364_METADATA: V2MetadataTable = V2MetadataTable {
    table: 364usize,
    description: "Table of codes that identify the type of comment text being sent in the specific comment record.",
    ttype: "User",
    steward: "InM",
    where_used: "NTE-4",
    hl7_version: "2.3.1",
};

pub static TABLE_0365_METADATA: V2MetadataTable = V2MetadataTable {
    table: 365usize,
    description: "HL7-defined table of codes that identify the status the equipment was in at the time the transaction was initiated.",
    ttype: "HL7",
    steward: "OO",
    where_used: "EQU-3",
    hl7_version: "2.4",
};

pub static TABLE_0366_METADATA: V2MetadataTable = V2MetadataTable {
    table: 366usize,
    description: "HL7-defined table of codes that identify the current state of control associated with the equipment.   Equipment can either work autonomously ('Local' control state) or it can be controlled by another system, e.g., LAS computer ('Remote' control state).",
    ttype: "HL7",
    steward: "OO",
    where_used: "EQU-4",
    hl7_version: "2.4",
};

pub static TABLE_0367_METADATA: V2MetadataTable = V2MetadataTable {
    table: 367usize,
    description: "HL7-defined table of codes that identify the highest level of the alert state (e.g.,highest alert severity) that is associated with the indicated equipment (e.g. processing event, inventory event, QC event).",
    ttype: "HL7",
    steward: "OO",
    where_used: "EQU-5, NDS-3",
    hl7_version: "2.4",
};

pub static TABLE_0368_METADATA: V2MetadataTable = V2MetadataTable {
    table: 368usize,
    description: "Table of codes that identify the comment the component is to initiate.",
    ttype: "User",
    steward: "OO",
    where_used: "ECD-2, ISD-2",
    hl7_version: "2.4",
};

pub static TABLE_0369_METADATA: V2MetadataTable = V2MetadataTable {
    table: 369usize,
    description: "Table of codes that identify the role of a sample.",
    ttype: "User",
    steward: "OO",
    where_used: "SPM-11,O  B  R-15",
    hl7_version: "2.4",
};

pub static TABLE_0370_METADATA: V2MetadataTable = V2MetadataTable {
    table: 370usize,
    description: "HL7-defined table of codes that identify the status of the unique container in which the specimen resides at the time the transaction was initiated.",
    ttype: "HL7",
    steward: "OO",
    where_used: "SAC-8",
    hl7_version: "2.4",
};

pub static TABLE_0371_METADATA: V2MetadataTable = V2MetadataTable {
    table: 371usize,
    description: "HL7-defined table of codes specifying any additive introduced to the specimen before or at the time of collection.  These additives may be introduced in order to preserve, maintain or enhance the particular nature or component of the specimen.",
    ttype: "HL7",
    steward: "OO",
    where_used: "SPM-6, SAC-27",
    hl7_version: "2.4",
};

pub static TABLE_0372_METADATA: V2MetadataTable = V2MetadataTable {
    table: 372usize,
    description:
        "Table of codes that identify the specimen component, e.g., supernatant, sediment, etc.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-28",
    hl7_version: "2.4",
};

pub static TABLE_0373_METADATA: V2MetadataTable = V2MetadataTable {
    table: 373usize,
    description:
        "Table of codes that identify the specimen treatment performed during lab processing.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-30",
    hl7_version: "2.4",
};

pub static TABLE_0374_METADATA: V2MetadataTable = V2MetadataTable {
    table: 374usize,
    description: "Table of codes that identify the specimen contaminant identifier associated with the specimen in the container.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-40",
    hl7_version: "2.4",
};

pub static TABLE_0375_METADATA: V2MetadataTable = V2MetadataTable {
    table: 375usize,
    description:
        "Table of codes that identify the artificial blood identifier associated with the specimen.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-42",
    hl7_version: "2.4",
};

pub static TABLE_0376_METADATA: V2MetadataTable = V2MetadataTable {
    table: 376usize,
    description: "Table of codes describing how a specimen and/or container needs to be handled from the time of collection through the initiation of testing.",
    ttype: "User",
    steward: "OO",
    where_used: "SPM-15, SAC-43,  PAC-7, OM 4-15",
    hl7_version: "2.4",
};

pub static TABLE_0377_METADATA: V2MetadataTable = V2MetadataTable {
    table: 377usize,
    description: "Table of codes that identify the other environmental factors associated with the specimen in a specific container, e.g., atmospheric exposure.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-44",
    hl7_version: "2.4",
};

pub static TABLE_0378_METADATA: V2MetadataTable = V2MetadataTable {
    table: 378usize,
    description: "Table of codes that identify a type of carrier.  Because the geometry can be different, the carrier type should, if possible, express the number of positions in the carrier.  The definition assumes hierarchical nesting using the following phrases: container is located in a carrier, carrier is located in a tray.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-9",
    hl7_version: "2.4",
};

pub static TABLE_0379_METADATA: V2MetadataTable = V2MetadataTable {
    table: 379usize,
    description: "Table of codes that identify a type of tray.  Because the geometry can be different, the tray type should, if possible, express the number of positions in the tray.  The definition assumes hierarchical nesting using the following phrases: container is located in a carrier, carrier is located in a tray.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-12",
    hl7_version: "2.4",
};

pub static TABLE_0380_METADATA: V2MetadataTable = V2MetadataTable {
    table: 380usize,
    description: "Table of codes that identify a type of separator being used (e.g., a gel separator in a container-not to be confused with the communication separators).  It is recommended the first table entry be \"NO\" meaning \"No Separator\".  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-25",
    hl7_version: "2.4",
};

pub static TABLE_0381_METADATA: V2MetadataTable = V2MetadataTable {
    table: 381usize,
    description: "Table of codes that identify a type of cap that is to be used with a container for decapping, piercing or other mechanisms.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-26",
    hl7_version: "2.4",
};

pub static TABLE_0382_METADATA: V2MetadataTable = V2MetadataTable {
    table: 382usize,
    description: "Table of codes that identify a drug interference  associated with a specimen.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-41",
    hl7_version: "2.4",
};

pub static TABLE_0383_METADATA: V2MetadataTable = V2MetadataTable {
    table: 383usize,
    description: "HL7-defined table of codes identifying the status of the inventoried item.  The status indicates the current status of the substance.",
    ttype: "HL7",
    steward: "OO",
    where_used: "INV-2",
    hl7_version: "2.4",
};

pub static TABLE_0384_METADATA: V2MetadataTable = V2MetadataTable {
    table: 384usize,
    description: "HL7-defined table of codes identifying the type of substance.",
    ttype: "HL7",
    steward: "",
    where_used: "INV-3",
    hl7_version: "2.4",
};

pub static TABLE_0385_METADATA: V2MetadataTable = V2MetadataTable {
    table: 385usize,
    description: "Table of codes that identify a manufacturer of a substance.  Relevant external code systems may be used, e.g., HIBCC Manufacturers Labeler ID Code (LIC), UPC, NDC, etc.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "INV-17, RQ1-2, SID-4",
    hl7_version: "2.4",
};

pub static TABLE_0386_METADATA: V2MetadataTable = V2MetadataTable {
    table: 386usize,
    description: "Table of codes that identify a supplier of a substance.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "INV-18",
    hl7_version: "2.4",
};

pub static TABLE_0387_METADATA: V2MetadataTable = V2MetadataTable {
    table: 387usize,
    description: "Table of codes identifying the response of the previously issued command.",
    ttype: "User",
    steward: "OO",
    where_used: "ECR-1, ISD-3",
    hl7_version: "2.4",
};

pub static TABLE_0388_METADATA: V2MetadataTable = V2MetadataTable {
    table: 388usize,
    description: "HL7-defined table of codes identifying the processing type that applies to the test code.  If this attribute is omitted, then regular production is the default.",
    ttype: "HL7",
    steward: "OO",
    where_used: "TCC-14",
    hl7_version: "2.4",
};

pub static TABLE_0389_METADATA: V2MetadataTable = V2MetadataTable {
    table: 389usize,
    description: "HL7-defined table of codes identifying the repeat status for the analyte/result (e.g. original, rerun, repeat, reflex).  The following are assumptions regarding the table values: Repeated without dilution —                                                   performed usually to confirm correctness of results (e.g., in case of results flagged as \"Panic\" or mechanical failures).  Repeated with dilution —                                                   performed usually in the case the original result exceeded the measurement range (technical limits).  Reflex test —                                                   this test is performed as the consequence of rules triggered based on other test result(s).",
    ttype: "HL7",
    steward: "OO",
    where_used: "TCD-8",
    hl7_version: "2.4",
};

pub static TABLE_0391_METADATA: V2MetadataTable = V2MetadataTable {
    table: 391usize,
    description: "HL7-defined table of codes specifying the optional segment groups which are to be included in a response.",
    ttype: "HL7",
    steward: "InM",
    where_used: "RCP.7",
    hl7_version: "2.4",
};

pub static TABLE_0392_METADATA: V2MetadataTable = V2MetadataTable {
    table: 392usize,
    description: "Table of codes identifying what search components (e.g., name, birthdate, social security number) of the record returned matched the original query where the responding system does not assign numeric match weights or confidence levels. It provides a method for passing a descriptive indication of the reason a particular record was found.",
    ttype: "User",
    steward: "InM",
    where_used: "QRI.2",
    hl7_version: "2.4",
};

pub static TABLE_0393_METADATA: V2MetadataTable = V2MetadataTable {
    table: 393usize,
    description: "Table of codes identifying the name or identity of the specific search algorithm to which the RCP-5 Search Confidence Threshold and the QRI-1 Candidate Confidence refer.",
    ttype: "User",
    steward: "InM",
    where_used: "QRI.3",
    hl7_version: "2.4",
};

pub static TABLE_0394_METADATA: V2MetadataTable = V2MetadataTable {
    table: 394usize,
    description:
        "HL7-defined table of codes identifying the timing and grouping of the response message(s).",
    ttype: "HL7",
    steward: "InM",
    where_used: "RCP.3",
    hl7_version: "2.4",
};

pub static TABLE_0395_METADATA: V2MetadataTable = V2MetadataTable {
    table: 395usize,
    description: "HL7-defined table of codes identifying whether the subscription is new or is being modified.",
    ttype: "HL7",
    steward: "InM",
    where_used: "RCP.5",
    hl7_version: "2.4",
};

pub static TABLE_0396_METADATA: V2MetadataTable = V2MetadataTable {
    table: 396usize,
    description: "HL7-defined table of specifying the coding system.  This table is maintained outside of the published Version 2 standards; the content is not listed here; the content is maintained outside of the Version 2 Product Family maintenance process.  For the list of codes in the table, see the HL7 Webpage rendering, at http://www.hl7.org/Special/committees/vocab/table_0396/index.cfm.",
    ttype: "HL7-EXT",
    steward: "Vo     c     abulary",
    where_used: "CWE.3, CWE.6, CWE, and numerous places",
    hl7_version: "2.4",
};

pub static TABLE_0397_METADATA: V2MetadataTable = V2MetadataTable {
    table: 397usize,
    description: "HL7-defined table of codes identifying how the field or parameter will be sorted and, if sorted, whether the sort will be case sensitive (the default) or not.",
    ttype: "HL7",
    steward: "InM",
    where_used: "SRT.2",
    hl7_version: "2.4",
};

pub static TABLE_0398_METADATA: V2MetadataTable = V2MetadataTable {
    table: 398usize,
    description: "HL7-defined table of codes identifying whether it is a fragmented message or part of an interactive continuation message.",
    ttype: "HL7",
    steward: "InM",
    where_used: "CH02, DSC.2",
    hl7_version: "2.4",
};

pub static TABLE_0399_METADATA: V2MetadataTable = V2MetadataTable {
    table: 399usize,
    description: "Table of codes that identifies a country of origin for a message.  It will be used primarily to specify default elements, such as currency denominations. The values to be used are those of ISO 3166. The ISO 3166 table has three separate forms of the country code: HL7 specifies that the 3-character (alphabetic) fo rm be used for the country code.",
    ttype: "External",
    steward: "InM",
    where_used: "MSH-17, CX.9, PPN.23,VID.2, XAD.6, XCN.22",
    hl7_version: "2.4",
};

pub static TABLE_0401_METADATA: V2MetadataTable = V2MetadataTable {
    table: 401usize,
    description: "Table of codes which specify codes that indicate an agency that the practitioner is authorized to bill for medical services.  Existing codes only for use in the United States.",
    ttype: "User",
    steward: "PA",
    where_used: "PRA-11",
    hl7_version: "2.4",
};

pub static TABLE_0402_METADATA: V2MetadataTable = V2MetadataTable {
    table: 402usize,
    description: "Table of codes which specify a categorization of an academic institution that grants a degree to a Staff Member.",
    ttype: "User",
    steward: "PA",
    where_used: "EDU-7",
    hl7_version: "2.4",
};

pub static TABLE_0403_METADATA: V2MetadataTable = V2MetadataTable {
    table: 403usize,
    description: "Table of codes which specify codes that indicate the ability that a Staff Member possesses with respect to the language.",
    ttype: "User",
    steward: "PA",
    where_used: "LAN-3",
    hl7_version: "2.4",
};

pub static TABLE_0404_METADATA: V2MetadataTable = V2MetadataTable {
    table: 404usize,
    description: "HL7-defined table of codes which specify the level of knowledge a person possesses with respect to a language ability identified.",
    ttype: "User",
    steward: "PA",
    where_used: "LAN-4",
    hl7_version: "2.4",
};

pub static TABLE_0405_METADATA: V2MetadataTable = V2MetadataTable {
    table: 405usize,
    description: "Table of codes which specify the hierarchical components of an organization unit, as defined by the institution.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "ORG-2",
    hl7_version: "2.4",
};

pub static TABLE_0406_METADATA: V2MetadataTable = V2MetadataTable {
    table: 406usize,
    description: "HL7-defined table of codes that specify the environment in which the provider acts in the role associated with the provider type, and inludes codes for venues outside of formal organized healthcare settings, such as Home. The provider environment is not the specialty for the provider.",
    ttype: "User",
    steward: "OO",
    where_used: "PRT-7, ROL-10",
    hl7_version: "2.4",
};

pub static TABLE_0409_METADATA: V2MetadataTable = V2MetadataTable {
    table: 409usize,
    description: "HL7-defined table of codes that specify a type of change being requested (if NMR query) or announced (if NMD unsolicited update).",
    ttype: "User",
    steward: "InM",
    where_used: "NSC-1",
    hl7_version: "2.3.1",
};

pub static TABLE_0411_METADATA: V2MetadataTable = V2MetadataTable {
    table: 411usize,
    description: "Table of codes that specify supplemental service information sent between a placer system and a filler system for the universal procedure code reported in OBR-4 Universal Service ID. This specifies ordering information detail that is not available in other specific tables for fields in the OBR segment.  These might be details such as whether a study is to be done on the right or left, for example, where the study is of the arm and the order master file does not distinguish right from left, or whether a study is to be done with or without contrast (when the order master file does not make such distinctions).",
    ttype: "User",
    steward: "OO",
    where_used: "OBR-46, OBR-47, AIS-11,A  IS-12",
    hl7_version: "2.4",
};

pub static TABLE_0412_METADATA: V2MetadataTable = V2MetadataTable {
    table: 412usize,
    description: "Table of codes that specify a category name (term given to a group of service items for the purpose of classification). Examples: Laboratory, Pharmacy, Diagnostic Imaging, etc.  No suggested values.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "OM7-3",
    hl7_version: "2.4",
};

pub static TABLE_0413_METADATA: V2MetadataTable = V2MetadataTable {
    table: 413usize,
    description: "Table of codes that provide an identifier for the consent specified for a service item.  No suggested values.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "OM7-12",
    hl7_version: "2.4",
};

pub static TABLE_0414_METADATA: V2MetadataTable = V2MetadataTable {
    table: 414usize,
    description: "Table of codes that specify a unit of time.  No suggested values.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "OM7-16",
    hl7_version: "2.4",
};

pub static TABLE_0415_METADATA: V2MetadataTable = V2MetadataTable {
    table: 415usize,
    description: "HL7-defined table of codes that specify a type of hospital receiving a transfer patient, which affects how a facility is reimbursed under diagnosis related group (DRG's), for example, exempt or non-exempt.",
    ttype: "HL7",
    steward: "FM",
    where_used: "DRG-11",
    hl7_version: "2.4",
};

pub static TABLE_0416_METADATA: V2MetadataTable = V2MetadataTable {
    table: 416usize,
    description: "HL7-defined table of codes that specify a procedure's priority ranking relative to its DRG.",
    ttype: "User",
    steward: "FM",
    where_used: "PR1-17",
    hl7_version: "2.4",
};

pub static TABLE_0417_METADATA: V2MetadataTable = V2MetadataTable {
    table: 417usize,
    description: "HL7-defined table of codes that specify",
    ttype: "User",
    steward: "FM",
    where_used: "PR1-18",
    hl7_version: "2.4",
};

pub static TABLE_0418_METADATA: V2MetadataTable = V2MetadataTable {
    table: 418usize,
    description: "HL7-defined table of codes specifying a number that identifies the significance or priority of the procedure code.",
    ttype: "User",
    steward: "FM",
    where_used: "PR1-14",
    hl7_version: "2.4",
};

pub static TABLE_0421_METADATA: V2MetadataTable = V2MetadataTable {
    table: 421usize,
    description: "Table of codes specifying the severity ranking of a patient's illness.",
    ttype: "User",
    steward: "FM",
    where_used: "ABS-3",
    hl7_version: "2.4",
};

pub static TABLE_0422_METADATA: V2MetadataTable = V2MetadataTable {
    table: 422usize,
    description:
        "Table of codes specifying a patient's prioritization within the context of this abstract.",
    ttype: "User",
    steward: "FM",
    where_used: "ABS-6",
    hl7_version: "2.4",
};

pub static TABLE_0423_METADATA: V2MetadataTable = V2MetadataTable {
    table: 423usize,
    description: "Table of codes specifying the reason a non-urgent patient presents to the emergency room for treatment instead of a clinic or physican office.",
    ttype: "User",
    steward: "FM",
    where_used: "ABS-9",
    hl7_version: "2.4",
};

pub static TABLE_0424_METADATA: V2MetadataTable = V2MetadataTable {
    table: 424usize,
    description: "Table of codes specifying the status of the birth in relation to the gestation",
    ttype: "User",
    steward: "FM",
    where_used: "ABS-11",
    hl7_version: "2.4",
};

pub static TABLE_0425_METADATA: V2MetadataTable = V2MetadataTable {
    table: 425usize,
    description: "Table of codes specifying whether the baby was born in or out of the facility.",
    ttype: "User",
    steward: "FM",
    where_used: "ABS-13",
    hl7_version: "2.4",
};

pub static TABLE_0426_METADATA: V2MetadataTable = V2MetadataTable {
    table: 426usize,
    description: "Table of codes specifying the blood product code.",
    ttype: "User",
    steward: "FM",
    where_used: "BLC-1",
    hl7_version: "2.4",
};

pub static TABLE_0427_METADATA: V2MetadataTable = V2MetadataTable {
    table: 427usize,
    description: "Table of codes specifying the incident that occurred during a patient's stay.",
    ttype: "User",
    steward: "FM",
    where_used: "RMI-1",
    hl7_version: "2.4",
};

pub static TABLE_0428_METADATA: V2MetadataTable = V2MetadataTable {
    table: 428usize,
    description: "Table of codes specifying a classification of the incident type.",
    ttype: "User",
    steward: "FM",
    where_used: "RMI-3",
    hl7_version: "2.4",
};

pub static TABLE_0429_METADATA: V2MetadataTable = V2MetadataTable {
    table: 429usize,
    description: "Table of codes specifying the code and/or text indicating the primary use for which the living subject was bred or grown.",
    ttype: "User",
    steward: "PA",
    where_used: "PID-38",
    hl7_version: "2.4",
};

pub static TABLE_0430_METADATA: V2MetadataTable = V2MetadataTable {
    table: 430usize,
    description:
        "Table of codes specifying how the patient was brought to the healthcare facility.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-38",
    hl7_version: "2.4",
};

pub static TABLE_0431_METADATA: V2MetadataTable = V2MetadataTable {
    table: 431usize,
    description: "Table of codes specifying what recreational drugs the patient uses.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-39",
    hl7_version: "2.4",
};

pub static TABLE_0432_METADATA: V2MetadataTable = V2MetadataTable {
    table: 432usize,
    description: "Table of codes specifying the acuity level assigned to the patient at the time of admission.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-40",
    hl7_version: "2.4",
};

pub static TABLE_0433_METADATA: V2MetadataTable = V2MetadataTable {
    table: 433usize,
    description:
        "Table of codes specifying non-clincal precautions that need to be taken with the patient.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-41",
    hl7_version: "2.4",
};

pub static TABLE_0434_METADATA: V2MetadataTable = V2MetadataTable {
    table: 434usize,
    description: "Table of codes specifying the patient's current medical condition for the purpose of communicating to non-medical outside parties, e.g. family, employer, religious minister, media, etc.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-42",
    hl7_version: "2.4",
};

pub static TABLE_0435_METADATA: V2MetadataTable = V2MetadataTable {
    table: 435usize,
    description: "Table of codes specifying the patient's instructions to the healthcare facility.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-45, PD1-15",
    hl7_version: "2.4",
};

pub static TABLE_0436_METADATA: V2MetadataTable = V2MetadataTable {
    table: 436usize,
    description:
        "Table of codes specifying the reason the patient should not be exposed to a substance.",
    ttype: "User",
    steward: "PA",
    where_used: "IAM-9",
    hl7_version: "2.4",
};

pub static TABLE_0437_METADATA: V2MetadataTable = V2MetadataTable {
    table: 437usize,
    description: "Table of codes specifying any type of allergy alert device the patient may be carrying or wearing.",
    ttype: "User",
    steward: "PA",
    where_used: "IAM-16",
    hl7_version: "2.4",
};

pub static TABLE_0438_METADATA: V2MetadataTable = V2MetadataTable {
    table: 438usize,
    description: "Table of codes specifying the verification status for the allergy.",
    ttype: "User",
    steward: "PA",
    where_used: "IAM-17",
    hl7_version: "2.4",
};

pub static TABLE_0440_METADATA: V2MetadataTable = V2MetadataTable {
    table: 440usize,
    description: "HL7-defined table of codes specifying the data type.",
    ttype: "HL7",
    steward: "InM",
    where_used: "RCD.2, RDF-2",
    hl7_version: "2.4",
};

pub static TABLE_0441_METADATA: V2MetadataTable = V2MetadataTable {
    table: 441usize,
    description: "Table of codes specifying the immunization registry status of the patient.",
    ttype: "User",
    steward: "PA",
    where_used: "PD1-16",
    hl7_version: "2.4",
};

pub static TABLE_0442_METADATA: V2MetadataTable = V2MetadataTable {
    table: 442usize,
    description: "Table of codes specifying the types of services provided by the location.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LOC-9",
    hl7_version: "2.4",
};

pub static TABLE_0443_METADATA: V2MetadataTable = V2MetadataTable {
    table: 443usize,
    description: "Table of codes specifying the functional involvement with the activity being transmitted (e.g., Case Manager, Evaluator, Transcriber, Nurse Care Practitioner, Midwife, Physician Assistant, etc.).",
    ttype: "User",
    steward: "PA",
    where_used: "ROL-3",
    hl7_version: "2.4",
};

pub static TABLE_0444_METADATA: V2MetadataTable = V2MetadataTable {
    table: 444usize,
    description: "HL7-defined table of codes specifying  the preferred display order of the components of this person name.",
    ttype: "HL7",
    steward: "InM",
    where_used: "PPN-19, XCN-18, XPN-11",
    hl7_version: "2.4",
};

pub static TABLE_0445_METADATA: V2MetadataTable = V2MetadataTable {
    table: 445usize,
    description: "Table of codes specifying the reliability of patient/person identifying data transmitted via a transaction.",
    ttype: "User",
    steward: "PA",
    where_used: "PID-32",
    hl7_version: "2.4",
};

pub static TABLE_0446_METADATA: V2MetadataTable = V2MetadataTable {
    table: 446usize,
    description: "Table of codes specifying the species of living organism.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PID-35",
    hl7_version: "2.4",
};

pub static TABLE_0447_METADATA: V2MetadataTable = V2MetadataTable {
    table: 447usize,
    description: "Table of codes specifying the specific breed of animal.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PID-36",
    hl7_version: "2.4",
};

pub static TABLE_0448_METADATA: V2MetadataTable = V2MetadataTable {
    table: 448usize,
    description:
        "Table of codes specifying the context in which a name is used.  No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "PPN.17, XCN.16, XPN.9",
    hl7_version: "2.4",
};

pub static TABLE_0450_METADATA: V2MetadataTable = V2MetadataTable {
    table: 450usize,
    description: "HL7-defined table of codes specifying the type of event of the message.",
    ttype: "HL7",
    steward: "OO",
    where_used: "EQP-1",
    hl7_version: "2.4",
};

pub static TABLE_0451_METADATA: V2MetadataTable = V2MetadataTable {
    table: 451usize,
    description:
        "Table of codes specifying the substance that is in inventory.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "INV-1",
    hl7_version: "2.4",
};

pub static TABLE_0452_METADATA: V2MetadataTable = V2MetadataTable {
    table: 452usize,
    description: "Table of codes specifying the major grouping of the service or occupation of the practitioner at a specific organization unit.   In the US, it is suggested to use ANSI ASC X12 Health Care Provider Taxonomy, Level 1-Type",
    ttype: "User",
    steward: "PA",
    where_used: "ORG-6",
    hl7_version: "2.4",
};

pub static TABLE_0453_METADATA: V2MetadataTable = V2MetadataTable {
    table: 453usize,
    description: "Table of codes specifying the more specific service or occupation within the healthcare provider type of the practitioner at a specific organization unit. Health Care Provider Classific ationIn the US, it is suggested to use ANSI ASC X12 Health Care Provider Taxonomy, Level 2-Classificatio n",
    ttype: "User",
    steward: "PA",
    where_used: "ORG-7",
    hl7_version: "2.4",
};

pub static TABLE_0454_METADATA: V2MetadataTable = V2MetadataTable {
    table: 454usize,
    description: "Table of codes specifying the segment of the population that a health care provider chooses to service, a specific medical service, a specialization in treating a specific disease, or any other descriptive characteristic about the provider  ’s practice relating to the services rendered of the practitioner at a specific organization unit.Health Care Provider Area of Specialization.  In the US it is suggested to use ANSI ASC X12 Health Care Provider Taxonomy, Level 2-Classification.",
    ttype: "User",
    steward: "PA",
    where_used: "ORG-8",
    hl7_version: "2.4",
};

pub static TABLE_0455_METADATA: V2MetadataTable = V2MetadataTable {
    table: 455usize,
    description: "Table of codes specifying the specific type of bill with digit 1 showing type of facility, digit 2 showing bill classification and digit 3 showing frequency.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "GP1-1",
    hl7_version: "2.4",
};

pub static TABLE_0456_METADATA: V2MetadataTable = V2MetadataTable {
    table: 456usize,
    description: "Externally defined table of codes specifying a service line revenue code.  These are claim codes indicating the identifying number for the product or service provided.  In the US, NUBC codes generally used, see code system 2.16.840.1.113883.6.301.3; more information may be found at http://www.nubc.org/become.html. The  UB-04 Data Specifications Manual with the codes is available by subscription from NUBC at http://www.nubc.org/become.html, UB form locater 42.",
    ttype: "User",
    steward: "FM",
    where_used: "GP1-2, FT1-41",
    hl7_version: "2.4",
};

pub static TABLE_0457_METADATA: V2MetadataTable = V2MetadataTable {
    table: 457usize,
    description: "Table of codes specifying the final status of the claim.",
    ttype: "User",
    steward: "FM",
    where_used: "GP1-3",
    hl7_version: "2.4",
};

pub static TABLE_0458_METADATA: V2MetadataTable = V2MetadataTable {
    table: 458usize,
    description: "Table of codes that specify the edits that result from processing the HCPCS/CPT procedures for a record after evaluating all the codes, revenue codes, and modifiers.  The codes listed as examples are not an exhaustive or current list, refer to OPPS Final Rule.  OCE (Outpatient Code Editor) edits also exist at the pre-procedure level.  This field is defined by CMS or other regulatory agencies.",
    ttype: "User",
    steward: "FM",
    where_used: "GP1-3",
    hl7_version: "2.4",
};

pub static TABLE_0459_METADATA: V2MetadataTable = V2MetadataTable {
    table: 459usize,
    description:
        "Table of codes specifying the action to be taken during reimbursement calculations.",
    ttype: "External",
    steward: "FM",
    where_used: "GP2-4",
    hl7_version: "2.4",
};

pub static TABLE_0460_METADATA: V2MetadataTable = V2MetadataTable {
    table: 460usize,
    description: "Table of codes specifying the OCE status of the line item.",
    ttype: "User",
    steward: "FM",
    where_used: "GP2-5",
    hl7_version: "2.4",
};

pub static TABLE_0461_METADATA: V2MetadataTable = V2MetadataTable {
    table: 461usize,
    description:
        "Table of codes specifying the license number for the facility.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LOC-7",
    hl7_version: "2.4",
};

pub static TABLE_0462_METADATA: V2MetadataTable = V2MetadataTable {
    table: 462usize,
    description: "Table of codes specifying the cost center to which this location belongs.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "LDP-12",
    hl7_version: "2.4",
};

pub static TABLE_0463_METADATA: V2MetadataTable = V2MetadataTable {
    table: 463usize,
    description: "Table of codes specifying an identifying stock number, if any, which might be used, for example, as a cross reference for materials management.  No suggested values.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "CDM-9",
    hl7_version: "2.4",
};

pub static TABLE_0464_METADATA: V2MetadataTable = V2MetadataTable {
    table: 464usize,
    description: "Table of codes specifying the facility of the institution for which this price (for the preceding CDM entry) is valid.  For use when needing multi-facility pricing. If null, assume all facilities. In a multi-facility environment, the facility associated with this chargeable item may not be the same as the sending or receiving facility identified in the MSH segment. Use only when the price is not the same for all facilities, that is, a null value indicates that this pricing is valid for all facilities.  No suggested values.",
    ttype: "User",
    steward: "InM/FM",
    where_used: "PRC-2",
    hl7_version: "2.4",
};

pub static TABLE_0465_METADATA: V2MetadataTable = V2MetadataTable {
    table: 465usize,
    description: "HL7-defined table of codes specifying an indication of the representation provided by the data item.",
    ttype: "HL7",
    steward: "InM",
    where_used: "PPN.16, XAD.11, XCN.15, XON.9",
    hl7_version: "2.4",
};

pub static TABLE_0466_METADATA: V2MetadataTable = V2MetadataTable {
    table: 466usize,
    description:
        "Table of codes specifying the derived Ambulatory Payment Classification (APC) code.",
    ttype: "User",
    steward: "FM",
    where_used: "GP2-7",
    hl7_version: "2.4",
};

pub static TABLE_0467_METADATA: V2MetadataTable = V2MetadataTable {
    table: 467usize,
    description: "Table of codes that specify the edits of the modifiers for each line or HCPCS/CPT.  This field is defined by CMS or other regulatory agencies in the US.",
    ttype: "User",
    steward: "FM",
    where_used: "GP1-8",
    hl7_version: "2.4",
};

pub static TABLE_0468_METADATA: V2MetadataTable = V2MetadataTable {
    table: 468usize,
    description:
        "Table of codes specifying any payment adjustment due to drugs or medical devices.",
    ttype: "User",
    steward: "FM",
    where_used: "GP2-9",
    hl7_version: "2.4",
};

pub static TABLE_0469_METADATA: V2MetadataTable = V2MetadataTable {
    table: 469usize,
    description: "Table of codes specifying the packaging status of the service.",
    ttype: "User",
    steward: "FM",
    where_used: "GP2-10",
    hl7_version: "2.4",
};

pub static TABLE_0470_METADATA: V2MetadataTable = V2MetadataTable {
    table: 470usize,
    description:
        "Table of codes specifying the fee schedule reimbursement type applied to the line item.",
    ttype: "User",
    steward: "FM",
    where_used: "GP2-12",
    hl7_version: "2.4",
};

pub static TABLE_0471_METADATA: V2MetadataTable = V2MetadataTable {
    table: 471usize,
    description: "Table of codes specifying the name of the query.  No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "QPD-1, QAK-3, QID-2",
    hl7_version: "2.4",
};

pub static TABLE_0472_METADATA: V2MetadataTable = V2MetadataTable {
    table: 472usize,
    description: "HL7-defined table of codes specifying that a second timing specification is to follow using the repeat delimiter.",
    ttype: "HL7",
    steward: "OO",
    where_used: "TQ1-12",
    hl7_version: "2.4",
};

pub static TABLE_0473_METADATA: V2MetadataTable = V2MetadataTable {
    table: 473usize,
    description:
        "Table of codes specifying whether or not the service (pharmaceutical) is in the formulary.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "OM7-22",
    hl7_version: "2.4",
};

pub static TABLE_0474_METADATA: V2MetadataTable = V2MetadataTable {
    table: 474usize,
    description: "Table of codes specifying the classification of the organization unit.",
    ttype: "User",
    steward: "PA",
    where_used: "ORG-3",
    hl7_version: "2.4",
};

pub static TABLE_0475_METADATA: V2MetadataTable = V2MetadataTable {
    table: 475usize,
    description: "Table of codes specifying the choice of, and providing the clinical rationale for, a selected charge type.",
    ttype: "User",
    steward: "OO",
    where_used: "BLG-4",
    hl7_version: "2.5",
};

pub static TABLE_0476_METADATA: V2MetadataTable = V2MetadataTable {
    table: 476usize,
    description: "Table of codes specifying the reason the procedure code found in OBR-44 Procedure Code is a duplicate of one ordered/charged previously for the same patient within the same date of service and has been determined to be medically necessary.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "OBR-48, FT1-28",
    hl7_version: "2.5",
};

pub static TABLE_0477_METADATA: V2MetadataTable = V2MetadataTable {
    table: 477usize,
    description: "Table of codes specifying the class of the drug or other substance if its usage is controlled by legislation.  In the USA, such legislation includes the federal Controlled Substance Act (CSA) or a State Uniform Controlled Substance Act. Values are drawn from the Pharmacy Law Digest July 1988.  Other countries should create their own versions of this table.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "RXE-35",
    hl7_version: "2.5",
};

pub static TABLE_0478_METADATA: V2MetadataTable = V2MetadataTable {
    table: 478usize,
    description: "HL7-defined table of codes specifying whether or not the pharmaceutical substance is part of the local formulary.",
    ttype: "HL7",
    steward: "OO",
    where_used: "SPM-4",
    hl7_version: "2.5",
};

pub static TABLE_0479_METADATA: V2MetadataTable = V2MetadataTable {
    table: 479usize,
    description: "Table of codes specifying the medical substance or treatment that has been ordered to be given to the patient, as encoded by the pharmacy or treatment supplier.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "RXE-2",
    hl7_version: "2.5",
};

pub static TABLE_0480_METADATA: V2MetadataTable = V2MetadataTable {
    table: 480usize,
    description: "HL7-defined table of codes specifying the general category of pharmacy order which may be used to determine the processing path the order will take.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXO-27, RXE-44, RXD-32, RXG-26, RXA-26",
    hl7_version: "2.5",
};

pub static TABLE_0482_METADATA: V2MetadataTable = V2MetadataTable {
    table: 482usize,
    description: "HL7-defined table of codes specifying whether the order is to be executed in an inpatient setting or an outpatient setting.",
    ttype: "HL7",
    steward: "OO",
    where_used: "ORC-29",
    hl7_version: "2.5",
};

pub static TABLE_0483_METADATA: V2MetadataTable = V2MetadataTable {
    table: 483usize,
    description: "HL7-defined table of codes of forms of authorization a recorder may receive from the responsible practitioner to create or change an order.",
    ttype: "HL7",
    steward: "OO",
    where_used: "ORC-30",
    hl7_version: "2.5",
};

pub static TABLE_0484_METADATA: V2MetadataTable = V2MetadataTable {
    table: 484usize,
    description: "Table of codes specifying the type of dispensing event that occurred.",
    ttype: "User",
    steward: "OO",
    where_used: "RXD-33",
    hl7_version: "2.5",
};

pub static TABLE_0485_METADATA: V2MetadataTable = V2MetadataTable {
    table: 485usize,
    description: "Table of codes describing the urgency of a request carried in an order. See the Comment/Usage Note in the table below, as the table contains both precoordinated codes that may be used in an HL7 field or component and also explanatory patterns illustrating the syntax used to construct expressions using the codes and other modifiers.",
    ttype: "User",
    steward: "OO",
    where_used: "TQ1-9",
    hl7_version: "2.5",
};

pub static TABLE_0487_METADATA: V2MetadataTable = V2MetadataTable {
    table: 487usize,
    description: "HL7-defined table of codes that describe the precise nature of an entity that may be used as the source material for an observation.  This is one of two code systems that are used instead of table 0070 (code system 2.16.840.1.113883.18.28) which conflated specimen types and specimen collection methods.",
    ttype: "HL7",
    steward: "",
    where_used: "SPM-4",
    hl7_version: "2.5",
};

pub static TABLE_0488_METADATA: V2MetadataTable = V2MetadataTable {
    table: 488usize,
    description: "HL7-defined table of codes specifying the specimen collection method.   Used in Version 2 messaging in the SPM segment.",
    ttype: "HL7",
    steward: "OO",
    where_used: "SPM-7",
    hl7_version: "2.5",
};

pub static TABLE_0489_METADATA: V2MetadataTable = V2MetadataTable {
    table: 489usize,
    description: "Table of codes specifying any known or suspected specimen hazards, e.g., exceptionally infectious agent or blood from a hepatitis patient.",
    ttype: "User",
    steward: "OO",
    where_used: "SPM-16, PAC-8",
    hl7_version: "2.5",
};

pub static TABLE_0490_METADATA: V2MetadataTable = V2MetadataTable {
    table: 490usize,
    description: "HL7-defined table of codes specifying the reasons a specimen may be rejected for a specified observation/result/analysis.",
    ttype: "HL7",
    steward: "OO",
    where_used: "SPM-21",
    hl7_version: "2.5",
};

pub static TABLE_0491_METADATA: V2MetadataTable = V2MetadataTable {
    table: 491usize,
    description:
        "Table of codes specifying the degree or grade of excellence of the specimen at receipt.",
    ttype: "User",
    steward: "OO",
    where_used: "SPM-22",
    hl7_version: "2.5",
};

pub static TABLE_0492_METADATA: V2MetadataTable = V2MetadataTable {
    table: 492usize,
    description: "Table of codes specifying the suitability of the specimen for the particular planned use as determined by the  filler.",
    ttype: "User",
    steward: "OO",
    where_used: "SPM-23",
    hl7_version: "2.5",
};

pub static TABLE_0493_METADATA: V2MetadataTable = V2MetadataTable {
    table: 493usize,
    description: "Table of codes specifying a mode or state of being that describes the nature of the specimen.",
    ttype: "User",
    steward: "OO",
    where_used: "SPM-24",
    hl7_version: "2.5",
};

pub static TABLE_0494_METADATA: V2MetadataTable = V2MetadataTable {
    table: 494usize,
    description: "HL7-defined table of codes specifying for child specimens the relationship between this specimen and the parent specimen.",
    ttype: "HL7",
    steward: "OO",
    where_used: "SPM-29",
    hl7_version: "2.5",
};

pub static TABLE_0495_METADATA: V2MetadataTable = V2MetadataTable {
    table: 495usize,
    description: "HL7-defined table of codes specifying the modifier for the body site.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXR-6",
    hl7_version: "2.5",
};

pub static TABLE_0496_METADATA: V2MetadataTable = V2MetadataTable {
    table: 496usize,
    description: "Table of codes specifying to what the subject is consenting, i.e. what type of service, surgical procedure, information access/release or other event.",
    ttype: "User",
    steward: "StrucDoc",
    where_used: "TXA:2; CON:2",
    hl7_version: "2.5",
};

pub static TABLE_0497_METADATA: V2MetadataTable = V2MetadataTable {
    table: 497usize,
    description:
        "HL7-defined table of codes specifying the method in which a subject provides consent.",
    ttype: "HL7",
    steward: "StrucDoc",
    where_used: "TXA:10; CON:10",
    hl7_version: "2.5",
};

pub static TABLE_0498_METADATA: V2MetadataTable = V2MetadataTable {
    table: 498usize,
    description:
        "HL7-defined table of codes specifying whether the consent has been sought and granted.",
    ttype: "HL7",
    steward: "StrucDoc",
    where_used: "TXA:11; CO N:11",
    hl7_version: "2.5",
};

pub static TABLE_0499_METADATA: V2MetadataTable = V2MetadataTable {
    table: 499usize,
    description: "Table of codes specifying the reason the subject's consent was not sought.",
    ttype: "User",
    steward: "StrucDoc",
    where_used: "CON:20",
    hl7_version: "2.5",
};

pub static TABLE_0500_METADATA: V2MetadataTable = V2MetadataTable {
    table: 500usize,
    description: "HL7-defined table of codes used to specify how much information was disclosed to the subject as part of the informed consent process.",
    ttype: "HL7",
    steward: "StrucDoc",
    where_used: "CON:21",
    hl7_version: "2.5",
};

pub static TABLE_0501_METADATA: V2MetadataTable = V2MetadataTable {
    table: 501usize,
    description:
        "Table of codes used to specify a reason the subject did not receive full disclosure.",
    ttype: "User",
    steward: "StrucDoc",
    where_used: "CON:22",
    hl7_version: "2.5",
};

pub static TABLE_0502_METADATA: V2MetadataTable = V2MetadataTable {
    table: 502usize,
    description: "HL7-defined table of codes used to specify a reason consent was granted by a person other than the subject of the consent.",
    ttype: "User",
    steward: "StrucDoc",
    where_used: "CON:23",
    hl7_version: "2.5",
};

pub static TABLE_0503_METADATA: V2MetadataTable = V2MetadataTable {
    table: 503usize,
    description: "HL7-defined table of codes used to specify the sequencing relationship between the current service request and the related service request(s) specified in this TQ2 segment.",
    ttype: "HL7",
    steward: "OO",
    where_used: "TQ2-2",
    hl7_version: "2.5",
};

pub static TABLE_0504_METADATA: V2MetadataTable = V2MetadataTable {
    table: 504usize,
    description: "HL7-defined table of codes used to specify the relationship between the start/end of the related service request(s) (from TQ2-3, TQ2-4  or TQ2-5) and the current service request from ORC-2, 3 or 4.",
    ttype: "HL7",
    steward: "OO",
    where_used: "TQ2-6",
    hl7_version: "2.5",
};

pub static TABLE_0505_METADATA: V2MetadataTable = V2MetadataTable {
    table: 505usize,
    description: "HL7-defined table of codes used to specify if this service request is the first or last service request in a cyclic series of service requests.",
    ttype: "HL7",
    steward: "OO",
    where_used: "TQ2-7",
    hl7_version: "2.5",
};

pub static TABLE_0506_METADATA: V2MetadataTable = V2MetadataTable {
    table: 506usize,
    description: "HL7-defined table of codes used to specify an additional or alternate relationship between this service request and other service requests.",
    ttype: "HL7",
    steward: "OO",
    where_used: "TQ2-10",
    hl7_version: "2.5",
};

pub static TABLE_0507_METADATA: V2MetadataTable = V2MetadataTable {
    table: 507usize,
    description: "Table of codes regarding the handling of a result.  For example, an order may specify that the result (e.g., an x-ray film) should be given to the patient for return to the requestor.",
    ttype: "User",
    steward: "OO",
    where_used: "OBR-49",
    hl7_version: "2.5",
};

pub static TABLE_0508_METADATA: V2MetadataTable = V2MetadataTable {
    table: 508usize,
    description: "Table of codes used to specify additional information about the blood component class associated with the Universal Service ID.  The placer of the order can specify any required processing of the blood product that must be completed prior to transfusion to the intended recipient.",
    ttype: "User",
    steward: "OO",
    where_used: "BPO-3",
    hl7_version: "2.5",
};

pub static TABLE_0509_METADATA: V2MetadataTable = V2MetadataTable {
    table: 509usize,
    description: "Table of codes that specify the reason the blood product was ordered.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "BPO-13",
    hl7_version: "2.5",
};

pub static TABLE_0510_METADATA: V2MetadataTable = V2MetadataTable {
    table: 510usize,
    description: "HL7-defined table of codes used to specify the current status of the specified blood product as indicated by the filler or placer.  For example, the first status change of a product that may trigger a Blood Product Dispense Status Message occurs when it first becomes linked to a patient and is ready to dispense. The placer system may use the Blood Product Dispense Status Message to request the transfusion service to dispense the product.  When the blood product is delivered or issued to a patient, the status of the blood product would be changed to indicate that it has now been \"dispensed\".",
    ttype: "HL7",
    steward: "OO",
    where_used: "BPX-2",
    hl7_version: "2.5",
};

pub static TABLE_0511_METADATA: V2MetadataTable = V2MetadataTable {
    table: 511usize,
    description: "HL7-defined table of codes used to specify the interpretation for the blood product observation status codes.  A status is considered preliminary until a blood product has reached a final disposition for the patient. For example, when the product is first cross-matched and a status message is sent, it would be considered preliminary. When the product is dispensed to the patient, that status would also be considered preliminary.  However, once the product is transfused, the status would be considered final.",
    ttype: "HL7",
    steward: "OO",
    where_used: "BPX-3, BTX-12",
    hl7_version: "2.5",
};

pub static TABLE_0512_METADATA: V2MetadataTable = V2MetadataTable {
    table: 512usize,
    description: "Table of codes that specify a commercial product.  Examples of commercial products are blood derivatives such as Rh Immune Globulin and Factor VIII concentrate, Leukoreduction filters and blood administration sets.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "BPX-8, BTX-5",
    hl7_version: "2.5",
};

pub static TABLE_0513_METADATA: V2MetadataTable = V2MetadataTable {
    table: 513usize,
    description: "HL7-defined table of codes used to specify the current status of the specified blood product as indicated by the placer.  For example, the placer may return the blood product to the transfusion service unused because an IV could not be started. The blood component may have been entered, but the line was clogged and could not be used, in which case the component must be wasted. A final status would indicate that the product has actually been \"transfused.\"",
    ttype: "HL7",
    steward: "OO",
    where_used: "BTX-13",
    hl7_version: "2.5",
};

pub static TABLE_0514_METADATA: V2MetadataTable = V2MetadataTable {
    table: 514usize,
    description: "Table of codes used to specify the type of adverse reaction that the recipient of the blood product experienced.",
    ttype: "User",
    steward: "OO",
    where_used: "BTX-18",
    hl7_version: "2.5",
};

pub static TABLE_0515_METADATA: V2MetadataTable = V2MetadataTable {
    table: 515usize,
    description: "Table of codes that specify the reason the transfusion of the blood product was interrupted.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "BTX-19",
    hl7_version: "2.5",
};

pub static TABLE_0516_METADATA: V2MetadataTable = V2MetadataTable {
    table: 516usize,
    description: "HL7-defined table of codes specifying the severity of an application error as reported during acknowledgment of messages. Knowing if something is Error, Warning or Information is intrinsic to how an application handles the content and the information flow.",
    ttype: "HL7",
    steward: "InM",
    where_used: "ERR-4",
    hl7_version: "2.5",
};

pub static TABLE_0517_METADATA: V2MetadataTable = V2MetadataTable {
    table: 517usize,
    description: "Table of codes used to specify who (if anyone) should be informed of the error. This field may also be used to indicate that a particular person should NOT be informed of the error (e.g. do not inform patient.)",
    ttype: "User",
    steward: "InM",
    where_used: "ERR-9",
    hl7_version: "2.5",
};

pub static TABLE_0518_METADATA: V2MetadataTable = V2MetadataTable {
    table: 518usize,
    description: "Table of codes used to specify what type of override can be used to override the specific error identified.",
    ttype: "User",
    steward: "InM",
    where_used: "ERR-10, OVR-1",
    hl7_version: "2.5",
};

pub static TABLE_0519_METADATA: V2MetadataTable = V2MetadataTable {
    table: 519usize,
    description: "Table of codes that specify the override codes that can be used to override enforcement of the application rule that generated an error.  No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "ERR-11",
    hl7_version: "2.5",
};

pub static TABLE_0520_METADATA: V2MetadataTable = V2MetadataTable {
    table: 520usize,
    description: "HL7-defined table of codes used to specify how important the most important waiting mesasge is.  For example, if there are 3 low priority messages, 1 medium priority message and 1 high priority message, the message waiting priority would be \"high\", because that is the highest priority of any new message waiting.",
    ttype: "HL7",
    steward: "InM",
    where_used: "MSA-8",
    hl7_version: "2.6",
};

pub static TABLE_0521_METADATA: V2MetadataTable = V2MetadataTable {
    table: 521usize,
    description: "Table of codes that specify the reason for the business rule override.  No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "OVR-2",
    hl7_version: "2.5",
};

pub static TABLE_0522_METADATA: V2MetadataTable = V2MetadataTable {
    table: 522usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "AUT-11",
    hl7_version: "",
};

pub static TABLE_0523_METADATA: V2MetadataTable = V2MetadataTable {
    table: 523usize,
    description: "HL7-defined table of codes used to specify if the change is computed as a percent change or as an absolute change.",
    ttype: "HL7",
    steward: "InM/OO",
    where_used: "DLT.3",
    hl7_version: "2.5",
};

pub static TABLE_0525_METADATA: V2MetadataTable = V2MetadataTable {
    table: 525usize,
    description: "Table of codes that specify the institutional privilege.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "PIP.1",
    hl7_version: "2.5",
};

pub static TABLE_0526_METADATA: V2MetadataTable = V2MetadataTable {
    table: 526usize,
    description: "Table of codes that specify the class category of institutional privilege.  No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "PIP.2",
    hl7_version: "2.5",
};

pub static TABLE_0527_METADATA: V2MetadataTable = V2MetadataTable {
    table: 527usize,
    description: "HL7-defined table of codes used to specify an alignment of the repetition to a calendar (e.g., to distinguish every  30 days from \"the 5th of every month\").",
    ttype: "HL7",
    steward: "InM/OO",
    where_used: "RPT.2",
    hl7_version: "2.5",
};

pub static TABLE_0528_METADATA: V2MetadataTable = V2MetadataTable {
    table: 528usize,
    description:
        "HL7-defined table of codes used to specify a common (periodical) activity of daily living.",
    ttype: "HL7",
    steward: "InM/OO",
    where_used: "RPT.8",
    hl7_version: "2.5",
};

pub static TABLE_0530_METADATA: V2MetadataTable = V2MetadataTable {
    table: 530usize,
    description: "Table of codes used to specify the agency or department that assigned the identifier in component 1.",
    ttype: "User",
    steward: "InM",
    where_used: "CX.10, PPN.24, XCN.23",
    hl7_version: "2.5",
};

pub static TABLE_0531_METADATA: V2MetadataTable = V2MetadataTable {
    table: 531usize,
    description: "Table of codes that specify the institution where a staff member is or was active. No suggested values.",
    ttype: "User",
    steward: "InM/PA",
    where_used: "DIN.2",
    hl7_version: "2.5",
};

pub static TABLE_0532_METADATA: V2MetadataTable = V2MetadataTable {
    table: 532usize,
    description: "HL7-defined table of codes used to specify an expansion on the original Yes/No indicator table by including \"flavors of null\".  It is intended to be applied to fields where the response is not limited to \"yes\" or \"no\".",
    ttype: "HL7",
    steward: "InM",
    where_used: "Numerous locations",
    hl7_version: "2.5",
};

pub static TABLE_0533_METADATA: V2MetadataTable = V2MetadataTable {
    table: 533usize,
    description: "Table of codes that specify the application specific code identifying the specific error that occurred.  No suggested values.",
    ttype: "User",
    steward: "InM",
    where_used: "ERR-5",
    hl7_version: "2.5",
};

pub static TABLE_0534_METADATA: V2MetadataTable = V2MetadataTable {
    table: 534usize,
    description: "Table of codes used to specify whether the clergy should be notified.",
    ttype: "User",
    steward: "PA",
    where_used: "PV2-49",
    hl7_version: "2.5",
};

pub static TABLE_0535_METADATA: V2MetadataTable = V2MetadataTable {
    table: 535usize,
    description: "Table of codes that indicate how a patient/subscriber authorization signature is obtained and how it is being retained by a provider.",
    ttype: "User",
    steward: "FM",
    where_used: "IN1-50",
    hl7_version: "2.5",
};

pub static TABLE_0536_METADATA: V2MetadataTable = V2MetadataTable {
    table: 536usize,
    description: "Table of codes used to specify the status of the certificate held by the health professional.",
    ttype: "User",
    steward: "PA",
    where_used: "CER-31",
    hl7_version: "2.5",
};

pub static TABLE_0537_METADATA: V2MetadataTable = V2MetadataTable {
    table: 537usize,
    description: "Table of codes that specify the institution the practitioner began or intends to begin practicing at (e.g., at hospital, at physician organization, at managed care network).  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "PRA-9, STF-12",
    hl7_version: "2.5",
};

pub static TABLE_0538_METADATA: V2MetadataTable = V2MetadataTable {
    table: 538usize,
    description: "Table of codes used to specify the relationship the staff person has with the institution for whom he/she provides services.",
    ttype: "User",
    steward: "PA",
    where_used: "STF-33",
    hl7_version: "2.5",
};

pub static TABLE_0539_METADATA: V2MetadataTable = V2MetadataTable {
    table: 539usize,
    description: "Table of codes that specify the organization unit in the General Ledger to which the staff member is currently assigned.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "STF-36",
    hl7_version: "2.5",
};

pub static TABLE_0540_METADATA: V2MetadataTable = V2MetadataTable {
    table: 540usize,
    description: "Table of codes used to specify the reason the staff member is inactive.",
    ttype: "User",
    steward: "PA",
    where_used: "STF-38",
    hl7_version: "2.5",
};

pub static TABLE_0541_METADATA: V2MetadataTable = V2MetadataTable {
    table: 541usize,
    description: "Table of codes that specify the modifying or qualifying description(s) about the specimen type.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SPM-5",
    hl7_version: "2.5",
};

pub static TABLE_0542_METADATA: V2MetadataTable = V2MetadataTable {
    table: 542usize,
    description: "Table of codes that specify the modifying or qualifying description(s) about the specimen source site.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SPM-9",
    hl7_version: "2.5",
};

pub static TABLE_0543_METADATA: V2MetadataTable = V2MetadataTable {
    table: 543usize,
    description: "Table of codes that specify the modifying or qualifying description(s) about the specimen collection site.  This field differs from Specimen Source Site in those cases where the source site must be approached via a particular site (e.g., anatomic location). For example, in the case where a liver biopsy is obtained via a percutaneous needle, the collection site would be the point of entry of the needle. For venous blood collected from the left radial vein, the collection site could be “antecubital fossa”.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SPM-10",
    hl7_version: "2.5",
};

pub static TABLE_0544_METADATA: V2MetadataTable = V2MetadataTable {
    table: 544usize,
    description: "HL7-defined table of codes used to specify at each receipt the status of the container in which the specimen is shipped in chain of custody cases where specimens are moved from lab to lab.  If the container is compromised in any way (seal broken, container cracked or leaking, etc.), then this status needs to be recorded for legal reasons.",
    ttype: "User",
    steward: "OO",
    where_used: "SPM-28, SHP-9,PAC-6",
    hl7_version: "2.5",
};

pub static TABLE_0546_METADATA: V2MetadataTable = V2MetadataTable {
    table: 546usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "AUT-12",
    hl7_version: "",
};

pub static TABLE_0547_METADATA: V2MetadataTable = V2MetadataTable {
    table: 547usize,
    description: "Table of codes used to specify the breadth/extent of the jurisdiction where the qualification is valid.",
    ttype: "User",
    steward: "PA",
    where_used: "CER-22",
    hl7_version: "2.5",
};

pub static TABLE_0548_METADATA: V2MetadataTable = V2MetadataTable {
    table: 548usize,
    description: "Table of codes used to specify the relationship of the consenter to the subject.",
    ttype: "User",
    steward: "StrucDoc",
    where_used: "CON:25",
    hl7_version: "2.5",
};

pub static TABLE_0549_METADATA: V2MetadataTable = V2MetadataTable {
    table: 549usize,
    description: "Table of codes that specify the National Drug Codes (NDC) that are required by the Health Insurance Portability and Accountability Act (HIPAA) for electronic claims for pharmacy charges.  No suggested values.",
    ttype: "External",
    steward: "FM",
    where_used: "FT1-29",
    hl7_version: "2.5",
};

pub static TABLE_0550_METADATA: V2MetadataTable = V2MetadataTable {
    table: 550usize,
    description: "HL7-defined table of codes used to specify the part of the body.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXR-2",
    hl7_version: "2.5",
};

pub static TABLE_0551_METADATA: V2MetadataTable = V2MetadataTable {
    table: 551usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "AUT-13",
    hl7_version: "",
};

pub static TABLE_0552_METADATA: V2MetadataTable = V2MetadataTable {
    table: 552usize,
    description: "Table of codes that specify the reason the patient did not sign an Advanced Beneficiary Notice.  No suggested values.",
    ttype: "HL7",
    steward: "OO",
    where_used: "ORC-26",
    hl7_version: "2.5",
};

pub static TABLE_0553_METADATA: V2MetadataTable = V2MetadataTable {
    table: 553usize,
    description:
        "Table of codes used to specify what invoice action is being performed by this message.",
    ttype: "User",
    steward: "FM",
    where_used: "IVC-4",
    hl7_version: "2.6",
};

pub static TABLE_0554_METADATA: V2MetadataTable = V2MetadataTable {
    table: 554usize,
    description: "Table of codes used to specify the reason for an invoice.",
    ttype: "User",
    steward: "FM",
    where_used: "IVC-5",
    hl7_version: "2.6",
};

pub static TABLE_0555_METADATA: V2MetadataTable = V2MetadataTable {
    table: 555usize,
    description: "Table of codes used to specify the type of invoice.",
    ttype: "User",
    steward: "FM",
    where_used: "IVC-6",
    hl7_version: "2.6",
};

pub static TABLE_0556_METADATA: V2MetadataTable = V2MetadataTable {
    table: 556usize,
    description: "Table of codes used to specify the benefit group.",
    ttype: "User",
    steward: "FM",
    where_used: "IVC-25",
    hl7_version: "2.6",
};

pub static TABLE_0557_METADATA: V2MetadataTable = V2MetadataTable {
    table: 557usize,
    description: "Table of codes used to specify the type of payee (e.g., organization, person).",
    ttype: "User",
    steward: "FM",
    where_used: "PYE-2",
    hl7_version: "2.6",
};

pub static TABLE_0558_METADATA: V2MetadataTable = V2MetadataTable {
    table: 558usize,
    description:
        "Table of codes used to specify the relationship to the invoice for Person Payee Types.",
    ttype: "User",
    steward: "FM",
    where_used: "PYE-3",
    hl7_version: "2.6",
};

pub static TABLE_0559_METADATA: V2MetadataTable = V2MetadataTable {
    table: 559usize,
    description:
        "Table of codes used to specify the processing status for the Product/Service Code.",
    ttype: "User",
    steward: "FM",
    where_used: "PLS-6",
    hl7_version: "2.6",
};

pub static TABLE_0560_METADATA: V2MetadataTable = V2MetadataTable {
    table: 560usize,
    description: "Table of codes that specify the adjustment quantity.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "ADJ-6",
    hl7_version: "2.6",
};

pub static TABLE_0561_METADATA: V2MetadataTable = V2MetadataTable {
    table: 561usize,
    description: "Table of codes used to specify the Product/Service Code.",
    ttype: "User",
    steward: "FM",
    where_used: "PSL-17",
    hl7_version: "2.6",
};

pub static TABLE_0562_METADATA: V2MetadataTable = V2MetadataTable {
    table: 562usize,
    description: "Table of codes used to specify special processing requested of Payer for this Product/Service Line Item (e.g., hold until paper supporting documentation is received by Payer).",
    ttype: "User",
    steward: "FM",
    where_used: "PSL-20",
    hl7_version: "2.6",
};

pub static TABLE_0563_METADATA: V2MetadataTable = V2MetadataTable {
    table: 563usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "AUT-14",
    hl7_version: "",
};

pub static TABLE_0564_METADATA: V2MetadataTable = V2MetadataTable {
    table: 564usize,
    description: "Table of codes used to specify the category of adjustment and is used to assist in determining which table is used for Adjustment Reason.",
    ttype: "User",
    steward: "FM",
    where_used: "ADJ-4",
    hl7_version: "2.6",
};

pub static TABLE_0565_METADATA: V2MetadataTable = V2MetadataTable {
    table: 565usize,
    description: "Table of codes used to specify the reason for this adjustment.",
    ttype: "User",
    steward: "FM",
    where_used: "ADJ-7",
    hl7_version: "2.6",
};

pub static TABLE_0566_METADATA: V2MetadataTable = V2MetadataTable {
    table: 566usize,
    description: "HL7-defined table of codes used to specify the type of blood unit",
    ttype: "HL7",
    steward: "OO",
    where_used: "BUI-3",
    hl7_version: "2.8",
};

pub static TABLE_0567_METADATA: V2MetadataTable = V2MetadataTable {
    table: 567usize,
    description:
        "Weight Units.  Note this table has been deprecated and is replacaced by table 0929.",
    ttype: "HL7",
    steward: "",
    where_used: "",
    hl7_version: "2.8.2",
};

pub static TABLE_0568_METADATA: V2MetadataTable = V2MetadataTable {
    table: 568usize,
    description:
        "Volume Units Note this table has been deprecated and is replacaced by table 0930.",
    ttype: "HL7",
    steward: "",
    where_used: "",
    hl7_version: "2.8.2",
};

pub static TABLE_0569_METADATA: V2MetadataTable = V2MetadataTable {
    table: 569usize,
    description: "Table of codes used to specify the action requested of a party that receives an adjustment.",
    ttype: "User",
    steward: "FM",
    where_used: "ADJ-11",
    hl7_version: "2.6",
};

pub static TABLE_0570_METADATA: V2MetadataTable = V2MetadataTable {
    table: 570usize,
    description: "Table of codes used to specify the method for the movement of payment.",
    ttype: "User",
    steward: "FM",
    where_used: "PMT-4",
    hl7_version: "2.6",
};

pub static TABLE_0571_METADATA: V2MetadataTable = V2MetadataTable {
    table: 571usize,
    description:
        "Table of codes used to specify the processing status for an Invoice Processing Result.",
    ttype: "User",
    steward: "FM",
    where_used: "IPR-4",
    hl7_version: "2.6",
};

pub static TABLE_0572_METADATA: V2MetadataTable = V2MetadataTable {
    table: 572usize,
    description: "Table of codes used to specify the tax status of a provider.",
    ttype: "User",
    steward: "FM",
    where_used: "IVC-28",
    hl7_version: "2.6",
};

pub static TABLE_0573_METADATA: V2MetadataTable = V2MetadataTable {
    table: 573usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "AUT-16",
    hl7_version: "",
};

pub static TABLE_0574_METADATA: V2MetadataTable = V2MetadataTable {
    table: 574usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "AUT-21",
    hl7_version: "",
};

pub static TABLE_0575_METADATA: V2MetadataTable = V2MetadataTable {
    table: 575usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "BPO-2",
    hl7_version: "",
};

pub static TABLE_0576_METADATA: V2MetadataTable = V2MetadataTable {
    table: 576usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "BPO-6",
    hl7_version: "",
};

pub static TABLE_0577_METADATA: V2MetadataTable = V2MetadataTable {
    table: 577usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "BPX-6",
    hl7_version: "",
};

pub static TABLE_0578_METADATA: V2MetadataTable = V2MetadataTable {
    table: 578usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "BPX-7",
    hl7_version: "",
};

pub static TABLE_0579_METADATA: V2MetadataTable = V2MetadataTable {
    table: 579usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "BPX-11",
    hl7_version: "",
};

pub static TABLE_0580_METADATA: V2MetadataTable = V2MetadataTable {
    table: 580usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "BPX-12",
    hl7_version: "",
};

pub static TABLE_0581_METADATA: V2MetadataTable = V2MetadataTable {
    table: 581usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "BPX-16",
    hl7_version: "",
};

pub static TABLE_0582_METADATA: V2MetadataTable = V2MetadataTable {
    table: 582usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "BTX-3",
    hl7_version: "",
};

pub static TABLE_0583_METADATA: V2MetadataTable = V2MetadataTable {
    table: 583usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "BTX-4",
    hl7_version: "",
};

pub static TABLE_0584_METADATA: V2MetadataTable = V2MetadataTable {
    table: 584usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "BTX-10",
    hl7_version: "",
};

pub static TABLE_0585_METADATA: V2MetadataTable = V2MetadataTable {
    table: 585usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CBS-5",
    hl7_version: "",
};

pub static TABLE_0586_METADATA: V2MetadataTable = V2MetadataTable {
    table: 586usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CNS-6",
    hl7_version: "",
};

pub static TABLE_0587_METADATA: V2MetadataTable = V2MetadataTable {
    table: 587usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CSP-1",
    hl7_version: "",
};

pub static TABLE_0588_METADATA: V2MetadataTable = V2MetadataTable {
    table: 588usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CSP-4",
    hl7_version: "",
};

pub static TABLE_0589_METADATA: V2MetadataTable = V2MetadataTable {
    table: 589usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CSR-3",
    hl7_version: "",
};

pub static TABLE_0590_METADATA: V2MetadataTable = V2MetadataTable {
    table: 590usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CSR-10",
    hl7_version: "",
};

pub static TABLE_0591_METADATA: V2MetadataTable = V2MetadataTable {
    table: 591usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CSR-12",
    hl7_version: "",
};

pub static TABLE_0592_METADATA: V2MetadataTable = V2MetadataTable {
    table: 592usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CSR-13",
    hl7_version: "",
};

pub static TABLE_0593_METADATA: V2MetadataTable = V2MetadataTable {
    table: 593usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CSR-14",
    hl7_version: "",
};

pub static TABLE_0594_METADATA: V2MetadataTable = V2MetadataTable {
    table: 594usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CSR-16",
    hl7_version: "",
};

pub static TABLE_0595_METADATA: V2MetadataTable = V2MetadataTable {
    table: 595usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CSS-1",
    hl7_version: "",
};

pub static TABLE_0596_METADATA: V2MetadataTable = V2MetadataTable {
    table: 596usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CSS-3",
    hl7_version: "",
};

pub static TABLE_0597_METADATA: V2MetadataTable = V2MetadataTable {
    table: 597usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CTI-2",
    hl7_version: "",
};

pub static TABLE_0598_METADATA: V2MetadataTable = V2MetadataTable {
    table: 598usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "CTI-3",
    hl7_version: "",
};

pub static TABLE_0599_METADATA: V2MetadataTable = V2MetadataTable {
    table: 599usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "INV-4",
    hl7_version: "",
};

pub static TABLE_0600_METADATA: V2MetadataTable = V2MetadataTable {
    table: 600usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "INV-5",
    hl7_version: "",
};

pub static TABLE_0601_METADATA: V2MetadataTable = V2MetadataTable {
    table: 601usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "INV-6",
    hl7_version: "",
};

pub static TABLE_0602_METADATA: V2MetadataTable = V2MetadataTable {
    table: 602usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "INV-11",
    hl7_version: "",
};

pub static TABLE_0603_METADATA: V2MetadataTable = V2MetadataTable {
    table: 603usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "INV-15",
    hl7_version: "",
};

pub static TABLE_0604_METADATA: V2MetadataTable = V2MetadataTable {
    table: 604usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "IPC-5",
    hl7_version: "",
};

pub static TABLE_0605_METADATA: V2MetadataTable = V2MetadataTable {
    table: 605usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "IPC-6",
    hl7_version: "",
};

pub static TABLE_0606_METADATA: V2MetadataTable = V2MetadataTable {
    table: 606usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "IPC-8",
    hl7_version: "",
};

pub static TABLE_0607_METADATA: V2MetadataTable = V2MetadataTable {
    table: 607usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "MFA-5",
    hl7_version: "",
};

pub static TABLE_0608_METADATA: V2MetadataTable = V2MetadataTable {
    table: 608usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "MFE-4",
    hl7_version: "",
};

pub static TABLE_0609_METADATA: V2MetadataTable = V2MetadataTable {
    table: 609usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "MSH-19",
    hl7_version: "",
};

pub static TABLE_0610_METADATA: V2MetadataTable = V2MetadataTable {
    table: 610usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "NDS-4",
    hl7_version: "",
};

pub static TABLE_0611_METADATA: V2MetadataTable = V2MetadataTable {
    table: 611usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "NTE-9",
    hl7_version: "",
};

pub static TABLE_0612_METADATA: V2MetadataTable = V2MetadataTable {
    table: 612usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OBR-4",
    hl7_version: "",
};

pub static TABLE_0613_METADATA: V2MetadataTable = V2MetadataTable {
    table: 613usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OBR-12",
    hl7_version: "",
};

pub static TABLE_0614_METADATA: V2MetadataTable = V2MetadataTable {
    table: 614usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OBR-38",
    hl7_version: "",
};

pub static TABLE_0615_METADATA: V2MetadataTable = V2MetadataTable {
    table: 615usize,
    description: "HL7-defined table of codes specifying a type of user authentication credential.",
    ttype: "HL7",
    steward: "InM",
    where_used: "UAC-1",
    hl7_version: "2.6",
};

pub static TABLE_0616_METADATA: V2MetadataTable = V2MetadataTable {
    table: 616usize,
    description: "Table of codes specifying the reason this address was marked as \"ended\".",
    ttype: "User",
    steward: "InM",
    where_used: "XAD.15",
    hl7_version: "2.6",
};

pub static TABLE_0617_METADATA: V2MetadataTable = V2MetadataTable {
    table: 617usize,
    description: "HL7-defined table of codes specifying how an address is intended to be used.",
    ttype: "HL7",
    steward: "InM",
    where_used: "XAD.18",
    hl7_version: "2.6",
};

pub static TABLE_0618_METADATA: V2MetadataTable = V2MetadataTable {
    table: 618usize,
    description: "Table of codes specifying that an address needs to be treated with special care or sensitivity.",
    ttype: "User",
    steward: "InM",
    where_used: "XAD.22, XTN.16",
    hl7_version: "2.6",
};

pub static TABLE_0619_METADATA: V2MetadataTable = V2MetadataTable {
    table: 619usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OBR-39",
    hl7_version: "",
};

pub static TABLE_0620_METADATA: V2MetadataTable = V2MetadataTable {
    table: 620usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OBR-40",
    hl7_version: "",
};

pub static TABLE_0621_METADATA: V2MetadataTable = V2MetadataTable {
    table: 621usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OBR-43",
    hl7_version: "",
};

pub static TABLE_0622_METADATA: V2MetadataTable = V2MetadataTable {
    table: 622usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OBX-3",
    hl7_version: "",
};

pub static TABLE_0623_METADATA: V2MetadataTable = V2MetadataTable {
    table: 623usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OBX-6",
    hl7_version: "",
};

pub static TABLE_0624_METADATA: V2MetadataTable = V2MetadataTable {
    table: 624usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OBX-15",
    hl7_version: "",
};

pub static TABLE_0625_METADATA: V2MetadataTable = V2MetadataTable {
    table: 625usize,
    description: "Table of codes specifying the state of an inventory item within the context of an inventory location.",
    ttype: "User",
    steward: "OO",
    where_used: "IVT-6",
    hl7_version: "2.6",
};

pub static TABLE_0626_METADATA: V2MetadataTable = V2MetadataTable {
    table: 626usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OBX-17",
    hl7_version: "",
};

pub static TABLE_0627_METADATA: V2MetadataTable = V2MetadataTable {
    table: 627usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "ODS-2",
    hl7_version: "",
};

pub static TABLE_0628_METADATA: V2MetadataTable = V2MetadataTable {
    table: 628usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "ODS-3",
    hl7_version: "",
};

pub static TABLE_0629_METADATA: V2MetadataTable = V2MetadataTable {
    table: 629usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "ODT-2",
    hl7_version: "",
};

pub static TABLE_0630_METADATA: V2MetadataTable = V2MetadataTable {
    table: 630usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-2",
    hl7_version: "",
};

pub static TABLE_0631_METADATA: V2MetadataTable = V2MetadataTable {
    table: 631usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-5",
    hl7_version: "",
};

pub static TABLE_0632_METADATA: V2MetadataTable = V2MetadataTable {
    table: 632usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-7",
    hl7_version: "",
};

pub static TABLE_0633_METADATA: V2MetadataTable = V2MetadataTable {
    table: 633usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-13",
    hl7_version: "",
};

pub static TABLE_0634_METADATA: V2MetadataTable = V2MetadataTable {
    table: 634usize,
    description: "Table of codes that denote a level or importance of an inventory item within the context of an inventory location.",
    ttype: "User",
    steward: "OO",
    where_used: "IVT-14",
    hl7_version: "2.6",
};

pub static TABLE_0635_METADATA: V2MetadataTable = V2MetadataTable {
    table: 635usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-14",
    hl7_version: "",
};

pub static TABLE_0636_METADATA: V2MetadataTable = V2MetadataTable {
    table: 636usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-16",
    hl7_version: "",
};

pub static TABLE_0637_METADATA: V2MetadataTable = V2MetadataTable {
    table: 637usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-19",
    hl7_version: "",
};

pub static TABLE_0638_METADATA: V2MetadataTable = V2MetadataTable {
    table: 638usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-27",
    hl7_version: "",
};

pub static TABLE_0639_METADATA: V2MetadataTable = V2MetadataTable {
    table: 639usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-31",
    hl7_version: "",
};

pub static TABLE_0640_METADATA: V2MetadataTable = V2MetadataTable {
    table: 640usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-33",
    hl7_version: "",
};

pub static TABLE_0641_METADATA: V2MetadataTable = V2MetadataTable {
    table: 641usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-34",
    hl7_version: "",
};

pub static TABLE_0642_METADATA: V2MetadataTable = V2MetadataTable {
    table: 642usize,
    description:
        "Table of codes specifying the calculation method used to determine the resupply schedule.",
    ttype: "User",
    steward: "OO",
    where_used: "IVT-21",
    hl7_version: "2.6",
};

pub static TABLE_0643_METADATA: V2MetadataTable = V2MetadataTable {
    table: 643usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-36",
    hl7_version: "",
};

pub static TABLE_0644_METADATA: V2MetadataTable = V2MetadataTable {
    table: 644usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-38",
    hl7_version: "",
};

pub static TABLE_0645_METADATA: V2MetadataTable = V2MetadataTable {
    table: 645usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-46",
    hl7_version: "",
};

pub static TABLE_0646_METADATA: V2MetadataTable = V2MetadataTable {
    table: 646usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-52",
    hl7_version: "",
};

pub static TABLE_0647_METADATA: V2MetadataTable = V2MetadataTable {
    table: 647usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM1-56",
    hl7_version: "",
};

pub static TABLE_0648_METADATA: V2MetadataTable = V2MetadataTable {
    table: 648usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM2-2",
    hl7_version: "",
};

pub static TABLE_0649_METADATA: V2MetadataTable = V2MetadataTable {
    table: 649usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM2-4",
    hl7_version: "",
};

pub static TABLE_0650_METADATA: V2MetadataTable = V2MetadataTable {
    table: 650usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM3-2",
    hl7_version: "",
};

pub static TABLE_0651_METADATA: V2MetadataTable = V2MetadataTable {
    table: 651usize,
    description: "Table of codes specifying the method used to calculate employee labor and measure employee productivity.",
    ttype: "User",
    steward: "OO",
    where_used: "SCP-2",
    hl7_version: "2.6",
};

pub static TABLE_0652_METADATA: V2MetadataTable = V2MetadataTable {
    table: 652usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM3-3",
    hl7_version: "",
};

pub static TABLE_0653_METADATA: V2MetadataTable = V2MetadataTable {
    table: 653usize,
    description:
        "Table of codes specifying the date format for a decontamination/sterilization instance.",
    ttype: "User",
    steward: "OO",
    where_used: "SCP-3",
    hl7_version: "2.6",
};

pub static TABLE_0654_METADATA: V2MetadataTable = V2MetadataTable {
    table: 654usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM3-4",
    hl7_version: "",
};

pub static TABLE_0655_METADATA: V2MetadataTable = V2MetadataTable {
    table: 655usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM3-5",
    hl7_version: "",
};

pub static TABLE_0656_METADATA: V2MetadataTable = V2MetadataTable {
    table: 656usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM3-6",
    hl7_version: "",
};

pub static TABLE_0657_METADATA: V2MetadataTable = V2MetadataTable {
    table: 657usize,
    description: "Table of codes specifying the kind of device as defined by the manufacturer.",
    ttype: "User",
    steward: "OO",
    where_used: "SCP-7",
    hl7_version: "2.6",
};

pub static TABLE_0658_METADATA: V2MetadataTable = V2MetadataTable {
    table: 658usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM4-5",
    hl7_version: "",
};

pub static TABLE_0659_METADATA: V2MetadataTable = V2MetadataTable {
    table: 659usize,
    description: "Table of codes specifying whether the sterilization load for a device is built in the sub-sterile area adjacent to an Operating Room or the Central Processing Department.",
    ttype: "User",
    steward: "OO",
    where_used: "SCP-8",
    hl7_version: "2.6",
};

pub static TABLE_0660_METADATA: V2MetadataTable = V2MetadataTable {
    table: 660usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM4-6",
    hl7_version: "",
};

pub static TABLE_0661_METADATA: V2MetadataTable = V2MetadataTable {
    table: 661usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM4-18",
    hl7_version: "",
};

pub static TABLE_0662_METADATA: V2MetadataTable = V2MetadataTable {
    table: 662usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM5-2",
    hl7_version: "",
};

pub static TABLE_0663_METADATA: V2MetadataTable = V2MetadataTable {
    table: 663usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OM7-9",
    hl7_version: "",
};

pub static TABLE_0664_METADATA: V2MetadataTable = V2MetadataTable {
    table: 664usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OMC-4",
    hl7_version: "",
};

pub static TABLE_0665_METADATA: V2MetadataTable = V2MetadataTable {
    table: 665usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "OC-11",
    hl7_version: "",
};

pub static TABLE_0666_METADATA: V2MetadataTable = V2MetadataTable {
    table: 666usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "ORC-17",
    hl7_version: "",
};

pub static TABLE_0667_METADATA: V2MetadataTable = V2MetadataTable {
    table: 667usize,
    description: "Table of codes specifying the state of the data as provided from a device.",
    ttype: "User",
    steward: "OO",
    where_used: "SDD-4",
    hl7_version: "2.6",
};

pub static TABLE_0668_METADATA: V2MetadataTable = V2MetadataTable {
    table: 668usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "ORC-18",
    hl7_version: "",
};

pub static TABLE_0669_METADATA: V2MetadataTable = V2MetadataTable {
    table: 669usize,
    description: "Table of codes specifying the status of the information provided in a device sterilization or decontamination cycle.",
    ttype: "User",
    steward: "OO",
    where_used: "SDD-5",
    hl7_version: "2.6",
};

pub static TABLE_0670_METADATA: V2MetadataTable = V2MetadataTable {
    table: 670usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PCR-1",
    hl7_version: "",
};

pub static TABLE_0671_METADATA: V2MetadataTable = V2MetadataTable {
    table: 671usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PCR-3",
    hl7_version: "",
};

pub static TABLE_0672_METADATA: V2MetadataTable = V2MetadataTable {
    table: 672usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PCR-10",
    hl7_version: "",
};

pub static TABLE_0673_METADATA: V2MetadataTable = V2MetadataTable {
    table: 673usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PCR-14",
    hl7_version: "",
};

pub static TABLE_0674_METADATA: V2MetadataTable = V2MetadataTable {
    table: 674usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PCR-16",
    hl7_version: "",
};

pub static TABLE_0675_METADATA: V2MetadataTable = V2MetadataTable {
    table: 675usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PDC-2",
    hl7_version: "",
};

pub static TABLE_0676_METADATA: V2MetadataTable = V2MetadataTable {
    table: 676usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PDC-5",
    hl7_version: "",
};

pub static TABLE_0677_METADATA: V2MetadataTable = V2MetadataTable {
    table: 677usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PDC-9",
    hl7_version: "",
};

pub static TABLE_0678_METADATA: V2MetadataTable = V2MetadataTable {
    table: 678usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PEO-1",
    hl7_version: "",
};

pub static TABLE_0679_METADATA: V2MetadataTable = V2MetadataTable {
    table: 679usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PEO-2",
    hl7_version: "",
};

pub static TABLE_0680_METADATA: V2MetadataTable = V2MetadataTable {
    table: 680usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PEO-18",
    hl7_version: "",
};

pub static TABLE_0681_METADATA: V2MetadataTable = V2MetadataTable {
    table: 681usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "PRA-1",
    hl7_version: "",
};

pub static TABLE_0682_METADATA: V2MetadataTable = V2MetadataTable {
    table: 682usize,
    description: "Table of codes specifying the state of a device.",
    ttype: "User",
    steward: "OO",
    where_used: "SCD-10",
    hl7_version: "2.6",
};

pub static TABLE_0683_METADATA: V2MetadataTable = V2MetadataTable {
    table: 683usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RQ1-4",
    hl7_version: "",
};

pub static TABLE_0684_METADATA: V2MetadataTable = V2MetadataTable {
    table: 684usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RQD-2",
    hl7_version: "",
};

pub static TABLE_0685_METADATA: V2MetadataTable = V2MetadataTable {
    table: 685usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RQD-3",
    hl7_version: "",
};

pub static TABLE_0686_METADATA: V2MetadataTable = V2MetadataTable {
    table: 686usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RQD-4",
    hl7_version: "",
};

pub static TABLE_0687_METADATA: V2MetadataTable = V2MetadataTable {
    table: 687usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RQD-6",
    hl7_version: "",
};

pub static TABLE_0688_METADATA: V2MetadataTable = V2MetadataTable {
    table: 688usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RQD-9",
    hl7_version: "",
};

pub static TABLE_0689_METADATA: V2MetadataTable = V2MetadataTable {
    table: 689usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXA-7",
    hl7_version: "",
};

pub static TABLE_0690_METADATA: V2MetadataTable = V2MetadataTable {
    table: 690usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXA-8",
    hl7_version: "",
};

pub static TABLE_0691_METADATA: V2MetadataTable = V2MetadataTable {
    table: 691usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXA-9",
    hl7_version: "",
};

pub static TABLE_0692_METADATA: V2MetadataTable = V2MetadataTable {
    table: 692usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXA-14",
    hl7_version: "",
};

pub static TABLE_0693_METADATA: V2MetadataTable = V2MetadataTable {
    table: 693usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXA-18",
    hl7_version: "",
};

pub static TABLE_0694_METADATA: V2MetadataTable = V2MetadataTable {
    table: 694usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXA-19",
    hl7_version: "",
};

pub static TABLE_0695_METADATA: V2MetadataTable = V2MetadataTable {
    table: 695usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXA-24",
    hl7_version: "",
};

pub static TABLE_0696_METADATA: V2MetadataTable = V2MetadataTable {
    table: 696usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXA-25",
    hl7_version: "",
};

pub static TABLE_0697_METADATA: V2MetadataTable = V2MetadataTable {
    table: 697usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXC-2",
    hl7_version: "",
};

pub static TABLE_0698_METADATA: V2MetadataTable = V2MetadataTable {
    table: 698usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXC-4",
    hl7_version: "",
};

pub static TABLE_0699_METADATA: V2MetadataTable = V2MetadataTable {
    table: 699usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXC-6",
    hl7_version: "",
};

pub static TABLE_0700_METADATA: V2MetadataTable = V2MetadataTable {
    table: 700usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXC-7",
    hl7_version: "",
};

pub static TABLE_0701_METADATA: V2MetadataTable = V2MetadataTable {
    table: 701usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXC-9",
    hl7_version: "",
};

pub static TABLE_0702_METADATA: V2MetadataTable = V2MetadataTable {
    table: 702usize,
    description: "Table of codes specifying the type of cycle that is being executed. A cycle type is a specific sterilization method used for a specific type of supply item.",
    ttype: "User",
    steward: "OO",
    where_used: "SCD-28",
    hl7_version: "2.6",
};

pub static TABLE_0703_METADATA: V2MetadataTable = V2MetadataTable {
    table: 703usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXC-11",
    hl7_version: "",
};

pub static TABLE_0704_METADATA: V2MetadataTable = V2MetadataTable {
    table: 704usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-5",
    hl7_version: "",
};

pub static TABLE_0705_METADATA: V2MetadataTable = V2MetadataTable {
    table: 705usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-6",
    hl7_version: "",
};

pub static TABLE_0706_METADATA: V2MetadataTable = V2MetadataTable {
    table: 706usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-15",
    hl7_version: "",
};

pub static TABLE_0707_METADATA: V2MetadataTable = V2MetadataTable {
    table: 707usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-17",
    hl7_version: "",
};

pub static TABLE_0708_METADATA: V2MetadataTable = V2MetadataTable {
    table: 708usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-21",
    hl7_version: "",
};

pub static TABLE_0709_METADATA: V2MetadataTable = V2MetadataTable {
    table: 709usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-23",
    hl7_version: "",
};

pub static TABLE_0710_METADATA: V2MetadataTable = V2MetadataTable {
    table: 710usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-25",
    hl7_version: "",
};

pub static TABLE_0711_METADATA: V2MetadataTable = V2MetadataTable {
    table: 711usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-26",
    hl7_version: "",
};

pub static TABLE_0712_METADATA: V2MetadataTable = V2MetadataTable {
    table: 712usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-27",
    hl7_version: "",
};

pub static TABLE_0713_METADATA: V2MetadataTable = V2MetadataTable {
    table: 713usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-29",
    hl7_version: "",
};

pub static TABLE_0714_METADATA: V2MetadataTable = V2MetadataTable {
    table: 714usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXD-30",
    hl7_version: "",
};

pub static TABLE_0715_METADATA: V2MetadataTable = V2MetadataTable {
    table: 715usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-5",
    hl7_version: "",
};

pub static TABLE_0716_METADATA: V2MetadataTable = V2MetadataTable {
    table: 716usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-6",
    hl7_version: "",
};

pub static TABLE_0717_METADATA: V2MetadataTable = V2MetadataTable {
    table: 717usize,
    description: "Table of codes specifying the policies governing the information to which access is controlled.",
    ttype: "User",
    steward: "PA",
    where_used: "ARV-3",
    hl7_version: "2.6",
};

pub static TABLE_0718_METADATA: V2MetadataTable = V2MetadataTable {
    table: 718usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-7",
    hl7_version: "",
};

pub static TABLE_0719_METADATA: V2MetadataTable = V2MetadataTable {
    table: 719usize,
    description: "Table of codes specifying the reason for the restricted access.  Note these codes are maintained within HL7, but outside of Version 2.",
    ttype: "HL7-EXT",
    steward: "PA",
    where_used: "ARV-4",
    hl7_version: "2.6",
};

pub static TABLE_0720_METADATA: V2MetadataTable = V2MetadataTable {
    table: 720usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-11",
    hl7_version: "",
};

pub static TABLE_0721_METADATA: V2MetadataTable = V2MetadataTable {
    table: 721usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-21",
    hl7_version: "",
};

pub static TABLE_0722_METADATA: V2MetadataTable = V2MetadataTable {
    table: 722usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-24",
    hl7_version: "",
};

pub static TABLE_0723_METADATA: V2MetadataTable = V2MetadataTable {
    table: 723usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-26",
    hl7_version: "",
};

pub static TABLE_0724_METADATA: V2MetadataTable = V2MetadataTable {
    table: 724usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-27",
    hl7_version: "",
};

pub static TABLE_0725_METADATA: V2MetadataTable = V2MetadataTable {
    table: 725usize,
    description: "HL7-defined table of codes specifying the functional state of an order.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXO-31, OBX-22",
    hl7_version: "2.6",
};

pub static TABLE_0726_METADATA: V2MetadataTable = V2MetadataTable {
    table: 726usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-29",
    hl7_version: "",
};

pub static TABLE_0727_METADATA: V2MetadataTable = V2MetadataTable {
    table: 727usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-31",
    hl7_version: "",
};

pub static TABLE_0728_METADATA: V2MetadataTable = V2MetadataTable {
    table: 728usize,
    description: "Table of codes specifying the clinical complexity level (CCL) value for the determined diagnosis related group (DRG) for this diagnosis. US Realm.",
    ttype: "HL7",
    steward: "FM",
    where_used: "DG1-23",
    hl7_version: "2.6",
};

pub static TABLE_0729_METADATA: V2MetadataTable = V2MetadataTable {
    table: 729usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-34",
    hl7_version: "",
};

pub static TABLE_0730_METADATA: V2MetadataTable = V2MetadataTable {
    table: 730usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-37",
    hl7_version: "",
};

pub static TABLE_0731_METADATA: V2MetadataTable = V2MetadataTable {
    table: 731usize,
    description: "HL7-defined table of codes specifying the status of a diagnosis for a diagnosis related group (DRG) determination. US Realm.",
    ttype: "HL7",
    steward: "FM",
    where_used: "DG1-25",
    hl7_version: "2.6",
};

pub static TABLE_0732_METADATA: V2MetadataTable = V2MetadataTable {
    table: 732usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-38",
    hl7_version: "",
};

pub static TABLE_0733_METADATA: V2MetadataTable = V2MetadataTable {
    table: 733usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXE-40",
    hl7_version: "",
};

pub static TABLE_0734_METADATA: V2MetadataTable = V2MetadataTable {
    table: 734usize,
    description: "Table of codes specifying the status of a grouper in general. US Realm.",
    ttype: "User",
    steward: "FM",
    where_used: "DRG-13",
    hl7_version: "2.6",
};

pub static TABLE_0735_METADATA: V2MetadataTable = V2MetadataTable {
    table: 735usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXG-7",
    hl7_version: "",
};

pub static TABLE_0736_METADATA: V2MetadataTable = V2MetadataTable {
    table: 736usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXG-8",
    hl7_version: "",
};

pub static TABLE_0737_METADATA: V2MetadataTable = V2MetadataTable {
    table: 737usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXG-9",
    hl7_version: "",
};

pub static TABLE_0738_METADATA: V2MetadataTable = V2MetadataTable {
    table: 738usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXG-13",
    hl7_version: "",
};

pub static TABLE_0739_METADATA: V2MetadataTable = V2MetadataTable {
    table: 739usize,
    description: "Table of codes specifying whether the length of stay is normal or respectively shorter or longer than normal.",
    ttype: "User",
    steward: "FM",
    where_used: "DRG-17",
    hl7_version: "2.6",
};

pub static TABLE_0740_METADATA: V2MetadataTable = V2MetadataTable {
    table: 740usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXG-16",
    hl7_version: "",
};

pub static TABLE_0741_METADATA: V2MetadataTable = V2MetadataTable {
    table: 741usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RX-18",
    hl7_version: "",
};

pub static TABLE_0742_METADATA: V2MetadataTable = V2MetadataTable {
    table: 742usize,
    description: "Table of codes specifying the status of the diagnosis related group (DRG) calculation regarding the financial aspects. US Realm.",
    ttype: "User",
    steward: "FM",
    where_used: "DRG-20",
    hl7_version: "2.6",
};

pub static TABLE_0743_METADATA: V2MetadataTable = V2MetadataTable {
    table: 743usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXG-22",
    hl7_version: "",
};

pub static TABLE_0744_METADATA: V2MetadataTable = V2MetadataTable {
    table: 744usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RX-24",
    hl7_version: "",
};

pub static TABLE_0745_METADATA: V2MetadataTable = V2MetadataTable {
    table: 745usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXG-25",
    hl7_version: "",
};

pub static TABLE_0746_METADATA: V2MetadataTable = V2MetadataTable {
    table: 746usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXG-33",
    hl7_version: "",
};

pub static TABLE_0747_METADATA: V2MetadataTable = V2MetadataTable {
    table: 747usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-1",
    hl7_version: "",
};

pub static TABLE_0748_METADATA: V2MetadataTable = V2MetadataTable {
    table: 748usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-4",
    hl7_version: "",
};

pub static TABLE_0749_METADATA: V2MetadataTable = V2MetadataTable {
    table: 749usize,
    description: "Table of codes specifying the status of the use of the gender information for diagnosis related group (DRG) determination. US Realm.",
    ttype: "User",
    steward: "FM",
    where_used: "DRG-26",
    hl7_version: "2.6",
};

pub static TABLE_0750_METADATA: V2MetadataTable = V2MetadataTable {
    table: 750usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-5",
    hl7_version: "",
};

pub static TABLE_0751_METADATA: V2MetadataTable = V2MetadataTable {
    table: 751usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-6",
    hl7_version: "",
};

pub static TABLE_0752_METADATA: V2MetadataTable = V2MetadataTable {
    table: 752usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-7",
    hl7_version: "",
};

pub static TABLE_0753_METADATA: V2MetadataTable = V2MetadataTable {
    table: 753usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-10",
    hl7_version: "",
};

pub static TABLE_0754_METADATA: V2MetadataTable = V2MetadataTable {
    table: 754usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-12",
    hl7_version: "",
};

pub static TABLE_0755_METADATA: V2MetadataTable = V2MetadataTable {
    table: 755usize,
    description: "Table of codes specifying the status of the use of the weight at birth for diagnosis related group (DRG) determination. US Realm.",
    ttype: "User",
    steward: "FM",
    where_used: "DRG-31",
    hl7_version: "2.6",
};

pub static TABLE_0756_METADATA: V2MetadataTable = V2MetadataTable {
    table: 756usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-19",
    hl7_version: "",
};

pub static TABLE_0757_METADATA: V2MetadataTable = V2MetadataTable {
    table: 757usize,
    description: "Table of codes specifying the status of the use of the respiration minutes information for diagnosis related group (DRG) determination. US Realm.",
    ttype: "User",
    steward: "FM",
    where_used: "DRG-32",
    hl7_version: "2.6",
};

pub static TABLE_0758_METADATA: V2MetadataTable = V2MetadataTable {
    table: 758usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-20",
    hl7_version: "",
};

pub static TABLE_0759_METADATA: V2MetadataTable = V2MetadataTable {
    table: 759usize,
    description: "Table of codes specifying the admission status for the diagnosis related group (DRG) determination. US Realm.",
    ttype: "User",
    steward: "FM",
    where_used: "DRG-33",
    hl7_version: "2.6",
};

pub static TABLE_0760_METADATA: V2MetadataTable = V2MetadataTable {
    table: 760usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-22",
    hl7_version: "",
};

pub static TABLE_0761_METADATA: V2MetadataTable = V2MetadataTable {
    table: 761usize,
    description: "Table of codes specifying the status of the use of this particular procedure for the diagnosis related group (DRG) determination. US Realm.",
    ttype: "User",
    steward: "FM",
    where_used: "PR1-21",
    hl7_version: "2.6",
};

pub static TABLE_0762_METADATA: V2MetadataTable = V2MetadataTable {
    table: 762usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-24",
    hl7_version: "",
};

pub static TABLE_0763_METADATA: V2MetadataTable = V2MetadataTable {
    table: 763usize,
    description: "Table of codes specifying the relevance of this particular procedure for the diagnosis related group (DRG) determination. US Realm.",
    ttype: "User",
    steward: "FM",
    where_used: "PR1-22",
    hl7_version: "2.6",
};

pub static TABLE_0764_METADATA: V2MetadataTable = V2MetadataTable {
    table: 764usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-26",
    hl7_version: "",
};

pub static TABLE_0765_METADATA: V2MetadataTable = V2MetadataTable {
    table: 765usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXO-32",
    hl7_version: "",
};

pub static TABLE_0766_METADATA: V2MetadataTable = V2MetadataTable {
    table: 766usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXR-5",
    hl7_version: "",
};

pub static TABLE_0767_METADATA: V2MetadataTable = V2MetadataTable {
    table: 767usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXV-4",
    hl7_version: "",
};

pub static TABLE_0768_METADATA: V2MetadataTable = V2MetadataTable {
    table: 768usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXV-6",
    hl7_version: "",
};

pub static TABLE_0769_METADATA: V2MetadataTable = V2MetadataTable {
    table: 769usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXV-9",
    hl7_version: "",
};

pub static TABLE_0770_METADATA: V2MetadataTable = V2MetadataTable {
    table: 770usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXV-11",
    hl7_version: "",
};

pub static TABLE_0771_METADATA: V2MetadataTable = V2MetadataTable {
    table: 771usize,
    description:
        "Table of codes specifying a high level categorization of resources.  No suggested values.",
    ttype: "User",
    steward: "PA",
    where_used: "STF-39",
    hl7_version: "2.6",
};

pub static TABLE_0772_METADATA: V2MetadataTable = V2MetadataTable {
    table: 772usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXV-13",
    hl7_version: "",
};

pub static TABLE_0773_METADATA: V2MetadataTable = V2MetadataTable {
    table: 773usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "RXV-15",
    hl7_version: "",
};

pub static TABLE_0774_METADATA: V2MetadataTable = V2MetadataTable {
    table: 774usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "SAC-15",
    hl7_version: "",
};

pub static TABLE_0775_METADATA: V2MetadataTable = V2MetadataTable {
    table: 775usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "SAC-20",
    hl7_version: "",
};

pub static TABLE_0776_METADATA: V2MetadataTable = V2MetadataTable {
    table: 776usize,
    description: "Table of codes specifying the status (useful for reporting and item usage purposes) that applies to an item.",
    ttype: "User",
    steward: "MM",
    where_used: "ITM-3",
    hl7_version: "2.6",
};

pub static TABLE_0777_METADATA: V2MetadataTable = V2MetadataTable {
    table: 777usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "SAC-24",
    hl7_version: "",
};

pub static TABLE_0778_METADATA: V2MetadataTable = V2MetadataTable {
    table: 778usize,
    description: "Table of codes specifying a classification of material items into like groups as defined and utilized within an operating room setting for charting procedures.",
    ttype: "User",
    steward: "OO",
    where_used: "ITM-4",
    hl7_version: "2.6",
};

pub static TABLE_0779_METADATA: V2MetadataTable = V2MetadataTable {
    table: 779usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "SAC-33",
    hl7_version: "",
};

pub static TABLE_0780_METADATA: V2MetadataTable = V2MetadataTable {
    table: 780usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "SAC-35",
    hl7_version: "",
};

pub static TABLE_0781_METADATA: V2MetadataTable = V2MetadataTable {
    table: 781usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "SAC-37",
    hl7_version: "",
};

pub static TABLE_0782_METADATA: V2MetadataTable = V2MetadataTable {
    table: 782usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "SAC-39",
    hl7_version: "",
};

pub static TABLE_0783_METADATA: V2MetadataTable = V2MetadataTable {
    table: 783usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "SID-1",
    hl7_version: "",
};

pub static TABLE_0784_METADATA: V2MetadataTable = V2MetadataTable {
    table: 784usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "SPM-8",
    hl7_version: "",
};

pub static TABLE_0785_METADATA: V2MetadataTable = V2MetadataTable {
    table: 785usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "SPM-27",
    hl7_version: "",
};

pub static TABLE_0786_METADATA: V2MetadataTable = V2MetadataTable {
    table: 786usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "STD-1",
    hl7_version: "",
};

pub static TABLE_0787_METADATA: V2MetadataTable = V2MetadataTable {
    table: 787usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "TCC-1",
    hl7_version: "",
};

pub static TABLE_0788_METADATA: V2MetadataTable = V2MetadataTable {
    table: 788usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "TCC-13",
    hl7_version: "",
};

pub static TABLE_0789_METADATA: V2MetadataTable = V2MetadataTable {
    table: 789usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "TCD-1",
    hl7_version: "",
};

pub static TABLE_0790_METADATA: V2MetadataTable = V2MetadataTable {
    table: 790usize,
    description: "Table of codes specifying the regulatory agency by which the item has been approved, such as the FDA or AMA.",
    ttype: "User",
    steward: "OO",
    where_used: "ITM-16",
    hl7_version: "2.6",
};

pub static TABLE_0791_METADATA: V2MetadataTable = V2MetadataTable {
    table: 791usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "TXA-24",
    hl7_version: "",
};

pub static TABLE_0792_METADATA: V2MetadataTable = V2MetadataTable {
    table: 792usize,
    description: "9999",
    ttype: "undefined",
    steward: "",
    where_used: "TXA-28",
    hl7_version: "",
};

pub static TABLE_0793_METADATA: V2MetadataTable = V2MetadataTable {
    table: 793usize,
    description: "Table of codes specifying an act containing a rule that the item is legally required to be included in notification reporting.",
    ttype: "User",
    steward: "OO",
    where_used: "ITM-18",
    hl7_version: "2.6",
};

pub static TABLE_0794_METADATA: V2MetadataTable = V2MetadataTable {
    table: 794usize,
    description: "9999",
    ttype: "undefined",
    steward: "InM",
    where_used: "CQ.2",
    hl7_version: "2.9",
};

pub static TABLE_0795_METADATA: V2MetadataTable = V2MetadataTable {
    table: 795usize,
    description: "9999",
    ttype: "HL7",
    steward: "OO",
    where_used: "DEV-17",
    hl7_version: "2.9",
};

pub static TABLE_0806_METADATA: V2MetadataTable = V2MetadataTable {
    table: 806usize,
    description: "Table of codes specifying the type of sterilization used for sterilizing the inventory supply item in the ITM segment.",
    ttype: "User",
    steward: "OO",
    where_used: "STZ-1",
    hl7_version: "2.6",
};

pub static TABLE_0809_METADATA: V2MetadataTable = V2MetadataTable {
    table: 809usize,
    description: "Table of codes specifying the maintenance cycle used for the inventory supply item, such as the number of times to sharpen after five uses.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "STZ-3",
    hl7_version: "2.6",
};

pub static TABLE_0811_METADATA: V2MetadataTable = V2MetadataTable {
    table: 811usize,
    description: "Table of codes specifying the type of maintenance performed on the inventory supply item.  This is different than the maintenance cycle in the sense that it can describe the number of maintenance cycles that can be performed before disposing of the inventory supply item.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "STZ-4",
    hl7_version: "2.6",
};

pub static TABLE_0818_METADATA: V2MetadataTable = V2MetadataTable {
    table: 818usize,
    description: "Table of codes specifying the packaging unit in which this inventory supply item can be ordered or issued when purchased from the vendor in the related vendor segment.",
    ttype: "User",
    steward: "OO",
    where_used: "PKG-2",
    hl7_version: "2.6",
};

pub static TABLE_0834_METADATA: V2MetadataTable = V2MetadataTable {
    table: 834usize,
    description: "Table of codes specifying the general type of data.",
    ttype: "Imported",
    steward: "InM",
    where_used: "RP.3, ED.2",
    hl7_version: "2.6",
};

pub static TABLE_0836_METADATA: V2MetadataTable = V2MetadataTable {
    table: 836usize,
    description: "Table of codes specifying the severity of the problem.  No suggested values.",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.6",
};

pub static TABLE_0838_METADATA: V2MetadataTable = V2MetadataTable {
    table: 838usize,
    description: "Table of codes specifying from whose perspective this problem was identified. No suggested values.",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.6",
};

pub static TABLE_0865_METADATA: V2MetadataTable = V2MetadataTable {
    table: 865usize,
    description: "Table of codes specifying to the receiving provider that the clinical history in the message is incomplete and that more will follow.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "RF1-12",
    hl7_version: "2.6",
};

pub static TABLE_0868_METADATA: V2MetadataTable = V2MetadataTable {
    table: 868usize,
    description:
        "Table of codes specifying the reason this contact number/email was marked as \"ended\".",
    ttype: "User",
    steward: "InM",
    where_used: "XTN.15",
    hl7_version: "2.6",
};

pub static TABLE_0871_METADATA: V2MetadataTable = V2MetadataTable {
    table: 871usize,
    description: "Table of codes specifying any known or suspected hazard associated with this material item.",
    ttype: "User",
    steward: "OO",
    where_used: "ITM-15",
    hl7_version: "2.6",
};

pub static TABLE_0879_METADATA: V2MetadataTable = V2MetadataTable {
    table: 879usize,
    description:
        "Table of codes specifying what service was delivered/received.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "PSL-7, PSL-22",
    hl7_version: "2.6",
};

pub static TABLE_0880_METADATA: V2MetadataTable = V2MetadataTable {
    table: 880usize,
    description: "Table of codes specifying additional optional modifier(s) for the Product/Service Code (e.g., after hours-evening, after hours-weekend).  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "PSL-8",
    hl7_version: "2.6",
};

pub static TABLE_0881_METADATA: V2MetadataTable = V2MetadataTable {
    table: 881usize,
    description: "Table of codes specifying the account role of the physician, for example, only billing for the professional part, the technical part or both.",
    ttype: "User",
    steward: "FM",
    where_used: "PSL-31",
    hl7_version: "2.6",
};

pub static TABLE_0882_METADATA: V2MetadataTable = V2MetadataTable {
    table: 882usize,
    description:
        "Table of codes specifying the role of the physician (\"self-employed\" or \"employed\").",
    ttype: "User",
    steward: "FM",
    where_used: "PSL-32",
    hl7_version: "2.6",
};

pub static TABLE_0894_METADATA: V2MetadataTable = V2MetadataTable {
    table: 894usize,
    description: "Table of codes specifying the side of the body (\"left\" or \"right\").",
    ttype: "User",
    steward: "FM",
    where_used: "PSL-33",
    hl7_version: "2.6",
};

pub static TABLE_0895_METADATA: V2MetadataTable = V2MetadataTable {
    table: 895usize,
    description: "Table of codes specifying the present on admission indicator for this particular diagnosis. US reimbursement formulas for some states and Medicare have mandated that each diagnosis code be flagged as to whether it was present on admission or not.",
    ttype: "User",
    steward: "FM",
    where_used: "DG1-26",
    hl7_version: "2.6",
};

pub static TABLE_0904_METADATA: V2MetadataTable = V2MetadataTable {
    table: 904usize,
    description: "HL7-defined table of codes specifying the scheme for the security check.",
    ttype: "HL7",
    steward: "InM",
    where_used: "CX.12, PPN.26, XCN.25",
    hl7_version: "2.7",
};

pub static TABLE_0905_METADATA: V2MetadataTable = V2MetadataTable {
    table: 905usize,
    description: "HL7-defined table of codes specifying the status of the shipment.",
    ttype: "User",
    steward: "OO",
    where_used: "SHP-3",
    hl7_version: "2.7",
};

pub static TABLE_0906_METADATA: V2MetadataTable = V2MetadataTable {
    table: 906usize,
    description: "HL7-defined table of codes specifying the priority for the shipment.",
    ttype: "User",
    steward: "OO",
    where_used: "SHP-6",
    hl7_version: "2.7",
};

pub static TABLE_0907_METADATA: V2MetadataTable = V2MetadataTable {
    table: 907usize,
    description: "HL7-defined table of codes specifying the confidentiality for the shipment.",
    ttype: "User",
    steward: "OO",
    where_used: "SHP-7",
    hl7_version: "2.7",
};

pub static TABLE_0908_METADATA: V2MetadataTable = V2MetadataTable {
    table: 908usize,
    description: "Table of codes specifying the type of package.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "PAC-5",
    hl7_version: "2.7",
};

pub static TABLE_0909_METADATA: V2MetadataTable = V2MetadataTable {
    table: 909usize,
    description: "HL7-defined table of codes specifying the scheme for the patient results release categorization.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OBX-26",
    hl7_version: "2.7",
};

pub static TABLE_0910_METADATA: V2MetadataTable = V2MetadataTable {
    table: 910usize,
    description: "Table of codes specifying the modality for the acquisition.",
    ttype: "User",
    steward: "InM/OO",
    where_used: "OM1-47",
    hl7_version: "2.7",
};

pub static TABLE_0912_METADATA: V2MetadataTable = V2MetadataTable {
    table: 912usize,
    description: "HL7-defined table of codes that represent functional involvement of a caregiver or member of a care team with an activity being transmitted (e.g., Case Manager, Evaluator, Transcriber, Nurse Care Practitioner, Midwife, Physician Assi stant, etc.).",
    ttype: "HL7-EXT",
    steward: "OO",
    where_used: "PRT-4",
    hl7_version: "2.7",
};

pub static TABLE_0913_METADATA: V2MetadataTable = V2MetadataTable {
    table: 913usize,
    description: "Table of codes specifying the denomination in which the quantity is expressed. The values for the denomination component are the three-character codes specified in  ISO-4217 (1.0.4217 iso4217).",
    ttype: "External",
    steward: "InM",
    where_used: "MO.2, MOP.3",
    hl7_version: "2.7",
};

pub static TABLE_0914_METADATA: V2MetadataTable = V2MetadataTable {
    table: 914usize,
    description: "Table of codes specifying the root cause.",
    ttype: "User",
    steward: "OO",
    where_used: "OBX-27",
    hl7_version: "2.8",
};

pub static TABLE_0915_METADATA: V2MetadataTable = V2MetadataTable {
    table: 915usize,
    description: "Table of codes specifying the process control code.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "OBX-28",
    hl7_version: "2.8",
};

pub static TABLE_0916_METADATA: V2MetadataTable = V2MetadataTable {
    table: 916usize,
    description: "Table of codes specifying additional clinical information about the patient or specimen to report the supporting and/or suspected diagnosis and clinical findings on requests for interpreted diagnostic studies.",
    ttype: "User",
    steward: "OO",
    where_used: "OBR-13",
    hl7_version: "2.7.1",
};

pub static TABLE_0917_METADATA: V2MetadataTable = V2MetadataTable {
    table: 917usize,
    description: "HL7-defined table of codes specifying the type of bolus.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXV-2",
    hl7_version: "2.8",
};

pub static TABLE_0918_METADATA: V2MetadataTable = V2MetadataTable {
    table: 918usize,
    description: "HL7-defined table of codes specifying the type of PCA.",
    ttype: "HL7",
    steward: "OO",
    where_used: "RXV-7",
    hl7_version: "2.8",
};

pub static TABLE_0919_METADATA: V2MetadataTable = V2MetadataTable {
    table: 919usize,
    description: "HL7-defined table of codes that define if a test should be a specific event with no other tests to be performed with this test, or not, or other special circumstances.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OM1-48",
    hl7_version: "2.8",
};

pub static TABLE_0920_METADATA: V2MetadataTable = V2MetadataTable {
    table: 920usize,
    description: "HL7-defined table of codes that indicate whether a Specimen/Attribute is Preferred or Alternate for collection of a particular specimen.",
    ttype: "HL7",
    steward: "OO",
    where_used: "OM4-16",
    hl7_version: "2.8",
};

pub static TABLE_0921_METADATA: V2MetadataTable = V2MetadataTable {
    table: 921usize,
    description: "Table of codes specifying the code for the certification type.",
    ttype: "User",
    steward: "FM",
    where_used: "IN3-26",
    hl7_version: "2.8",
};

pub static TABLE_0922_METADATA: V2MetadataTable = V2MetadataTable {
    table: 922usize,
    description: "Table of codes specifying the code for the certification category.",
    ttype: "User",
    steward: "FM",
    where_used: "IN3-27",
    hl7_version: "2.8",
};

pub static TABLE_0923_METADATA: V2MetadataTable = V2MetadataTable {
    table: 923usize,
    description: "HL7-defined table of codes specifying whether the process was interrrupted and whether the needle had been inserted in the donor's arm prior to the interruption.",
    ttype: "User",
    steward: "OO",
    where_used: "DON-12",
    hl7_version: "2.8",
};

pub static TABLE_0924_METADATA: V2MetadataTable = V2MetadataTable {
    table: 924usize,
    description:
        "Table of codes specifying the unit of measure (UoM) for the cumulative dosage limit.",
    ttype: "User",
    steward: "OO",
    where_used: "CDO-4",
    hl7_version: "2.8",
};

pub static TABLE_0925_METADATA: V2MetadataTable = V2MetadataTable {
    table: 925usize,
    description: "HL7-defined table of codes specifying the phlebotomy issue.",
    ttype: "HL7",
    steward: "OO",
    where_used: "DON-14",
    hl7_version: "2.8",
};

pub static TABLE_0926_METADATA: V2MetadataTable = V2MetadataTable {
    table: 926usize,
    description: "HL7-defined table of codes specifying the status of the phlebotomy.",
    ttype: "HL7",
    steward: "OO",
    where_used: "DON-21",
    hl7_version: "2.8",
};

pub static TABLE_0927_METADATA: V2MetadataTable = V2MetadataTable {
    table: 927usize,
    description: "HL7-defined table of codes specifying the arm(s) receiving the stick.",
    ttype: "HL7",
    steward: "OO",
    where_used: "DON-22",
    hl7_version: "2.8",
};

pub static TABLE_0929_METADATA: V2MetadataTable = V2MetadataTable {
    table: 929usize,
    description: "HL7-defined table of codes specifying the units of weight.",
    ttype: "HL7",
    steward: "OO",
    where_used: "BUI-5",
    hl7_version: "2.8",
};

pub static TABLE_0930_METADATA: V2MetadataTable = V2MetadataTable {
    table: 930usize,
    description: "HL7-defined table of codes specifying the units of volume.",
    ttype: "HL7",
    steward: "OO",
    where_used: "BUI-7",
    hl7_version: "2.8",
};

pub static TABLE_0931_METADATA: V2MetadataTable = V2MetadataTable {
    table: 931usize,
    description: "HL7-defined table of codes specifying  the units of transport temperature.",
    ttype: "HL7",
    steward: "OO",
    where_used: "BUI-12",
    hl7_version: "2.8",
};

pub static TABLE_0932_METADATA: V2MetadataTable = V2MetadataTable {
    table: 932usize,
    description: "HL7-defined table of codes specifying  the units of donation duration.",
    ttype: "User",
    steward: "OO",
    where_used: "DON-6",
    hl7_version: "2.8",
};

pub static TABLE_0933_METADATA: V2MetadataTable = V2MetadataTable {
    table: 933usize,
    description: "HL7-defined table of codes specifying the type of intended procedure.",
    ttype: "HL7",
    steward: "OO",
    where_used: "DON-7",
    hl7_version: "2.8",
};

pub static TABLE_0934_METADATA: V2MetadataTable = V2MetadataTable {
    table: 934usize,
    description:
        "Table of codes specifying the profile of the order workflow.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "ORC-34",
    hl7_version: "2.8",
};

pub static TABLE_0935_METADATA: V2MetadataTable = V2MetadataTable {
    table: 935usize,
    description: "HL7-defined table of codes specifying the reason for the process interruption.",
    ttype: "User",
    steward: "OO",
    where_used: "DON-13",
    hl7_version: "2.8",
};

pub static TABLE_0936_METADATA: V2MetadataTable = V2MetadataTable {
    table: 936usize,
    description: "HL7-defined table of codes specifying types of observations to enable systems to distinguish between observations sent along with an order, versus observations sent as the result to an order.",
    ttype: "HL7",
    steward: "",
    where_used: "",
    hl7_version: "2.8.1",
};

pub static TABLE_0937_METADATA: V2MetadataTable = V2MetadataTable {
    table: 937usize,
    description: "HL7-defined table of codes specifying the observation sub-type.",
    ttype: "HL7",
    steward: "",
    where_used: "",
    hl7_version: "2.8.2",
};

pub static TABLE_0938_METADATA: V2MetadataTable = V2MetadataTable {
    table: 938usize,
    description:
        "HL7-defined table of codes specifying the limit for the collection event or process step.",
    ttype: "HL7",
    steward: "",
    where_used: "",
    hl7_version: "2.8.2",
};

pub static TABLE_0939_METADATA: V2MetadataTable = V2MetadataTable {
    table: 939usize,
    description: "HL7-defined table of codes specifying the communication location.",
    ttype: "HL7",
    steward: "",
    where_used: "",
    hl7_version: "2.8.2",
};

pub static TABLE_0940_METADATA: V2MetadataTable = V2MetadataTable {
    table: 940usize,
    description: "HL7-defined table of codes specifying the type of limitation.",
    ttype: "HL7",
    steward: "",
    where_used: "",
    hl7_version: "2.8.2",
};

pub static TABLE_0941_METADATA: V2MetadataTable = V2MetadataTable {
    table: 941usize,
    description: "Table of codes specifying procedure codes that may impact payer coverage requirements, for example procedure code 1234 is not covered by a payer ABCD or may be covered in conjunction with a specific diagnosis code which can be identifeid in DPS-1 Diagnosis Code. The procedure codes should be drawn from appropriate externally defined procedure codes, for example in the US Realm these include CPT-4 codes defined by the American Medical Association and ICD codes published by CMS.  No suggested values.",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.8.2",
};

pub static TABLE_0942_METADATA: V2MetadataTable = V2MetadataTable {
    table: 942usize,
    description: "HL7-defined table of codes that specify the type of measurement of the state of an automated laboratory instrument.",
    ttype: "HL7",
    steward: "",
    where_used: "INV-21",
    hl7_version: "2.9",
};

pub static TABLE_0943_METADATA: V2MetadataTable = V2MetadataTable {
    table: 943usize,
    description: "Table of codes that identify the destination for transport of a specific container. No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "DST-1",
    hl7_version: "2.9",
};

pub static TABLE_0944_METADATA: V2MetadataTable = V2MetadataTable {
    table: 944usize,
    description: "Table of codes that identify the route for transport of a specific container.    No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "DST-2",
    hl7_version: "2.9",
};

pub static TABLE_0945_METADATA: V2MetadataTable = V2MetadataTable {
    table: 945usize,
    description: "Vendor-defined codes of the pre‑configured dilution to be applied on the instrument, which can be used instead of a numeric declaration.",
    ttype: "User",
    steward: "OO",
    where_used: "TCD-11",
    hl7_version: "2.9",
};

pub static TABLE_0946_METADATA: V2MetadataTable = V2MetadataTable {
    table: 946usize,
    description: "Table of codes specifying the type of supplier that will distribute the supply items associated to a contract number.",
    ttype: "User",
    steward: "OO",
    where_used: "CTR-8",
    hl7_version: "2.9",
};

pub static TABLE_0947_METADATA: V2MetadataTable = V2MetadataTable {
    table: 947usize,
    description: "Table of codes specifying the purchasing channel with which the contract is associated  such as Hospital, Retail, etc.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "CTR-20",
    hl7_version: "2.9",
};

pub static TABLE_0948_METADATA: V2MetadataTable = V2MetadataTable {
    table: 948usize,
    description: "HL7-defined table of codes specifying the type of relationship identified by Relationship Instance Identifier (REL-3) that is established between the Source Information Instance (REL-4)  and the Target Information Instance (REL-5).",
    ttype: "User",
    steward: "",
    where_used: "",
    hl7_version: "2.9",
};

pub static TABLE_0949_METADATA: V2MetadataTable = V2MetadataTable {
    table: 949usize,
    description: "HL7-defined table of codes that describe reasons for the chosen order control codes – this table is extensible; while these codes are intended to be generally useful, they were developed to cover situations for replacement orders (ORC-1 = RO) and recommendations for replacement orders (ORC-1 = RP)",
    ttype: "User",
    steward: "OO",
    where_used: "ORC-16",
    hl7_version: "2.9",
};

pub static TABLE_0950_METADATA: V2MetadataTable = V2MetadataTable {
    table: 950usize,
    description:
        "HL7-defined table of codes used to further define the status identified in ORC-5",
    ttype: "User",
    steward: "OO",
    where_used: "ORC-25",
    hl7_version: "2.9",
};

pub static TABLE_0951_METADATA: V2MetadataTable = V2MetadataTable {
    table: 951usize,
    description: "HL7-defined table of codes that provide additional information to the universal service identifier on why a test, study or review was ordered. Current suggested values are in support of the IHE LCC LAB-7 transaction.",
    ttype: "User",
    steward: "OO",
    where_used: "OBR-31",
    hl7_version: "2.9",
};

pub static TABLE_0952_METADATA: V2MetadataTable = V2MetadataTable {
    table: 952usize,
    description: "Table of codes that specify the security classification; the codes are non-overlapping n the following  hierarchical order: Very Restricted > Restricted > Normal > Moderate > Low > Unrestricted More information may be found the HL7 Healthcare Privacy and Security Classification System (HCS), Release 1 (see: http://www.hl7.org/implement/standards/product_brief.cfm?product_id=345) For the list of codes in the table, see the HL7 Webpage rendering.",
    ttype: "External",
    steward: "InM",
    where_used: "MSH-26, ARV-7",
    hl7_version: "2.9",
};

pub static TABLE_0953_METADATA: V2MetadataTable = V2MetadataTable {
    table: 953usize,
    description: "Security observation values used to indicate security control metadata, meaning how security tagged data needs to be handled and what it can be used for. This value set is the union of V:SecurityPolicy,V:ObligationPolicy, V:RefrainPolicy, V:PurposeOfUse, and V:GeneralPurpose of Use in FHIR.   It is used to convey one or more nonhierarchical security control metadata dictating handling caveats, purpose of use, dissemination controls and other refrain policies, and obligations to which a custodian or receiver is required to comply. For the list of codes in the table, see the HL7 Webpage rendering.",
    ttype: "HL7-EXT",
    steward: "InM",
    where_used: "MSH-27, ARV-8",
    hl7_version: "2.9",
};

pub static TABLE_0954_METADATA: V2MetadataTable = V2MetadataTable {
    table: 954usize,
    description: "Describes an individual's typical arrangement of working hours for an occupation. For the list of codes in the table, see the HL7 Webpage rendering.",
    ttype: "HL7-EXT",
    steward: "PH",
    where_used: "ODH2-9",
    hl7_version: "2.9",
};

pub static TABLE_0955_METADATA: V2MetadataTable = V2MetadataTable {
    table: 955usize,
    description: "The type of business associated with the patient's occupation.  For enumeration of codes, see https://phinvads.cdc.gov/vads/ViewValueSet.action?oid=2.16.840.1.114222.4.11 .7187",
    ttype: "External",
    steward: "PH",
    where_used: "OH2-5, OH3-4",
    hl7_version: "2.9",
};

pub static TABLE_0956_METADATA: V2MetadataTable = V2MetadataTable {
    table: 956usize,
    description: "Reflects the amount of supervisory or management responsibilities for an individual’s job.  In the military, this is the person’s pay grade, which serves as a proxy for supervisory level and can be interpreted across branches. For enumeration of codes, see https://phinvads.cdc.gov/vads/ViewValueSet.action?oid=2.16.840.1.114222.4.11 .7613 The FHIR defining URL for the value set has been created as http://hl7.org/fhir/ValueSet/supervisory-lev el-odh-us, however note that this is not resolvable at this time.",
    ttype: "External",
    steward: "PH",
    where_used: "OH1-14",
    hl7_version: "2.9",
};

pub static TABLE_0957_METADATA: V2MetadataTable = V2MetadataTable {
    table: 957usize,
    description: "Concepts representing whether a person does or does not currently have a job or is not currently in the labor pool seeking employment.",
    ttype: "HL7-EXT",
    steward: "PH",
    where_used: "OH1-3",
    hl7_version: "2.9",
};

pub static TABLE_0958_METADATA: V2MetadataTable = V2MetadataTable {
    table: 958usize,
    description: "The occupation in which an individual is engaged.  In the US, references External Vocabulary code system:  Occupation CDC Census 2010 https://phinvads.cdc.gov/vads/ViewCodeSystem.action?id=2.16.840.1.114222.4. 5.314#",
    ttype: "External",
    steward: "PA",
    where_used: "OH2-4, OH3-3",
    hl7_version: "2.9",
};

pub static TABLE_0959_METADATA: V2MetadataTable = V2MetadataTable {
    table: 959usize,
    description: "A patient's employment type as defined by work classification (e.g. paid vs. unpaid, self-employed vs. not self-employed, government vs. private, etc.).For the list of codes in the table, see the FHIR rendering.",
    ttype: "HL7-EXT",
    steward: "PH",
    where_used: "ODH2-6",
    hl7_version: "2.9",
};

pub static TABLE_0960_METADATA: V2MetadataTable = V2MetadataTable {
    table: 960usize,
    description: "Used to specify why the normally expected content of the data element is missing.  Note that this content is maintained in the FHIR product family.",
    ttype: "HL7-EXT",
    steward: "OO",
    where_used: "OBX-32",
    hl7_version: "2.9",
};

pub static TABLE_0961_METADATA: V2MetadataTable = V2MetadataTable {
    table: 961usize,
    description: "Contains codes used to identify medical devices.",
    ttype: "HL7-EXT",
    steward: "OO",
    where_used: "DEV-3",
    hl7_version: "2.9",
};

pub static TABLE_0962_METADATA: V2MetadataTable = V2MetadataTable {
    table: 962usize,
    description: "Contains codes describing the availability status of the device.",
    ttype: "HL7-EXT",
    steward: "OO",
    where_used: "DEV-4",
    hl7_version: "2.9",
};

pub static TABLE_0963_METADATA: V2MetadataTable = V2MetadataTable {
    table: 963usize,
    description: "Contains codes used to identify medical devices safety characteristics.",
    ttype: "HL7-EXT",
    steward: "OO",
    where_used: "DEV-14",
    hl7_version: "2.9",
};

pub static TABLE_0964_METADATA: V2MetadataTable = V2MetadataTable {
    table: 964usize,
    description:
        "Table of codes describing reasons why a service was performed.  No suggested values.",
    ttype: "User",
    steward: "FM",
    where_used: "FT1-56",
    hl7_version: "2.9",
};

pub static TABLE_0965_METADATA: V2MetadataTable = V2MetadataTable {
    table: 965usize,
    description: "Values describing the scope of contracts, for example Local, Regional and Global.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "CTR-9",
    hl7_version: "2.9",
};

pub static TABLE_0966_METADATA: V2MetadataTable = V2MetadataTable {
    table: 966usize,
    description: "Values the describing the declared level of pricing for a particular product under a contract, usually based on a discount for larger orders under the contract – often assigned numerically, for example: 01 = Tier 1, 02 = Tier 2, 03 = Tier 3 No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "CTR-18",
    hl7_version: "2.9",
};

pub static TABLE_0967_METADATA: V2MetadataTable = V2MetadataTable {
    table: 967usize,
    description: "Values describing the shape or type of the container.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-47",
    hl7_version: "2.9",
};

pub static TABLE_0968_METADATA: V2MetadataTable = V2MetadataTable {
    table: 968usize,
    description: "Values describing the material a container is made of or indication that the container may be a virtual type.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-48",
    hl7_version: "2.9",
};

pub static TABLE_0969_METADATA: V2MetadataTable = V2MetadataTable {
    table: 969usize,
    description: "Codes created by an organization as a shorthand way to express a combination of container attributes.  No suggested values.",
    ttype: "User",
    steward: "OO",
    where_used: "SAC-49",
    hl7_version: "2.9",
};

pub static TABLE_0970_METADATA: V2MetadataTable = V2MetadataTable {
    table: 970usize,
    description: "Result Code of the online verification of insurance data",
    ttype: "User",
    steward: "FM",
    where_used: "IN3-29",
    hl7_version: "2.9",
};

pub static TABLE_0971_METADATA: V2MetadataTable = V2MetadataTable {
    table: 971usize,
    description: "Error Code of the online verification of insurance data",
    ttype: "User",
    steward: "FM",
    where_used: "IN3-30",
    hl7_version: "2.9",
};

pub static TABLE_9999_METADATA: V2MetadataTable = V2MetadataTable {
    table: 9999usize,
    description: "Used as a 'placeholder' in the several datatypes for any table (may be external, local, or may vary).  If the field is of data type CQ, CE, CF, CNE or CWE and one or more externally or locally defined tables may be used, the number 9999 will appear as a placeholder for the actual table in the segment table column. This is to indicate that table values are used, but no HL7/User-defined table has been allocated. The narrative may constrain which external tables can be used.",
    ttype: "HL7",
    steward: "INM",
    where_used: "CE, DF, CQ, CNE, CWE",
    hl7_version: "2.4        2.4",
};
