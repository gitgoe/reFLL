pub mod fuzzylogic{

    use crate::fuzzy_set::fuzzylogic::FuzzySet;
    
    const EPSILON_VALUE: f32 = 1.0E-3;

    // CONSTANTS
    // possible logic operators
    #[allow(non_camel_case_types)]
    pub enum OperatorEnum {
        AND,
        OR,
    }

    // possible join associations modes
    #[allow(non_camel_case_types)]
    pub enum ModeEnum {
        FS,
        FS_FS,
        FS_FRA,
        FRA_FRA,
     }
    
    #[allow(non_snake_case)]
    pub struct FuzzyRuleAntecedent{
        op: OperatorEnum,
        mode: ModeEnum,
        fuzzySet1:Option<FuzzySet>,
        fuzzySet2:Option<FuzzySet>,
        fuzzyRuleAntecedent1: Option<Box<FuzzyRuleAntecedent>>,
        fuzzyRuleAntecedent2: Option<Box<FuzzyRuleAntecedent>>,
    } 

    impl FuzzyRuleAntecedent {
        pub fn new() -> FuzzyRuleAntecedent {
            FuzzyRuleAntecedent{
                op: OperatorEnum::AND,
                mode: ModeEnum::FS,
                fuzzySet1:None,
                fuzzySet2:None,
                fuzzyRuleAntecedent1: None,
                fuzzyRuleAntecedent2: None,
            }
        }
        // Method to create a FuzzyRuleAntecedent with just one single FuzzySet
        pub fn join_single(&mut self, fuzzy_set: Option<FuzzySet>)-> bool {
            // check if FuzzySet is valide
            if fuzzy_set.is_some() {
                
                // set the mode and reference
                self.mode = ModeEnum::FS;
                self.fuzzySet1 = fuzzy_set;
                return true;
            }
            return false;
        }

        // Method to create a FuzzyRuleAntecedent with two FuzzySet, with AND
        pub fn join_with_and(&mut self, fuzzy_set1: Option<FuzzySet>, fuzzy_set2: Option<FuzzySet>)-> bool {
             // check if FuzzySet is valide
             if fuzzy_set1.is_some() && fuzzy_set2.is_some() {
                
                // set the mode and reference
                self.op = OperatorEnum::AND;
                self.mode = ModeEnum::FS_FS;
                self.fuzzySet1 = fuzzy_set1;
                self.fuzzySet2 = fuzzy_set2;
                return true;
            }
            return false;
        }

        // Method to create a FuzzyRuleAntecedent with two FuzzySet, with OR
        pub fn join_with_or(&mut self, fuzzy_set1: Option<FuzzySet>, fuzzy_set2: Option<FuzzySet>)-> bool {
            // check if FuzzySet is valide
            if fuzzy_set1.is_some() && fuzzy_set2.is_some() {
               
               // set the mode and reference
               self.op = OperatorEnum::OR;
               self.mode = ModeEnum::FS_FS;
               self.fuzzySet1 = fuzzy_set1;
               self.fuzzySet2 = fuzzy_set2;
               return true;
           }
           return false;
       }

        // Method to create a FuzzyRuleAntecedent with one FuzzySet and one FuzzyRuleAntecedent, with AND
       pub fn join_with_and_with_fra(&mut self, fuzzy_set: Option<FuzzySet>, fuzzy_rule_antecedent: Option<Box<FuzzyRuleAntecedent>>)-> bool {
            // check if FuzzySet and fuzzy_rule_antecedent are valide
            if fuzzy_set.is_some() && fuzzy_rule_antecedent.is_some(){
                
                // set the mode and reference
                self.op = OperatorEnum::AND;
                self.mode = ModeEnum::FS_FRA;
                self.fuzzySet1 = fuzzy_set;
                self.fuzzyRuleAntecedent1 = fuzzy_rule_antecedent;
                return true;
            }
            return false;
        }

        // Method to create a FuzzyRuleAntecedent with one FuzzySet and one FuzzyRuleAntecedent, with AND (Inverse Params)
       pub fn join_with_and_with_fra_reverse(&mut self, fuzzy_rule_antecedent: Option<Box<FuzzyRuleAntecedent>>, fuzzy_set: Option<FuzzySet> )-> bool {
            return self.join_with_and_with_fra(fuzzy_set, fuzzy_rule_antecedent);
        }

        // Method to create a FuzzyRuleAntecedent with one FuzzySet and one FuzzyRuleAntecedent, with OR
       pub fn join_with_or_with_fra(&mut self, fuzzy_set: Option<FuzzySet>, fuzzy_rule_antecedent: Option<Box<FuzzyRuleAntecedent>>)-> bool {
        // check if FuzzySet and fuzzy_rule_antecedent are valide
        if fuzzy_set.is_some() && fuzzy_rule_antecedent.is_some(){
            
            // set the mode and reference
            self.op = OperatorEnum::OR;
            self.mode = ModeEnum::FS_FRA;
            self.fuzzySet1 = fuzzy_set;
            self.fuzzyRuleAntecedent1 = fuzzy_rule_antecedent;
            return true;
        }
        return false;
    }

    // Method to create a FuzzyRuleAntecedent with one FuzzySet and one FuzzyRuleAntecedent, with AND (Inverse Params)
    pub fn join_with_or_with_fra_reverse(&mut self, fuzzy_rule_antecedent: Option<Box<FuzzyRuleAntecedent>>, fuzzy_set: Option<FuzzySet> )-> bool {
        return self.join_with_or_with_fra(fuzzy_set, fuzzy_rule_antecedent);
    }

        pub fn evaluate(&self) -> f32{

            // switch by the mode value
            match self.mode {

                ModeEnum::FS => {
                    // case it is a single FuzzySet join, just return its pertinence
                    return self.fuzzySet1.unwrap().get_pertinence();
                },

                ModeEnum::FS_FS => {
                    match self.op {
                        OperatorEnum::AND => {
                            // case the operator is AND, check if both has pertinence bigger then 0.0
                            if self.fuzzySet1.unwrap().get_pertinence() > 0.0 && self.fuzzySet2.unwrap().get_pertinence() > 0.0 {
                                // in this case, return the small pertinence between two FuzzySet
                                if self.fuzzySet1.unwrap().get_pertinence() < self.fuzzySet2.unwrap().get_pertinence(){
                                    return self.fuzzySet1.unwrap().get_pertinence();
                                }else{
                                    return self.fuzzySet2.unwrap().get_pertinence();
                                }
                            } 
                            return 0.0;     
                        },
                        OperatorEnum::OR => {
                             // case the operator is OR, check if one has pertinence bigger then 0.0
                             if self.fuzzySet1.unwrap().get_pertinence() > 0.0 || self.fuzzySet2.unwrap().get_pertinence() > 0.0 {
                                // in this case, return the one pertinence is bigger
                                if self.fuzzySet1.unwrap().get_pertinence() > self.fuzzySet2.unwrap().get_pertinence(){
                                    return self.fuzzySet1.unwrap().get_pertinence();
                                }else{
                                    return self.fuzzySet2.unwrap().get_pertinence();
                                }
                            } 
                            return 0.0; 
                        }, 
                    }

                },
                ModeEnum::FS_FRA => {
                    // case it is a join of one FuzzySet and one FuzzyRuleAntecedent, switch by the operator
                    match self.op {
                        OperatorEnum::AND => {
                           // case the operator is AND, check if both has pertinence bigger then 0.0
                            if self.fuzzySet1.unwrap().get_pertinence() > 0.0 && self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate() > 0.0 {
                                // in this case, return the small pertinence between two FuzzySet
                                if self.fuzzySet1.unwrap().get_pertinence() < self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate() {
                                    return self.fuzzySet1.unwrap().get_pertinence();
                                }else{
                                    return self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate();
                                }
                            } 
                            return 0.0;     
                        },
                        OperatorEnum::OR => {
                             // case the operator is OR, check if one has pertinence bigger then 0.0
                            if self.fuzzySet1.unwrap().get_pertinence() > 0.0  || self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate() > 0.0 {
                                // in this case, return the one pertinence is bigger
                                if self.fuzzySet1.unwrap().get_pertinence() > self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate() {
                                    return self.fuzzySet1.unwrap().get_pertinence();
                                }else{
                                    return self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate();
                                }
                            } 
                            return 0.0; 
                        }, 
                    }
                },
                ModeEnum::FRA_FRA => {
                    // case it is a join of two FuzzyRuleAntecedent, switch by the operator
                    match self.op {
                        OperatorEnum::AND => {
                           // case the operator is AND, check if both has pertinence bigger then 0.0
                            if self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate() > 0.0 && self.fuzzyRuleAntecedent2.as_deref().unwrap().evaluate() > 0.0 {
                                // in this case, return the small pertinence between two FuzzySet
                                if self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate() < self.fuzzyRuleAntecedent2.as_deref().unwrap().evaluate() {
                                    return self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate();
                                }else{
                                    return self.fuzzyRuleAntecedent2.as_deref().unwrap().evaluate();
                                }
                            } 
                            return 0.0;     
                        },
                        OperatorEnum::OR => {
                             // case the operator is OR, check if one has pertinence bigger then 0.0
                            if self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate() > 0.0  || self.fuzzyRuleAntecedent2.as_deref().unwrap().evaluate() > 0.0 {
                                // in this case, return the one pertinence is bigger
                                if self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate() > self.fuzzyRuleAntecedent2.as_deref().unwrap().evaluate() {
                                    return self.fuzzyRuleAntecedent1.as_deref().unwrap().evaluate();
                                }else{
                                    return self.fuzzyRuleAntecedent2.as_deref().unwrap().evaluate();
                                }
                            } 
                            return 0.0; 
                        }, 
                    }
                }
            }
            
        }
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::fuzzylogic::*;
    use crate::fuzzy_set::fuzzylogic::FuzzySet;

    #[test]
    fn test_join_single_and_evaluate() {
        
        let mut fuzzy_ruleantecedent = FuzzyRuleAntecedent::new();
        let mut fuzzy_set:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        fuzzy_set.set_pertinence(0.25);

        assert_eq!(fuzzy_ruleantecedent.join_single(Some(fuzzy_set)), true);
        assert_eq!(fuzzy_ruleantecedent.evaluate(), 0.25);  
    }

    #[test]
    fn test_join_two_fuzzyset_and_evaluate() {
        
        let mut fuzzy_ruleantecedent1 = FuzzyRuleAntecedent::new();
        let mut fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        fuzzy_set1.set_pertinence(0.25);

        let mut fuzzy_ruleantecedent2 = FuzzyRuleAntecedent::new();
        let mut fuzzy_set2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);
        fuzzy_set2.set_pertinence(0.75);

        assert_eq!(fuzzy_ruleantecedent1.join_with_and(Some(fuzzy_set1),Some(fuzzy_set2)), true);
        assert_eq!(fuzzy_ruleantecedent1.evaluate(), 0.25);

        assert_eq!(fuzzy_ruleantecedent2.join_with_or(Some(fuzzy_set1),Some(fuzzy_set2)), true);
        assert_eq!(fuzzy_ruleantecedent2.evaluate(), 0.75);  
    }

    #[test]
    fn test_join_one_fuzzyset_and_one_fuzzyantecedent_and_evaluate1() {
        
        let mut fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        fuzzy_set1.set_pertinence(0.25);

        let mut fuzzy_ruleantecedent1 = FuzzyRuleAntecedent::new();
        let mut fuzzy_set2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);
        fuzzy_set2.set_pertinence(0.75);
        assert_eq!(fuzzy_ruleantecedent1.join_single(Some(fuzzy_set2)), true);
        
        let mut fuzzy_ruleantecedent2 = FuzzyRuleAntecedent::new();
        assert_eq!(fuzzy_ruleantecedent2.join_with_and_with_fra(Some(fuzzy_set1),Some(Box::new(fuzzy_ruleantecedent1))), true);
        assert_eq!(fuzzy_ruleantecedent2.evaluate(), 0.25);
    
    }

    #[test]
    fn test_join_one_fuzzyset_and_one_fuzzyantecedent_and_evaluate2() {
        
        let mut fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        fuzzy_set1.set_pertinence(0.25);

        let fuzzy_ruleantecedent1 = FuzzyRuleAntecedent::new();
        let mut fuzzy_set2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);
        fuzzy_set2.set_pertinence(0.75);
    
        let mut fuzzy_ruleantecedent3 = FuzzyRuleAntecedent::new();
        assert_eq!(fuzzy_ruleantecedent3.join_with_and_with_fra_reverse(Some(Box::new(fuzzy_ruleantecedent1)), Some(fuzzy_set1)), true);
        assert_eq!(fuzzy_ruleantecedent3.evaluate(), 0.25);
    }

    #[test]
    fn test_join_one_fuzzyset_and_one_fuzzyantecedent_and_evaluate3() {
        
        let mut fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        fuzzy_set1.set_pertinence(0.25);

        let fuzzy_ruleantecedent1 = FuzzyRuleAntecedent::new();
        let mut fuzzy_set2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);
        fuzzy_set2.set_pertinence(0.75);
    
        let mut fuzzy_ruleantecedent4 = FuzzyRuleAntecedent::new();
        assert_eq!(fuzzy_ruleantecedent4.join_with_or_with_fra(Some(fuzzy_set1),Some(Box::new(fuzzy_ruleantecedent1))), true);
        assert_eq!(fuzzy_ruleantecedent4.evaluate(), 0.25);
    }

    
}
