use candid::{CandidType, Principal, Encode, Decode}; 
use ic_stable_structures::Storable;
use serde::{Serialize, Deserialize}; 
use std::{borrow::Cow, cell::RefCell}; 
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap }; 
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager,VirtualMemory};
use std::collections::HashMap; 

type Memory = VirtualMemory<DefaultMemoryImpl>; 
// type IdCell = Cell<u64, Memory>; 

// Farmer Struct 
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Farmer {
  id: u64, 
  principal_id: Principal, 
  farmer_name: String, 
  farm_name: String, 
  farm_description: String, 
  amount_invested: u64, 
  investors_ids: Principal, 
  verified: bool, 
  agri_business: Option<String>, 
  insured: bool
}

impl Default for Farmer {
    fn default() -> Self {
        Self {
         id: 0, 
         principal_id: Principal::anonymous(),
         farmer_name: String::new(), 
         farm_name: String::new(), 
         farm_description: String::new(), 
         amount_invested: 0, 
         investors_ids: Principal::anonymous(), 
         verified: false, 
         agri_business: None, 
         insured: false
        }
    } 
}

// New farmer struct 
#[derive(CandidType, Serialize, Deserialize)] 
pub struct NewFarmer {
    farmer_name: String, 
    farm_name: String, 
    farm_description: String
}

// Necessary as Internet Computer's architecture requires data to be serialized before it can be stored in stable memory or sent across canisters
impl Storable for Farmer {
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

// Thread Local will allow us to achieve interior mutability, a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    ); 

    static FARMER_STORAGE: RefCell<StableBTreeMap<u64, Farmer, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    )); 
    
    static FARMER_ID: RefCell<u64> = RefCell::new(0);
    
    // Mapping farmers with their farm names: for ensuring there are no duplicate farm names
    static REGISTERED_FARMERS: RefCell<HashMap<String, Farmer>> = RefCell::new(HashMap::new());
}


// Success Message
#[derive(CandidType, Deserialize, Serialize)] 
pub enum Success {
  FarmCreatedSuccesfully { msg: String }, 
}

// Error Messages 
#[derive(CandidType, Deserialize, Serialize)] 
pub enum Error {
    FieldEmpty { msg: String }, 
    FarmNameTaken { msg: String }, 
    PrincipalIdAlreadyRegistered { msg: String }
}


// Fucntion for registering farm 
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
   let mut is_principal_id_registered = false;

   REGISTERED_FARMERS.with(|farmers| {
      for farmer in farmers.borrow().values() {
        if farmer.principal_id == new_farmer_principal_id {
            is_principal_id_registered = true;
            break;
        }
      }
   }); 

   if is_principal_id_registered {
    return Err(Error::PrincipalIdAlreadyRegistered { msg: format!("The principal id {} has already been registered!", new_farmer_principal_id) });
   }


   let id = _increament_farmer_id(); 

   let farmer =  Farmer {
       id, 
       principal_id: new_farmer_principal_id, 
       farm_name: new_farmer.farmer_name.clone(), 
       farmer_name: new_farmer.farm_name, 
       farm_description: new_farmer.farm_description, 
       amount_invested: 0, 
       investors_ids: Principal::anonymous(), 
       verified: false, 
       agri_business: None, 
       insured: false
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

fn _increament_farmer_id() -> u64 {
    FARMER_ID.with(|id| {
        let new_id = *id.borrow_mut() + 1;
        *id.borrow_mut() = new_id;
        new_id
    })
}


pub fn check_user_management() -> String {
    format!("Checking user management! It works")
}