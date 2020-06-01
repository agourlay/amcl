use crate::Strategy::{Additive, Multiplicative};

enum Strategy{
    Additive(f64),
    Multiplicative(f64),
}

impl Strategy {
    fn additive(gain: f64) -> Result<Strategy, String> {
        if gain <= 0.0 {
            Err("additive strategy must be > 0".to_string())
        } else {
            Ok(Additive(gain))
        }
    }

    fn multiplicative(gain: f64) -> Result<Strategy, String> {
        if gain <= 0.0 {
            Err("multiplicative strategy must be > 0".to_string())
        } else {
            Ok(Multiplicative(gain))
        }
    }
}

pub struct Controller {
    increase_strategy: Strategy,
    decrease_strategy: Strategy,
    current_value: f64,
    max_value: Option<f64>,
    min_value: Option<f64>,
}

impl Controller {

    pub fn aimd(additive_gain: f64, decrease_gain: f64) -> Result<Controller, String> {
        let increase_strategy = Strategy::additive(additive_gain)?;
        let decrease_strategy = Strategy::multiplicative(decrease_gain)?;
        Ok(Controller::new(increase_strategy, decrease_strategy))
    }

    pub fn mimd(additive_gain: f64, decrease_gain: f64) -> Result<Controller, String> {
        let increase_strategy = Strategy::multiplicative(additive_gain)?;
        let decrease_strategy = Strategy::multiplicative(decrease_gain)?;
        Ok(Controller::new(increase_strategy, decrease_strategy))
    }

    pub fn miad(additive_gain: f64, decrease_gain: f64) -> Result<Controller, String> {
        let increase_strategy = Strategy::multiplicative(additive_gain)?;
        let decrease_strategy = Strategy::additive(decrease_gain)?;
        Ok(Controller::new(increase_strategy, decrease_strategy))
    }

    pub fn aiad(additive_gain: f64, decrease_gain: f64) -> Result<Controller, String> {
        let increase_strategy = Strategy::additive(additive_gain)?;
        let decrease_strategy = Strategy::additive(decrease_gain)?;
        Ok(Controller::new(increase_strategy, decrease_strategy))
    }

    fn new(increase_strategy: Strategy, decrease_strategy: Strategy) -> Controller {
        Controller {
            increase_strategy,
            decrease_strategy,
            current_value: 0.0,
            max_value: None,
            min_value: None
        }
    }

    pub fn with_max_value(&mut self, max: f64) {
        self.max_value = Some(max);
        // update current
        if self.current_value > max {
            self.current_value = max
        }
    }

    pub fn with_min_value(&mut self, min: f64) {
        self.min_value = Some(min);
        // update current
        if self.current_value < min {
            self.current_value = min
        }
    }

    pub fn set_value(&mut self, new_value: f64) {
        self.current_value = new_value
    }

    pub fn update(&mut self, success: bool) {
        let old_value = self.current_value;
        let final_value = if success {
            let max_reached = self.max_value.map_or_else(|| false, |max| old_value >= max);
            if max_reached {
                old_value
            } else {
                let new_value = match self.increase_strategy {
                    Additive(gain) => old_value + gain,
                    Multiplicative(gain) => old_value * gain
                };
                // overflow protection
                match self.max_value {
                    Some(max) if new_value > max => max,
                    _ => new_value
                }
            }
        } else {
            let min_reached = self.min_value.map_or_else(|| false, |min| old_value <= min);
            if min_reached {
                old_value
            } else {
                let new_value = match self.decrease_strategy {
                    Additive(gain) => old_value - gain,
                    Multiplicative(gain) => old_value / gain
                };
                // overflow protection
                match self.min_value {
                    Some(min) if new_value < min => min,
                    _ => new_value
                }
            }
        };
        self.current_value = final_value
    }

    pub fn current(&self) -> f64 {
        self.current_value
    }
}

#[cfg(test)]
mod strategy_tests {
    use super::*;

    #[test]
    fn aimd() {
        let mut aimd = Controller::aimd(1.0, 2.0).unwrap();
        aimd.update(true);
        aimd.update(true);
        assert_eq!(aimd.current(), 2.0);
        aimd.update(true);
        aimd.update(true);
        assert_eq!(aimd.current(), 4.0);
        aimd.update(false);
        assert_eq!(aimd.current(), 2.0);
    }

    #[test]
    fn aimd_with_max() {
        let mut aimd = Controller::aimd(2.0, 2.0).unwrap();
        aimd.with_max_value(50.0);
        for _i in 1..=100 {
            aimd.update(true);
        }
        assert_eq!(aimd.current(), 50.0);
    }

    #[test]
    fn aimd_with_min() {
        let mut aimd = Controller::aimd(1.0, 2.0).unwrap();
        assert_eq!(aimd.current(), 0.0);
        aimd.with_min_value(50.0);
        assert_eq!(aimd.current(), 50.0);
        for _i in 1..=100 {
            aimd.update(true);
        }
        assert_eq!(aimd.current(), 150.0);
        for _i in 1..=1000 {
            aimd.update(false);
        }
        assert_eq!(aimd.current(), 50.0);
    }

    #[test]
    fn mimd() {
        let mut mimd = Controller::mimd(2.0, 2.0).unwrap();
        mimd.set_value(1.0);
        mimd.update(true);
        mimd.update(true);
        assert_eq!(mimd.current(), 4.0);
        mimd.update(true);
        mimd.update(true);
        assert_eq!(mimd.current(), 16.0);
        mimd.update(false);
        assert_eq!(mimd.current(), 8.0);
    }

    #[test]
    fn miad() {
        let mut miad = Controller::miad(2.0, 2.0).unwrap();
        miad.set_value(1.0);
        miad.update(true);
        miad.update(true);
        assert_eq!(miad.current(), 4.0);
        miad.update(true);
        miad.update(true);
        assert_eq!(miad.current(), 16.0);
        miad.update(false);
        assert_eq!(miad.current(), 14.0);
    }

    #[test]
    fn aiad() {
        let mut aiad = Controller::aiad(1.0, 1.0).unwrap();
        aiad.update(true);
        aiad.update(true);
        assert_eq!(aiad.current(), 2.0);
        aiad.update(true);
        aiad.update(true);
        assert_eq!(aiad.current(), 4.0);
        aiad.update(false);
        assert_eq!(aiad.current(), 3.0);
    }

}
