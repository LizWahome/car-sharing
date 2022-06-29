# Car-sharing app

## About

This is a car sharing smart contract that lets people rent cars for a short period of time.

## Declaration

HashMap is an important data structure which allows us to store data in key-value pairs. In Rust , HashMap store values by key.<br>
The `Contract` struct stores all the data required in the smart-contract using HashMaps and Vectors for the various data-types.

    pub struct Contract {
        hires: HashMap<u16, Hire>,
        rented: Vec<u16>,
        clients: HashMap<u16, Client>,
        responses: HashMap<u16, String>,
        cars: HashMap<u16, Car>,
        ids: u16    
    }

Here is where information per car hire will be stored which is client_id, car_id, period, start_time.

    pub struct Hire{
        .../ 
    }

In this part information about the client is stored here, which is their name and deposit which is in NEAR.

    pub struct Client {
        .../ 
    }

The car details are stored here which include price, car plates and type.

    pub struct Car {
        .../ 
    }

## Initialization

Initialize the default state of the smart-contract with the `Default` keyword.

    impl Default for Contract {
        fn default() -> Self {
            Self { 
                    .../ 
            }
        }
    }

## Implementation of the contract

In this section I defined all the implementations and methods that will be used on the contract.

    #[near_bindgen]
    impl Contract {
        ...//implementation here...
    }

## new car function

The client calls this function to request for a new car. Only the manager can add and update the car list as per predefined parameters.

    #[private]
        pub fn new_car(&mut self, price: u16, plates: String, types: String) {
            let new_car = Car {
                .../
            };

            self.cars.insert(self.ids, new_car);
            log!("the car id is : {}",self.ids);
            self.ids += 1;
        }

## new client function

This function calls for a new client where the clients inputs their name to create an account to be able to deposit some NEAR tokens for future transactions.

    pub fn new_client(&mut self, name: String) {
            let new_client = Client {
                .../
            };

            self.clients.insert(self.ids, new_client);
            log!("the client id is : {}",self.ids);
            self.ids += 1;
        }

## deposit function
This a payable function that allows client to deposit NEAR tokens that they will use to rent for a new car.

    #[payable]
        pub fn deposit(&mut self, id: u16) {
            let deposit = (env::attached_deposit() / 10u128.pow(24)) as u16;
            if let Some(client) = self.clients.get_mut(&id) {
                client.deposit += deposit
            }
        }

## view cars function

This function gives the client the oportunity to view the available cars in the list that can be rented out.

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

## view deposit function

The view_deposit function lets the client view their current deposit.

    pub fn view_deposit(&self, id: u16) -> u16 {
            self.clients[&id].deposit
        }

## Hire function

This is the main car renting method which creates a new hire as per the clients needs and preference. The car rented is removed from the available cars list and also the expected period a car is rented provided to ensure the clients deposit is sufficient.

    pub fn hire(&mut self, id: u16, types: String, period: u16){
        .../
    }

<details>
<summary>
    more details here

</summary>

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

</details>

## return car function

This method computes the time taken for a car hire and charges the client for that exact period. The time is obtained by leveraging on block_timestamp. The car is then added to the list of available cars for hire client.

    pub fn return_car(&mut self, id: u16,){
            let hire = &self.hires[&id];
            let time = env::block_timestamp() - hire.start_time;
            let cost =  time as f32/ (1000000000.0 * 60.0)* (self.cars[&hire.car_id].price)as f32;
            if let Some(client) = self.clients.get_mut(&hire.client_id) {
                client.deposit -= cost as u16;
            }
        }
