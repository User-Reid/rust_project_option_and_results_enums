#[derive(Debug)]
struct Food {
    name: String
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
             return None;
        }

        if self.reservations < 12 {
             Some(Food {
                name: String::from("Uni Sashimi")
            })
        } else {
             Some(Food {
                name: String::from("Strip Steak")
            })
        }
    }
    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            Err(String::from("We got mice effin everywhere dog."))
        } else if address.is_empty() {
            Err(String::from("No delivery address specified"))
        } else {
            Ok(Food {
                name: String::from("Burger")
            })
        }




    }
}

fn main() {
    let taco_bell: Restaurant = Restaurant {
        reservations: 11,
        has_mice_infestation: true
    };

    let panda_express: Restaurant = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    println!("{:?}", taco_bell.chef_special());

    println!("{:?}", taco_bell.deliver_burger("123 Elm Street"));

    println!("{:?}", panda_express.chef_special());

    println!("{:?}", panda_express.deliver_burger(""));

    println!("{:?}", panda_express.deliver_burger("taco street"));


}