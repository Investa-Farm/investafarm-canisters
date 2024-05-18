use candid::{CandidType, Principal, Encode, Decode}; 
use ic_cdk::{query, update};
use ic_stable_structures::Storable;
use serde::{Serialize, Deserialize}; 
// use std::cell::Ref;
use std::{borrow::Cow, cell::RefCell}; 
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap }; 
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager,VirtualMemory};
use std::collections::HashMap; 
// use std::collections::BTreeMap;

pub type Memory = VirtualMemory<DefaultMemoryImpl>; 
// type IdCell = Cell<u64, Memory>; 

// Farmer Struct 
#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Farmer {
  pub id: u64, 
  pub principal_id: Principal, 
  pub farmer_name: String, 
  pub farm_name: String, 
  pub farm_description: String, 
  pub amount_invested: Option<u64>, 
  pub investors_ids: Principal, 
  pub verified: bool, 
  pub agri_business: String, 
  pub insured: Option<bool>, 
  pub publish: bool
}

impl Default for Farmer {
    fn default() -> Self {
        Self {
         id: 0, 
         principal_id: Principal::anonymous(),
         farmer_name: String::new(), 
         farm_name: String::new(), 
         farm_description: String::new(), 
         amount_invested: None, 
         investors_ids: Principal::anonymous(), 
         verified: false, 
         agri_business: String::new(), 
         insured: None,
         publish: false 
        }
    } 
}

#[derive(CandidType, Serialize, Deserialize)] 
pub struct NewFarmer {
    pub farmer_name: String, 
    pub farm_name: String, 
    pub farm_description: String 
}

// Investor Struct 
#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct Investor {
    pub id: u64, 
    name: String, 
    pub verified: bool, 
    principal_id: Principal
}

impl Default for Investor {
   fn default() -> Self {
       Self {
        id: 0, 
        name: String::new(), 
        verified: false, 
        principal_id: Principal::anonymous()
       }
   }    
}

#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct NewInvestor {
    name: String
}

// Supply Agri Business Struct 
#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct SupplyAgriBusiness {
    pub id: u64, 
    agribusiness_name: String, 
    items_to_be_supplied: Option<AgribusinessItemsToBeSupplied>, 
    // supplied_items: SuppliedItems, 
    pub verified: bool, 
    principal_id: Principal    
} 

impl Default for SupplyAgriBusiness {
    fn default() -> Self {
        Self {
         id: 0, 
         agribusiness_name: String::new(), 
         items_to_be_supplied: None, 
         //supplied_items: SuppliedItems, 
         verified: false, 
         principal_id: Principal::anonymous()
        }
    }    
}

#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct NewSupplyAgriBusiness {
    agribusiness_name: String, 
    items_to_be_supplied: Option<AgribusinessItemsToBeSupplied> 
}

type AgribusinessItemsToBeSupplied = HashMap<String, u64>; 

#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct SuppliedItems {
   principal_id: Principal, 
   item_name: String, 
   amount: u64
}

// Farms Agri Business Struct 
#[derive(CandidType, Serialize, Deserialize, Clone)]  
pub struct FarmsAgriBusiness {
    pub id: u64, 
    pub agribusiness_name: String, 
    pub total_farmers: u64, 
    pub principal_id: Principal, 
    pub verified: bool, 
    // pub farms: Option<FarmsForAgriBusiness>
}

impl Default for FarmsAgriBusiness {
    fn default() -> Self {
        Self {
         id: 0, 
         agribusiness_name: String::new(), 
         total_farmers: 0, 
         verified: false, 
         principal_id: Principal::anonymous(), 
        //  farms: None
        }
    }    
}

#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct NewFarmsAgriBusiness {
    agribusiness_name: String, 
    total_farmers: u64, 
    // farms: Option<FarmsForAgriBusiness>
}

// type FarmsForAgriBusiness = HashMap<Farmer, u64>; 
// pub type FarmsForAgriBusiness = BTreeMap<u64, Farmer>; 

