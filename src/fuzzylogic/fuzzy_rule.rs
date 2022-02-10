

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
        let mut fuzzy_set2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);
        fuzzy_set2.set_pertinence(0.75);
        assert_eq!(fuzzy_ruleantecedent2.join_single(Some(fuzzy_set2)), true);

        //let mut fuzzy_ruleantecedent3 = FuzzyRuleAntecedent::new();
        
    }
}