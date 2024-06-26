use candid::{CandidType, Principal, Encode, Decode}; //serialization and deserialization data in ICP
use ic_cdk::{query, update}; //macros
use ic_stable_structures::Storable;
use serde::{Serialize, Deserialize}; //serializing and deserializing Rust data structure
// use std::cell::Ref;
use std::{borrow::Cow, cell::RefCell}; //interior mutability with runtime borrow checking
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap }; //defining and working with stable data structures
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager,VirtualMemory}; //stable memory management
use std::collections::HashMap; 
// use std::collections::BTreeMap;
use std::time::Duration; 

/**
* Memory Type Alias
* This type alias defines `Memory` as a `VirtualMemory` using the `DefaultMemoryImpl`.
* `VirtualMemory` is a structure that allows managing virtualized stable memory,
* and `DefaultMemoryImpl` is the default implementation for this virtual memory.
* @param None
* @return A type alias for `VirtualMemory` with `DefaultMemoryImpl`.
*/
pub type Memory = VirtualMemory<DefaultMemoryImpl>; 
// type IdCell = Cell<u64, Memory>; 

/**
* Farmer Struct
* Represents a Farmer and their associated details.
* This struct includes various fields related to the farmer's identity, farm details, investmentor, and loan information.
* Implements traits for serialization, deserialization, cloning, equality comparison, and hashing.
* @param Defined in-line
* @return Farmer struct with the specified fields.
*/
#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Farmer {
  pub id: u64, //Unique identifier for the farmer.
  pub principal_id: Principal, //Principal ID of the farmer.
  pub farmer_name: String, //Name of the farmer.
  pub farm_name: String, //Name of the farm.
  pub farm_description: String, //Description of the farm.
  pub farm_assets: Option<Vec<(String, (u64, u64))>>, // Maps supply item names to their quantities
  pub amount_invested: Option<u64>, // Amount Invested into the farm.
  pub investors_ids: Principal, //Principle IDs of Investors.
  pub verified: bool, //verification status.
  pub agri_business: String, //Type of Afribusiness.
  pub insured: Option<bool>, //Insurance Status.
  pub publish: bool, //Publication Status.
  pub ifarm_tokens: Option<u64>, //iFarm Tokens held.
  pub credit_score: Option<u64>, //Credit Score.
  pub current_loan_ask: Option<u64>,  //Loan Amount.
  pub loaned: bool, //Loan Status.
  pub loan_maturity: Option<Duration>, //Time to loan maurity.
  pub time_for_funding_round_to_expire: Option<Duration> //Time to loan expiry.
}


/**
* Default Implementation for Farmer [Constructor]
* Provides a default implementation for the Farmer struct.
* Sets default values for all fields.
* @param None
* @return Farmer instance with default values.
*/
impl Default for Farmer {
    fn default() -> Self {
        Self {
         id: 0, 
         principal_id: Principal::anonymous(),
         farmer_name: String::new(), 
         farm_name: String::new(), 
         farm_description: String::new(), 
         amount_invested: None, 
         farm_assets: None,
         investors_ids: Principal::anonymous(), 
         verified: false, 
         agri_business: String::new(), 
         insured: None,
         publish: false, 
         ifarm_tokens: None, 
         credit_score: None, 
         current_loan_ask: None, 
         loaned: false, 
         loan_maturity: None, 
         time_for_funding_round_to_expire: None
        }
    } 
}

/**
* NewFarmer Struct
* Represents the initial information required to create a new farmer.
* This struct includes basic details about the farmer and their farm.
* @param Defined Inline
* @return NewFarmer instance with the specified fields.
*/
#[derive(CandidType, Serialize, Deserialize)] 
pub struct NewFarmer {
    pub farmer_name: String, //Farmer Name
    pub farm_name: String, //Farm Name
    pub farm_description: String //Farm Description
}

/**
* Investor Struct
* Represents an investor with basic details.
* @param Defined In-Line
* @return Investor struct with the specified fields.
*/
#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct Investor {
    pub id: u64, //Unique identifier for the investor.
    name: String, //Name of the investor.
    pub verified: bool, //Indicates if the investor is verified.
    principal_id: Principal //Investor's principal ID.
}

/**
* Default Implementation for Investor [Constructor]
* Provides a default implementation for the Investor struct.
* Sets default values for all fields.
* @param None
* @return Investor instance with default values.
*/
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

