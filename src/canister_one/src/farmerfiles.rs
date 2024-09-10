use candid::{CandidType, Decode, Encode};
use ic_cdk::{query, update};
use ic_stable_structures::{memory_manager::MemoryId, BoundedStorable, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell};

use crate::entitymanagement::{self, Error, Success};

#[derive(CandidType, Serialize, Deserialize, Default, Clone)]
pub struct FinancialReport {
    title: String,
    summary: String,
    highlights: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize, Default, Clone)]
pub struct FarmSection {
    title: String,
    content: Option<String>,
    items: Option<Vec<String>>,
}

#[derive(CandidType, Serialize, Deserialize, Default, Clone)]
pub struct FarmReport {
    title: String,
    sections: Vec<FarmSection>,
}

#[derive(CandidType, Serialize, Deserialize, Default, Clone)]
pub struct FarmerReport {
    embed_url: String,
    farmer_id: u64,
    farmer_name: String,
    file_name: String,
    financial: Vec<FinancialReport>,
    farm: Vec<FarmReport>,
}

struct FarmerReportVec(Vec<FarmerReport>);

impl Storable for FarmerReportVec {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        FarmerReportVec(Decode!(bytes.as_ref(), Vec<FarmerReport>).unwrap())
    }
}

impl From<Vec<FarmerReport>> for FarmerReportVec {
    fn from(vec: Vec<FarmerReport>) -> Self {
        FarmerReportVec(vec)
    }
}

impl From<FarmerReportVec> for Vec<FarmerReport> {
    fn from(vec: FarmerReportVec) -> Self {
        vec.0
    }
}

impl BoundedStorable for FarmerReportVec {
    const MAX_SIZE: u32 = 16_384; // Adjust this size based on your needs
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    pub static FARM_REPORTS: RefCell<StableBTreeMap<u64, FarmerReportVec, entitymanagement::Memory>> =
    RefCell::new(StableBTreeMap::init(
      entitymanagement::MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));
}

#[update]
fn upload_financial_report(
    farmer_id: u64,
    financial: Vec<FinancialReport>,
) -> Result<Success, Error> {
    FARM_REPORTS.with(|reports| {
        let mut reports = reports.borrow_mut();
        if let Some(mut report_vec) = reports.get(&farmer_id) {
            for report in report_vec.0.iter_mut() {
                report.financial.extend(financial.clone());
            }
            reports.insert(farmer_id, report_vec);
            Ok(Success::ReportUploadedSuccesfully {
                msg: format!("Financial report uploaded successfully"),
            })
        } else {
            Err(Error::Error {
                msg: format!("Farmer not found"),
            })
        }
    })
}

#[update]
fn upload_farm_report(
    farmer_id: u64,
    farm: Vec<FarmReport>,
) -> Result<Success, Error> {
    FARM_REPORTS.with(|reports| {
        let mut reports = reports.borrow_mut();
        if let Some(mut report_vec) = reports.get(&farmer_id) {
            for report in report_vec.0.iter_mut() {
                report.farm.extend(farm.clone());
            }
            reports.insert(farmer_id, report_vec);
            Ok(Success::ReportUploadedSuccesfully {
                msg: format!("Farm report uploaded successfully"),
            })
        } else {
            Err(Error::Error {
                msg: format!("Farmer not found"),
            })
        }
    })
}

#[update]
pub fn delete_farmer_report(farmer_id: u64, report_index: usize) -> Result<Success, Error> {
    FARM_REPORTS.with(|reports| {
        let mut reports = reports.borrow_mut();
        if let Some(report_vec) = reports.get(&farmer_id) {
            let mut updated_vec = report_vec.0.clone();
            if report_index < updated_vec.len() {
                updated_vec.remove(report_index);
                reports.insert(farmer_id, FarmerReportVec(updated_vec));
                return Ok(Success::ReportDeletedSuccessfully {
                    msg: format!("Report deleted successfully"),
                });
            }
        }
        Err(Error::Error {
            msg: format!("Report not found"),
        })
    })
}

#[query]
fn get_farmer_reports(farmer_id: u64) -> Option<Vec<FarmerReport>> {
    FARM_REPORTS.with(|reports| reports.borrow().get(&farmer_id).map(|v| v.0.clone()))
}