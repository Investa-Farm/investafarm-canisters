use candid::{CandidType, Principal, Encode, Decode}; 
use ic_stable_structures::Storable;
use serde::{Serialize, Deserialize}; 
// use std::cell::Ref;
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

// Investor Struct 
#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct Investor {
    id: u64, 
    name: String, 
    verified: bool, 
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

impl BoundedStorable for Farmer {
    const MAX_SIZE: u32 = 1024; 
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for Investor {
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

    static INVESTOR_STORAGE: RefCell<StableBTreeMap<u64, Investor, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    )); 
    
    static FARMER_ID: RefCell<u64> = RefCell::new(0);

    static INVESTOR_ID: RefCell<u64> = RefCell::new(0);
    
    // Mapping farmers with their farm names: for ensuring there are no duplicate farm names
    static REGISTERED_FARMERS: RefCell<HashMap<String, Farmer>> = RefCell::new(HashMap::new());

    // Mapping Investors with their investor names
    static REGISTERED_INVESTORS: RefCell<HashMap<String, Investor>> = RefCell::new(HashMap::new());
}


// Success Message
#[derive(CandidType, Deserialize, Serialize)] 
pub enum Success {
  FarmCreatedSuccesfully { msg: String }, 
  InvestorRegisteredSuccesfully { msg: String }
}

// Error Messages 
#[derive(CandidType, Deserialize, Serialize)] 
pub enum Error {
    FieldEmpty { msg: String }, 
    FarmNameTaken { msg: String }, 
    PrincipalIdAlreadyRegistered { msg: String }
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
//    let mut is_principal_id_registered = false;

//    REGISTERED_FARMERS.with(|farmers| {
//       for farmer in farmers.borrow().values() {
//         if farmer.principal_id == new_farmer_principal_id {
//             is_principal_id_registered = true;
//             break;
//         }
//       }
//    }); 

//    if is_principal_id_registered {
//     return Err(Error::PrincipalIdAlreadyRegistered { msg: format!("The principal id {} has already been registered!", new_farmer_principal_id) });
//    }

    _is_principal_id_registered(new_farmer_principal_id)?;


   let id = FARMER_ID.with(|id| _increament_id(id)); 

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

fn _increament_id(id: &RefCell<u64>) -> u64 {
    let mut id_borrowed = id.borrow_mut();
    let new_id = *id_borrowed + 1;
    *id_borrowed = new_id;
    new_id
}

fn _is_principal_id_registered(new_principal_id: Principal) -> Result<(), Error> {
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

   // Mapping farmer name
   REGISTERED_INVESTORS.with(|investors| {
    investors.borrow_mut().insert(investor.name, investor_clone1)
   }); 

   INVESTOR_STORAGE.with(|investors| {
      investors.borrow_mut().insert(id, investor_clone2)
   }); 

   Ok(Success::InvestorRegisteredSuccesfully { msg: format!("Investor has been registered succesfully") })

}