/**
* NewInvestor Struct
* Represents the initial information required to create a new investor.
* @param Defined In-Line
* @return NewInvestor instance with the specified fields.
*/
#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct NewInvestor {
    name: String
}

/**
* SupplyAgriBusiness Struct
* Represents a supply-oriented agricultural business with details.
* @param Defined In-Line
* @return SupplyAgriBusiness instance
*/
#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct SupplyAgriBusiness {
    pub id: u64, //Unique identifier for the business.
    agribusiness_name: String, //Name of the agricultural business.
    items_to_be_supplied: Option<AgribusinessItemsToBeSupplied>, //Items planned to be supplied by the business
    //supplied_items: Option<SuppliedItems>, 
    pub verified: bool, //Indicates if the business is verified.
    principal_id: Principal //ID associated with the business's principal.
} 

/**
* Default Implementation for SupplyAgriBusiness [Constructor]
* Provides a default implementation for the SupplyAgriBusiness struct.
* Sets default values for all fields.
* @param None
* @return SupplyAgriBusiness instance with default values.
*/
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

/**
* NewSupplyAgriBusiness Struct
* Represents the initial information required to create a new supply agribusiness.
* @param Defined In-Line
* @return NewSupplyAgriBusiness instance with the specified fields.
*/
#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct NewSupplyAgriBusiness {
    agribusiness_name: String, //Name of the new agricultural business.
    items_to_be_supplied: Option<AgribusinessItemsToBeSupplied> //Items planned to be supplied by the business.
}

/**
* Type alias for items to be supplied by an agricultural business.
*/
type AgribusinessItemsToBeSupplied = Vec<(String, (u64, u64))>; 

/**
* SuppliedItems Struct
* Represents items supplied by an agricultural business.
* @param Defined In-Line
*/
#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct SuppliedItems {
   principal_id: Principal, //ID associated with the principal of the item.
   item_name: String, //Name of the item supplied.
   amount: u64, //Amount of the item supplied.
   price: u64 // Price in I-Farm Tokens
}


/**
* OrderStatus
* Enum for the status of an order.
*/
#[derive(Default, Debug, Serialize, Deserialize)]
pub enum OrderStatus {
    #[default]
    Pending,
    Complete,
    Cancelled,
}

/**
* Order Struct
* Represents a Order to the supply agribusiness.
* @param Defined In-Line
* @return Order instance
*/
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Order {
    pub principal_id: Principal,
    pub order_id: u64,
    pub farmer_id: u64,
    pub supply_agribusiness_id: u64,
    pub items: HashMap<String, (u64,u64)>, // item_name -> amount
    pub total_price: u64,
    pub status: bool
}

/**
* Default Implementation for Order [Constructor]
* Provides a default implementation for the Order struct.
* Sets default values for all fields.
* @param None
* @return Order instance with default values.
*/
impl Default for Order {
    fn default() -> Self {
        Order {
            principal_id: Principal::anonymous(),
            order_id: 0,
            farmer_id: 0,
            supply_agribusiness_id: 0,
            items: HashMap::new(),
            total_price: 0,
            status: false
        }
    }
}

/**
* NewOrder Struct
* Represents the initial information required to create a new order.
* @param Defined In-Line
* @return NewOrder instance with the specified fields.
*/
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct NewOrder {
    pub farmer_id: u64,
    pub supply_agribusiness_id: u64,
    pub items: HashMap<String, (u64,u64)>, // item_name -> amount
    pub total_price:  u64
}


/**
* FarmsAgriBusiness Struct
* Represents a farms-oriented agricultural business with details.
* @param Defined In-Line
* @return FarmsAgriBusiness instance
*/
#[derive(CandidType, Serialize, Deserialize, Clone)]  
pub struct FarmsAgriBusiness {
    pub id: u64, //Unique identifier for the farms agribusiness.
    pub agribusiness_name: String, // Name of the farms agribusiness.
    pub total_farmers: u64, //Total number of the farmers associated.
    pub principal_id: Principal, //Farms agribusiness principle ID.
    pub verified: bool, 
    // pub farms: Option<FarmsForAgriBusiness>
}

/**
* Default Implementation for FarmsAgriBusiness [Constructor]
* Provides a default implementation for the FarmsAgriBusiness struct.
* Sets default values for all fields.
* @param None
* @return Default FarmsAgriBusiness instance
*/
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

