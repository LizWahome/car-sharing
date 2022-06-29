use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    hires: HashMap<u16, Hire>,
    rented: Vec<u16>,
    clients: HashMap<u16, Client>,
    responses: HashMap<u16, String>,
    cars: HashMap<u16, Car>,
    ids: u16
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Clone)]
pub struct Hire{
    client_id: u16,
    car_id: u16,
    period: u16,
    start_time: u64,
    // end_time: u64,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Clone)]
pub struct Client {
    name: String,
    deposit: u16,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Clone)]
pub struct Car {
    price: u16,
    plates: String,
    types: String
}

impl Default for Contract {
    fn default() -> Self {
        Self { 
            hires: HashMap::new(),
            rented: Vec::new(),
            clients:  HashMap::new(), 
            responses:  HashMap::new(), 
            cars:  HashMap::new(),
            ids: 1,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn new_car(&mut self, price: u16, plates: String, types: String) {
        let new_car = Car {
            price,
            plates,
            types,
        };

        self.cars.insert(self.ids, new_car);
        log!("the car id is : {}",self.ids);
        self.ids += 1;
    }

    pub fn new_client(&mut self, name: String) {
        let new_client = Client {
            name,
            deposit: 0
        };

        self.clients.insert(self.ids, new_client);
        log!("the client id is : {}",self.ids);
        self.ids += 1;
    }

    #[payable]
    pub fn deposit(&mut self, id: u16) {
        let deposit = (env::attached_deposit() / 10u128.pow(24)) as u16;
        if let Some(client) = self.clients.get_mut(&id) {
            client.deposit += deposit
        }
    }

    pub fn view_cars(&self) -> Vec<String>{
        let mut cars: Vec<String> = vec![];
        for car in &self.cars {
            if !self.rented.contains(&car.0) {
                cars.push(car.1.types.clone());
                cars.push(car.1.price.to_string());
            }
        }
        cars
    }

    pub fn view_deposit(&self, id: u16) -> u16 {
        self.clients[&id].deposit
    }

    pub fn hire(&mut self, id: u16, types: String, period: u16){
        let mut car_id = match self.cars.keys().next() {
            Some(&x) => x as u16,
            None => 1,
        };
        for car in &self.cars {
            if car.1.types == types {
                car_id = *car.0;
            }
            else {
                log!("Your choice is not available currently")
            }
        }
        if self.cars[&car_id].price * period > self.clients[&id].deposit {
            log!("Sorry your deposit is low");
            return ;
        }
        log!("intial block_timestamp {}",env::block_timestamp());
        let new_hire = Hire {
            client_id: id,
            car_id,
            period,
            start_time: env::block_timestamp(),
            // end_time: 0,
        };

        self.hires.insert(self.ids, new_hire);
        self.rented.push(car_id);
        log!("the hire id is : {}",self.ids);
        self.ids += 1;
    }

    pub fn return_car(&mut self, id: u16,){
        let hire = &self.hires[&id];
        // let cost = self.cars[&hire.car_id].price * hire.period;
        let time = env::block_timestamp() - hire.start_time;
        log!("time taken is: {} hours", time / (1000000000 * 60));
        let cost =  time as f32/ (1000000000.0 * 60.0)* (self.cars[&hire.car_id].price)as f32;

        if let Some(client) = self.clients.get_mut(&hire.client_id) {
            client.deposit -= cost as u16;
        }
        self.rented.remove(self.rented.iter().position(|x| *x == hire.car_id).expect("not found"));
        log!("Succesfull");
    }

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};


    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "bussiness.testnet".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "client.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_new_car() {
        let mut contract = Contract::default();
        contract.new_car(2, "KDB 128j".to_string(), "nissan".to_string());
        assert_eq!(1, contract.cars.len())
    }

    #[test]
    fn test_new_client() {
        let mut contract = Contract::default();
        contract.new_client("michael jackson".to_string());
        assert_eq!(1, contract.clients.len());
    }

    #[test]
    fn test_deposit() {
        let mut context = get_context(vec![], false);
        context.attached_deposit = 10 * 10u128.pow(24);
        context.is_view = false;
        testing_env!(context);

        let mut contract = Contract::default();
        contract.new_client("michael jackson".to_string());
        contract.deposit(1);
        assert_eq!(10, contract.clients[&1].deposit)
    }

    #[test]
    fn test_new_hire() {
        let mut context = get_context(vec![], false);
        context.attached_deposit = 30 * 10u128.pow(24);
        context.is_view = false;
        testing_env!(context);

        let mut contract = Contract::default();
        contract.new_car(2, "KDB 128j".to_string(), "nissan".to_string());
        contract.new_client("michael jackson".to_string());
        contract.deposit(2);
        contract.hire(2, "nissan".to_string(), 12);
        assert_eq!(1, contract.hires.len());
    }

}
