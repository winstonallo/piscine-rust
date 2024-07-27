#[derive(Debug, PartialEq)]
pub enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}

impl PizzaStatus {
    pub fn from_delivery_time(ordered_days_ago: u32) -> Self {
        match ordered_days_ago {
            0..=1 => PizzaStatus::Ordered,
            2..=6 => PizzaStatus::Cooking,
            7..=9 => PizzaStatus::Cooked,
            10..=17 => PizzaStatus::Delivering,
            _ => PizzaStatus::Delivered,
        }
    }

    pub fn get_delivery_time_in_days(&self) -> u32 {
        match self {
            PizzaStatus::Ordered => 17,
            PizzaStatus::Cooking => 15,
            PizzaStatus::Cooked => 10,
            PizzaStatus::Delivering => 7,
            PizzaStatus::Delivered => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_delivery_time() {
        let result = PizzaStatus::from_delivery_time(10);
        assert_eq!(result, PizzaStatus::Delivering);
    }

    #[test]
    fn get_delivery_time_in_days() {
        let result = PizzaStatus::get_delivery_time_in_days(&PizzaStatus::Cooking);
        assert_eq!(result, 15);
    }
}