/**
* NewFarmsAgriBusiness Struct
* Represents the initial information required to create a new farms agribusiness.
* @param Defined In-Line
* @return NewFarmsAgriBusiness instance with the specified fields.
*/
#[derive(CandidType, Serialize, Deserialize, Clone)] 
pub struct NewFarmsAgriBusiness {
    agribusiness_name: String, // Name of the farms agribusiness
    total_farmers: u64, // Total number of farmers assciated.
    // farms: Option<FarmsForAgriBusiness>
}

// type FarmsForAgriBusiness = HashMap<Farmer, u64>; 
// pub type FarmsForAgriBusiness = BTreeMap<u64, Farmer>; 

/**
* Implementation of Storable for Farmer
* ICP architecture requirement: data is serialized before storage in stable memory or shared across canisters
* Provides methods to convert a `Farmer` instance to bytes and to create a `Farmer` instance from bytes.
* This is used for storing and retrieving `Farmer` instances in stable storage.

* to_bytes Method
* @params Defined In-line
* @return A Cow (Clone on Write) containing the byte representation of the `Farmer` instance.
*
* from_bytes Method
* @param bytes: Defined In-line
* @return Farmer instance created from the byte representation.
*/
impl Storable for Farmer {
    //Converts a Farmer instance to a byte representation.
    fn to_bytes(&self) -> Cow<[u8]> { //&self: Reference to the `Farmer` instance.
        Cow::Owned(Encode!(self).unwrap())//Cow (Clone on Write) containing the byte representation of the `Farmer` instance.
    }     
    //Creates a `Farmer` instance from a byte representation.
    fn from_bytes(bytes: Cow<[u8]>) -> Self { //Cow (Clone on Write) containing the byte representation of a `Farmer`.
        Decode!(bytes.as_ref(), Self).unwrap()//Farmer instance created from the byte representation.
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

impl Storable for Order {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }     
 
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }       
}

/**
* Implementation of BoundedStorable for Farmer
* Specifies the storage constraints for the Farmer struct when used in a stable data structure.
* Provides information on the maximum size and whether the size is fixed.
*/
impl BoundedStorable for Farmer {
    const MAX_SIZE: u32 = 1024; //maximum size (in bytes) that a Farmer instance can occupy.
    const IS_FIXED_SIZE: bool = false;// State of whether the size of a Farmer Struct is fixed.
}

/**
* Implementation of BoundedStorable for Investor
* Specifies the storage constraints for the Investor struct when used in a stable data structure.
* Provides information on the maximum size and whether the size is fixed.
*/
impl BoundedStorable for Investor {
    const MAX_SIZE: u32 = 1024; // State of whether the size of an Investor Struct is fixed.
    const IS_FIXED_SIZE: bool = false; //maximum size (in bytes) that an Investor instance can occupy.
}

/**
* Implementation of BoundedStorable for SupplyAgriBusiness
* Specifies the storage constraints for the SupplyAgriBusiness struct when used in a stable data structure.
* Provides information on the maximum size and whether the size is fixed.
*/
impl BoundedStorable for SupplyAgriBusiness {
    const MAX_SIZE: u32 = 1024; // State of whether the size of a FaSupplyAgriBusinessrmer Struct is fixed.
    const IS_FIXED_SIZE: bool = false; //maximum size (in bytes) that a SupplyAgriBusiness instance can occupy.
}

/**
* Implementation of BoundedStorable for FarmsAgriBusiness
* Specifies the storage constraints for the FarmsAgriBusiness struct when used in a stable data structure.
* Provides information on the maximum size and whether the size is fixed.
*/
impl BoundedStorable for FarmsAgriBusiness {
    const MAX_SIZE: u32 = 1024; // State of whether the size of a FarmsAgriBusiness Struct is fixed.
    const IS_FIXED_SIZE: bool = false; //maximum size (in bytes) that a FarmsAgriBusiness instance can occupy.
}

/**
* Implementation of BoundedStorable for Order 
* Specifies the storage constraints for the Order struct when used in a stable data structure.
* Provides information on the maximum size and whether the size is fixed.
*/
impl BoundedStorable for Order {
    const MAX_SIZE: u32 = 1024; // State of whether the size of an Order Struct is fixed.
    const IS_FIXED_SIZE: bool = false; //maximum size (in bytes) that an Order instance can occupy.
}


