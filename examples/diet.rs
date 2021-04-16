//! Nutrition guidelines, based on USDA Dietary Guidelines for Americans, 2005
//! https://health.gov/sites/default/files/2020-01/DGA2005.pdf
//!
use std::collections::{hash_map::Entry, HashMap};

use good_lp::{
    constraint, variable, variables, Expression, Solution, SolverModel, StaticSolver,
    Variable,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Dish {
    Hamburger,
    Chicken,
    HotDog,
    Fries,
    Macaroni,
    Pizza,
    Salad,
    Milk,
    IceCream,
}

impl Dish {
    pub const FOODS: [Dish; 9] = [
        Self::Hamburger,
        Self::Chicken,
        Self::HotDog,
        Self::Fries,
        Self::Macaroni,
        Self::Pizza,
        Self::Salad,
        Self::Milk,
        Self::IceCream,
    ];
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Nutrient {
    Calories,
    Protein,
    Sodium,
    Fat,
}

impl Nutrient {
    pub const ALL: [Nutrient; 4] = [Self::Calories, Self::Protein, Self::Sodium, Self::Fat];
}

#[derive(Debug)]
pub enum Quantity {
    Min,
    Max,
    Value,
}

#[derive(Debug)]
pub struct FoodProperty {
    nutrient: Nutrient,
    volume: f64,
    level: Quantity,
}

#[derive(Debug)]
pub enum FoodCategory {
    Calories(f64, Quantity),
    Protein(f64, Quantity),
    Sodium(f64, Quantity),
    Fat(f64, Quantity),
}

#[derive(Debug)]
pub struct Guideline {
    pub limit: FoodProperty,
}

#[derive(Debug)]
pub struct FoodProperties {
    food: Dish,
    cost: f64,
    nutrients: Vec<FoodProperty>,
}

#[allow(clippy::unnecessary_wraps)]
fn solve_diet_example<S: StaticSolver>(
    solver: S,
    guidelines: &[Guideline],
    food_properties: &[FoodProperties],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut vars = variables!();

    // Free Variables
    let food_vars = Dish::FOODS
        .iter()
        .map(|f| {
            let f_var = vars.add(variable().min(0.0));
            (f, f_var)
        })
        .collect::<HashMap<&Dish, Variable>>();

    println!("{:?}", food_vars);

    // Food Cost Summation
    let objective: Expression = food_properties
        .iter()
        .map(|f| {
            let e: Expression = *food_vars.get(&f.food).expect("Unmapped food") * f.cost;
            e
        })
        .sum();

    // Define the Problem
    let mut p = vars.minimise(objective).using(solver);

    // Subject to
    let mut h: HashMap<&Nutrient, Vec<Expression>> = HashMap::new();
    for food in food_properties {
        let food_var = food_vars.get(&food.food).expect("Library test");
        for category in &food.nutrients {
            match h.entry(&category.nutrient) {
                Entry::Vacant(e) => {
                    e.insert(vec![*food_var * category.volume]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(*food_var * category.volume);
                }
            }
        }
    }

    println!("{:?}", h);

    for guide in guidelines {
        let food_sum: Expression = h
            .get(&guide.limit.nutrient)
            .expect("Library test")
            .iter()
            .sum();
        println!("Food {:?}", guide);
        println!("food sum {:?}", food_sum);
        match guide.limit.level {
            Quantity::Min => {
                p.add_constraint(constraint!(food_sum >= guide.limit.volume + 0.0001));
            }
            Quantity::Max => {
                p.add_constraint(constraint!(food_sum <= guide.limit.volume));
            }
            Quantity::Value => (),
        }
    }

    // Solve Problem
    let solution = p.solve().expect("Library test");
    for food in &Dish::FOODS {
        let dish_var = food_vars.get(food).expect("Library test");
        println!("Food {:?} Count {:?}", food, solution.value(*dish_var));
    }

    Ok(())
}

fn main() {
    let food_guidelines = vec![
        Guideline {
            limit: FoodProperty {
                nutrient: Nutrient::Calories,
                volume: 1800.,
                level: Quantity::Min,
            },
        },
        Guideline {
            limit: FoodProperty {
                nutrient: Nutrient::Calories,
                volume: 2200.,
                level: Quantity::Max,
            },
        },
        Guideline {
            limit: FoodProperty {
                nutrient: Nutrient::Protein,
                volume: 91.,
                level: Quantity::Min,
            },
        },
        Guideline {
            limit: FoodProperty {
                nutrient: Nutrient::Fat,
                volume: 0.,
                level: Quantity::Min,
            },
        },
        Guideline {
            limit: FoodProperty {
                nutrient: Nutrient::Calories,
                volume: 65.,
                level: Quantity::Max,
            },
        },
        Guideline {
            limit: FoodProperty {
                nutrient: Nutrient::Sodium,
                volume: 0.,
                level: Quantity::Min,
            },
        },
        Guideline {
            limit: FoodProperty {
                nutrient: Nutrient::Sodium,
                volume: 1779.,
                level: Quantity::Max,
            },
        },
    ];

    let food_properties = vec![
        FoodProperties {
            food: Dish::Hamburger,
            cost: 2.49,
            nutrients: vec![
                FoodProperty {
                    nutrient: Nutrient::Calories,
                    volume: 410.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Protein,
                    volume: 24.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Sodium,
                    volume: 730.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Fat,
                    volume: 26.,
                    level: Quantity::Value,
                },
            ],
        },
        FoodProperties {
            food: Dish::Chicken,
            cost: 2.89,
            nutrients: vec![
                FoodProperty {
                    nutrient: Nutrient::Calories,
                    volume: 420.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Protein,
                    volume: 32.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Sodium,
                    volume: 1190.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Fat,
                    volume: 10.,
                    level: Quantity::Value,
                },
            ],
        },
        FoodProperties {
            food: Dish::HotDog,
            cost: 1.5,
            nutrients: vec![
                FoodProperty {
                    nutrient: Nutrient::Calories,
                    volume: 560.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Protein,
                    volume: 20.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Sodium,
                    volume: 1800.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Fat,
                    volume: 32.,
                    level: Quantity::Value,
                },
            ],
        },
        FoodProperties {
            food: Dish::Fries,
            cost: 1.89,
            nutrients: vec![
                FoodProperty {
                    nutrient: Nutrient::Calories,
                    volume: 380.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Protein,
                    volume: 4.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Sodium,
                    volume: 270.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Fat,
                    volume: 19.,
                    level: Quantity::Value,
                },
            ],
        },
        FoodProperties {
            food: Dish::Macaroni,
            cost: 2.09,
            nutrients: vec![
                FoodProperty {
                    nutrient: Nutrient::Calories,
                    volume: 320.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Protein,
                    volume: 12.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Sodium,
                    volume: 930.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Fat,
                    volume: 10.,
                    level: Quantity::Value,
                },
            ],
        },
        FoodProperties {
            food: Dish::Pizza,
            cost: 1.99,
            nutrients: vec![
                FoodProperty {
                    nutrient: Nutrient::Calories,
                    volume: 320.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Protein,
                    volume: 15.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Sodium,
                    volume: 820.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Fat,
                    volume: 12.,
                    level: Quantity::Value,
                },
            ],
        },
        FoodProperties {
            food: Dish::Salad,
            cost: 2.49,
            nutrients: vec![
                FoodProperty {
                    nutrient: Nutrient::Calories,
                    volume: 320.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Protein,
                    volume: 31.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Sodium,
                    volume: 1230.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Fat,
                    volume: 12.,
                    level: Quantity::Value,
                },
            ],
        },
        FoodProperties {
            food: Dish::Milk,
            cost: 0.89,
            nutrients: vec![
                FoodProperty {
                    nutrient: Nutrient::Calories,
                    volume: 100.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Protein,
                    volume: 8.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Sodium,
                    volume: 125.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Fat,
                    volume: 2.5,
                    level: Quantity::Value,
                },
            ],
        },
        FoodProperties {
            food: Dish::IceCream,
            cost: 1.59,
            nutrients: vec![
                FoodProperty {
                    nutrient: Nutrient::Calories,
                    volume: 330.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Protein,
                    volume: 8.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Sodium,
                    volume: 180.,
                    level: Quantity::Value,
                },
                FoodProperty {
                    nutrient: Nutrient::Fat,
                    volume: 10.,
                    level: Quantity::Value,
                },
            ],
        },
    ];

    solve_diet_example(good_lp::default_solver, &food_guidelines, &food_properties).unwrap();
}