// Necessary as Internet Computer's architecture requires data to be serialized before it can be stored in stable memory or sent across canisters
impl Storable for Farmer {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }     
 
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }    
}

impl Storable for Investor {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }     
 
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }       
}

impl Storable for SupplyAgriBusiness {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }     
 
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }       
}

impl Storable for FarmsAgriBusiness {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }     
 
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }       
}

impl BoundedStorable for Farmer {
    const MAX_SIZE: u32 = 1024; 
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for Investor {
    const MAX_SIZE: u32 = 1024; 
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for SupplyAgriBusiness {
    const MAX_SIZE: u32 = 1024; 
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for FarmsAgriBusiness {
    const MAX_SIZE: u32 = 1024; 
    const IS_FIXED_SIZE: bool = false;
}

// Thread Local will allow us to achieve interior mutability, a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    ); 

    pub static FARMER_STORAGE: RefCell<StableBTreeMap<u64, Farmer, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    )); 

    pub static INVESTOR_STORAGE: RefCell<StableBTreeMap<u64, Investor, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    )); 

    pub static SUPPLY_AGRIBUSINESS_STORAGE: RefCell<StableBTreeMap<u64, SupplyAgriBusiness, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    )); 

    pub static FARMS_AGRIBUSINESS_STORAGE: RefCell<StableBTreeMap<u64, FarmsAgriBusiness, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    )); 

    pub static FARMS_FOR_AGRIBUSINESS_STORAGE: RefCell<StableBTreeMap<u64, Farmer, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    )); 
    
    pub static FARMER_ID: RefCell<u64> = RefCell::new(0);

    static INVESTOR_ID: RefCell<u64> = RefCell::new(1);

    static SUPPLY_AGRIBUSINESS_ID: RefCell<u64> = RefCell::new(2);

    static FARMS_AGRIBUSINESS_ID: RefCell<u64> = RefCell::new(3);
    
    // Mapping farmers with their farm names: for ensuring there are no duplicate farm names
    static REGISTERED_FARMERS: RefCell<HashMap<String, Farmer>> = RefCell::new(HashMap::new());

    // Mapping Investors with their investor names
    static REGISTERED_INVESTORS: RefCell<HashMap<String, Investor>> = RefCell::new(HashMap::new());

    // Mapping supply agri business with their names
    pub static REGISTERED_SUPPLY_AGRIBUSINESS: RefCell<HashMap<String, SupplyAgriBusiness>> = RefCell::new(HashMap::new());

    // Mapping farmer agri business with their names 
    pub static REGISTERED_FARMS_AGRIBUSINESS: RefCell<HashMap<String, FarmsAgriBusiness>> = RefCell::new(HashMap::new());
}

// Success Message
#[derive(CandidType, Deserialize, Serialize)] 
pub enum Success {
  FarmCreatedSuccesfully { msg: String }, 
  InvestorRegisteredSuccesfully { msg: String }, 
  SupplyAgriBizRegisteredSuccesfully { msg: String },
  FarmsAgriBizRegisteredSuccesfully { msg: String }, 
  FarmerLogInSuccesfull { msg: String }, 
  InvestorLogInSuccesfull { msg: String }, 
  SupplyAgriBusinessLogInSuccesfull { msg: String }, 
  FarmsAgriBusinessLogInSuccesfull { msg: String }, 
  FarmPublishedSuccesfully { msg: String }, 
  FarmDeletedSuccesfully { msg: String }, 
  ReportUploadedSuccesfully { msg: String }
}

// Error Messages 
#[derive(CandidType, Deserialize, Serialize)] 
pub enum Error {
    FieldEmpty { msg: String }, 
    FarmNameTaken { msg: String }, 
    PrincipalIdAlreadyRegistered { msg: String }, 
    YouAreNotRegistered { msg: String }, 
    NotAuthorized { msg: String }, 
    ErrorOccured { msg: String }, 
    Error { msg: String }
}

