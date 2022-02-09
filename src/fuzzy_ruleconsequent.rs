pub mod fuzzylogic{

    use crate::fuzzy_set::fuzzylogic::FuzzySet;

    #[allow(non_snake_case)]
    pub struct FuzzyRuleConsequent{
        fuzzySetOutputArray: Vec<FuzzySet>,
    } 

    impl FuzzyRuleConsequent {
        pub fn new() -> FuzzyRuleConsequent {
            FuzzyRuleConsequent{
                fuzzySetOutputArray: vec![]
            }
        }

        // Method to include a new FuzzySet (for Output) into FuzzyRuleConsequent
        pub fn add_output(&mut self, fuzzy_set: FuzzySet) -> usize{
            self.fuzzySetOutputArray.push(fuzzy_set);
            return self.fuzzySetOutputArray.len()
        }

        // Method that clean all fuzzySetOutputArray from memory
        pub fn clean_fuzzysets(&mut self) -> usize {
            self.fuzzySetOutputArray.clear();
            return self.fuzzySetOutputArray.len();
        }

        // Method to evaluate this FuzzyRuleConsequent
        pub fn evaluate(&mut self, power: f32) -> bool {
            for fs in self.fuzzySetOutputArray.iter_mut() {
                fs.set_pertinence(power);
                println!("set_pertinence: {:?}", power);
            }
            return true;
        }

        pub fn fuzzyset(&mut self, pos: usize) -> FuzzySet {
            let array =  &*self.fuzzySetOutputArray;
            return array[pos];
        }


    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::fuzzylogic::*;
    use crate::fuzzy_set::fuzzylogic::FuzzySet;

    #[test]
    fn test_new_fuzzy_ruleconsequent() {
        let mut fuzzy_ruleconsequent:FuzzyRuleConsequent =  FuzzyRuleConsequent::new();

        let mut fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        let mut fuzzy_set2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);
        
        assert_eq!(fuzzy_ruleconsequent.add_output(fuzzy_set1), 1);
        assert_eq!(fuzzy_ruleconsequent.add_output(fuzzy_set2), 2);

        fuzzy_ruleconsequent.evaluate(0.5);

        assert_eq!(fuzzy_ruleconsequent.fuzzyset(0).get_pertinence(), 0.5);
        assert_eq!(fuzzy_ruleconsequent.fuzzyset(1).get_pertinence(), 0.5);

        assert_eq!(fuzzy_ruleconsequent.clean_fuzzysets(), 0);

        
    }
}