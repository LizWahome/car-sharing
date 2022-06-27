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
    // start_time: blockTimestamp(),
    // end_time: blockTimestamp(),
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
        self.clients[&id].deposit += deposit;
    }

    pub fn view_cars(&self) -> Vec<String>{
        let cars: Vec<String> = vec![];
        for car in self.cars {
            if !self.rented.contains(car[0]) {
                cars.add(car.types);
                cars.add(car.price.to_string());
            }
        }
        cars
    }

    pub fn hire(&mut self, id: u16, types: String, period: u16){
        let car_id = self.cars[0];
        for car in self.cars {
            if car[1].types == types {
                id = car[0];
            }
            else {
                log!("Your choice is not available currently")
            }
        }
        if self.cars[&car_id][1].price * period > self.clients[&id][1].deposit {
            log!("Sorry your deposit is low");
            return ;
        }
        let new_hire = Hire {
            client_id: id,
            car_id,
            period,
            // start_time: blockTimestamp(),
            // end_time: blockTimestamp(),
        };

        self.hires.insert(self.ids, new_hire);
        self.rented.add(car_id);
        self.ids += 1;
    }

    pub fn return_car(&mut self, id: u16,){
        let hire =self.hires[&id];
        let cost = self.cars[&hire.car_id][1].price * hire.period;
        self.clients[&hire.client_id][1].deposit -= cost;
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
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
