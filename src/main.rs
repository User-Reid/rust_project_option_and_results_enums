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
            Err("Sorry, we have a mice problem".to_string())
        } else if address.is_empty() {
            Err("No delivery address specified".to_string())
        } else {
            Ok(Food {
                name: "Burger".to_string()
            })
        }


    }
}

fn main() {
    let chipotle: Restaurant = Restaurant {
        reservations: 11,
        has_mice_infestation: true
    };

    let salata: Restaurant = Restaurant {
        reservations: 15,
        has_mice_infestation: false
    };

    println!("{:?}", chipotle.chef_special());
    println!("{:?}", chipotle.deliver_burger("123 Elm Street"));
    println!("{:?}", salata.chef_special());
    println!("{:?}", salata.deliver_burger(""));
    println!("{:?}", salata.deliver_burger("asdfjkl;"));

}