// Login function 
#[update] 
pub fn who_am_i() -> Principal {
    let caller = ic_cdk::caller(); 
    return caller; 
}

// FUNCTION FOR REGISTERING FARM
pub fn register_farm(new_farmer: NewFarmer) -> Result<Success, Error>{
   if new_farmer.farmer_name.is_empty() || new_farmer.farm_name.is_empty() || new_farmer.farm_description.is_empty() {
      return Err(Error::FieldEmpty { msg: format!("Kindly ensure all required fieilds are filled!") })
   } 

   // Checking whether the farm name is taken (This code doesn't work)
   let mut is_farm_name_taken = false;

   REGISTERED_FARMERS.with(|farmers| {
        if farmers.borrow().contains_key(&new_farmer.farm_name) {
            is_farm_name_taken = true;
        }
   });

   if is_farm_name_taken {
      return Err(Error::FarmNameTaken { msg: format!("The farm name '{}' is already taken!", new_farmer.farm_name) });
   }

   // Check if principal ID is already registered 
   let new_farmer_principal_id = ic_cdk::caller(); 

    _is_principal_id_registered(new_farmer_principal_id)?;


   let id = FARMER_ID.with(|id| _increament_id(id)); 

   let farmer =  Farmer {
       id, 
       principal_id: new_farmer_principal_id, 
       farm_name: new_farmer.farmer_name.clone(), 
       farmer_name: new_farmer.farm_name.clone(), 
       farm_description: new_farmer.farm_description, 
       amount_invested: None, 
       investors_ids: Principal::anonymous(), 
       verified: false, 
       agri_business: String::new(), 
       insured: None, 
       publish: true
   }; 

   let farmer_clone1 = farmer.clone();
   let farmer_clone2 = farmer.clone(); 

   // Mapping farmer name
   REGISTERED_FARMERS.with(|farmers| {
    farmers.borrow_mut().insert(farmer.farm_name.clone(), farmer_clone1)
   }); 

   FARMER_STORAGE.with(|farmers| {
      farmers.borrow_mut().insert(id, farmer_clone2)
   }); 

   Ok(Success::FarmCreatedSuccesfully { msg: format!("Farm has been created succesfully") })
   
}

pub fn _increament_id(id: &RefCell<u64>) -> u64 {
    let mut id_borrowed = id.borrow_mut();
    let new_id = *id_borrowed + 1;
    *id_borrowed = new_id;
    new_id
}

pub fn _is_principal_id_registered(new_principal_id: Principal) -> Result<(), Error> {
    let mut is_principal_id_registered = false; 

    REGISTERED_FARMERS.with(|farmers| {
        for farmer in farmers.borrow().values() {
            if farmer.principal_id == new_principal_id {
                is_principal_id_registered = true;
                break;
            }
        }
    }); 

    REGISTERED_INVESTORS.with(|investors| {
        for investor in investors.borrow().values() {
            if investor.principal_id == new_principal_id {
                is_principal_id_registered = true; 
                break; 
            }
        }
    }); 

    REGISTERED_SUPPLY_AGRIBUSINESS.with(|agribusiness| {
        for agribiz in agribusiness.borrow().values() {
            if agribiz.principal_id == new_principal_id {
                is_principal_id_registered = true; 
                break; 
            }
        }
    }); 

    REGISTERED_FARMS_AGRIBUSINESS.with(|agribusiness| {
        for agribiz in agribusiness.borrow().values() {
            if agribiz.principal_id == new_principal_id {
                is_principal_id_registered = true; 
                break; 
            }
        }
    }); 

    if is_principal_id_registered {
        return Err(Error::PrincipalIdAlreadyRegistered { msg: format!("The principal id {} has already been registered!", new_principal_id) });
    }

    Ok(())
}