// Thread Local will allow us to achieve interior mutability, a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
thread_local! {

    /**
    * MEMORY_MANAGER
    * Manages stable memory allocations using a MemoryManager with the `DefaultMemoryImpl`.
    * Uses `RefCell` to allow mutable access.
    * @param None
    * @return A thread-local `RefCell` containing the MemoryManager initialized with `DefaultMemoryImpl`.
    */
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    ); 

    /**
    * FARMER_STORAGE
    * Stores Farmer instances in a `StableBTreeMap` using memory managed by the MEMORY_MANAGER.
    * Uses `RefCell` to allow mutable access.
    * @param None
    * @return A thread-local `RefCell` containing the `StableBTreeMap` for the Farmer instances.
    */
    pub static FARMER_STORAGE: RefCell<StableBTreeMap<u64, Farmer, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    )); 

    /**
    * INVESTOR_STORAGE
    * Stores Investor instances in a `StableBTreeMap` using memory managed by the MEMORY_MANAGER.
    * Uses `RefCell` to allow mutable access.
    * @param None
    * @return A thread-local `RefCell` containing the `StableBTreeMap` for the Investor instances.
    */
    pub static INVESTOR_STORAGE: RefCell<StableBTreeMap<u64, Investor, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    )); 

     /**
    * SUPPLY_AGRIBUSINESS_STORAGE
    * Stores SupplyAgriBusiness instances in a `StableBTreeMap` using memory managed by the MEMORY_MANAGER.
    * Uses `RefCell` to allow mutable access.
    * @param None
    * @return A thread-local `RefCell` containing the `StableBTreeMap` for the SupplyAgriBusiness instances.
    */
    pub static SUPPLY_AGRIBUSINESS_STORAGE: RefCell<StableBTreeMap<u64, SupplyAgriBusiness, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    )); 

    /**
    * FARMS_AGRIBUSINESS_STORAGE
    * Stores FarmsAgriBusiness instances in a `StableBTreeMap` using memory managed by the MEMORY_MANAGER.
    * Uses `RefCell` to allow mutable access.
    *@param None
    *@return A thread-local `RefCell` containing the `StableBTreeMap` for the FarmsAgriBusiness instances.
    */
    pub static FARMS_AGRIBUSINESS_STORAGE: RefCell<StableBTreeMap<u64, FarmsAgriBusiness, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    )); 

    /**
    * FARMS_FOR_AGRIBUSINESS_STORAGE
    * Stores Farmer instances related to an agribusiness in a `StableBTreeMap` using memory managed by  the MEMORY_MANAGER.
    * Uses `RefCell` to allow mutable access.
    * @param None
    * @return A thread-local `RefCell` containing the `StableBTreeMap` for Farmer instances related to agribusiness.
    */
    pub static FARMS_FOR_AGRIBUSINESS_STORAGE: RefCell<StableBTreeMap<u64, Farmer, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    )); 

    /**
    * ORDER_STORAGE
    * Stores Order instances in a `StableBTreeMap` using memory managed by the MEMORY_MANAGER.
    * Uses `RefCell` to allow mutable access.
    *@param None
    *@return A thread-local `RefCell` containing the `StableBTreeMap` for the Order instances.
    */
    pub static ORDER_STORAGE: RefCell<StableBTreeMap<u64, Order, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    )); 
    
    //Stores the current Farmer ID.
    pub static FARMER_ID: RefCell<u64> = RefCell::new(0);

    //Stores the current Investor ID.
    static INVESTOR_ID: RefCell<u64> = RefCell::new(1);

    //Stores the current Supply Agribusiness ID.
    static SUPPLY_AGRIBUSINESS_ID: RefCell<u64> = RefCell::new(2);

    //Stores the current Farms Agribusiness ID.
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

// Success Messages
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
  ReportUploadedSuccesfully { msg: String }, 
  CreditScoreAdded { msg: String }, 
  AppliedForLoanSuccesfully { msg: String }
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

/** Login function
* Function: who_am_i
* Description: Retrieves the principal ID of the caller.
* @param None
* @return Principal - Principal ID of the caller
*/
#[update] 
pub fn who_am_i() -> Principal {
    let caller = ic_cdk::caller(); 
    return caller; 
}

