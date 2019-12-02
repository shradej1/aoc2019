#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FuelRequirement {
    fuel_for_modules: u64,
    fuel_for_modules_and_fuel: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Module {
    pub mass: u32,
}

fn mass_to_fuel(mass: u32) -> u32 {
    if mass / 3 <= 2 {
        0
    } else {
        mass / 3 - 2
    }
}

/// Returns the amount of fuel required for just the modules,
/// and the total amount of fuel for both the modules and additional fuel
pub fn compute_fuel_requirement(modules: &[Module]) -> FuelRequirement {
    let mut fuel: u64 = 0;
    let mut extra_fuel_agg: u64 = 0;
    for m in modules {
        let mut new_fuel = mass_to_fuel(m.mass);
        fuel += new_fuel as u64;
        while new_fuel > 0 {
            new_fuel = mass_to_fuel(new_fuel);
            extra_fuel_agg += new_fuel as u64;
        }
    }
    FuelRequirement {
        fuel_for_modules: fuel,
        fuel_for_modules_and_fuel: fuel + extra_fuel_agg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_to_fuel() {
        assert_eq!(mass_to_fuel(12), 2);
        assert_eq!(mass_to_fuel(14), 2);
        assert_eq!(mass_to_fuel(1969), 654);
        assert_eq!(mass_to_fuel(100756), 33583);
    }

    #[test]
    fn test_compute_fuel_requirement() {
        assert_eq!(
            2,
            compute_fuel_requirement(&[Module { mass: 14 }]).fuel_for_modules_and_fuel
        );
        assert_eq!(
            966,
            compute_fuel_requirement(&[Module { mass: 1969 }]).fuel_for_modules_and_fuel
        );
        assert_eq!(
            50346,
            compute_fuel_requirement(&[Module { mass: 100756 }]).fuel_for_modules_and_fuel
        );
    }

    #[test]
    fn test_negative_fuel_is_zero() {
        assert_eq!(mass_to_fuel(1), 0);
    }

    #[test]
    fn answer() {
        let ans = compute_fuel_requirement(&get_modules());
        assert_eq!(3254441, ans.fuel_for_modules);
        assert_eq!(4878818, ans.fuel_for_modules_and_fuel);
    }
}

pub fn get_modules() -> Vec<Module> {
    vec![
        Module { mass: 126360 },
        Module { mass: 61158 },
        Module { mass: 149929 },
        Module { mass: 88405 },
        Module { mass: 87526 },
        Module { mass: 51688 },
        Module { mass: 75356 },
        Module { mass: 116265 },
        Module { mass: 134986 },
        Module { mass: 111581 },
        Module { mass: 135675 },
        Module { mass: 69679 },
        Module { mass: 74035 },
        Module { mass: 144951 },
        Module { mass: 86157 },
        Module { mass: 68946 },
        Module { mass: 76761 },
        Module { mass: 114768 },
        Module { mass: 70694 },
        Module { mass: 84768 },
        Module { mass: 147379 },
        Module { mass: 78755 },
        Module { mass: 109688 },
        Module { mass: 118595 },
        Module { mass: 54608 },
        Module { mass: 77033 },
        Module { mass: 54654 },
        Module { mass: 61473 },
        Module { mass: 69644 },
        Module { mass: 81744 },
        Module { mass: 97148 },
        Module { mass: 106473 },
        Module { mass: 61541 },
        Module { mass: 98898 },
        Module { mass: 70394 },
        Module { mass: 117635 },
        Module { mass: 128388 },
        Module { mass: 140622 },
        Module { mass: 108691 },
        Module { mass: 126962 },
        Module { mass: 137756 },
        Module { mass: 125904 },
        Module { mass: 75675 },
        Module { mass: 127051 },
        Module { mass: 126388 },
        Module { mass: 85591 },
        Module { mass: 51583 },
        Module { mass: 101392 },
        Module { mass: 62959 },
        Module { mass: 135077 },
        Module { mass: 90916 },
        Module { mass: 127119 },
        Module { mass: 112427 },
        Module { mass: 79703 },
        Module { mass: 54739 },
        Module { mass: 50092 },
        Module { mass: 92505 },
        Module { mass: 53719 },
        Module { mass: 60887 },
        Module { mass: 62642 },
        Module { mass: 76382 },
        Module { mass: 85763 },
        Module { mass: 125799 },
        Module { mass: 67285 },
        Module { mass: 147992 },
        Module { mass: 80713 },
        Module { mass: 133619 },
        Module { mass: 131433 },
        Module { mass: 141765 },
        Module { mass: 109553 },
        Module { mass: 122534 },
        Module { mass: 88734 },
        Module { mass: 115622 },
        Module { mass: 82195 },
        Module { mass: 130771 },
        Module { mass: 121649 },
        Module { mass: 89355 },
        Module { mass: 121364 },
        Module { mass: 71664 },
        Module { mass: 130412 },
        Module { mass: 88936 },
        Module { mass: 63234 },
        Module { mass: 80274 },
        Module { mass: 108251 },
        Module { mass: 136663 },
        Module { mass: 139149 },
        Module { mass: 85052 },
        Module { mass: 67973 },
        Module { mass: 116461 },
        Module { mass: 75070 },
        Module { mass: 144261 },
        Module { mass: 106539 },
        Module { mass: 79712 },
        Module { mass: 116112 },
        Module { mass: 55755 },
        Module { mass: 121428 },
        Module { mass: 79362 },
        Module { mass: 103489 },
        Module { mass: 103157 },
        Module { mass: 64403 },
    ]
}