// FUNCTION FOR REGISTERING INVESTOR
pub fn register_investor(new_investor: NewInvestor) -> Result<Success, Error> {

    if new_investor.name.is_empty() {
        return Err(Error::FieldEmpty { msg: format!("Kindly fill in your name!") });
    }

    // Checking whether the principal ID is already registered 
    let new_investor_principal_id = ic_cdk::caller(); 

    let result = _is_principal_id_registered(new_investor_principal_id); 
    if let Err(e) = result {
        return Err(e); 
    }

    // Increamenting the ID 
    let id = INVESTOR_ID.with(|id| _increament_id(id)); 

    let investor = Investor {
        id, 
        principal_id: new_investor_principal_id, 
        name: new_investor.name, 
        verified: false
    }; 

   let investor_clone1 = investor.clone();
   let investor_clone2 = investor.clone(); 

   // Mapping investor name
   REGISTERED_INVESTORS.with(|investors| {
    investors.borrow_mut().insert(investor.name, investor_clone1)
   }); 

   INVESTOR_STORAGE.with(|investors| {
      investors.borrow_mut().insert(id, investor_clone2)
   }); 

   Ok(Success::InvestorRegisteredSuccesfully { msg: format!("Investor has been registered succesfully") })
}

// FUNCTION FOR REGISTERING SUPPLY AGRI BUSINESS 
pub fn register_supply_agribusiness(new_supply_agribusiness: NewSupplyAgriBusiness) -> Result<Success, Error> {
    if new_supply_agribusiness.agribusiness_name.is_empty() {
        return Err(Error::FieldEmpty { msg: format!("Kindly fill in supply agri business name!") }); 
    } 

    // Check whether principal ID is already registered 
    let new_supply_agribusiness_principal_id = ic_cdk::caller(); 

    let result = _is_principal_id_registered(new_supply_agribusiness_principal_id); 
    if let Err(e) = result {
        return Err(e); 
    }

    // Increamenting the ID 
    let id = SUPPLY_AGRIBUSINESS_ID.with(|id| _increament_id(id)); 

    let supply_agri_business = SupplyAgriBusiness {
        id: 0, 
        agribusiness_name: new_supply_agribusiness.agribusiness_name, 
        items_to_be_supplied: new_supply_agribusiness.items_to_be_supplied, 
        //supplied_items: SuppliedItems, 
        verified: false, 
        principal_id: new_supply_agribusiness_principal_id
    }; 

    let supply_agri_business_clone1 = supply_agri_business.clone(); 
    let supply_agri_business_clone2 = supply_agri_business.clone(); 

    // Mapping the agri business name 
    REGISTERED_SUPPLY_AGRIBUSINESS.with(|agribusiness| {
        agribusiness.borrow_mut().insert(supply_agri_business.agribusiness_name, supply_agri_business_clone1)
    }); 

    SUPPLY_AGRIBUSINESS_STORAGE.with(|supplyagribusiness| {
        supplyagribusiness.borrow_mut().insert(id, supply_agri_business_clone2)
    }); 

    Ok(Success::SupplyAgriBizRegisteredSuccesfully { msg: format!("Supply Agri Business has been registered succesfully") })
}


// FUNCTION FOR REGISTERING FARMS AGRI BUSINESS 
pub fn register_farms_agribusiness(new_farms_agribusiness: NewFarmsAgriBusiness) -> Result<Success, Error> {
    if new_farms_agribusiness.agribusiness_name.is_empty() {
        return Err(Error::FieldEmpty { msg: format!("Kindly fill in all fields!") }); 
    } 

    // Check whether principal ID is already registered 
    let new_farms_agribusiness_principal_id = ic_cdk::caller(); 

    let result = _is_principal_id_registered(new_farms_agribusiness_principal_id); 
    if let Err(e) = result {
        return Err(e); 
    }

    // Increamenting the ID 
    let id = FARMS_AGRIBUSINESS_ID.with(|id| _increament_id(id)); 

    let farms_agri_business = FarmsAgriBusiness {
        id: 0, 
        agribusiness_name: new_farms_agribusiness.agribusiness_name,  
        verified: false, 
        principal_id: new_farms_agribusiness_principal_id, 
        total_farmers: new_farms_agribusiness.total_farmers, 
        // farms: new_farms_agribusiness.farms
    }; 

    let farms_agri_business_clone1 = farms_agri_business.clone(); 
    let farms_agri_business_clone2 = farms_agri_business.clone(); 

    // Mapping the agri business name 
    REGISTERED_FARMS_AGRIBUSINESS.with(|agribusiness| {
        agribusiness.borrow_mut().insert(farms_agri_business.agribusiness_name, farms_agri_business_clone1)
    }); 

    FARMS_AGRIBUSINESS_STORAGE.with(|supplyagribusiness| {
        supplyagribusiness.borrow_mut().insert(id, farms_agri_business_clone2)
    }); 

    Ok(Success::SupplyAgriBizRegisteredSuccesfully { msg: format!("Supply Agri Business has been registered succesfully") })
}

