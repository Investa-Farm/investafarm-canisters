use candid::{self, CandidType, Deserialize};
use ic_cdk::{query, update};

#[derive(CandidType, Deserialize, Eq, PartialEq, Debug)]
pub struct SupportedStandard {
    pub url: String,
    pub name: String,
}

#[query]
fn icrc10_supported_standards() -> Vec<SupportedStandard> {
    vec![
        SupportedStandard {
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-10/ICRC-10.md".to_string(),
            name: "ICRC-10".to_string(),
        },
        SupportedStandard {
            url: "https://github.com/dfinity/wg-identity-authentication/blob/main/topics/icrc_28_trusted_origins.md".to_string(),
            name: "ICRC-28".to_string(),
        },
    ]
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Icrc28TrustedOriginsResponse {
    pub trusted_origins: Vec<String>
}

#[update]
fn icrc28_trusted_origins() -> Icrc28TrustedOriginsResponse {
    let trusted_origins = vec![
        String::from("http://localhost:3000"),
        String::from("https://b3aqn-zyaaa-aaaao-qa56q-cai.icp0.io/"),
        // String::from("https://your-frontend-canister-id.raw.icp0.io"),
        // String::from("https://your-frontend-canister-id.ic0.app"),
    ];

    Icrc28TrustedOriginsResponse { trusted_origins }
}