/**
* Function: register_farm
* Description: Registers a new farm with the specified details.
* @param new_farmer: NewFarmer - Structure containing new farmer details
* @return Result<Success, Error> - Success message if farm registration is successful, or an error message otherwise
*/
pub fn register_farm(new_farmer: NewFarmer) -> Result<Success, Error>{
    // Validate that all required fields are filled
   if new_farmer.farmer_name.is_empty() || new_farmer.farm_name.is_empty() || new_farmer.farm_description.is_empty() {
      return Err(Error::FieldEmpty { msg: format!("Kindly ensure all required fieilds are filled!") })
   } 

   /* Checking whether the farm name is taken (This code doesn't work)
   *let mut is_farm_name_taken = false;
   *
   *REGISTERED_FARMERS.with(|farmers| {
   *     if farmers.borrow().contains_key(&new_farmer.farm_name) {
   *         is_farm_name_taken = true;
   *     }
   *});
   *
   *if is_farm_name_taken {
   *   return Err(Error::FarmNameTaken { msg: format!("The farm name '{}' is already taken!", new_farmer.farm_name) });
   *}
   */

   // Checking whether the farm name is taken
   let farm_name = &new_farmer.farm_name;
   REGISTERED_FARMERS.with(|farmers| {
        if farmers.borrow().contains_key(farm_name) {
            return Err(Error::FarmNameTaken {
                msg: format!("The farm name '{}' is already taken!", farm_name),
            });
        }
        Ok(()) // or continue with your logic
    });

   // Check if principal ID is already registered 
   let new_farmer_principal_id = ic_cdk::caller(); 
    _is_principal_id_registered(new_farmer_principal_id)?;

   //Increment the farmer ID
   let id = FARMER_ID.with(|id| _increament_id(id)); 

   // Create a new farmer instance
   let farmer =  Farmer {
       id, 
       principal_id: new_farmer_principal_id, 
       farm_name: new_farmer.farmer_name.clone(), 
       farmer_name: new_farmer.farm_name.clone(), 
       farm_description: new_farmer.farm_description, 
       farm_assets: None,
       amount_invested: None, 
       investors_ids: Principal::anonymous(), 
       verified: false, 
       agri_business: String::new(), 
       insured: None, 
       publish: true, 
       ifarm_tokens: None, 
       credit_score: None, 
       current_loan_ask: None, 
       loaned: false, 
       loan_maturity: None, 
       time_for_funding_round_to_expire: None
   }; 

   //Is this cloning necessary. Seems expensive.
        //let farmer_clone1 = farmer.clone();
        //let farmer_clone2 = farmer.clone(); 

   // Mapping farmer name
        //REGISTERED_FARMERS.with(|farmers| {
        //    farmers.borrow_mut().insert(farmer.farm_name.clone(), farmer_clone1)
        //}); 

        //FARMER_STORAGE.with(|farmers| {
        //    farmers.borrow_mut().insert(id, farmer_clone2)
        //}); 

    //Suggestion: Insert Farmer Directly
    REGISTERED_FARMERS.with(|farmers| {
        farmers.borrow_mut().insert(new_farmer.farm_name.clone(), farmer);
    });

    // Store the farmer instance
    FARMER_STORAGE.with(|farmers| {
        farmers.borrow_mut().insert(id, farmer);
    });

   Ok(Success::FarmCreatedSuccesfully { msg: format!("Farm has been created succesfully") })
   
}

/**
* Function: _increament_id
* Description: Increments the provided ID and returns the new value.
* @param id: &RefCell<u64> - Reference to the ID to be incremented.
* @return u64 - New incremented ID value
*/
pub fn _increament_id(id: &RefCell<u64>) -> u64 {// is this more secure, I'd suggest randomization.
    let mut id_borrowed = id.borrow_mut();
    let new_id = *id_borrowed + 1;
    *id_borrowed = new_id;
    new_id
}

