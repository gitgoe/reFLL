

 use crate::fuzzylogic::fuzzy_ruleantecedent::FuzzyRuleAntecedent;
 use crate::fuzzylogic::fuzzy_ruleconsequent::FuzzyRuleConsequent;


#[allow(non_snake_case)]
pub struct FuzzyRule{
    index: i32,
    fired: bool,
    fuzzyRuleAntecedent: Option<FuzzyRuleAntecedent>,
    fuzzyRuleConsequent: Option<FuzzyRuleConsequent>,
}

impl FuzzyRule {
    pub fn new(index: i32, fuzzy_ruleantecedent: Option<FuzzyRuleAntecedent>, fuzzy_ruleconsequent: Option<FuzzyRuleConsequent>) -> FuzzyRule {
        FuzzyRule{
            index,
            fired: false,
            fuzzyRuleAntecedent: fuzzy_ruleantecedent,
            fuzzyRuleConsequent: fuzzy_ruleconsequent,
        }
    }

    // Method to get the value of index
    pub fn get_index(&self) -> i32 {
        return self.index;
    }


    // Method to evaluate the total expression
    pub fn is_fired(&self) -> bool {
        return self.fired;
    }

    // Method to evaluate the total expression
    pub fn evaluate_expression(&mut self) -> bool {
        
        // check if the FuzzyRuleAntecedent and FuzzyRuleConsequent are valid
        if self.fuzzyRuleAntecedent.is_some() && self.fuzzyRuleConsequent.is_some() {
            // call the evaluator in the FuzzyRuleAntecedent
            let power_of_antecedent = self.fuzzyRuleAntecedent.as_ref().unwrap().evaluate();

            // if the power of FuzzyRuleAntecedent is bigget the 0.0, this rule was fired, else, no
            if power_of_antecedent > 0.0 {
                self.fired = true;
            } else {
                self.fired = false;
            }   
            // pass the power of FuzzyRuleAntecedent to FuzzyRuleConsequent by its evaluator
            self.fuzzyRuleConsequent.as_mut().unwrap().evaluate(power_of_antecedent);
        }
        return self.fired;
    }


}   

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::fuzzylogic::fuzzy_set::FuzzySet;

    #[test]
    fn test_index_and_evaluate_expression_and_is_fired() {

        let mut fuzzy_ruleantecedent1 = FuzzyRuleAntecedent::new();
        let mut fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        fuzzy_set1.set_pertinence(0.25);
        assert_eq!(fuzzy_ruleantecedent1.join_single(Some(fuzzy_set1)), true);

        let mut fuzzy_ruleantecedent2 = FuzzyRuleAntecedent::new();
        let mut fuzzy_set2:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        fuzzy_set2.set_pertinence(0.75);
        assert_eq!(fuzzy_ruleantecedent2.join_single(Some(fuzzy_set2)), true);

        let mut fuzzy_ruleantecedent3 = FuzzyRuleAntecedent::new();
        assert_eq!(fuzzy_ruleantecedent3.join_with_and_with_frafra(Some(Box::new(fuzzy_ruleantecedent1)), Some(Box::new(fuzzy_ruleantecedent2))), true);
        
        let mut fuzzy_ruleconsequent:FuzzyRuleConsequent =  FuzzyRuleConsequent::new();
        let fuzzy_set3:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        assert_eq!(fuzzy_ruleconsequent.add_output(fuzzy_set3), 1);
        
        let mut fuzzy_rule = FuzzyRule::new(1,Some(fuzzy_ruleantecedent3), Some(fuzzy_ruleconsequent));
        
        assert_eq!(fuzzy_rule.get_index(), 1);

        assert_eq!(fuzzy_rule.is_fired(), false);

        assert_eq!(fuzzy_rule.evaluate_expression(), true);

        assert_eq!(fuzzy_rule.is_fired(), true);
    }
}