// DISPLAYING FARMERS 
pub fn return_farmers() -> Vec<Farmer> {
    FARMER_STORAGE.with(|farmer| farmer.borrow().iter().map(|(_, item) | item.clone()).collect())
}

// DISPLAYING INVESTORS 
pub fn return_investors() -> Vec<Investor> {
    INVESTOR_STORAGE.with(|farmer| farmer.borrow().iter().map(|(_, item)| item.clone()).collect())
}

// DISPLAYING SUPPLY AGRI BUSINESS
pub fn return_supply_agribusiness() -> Vec<SupplyAgriBusiness> {
    SUPPLY_AGRIBUSINESS_STORAGE.with(|agribusiness| agribusiness.borrow().iter().map(|(_, item)| item.clone()).collect())
}

// DISPLAYING FARMS AGRI BUSINESS
pub fn return_farms_agribusiness() -> Vec<FarmsAgriBusiness> {
    FARMS_AGRIBUSINESS_STORAGE.with(|agribusiness| agribusiness.borrow().iter().map(|(_, item)| item.clone()).collect())
}

// FUNCTION FOR LOGGIN INTO THE SITE
#[query]
pub fn log_in() -> Result<Success, Error> {
    let principal_id = ic_cdk::caller(); 

    let result = REGISTERED_FARMERS.with(|farmers| {
        for farmer in farmers.borrow().values() {
            if farmer.principal_id == principal_id {
                return Ok(Success::FarmerLogInSuccesfull { msg: format!("You've logged in as a farmer succesfully") });
            }
        }
        Err(Error::YouAreNotRegistered { msg: format!("You are not registered!") })
    }); 

    if let Ok(res) = result {
        return Ok(res);
    }

    let result = REGISTERED_INVESTORS.with(|investors| {
        for investor in investors.borrow().values() {
            if investor.principal_id == principal_id {
                return Ok(Success::InvestorLogInSuccesfull { msg: format!("You've logged in as an Investor succesfully") });
            }
        }
        Err(Error::YouAreNotRegistered { msg: format!("You are not registered!") })
    }); 

    if let Ok(res) = result {
        return Ok(res);
    }

    let result = REGISTERED_SUPPLY_AGRIBUSINESS.with(|agribusiness| {
        for agribiz in agribusiness.borrow().values() {
            if agribiz.principal_id == principal_id {
                return Ok(Success::SupplyAgriBizRegisteredSuccesfully { msg: format!("You've logged in as an Investor succesfully") });
            }
        }
        Err(Error::YouAreNotRegistered { msg: format!("You are not registered!") })
    }); 

    if let Ok(res) = result {
        return Ok(res);
    }

    let result = REGISTERED_FARMS_AGRIBUSINESS.with(|agribusiness| {
        for agribiz in agribusiness.borrow().values() {
            if agribiz.principal_id == principal_id {
                return Ok(Success::FarmsAgriBizRegisteredSuccesfully { msg: format!("You've logged in as an Investor succesfully") });
            }
        }
        Err(Error::YouAreNotRegistered { msg: format!("You are not registered!") })
    }); 
   
   result 
}