/**
* Function: _is_principal_id_registered
* Description: Checks if the provided principal ID is already registered in any of the user categories.
* @param new_principal_id: Principal - Principal ID to be checked
* @return Result<(), Error> - Returns Ok(()) if ID is not registered, otherwise returns an Error with a message
*/
pub fn _is_principal_id_registered(new_principal_id: Principal) -> Result<(), Error> {
    let mut is_principal_id_registered = false; 

    // Check if the principal ID is already registered in farmers
    REGISTERED_FARMERS.with(|farmers| {
        for farmer in farmers.borrow().values() {
            if farmer.principal_id == new_principal_id {
                is_principal_id_registered = true;
                break;
            }
        }
    }); 

    // Check if the principal ID is already registered in investors
    REGISTERED_INVESTORS.with(|investors| {
        for investor in investors.borrow().values() {
            if investor.principal_id == new_principal_id {
                is_principal_id_registered = true; 
                break; 
            }
        }
    }); 

    // Check if the principal ID is already registered in supply agribusiness
    REGISTERED_SUPPLY_AGRIBUSINESS.with(|agribusiness| {
        for agribiz in agribusiness.borrow().values() {
            if agribiz.principal_id == new_principal_id {
                is_principal_id_registered = true; 
                break; 
            }
        }
    }); 

    // Check if the principal ID is already registered in farms agribusiness
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

/**
* Function: register_investor
* Description: Registers a new investor with the specified details.
* @param new_investor: NewInvestor - Structure containing new investor details
* @return Result<Success, Error> - Success message if investor registration is successful, or an error message otherwise
*/
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

   //let investor_clone1 = investor.clone();
   //let investor_clone2 = investor.clone(); 

   // Mapping investor name
   //REGISTERED_INVESTORS.with(|investors| {
    //investors.borrow_mut().insert(investor.name, investor_clone1)
   //}); 

   //INVESTOR_STORAGE.with(|investors| {
   //   investors.borrow_mut().insert(id, investor_clone2)
   //}); 

   //suggestion
   // Map investor name
    REGISTERED_INVESTORS.with(|investors| {
        investors.borrow_mut().insert(investor.name.clone(), investor);
    });

    // Store the investor instance
    INVESTOR_STORAGE.with(|investors| {
        investors.borrow_mut().insert(id, investor);
    });


   Ok(Success::InvestorRegisteredSuccesfully { msg: format!("Investor has been registered succesfully") })
}

/**
* Function: register_supply_agribusiness
* Description: Registers a new supply agribusiness with the specified details.
* @param new_supply_agribusiness: NewSupplyAgriBusiness - Structure containing new supply agribusiness details
* @return Result<Success, Error> - Success message if supply agribusiness registration is successful, or an error message otherwise
*/
pub fn register_supply_agribusiness(new_supply_agribusiness: NewSupplyAgriBusiness) -> Result<Success, Error> {
    // Validate that the supply agribusiness name field is filled
    if new_supply_agribusiness.agribusiness_name.is_empty() {
        return Err(Error::FieldEmpty { msg: format!("Kindly fill in supply agri business name!") }); 
    } 

    // Check whether principal ID is already registered 
    let new_supply_agribusiness_principal_id = ic_cdk::caller(); 

    let result = _is_principal_id_registered(new_supply_agribusiness_principal_id); 
    if let Err(e) = result {
        return Err(e); 
    }

    // Increamenting the supply agribusiness ID 
    let id = SUPPLY_AGRIBUSINESS_ID.with(|id| _increament_id(id)); 

    let supply_agri_business = SupplyAgriBusiness {
        id: 0, 
        agribusiness_name: new_supply_agribusiness.agribusiness_name, 
        items_to_be_supplied: new_supply_agribusiness.items_to_be_supplied, 
        //supplied_items: SuppliedItems, 
        verified: false, 
        principal_id: new_supply_agribusiness_principal_id
    }; 

    //let supply_agri_business_clone1 = supply_agri_business.clone(); 
    //let supply_agri_business_clone2 = supply_agri_business.clone(); 

    // Mapping the agri business name 
    //REGISTERED_SUPPLY_AGRIBUSINESS.with(|agribusiness| {
    //    agribusiness.borrow_mut().insert(supply_agri_business.agribusiness_name, supply_agri_business_clone1)
    //}); 

    //SUPPLY_AGRIBUSINESS_STORAGE.with(|supplyagribusiness| {
    //    supplyagribusiness.borrow_mut().insert(id, supply_agri_business_clone2)
    //}); 

    //suggestion
    // Map the agribusiness name
    REGISTERED_SUPPLY_AGRIBUSINESS.with(|agribusiness| {
        agribusiness.borrow_mut().insert(supply_agri_business.agribusiness_name.clone(), supply_agri_business);
    });

    // Store the supply agribusiness instance
    SUPPLY_AGRIBUSINESS_STORAGE.with(|supplyagribusiness| {
        supplyagribusiness.borrow_mut().insert(id, supply_agri_business);
    });

    Ok(Success::SupplyAgriBizRegisteredSuccesfully { msg: format!("Supply Agri Business has been registered succesfully") })
}


/**
* Function: register_farms_agribusiness
* Description: Registers a new farms agribusiness with the specified details.
* @param new_farms_agribusiness: NewFarmsAgriBusiness - Structure containing new farms agribusiness details
* @return Result<Success, Error> - Success message if farms agribusiness registration is successful, or an error message otherwise
*/ 
pub fn register_farms_agribusiness(new_farms_agribusiness: NewFarmsAgriBusiness) -> Result<Success, Error> {
    // Validate that the farms agribusiness name field is filled
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
        id, 
        agribusiness_name: new_farms_agribusiness.agribusiness_name,  
        verified: false, 
        principal_id: new_farms_agribusiness_principal_id, 
        total_farmers: new_farms_agribusiness.total_farmers, 
        // farms: new_farms_agribusiness.farms
    }; 

    let farms_agri_business_clone1 = farms_agri_business.clone(); 
    let farms_agri_business_clone2 = farms_agri_business.clone(); 

    //Mapping the agri business name 
    REGISTERED_FARMS_AGRIBUSINESS.with(|agribusiness| {
        agribusiness.borrow_mut().insert(farms_agri_business.agribusiness_name, farms_agri_business_clone1)
    }); 

    FARMS_AGRIBUSINESS_STORAGE.with(|supplyagribusiness| {
        supplyagribusiness.borrow_mut().insert(id, farms_agri_business_clone2)
    });

    Ok(Success::SupplyAgriBizRegisteredSuccesfully { msg: format!("Supply Agri Business has been registered succesfully") })
}

/**
* add_supply_items
* Adds supply items to a supply agribusiness if empty.
* @param supply_agribusiness_id: u64, items: Vec<SupplyItem>
* @return type: Result<Success, Error>
*/
pub fn add_supply_items(supply_agribusiness_id: u64, items: Vec<(String, (u64, u64))>) -> Result<Success, Error> {
    SUPPLY_AGRIBUSINESS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        
        // Use entry() method to access or insert into the BTreeMap
        match storage.get_mut().entry(supply_agribusiness_id) {
            // If the entry exists
            Some(entry) => {
                let supply_agribusiness = entry;
                
                // Check if items is empty
                if supply_agribusiness.items.is_empty() {
                    supply_agribusiness.items = items;
                    return Ok(Success::ItemsAdded { msg: "Supply items added successfully.".to_string() });
                } else {
                    return Err(Error::ItemsNotEmpty { msg: "Supply items already exist.".to_string() });
                }
            }
            // If the entry does not exist
            None => {
                Err(Error::AgribusinessNotFound { msg: "Supply agribusiness not found.".to_string() })
            }
        }
    })
}

/**
* Function: return_farmers
* Description: Returns a vector of all registered farmers.
* @param None
* @return Vec<Farmer> - Vector containing all registered farmers
*/
pub fn return_farmers() -> Vec<Farmer> {
    FARMER_STORAGE.with(|farmer| farmer.borrow().iter().map(|(_, item) | item.clone()).collect())
}

/**
* Function: return_investors
* Description: Returns a vector of all registered investors.
* @param None
* @return Vec<Investor> - Vector containing all registered investors
*/ 
pub fn return_investors() -> Vec<Investor> {
    INVESTOR_STORAGE.with(|farmer| farmer.borrow().iter().map(|(_, item)| item.clone()).collect())
}

/**
* Function: return_supply_agribusiness
* Description: Returns a vector of all registered supply agribusinesses.
* @param None
* @return Vec<SupplyAgriBusiness> - Vector containing all registered supply agribusinesses
*/
pub fn return_supply_agribusiness() -> Vec<SupplyAgriBusiness> {
    SUPPLY_AGRIBUSINESS_STORAGE.with(|agribusiness| agribusiness.borrow().iter().map(|(_, item)| item.clone()).collect())
}

/**
* Function: return_farms_agribusiness
* Description: Returns a vector of all registered farms agribusinesses.
* @param None
* @return Vec<FarmsAgriBusiness> - Vector containing all registered farms agribusinesses
*/
pub fn return_farms_agribusiness() -> Vec<FarmsAgriBusiness> {
    FARMS_AGRIBUSINESS_STORAGE.with(|agribusiness| agribusiness.borrow().iter().map(|(_, item)| item.clone()).collect())
}

/**
* Function: log_in
* Description: Logs in the caller based on their principal ID, determining their registered role.
* @param None
* @return Result<Success, Error> - Success message if login is successful, or an error message otherwise
*/
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