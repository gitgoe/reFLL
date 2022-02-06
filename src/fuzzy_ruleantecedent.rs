pub mod fuzzylogic{

    use crate::fuzzy_set::fuzzylogic::FuzzySet;
    
    const EPSILON_VALUE: f32 = 1.0E-3;

    // CONSTANTS
    // possible logic operators
    enum Op {
       AND = 1,
        OR  =2,
    }

    // possible join associations modes
    #[allow(non_snake_case)]
    enum Mode {
        FS = 1,
        FS_FS =2,
        FS_FRA =3,
        FRA_FRA =4,
     }
    
    #[allow(non_snake_case)]
    pub struct FuzzyRuleAntecedent{
        op: i32,
        mode: i32,
        fuzzySet1:Option<FuzzySet>,
        fuzzySet2:Option<FuzzySet>,
        fuzzyRuleAntecedent1: Option<Box<FuzzyRuleAntecedent>>,
        fuzzyRuleAntecedent2: Option<Box<FuzzyRuleAntecedent>>,
    } 

    impl FuzzyRuleAntecedent {
        pub fn new() -> FuzzyRuleAntecedent {
            FuzzyRuleAntecedent{
                op: 0,
                mode: 0,
                fuzzySet1:None,
                fuzzySet2:None,
                fuzzyRuleAntecedent1: None,
                fuzzyRuleAntecedent2: None,
            }
        }
        // Method to create a FuzzyRuleAntecedent with just one single FuzzySet
        pub fn join_single(&mut self, fuzzy_set: FuzzySet)-> bool {
            return true;
        }

          // Method to create a FuzzyRuleAntecedent with two FuzzySet, with AND
        pub fn join_with_and(&mut self, fuzzy_set1: FuzzySet, fuzzy_set2: FuzzySet)-> bool {
            return true;
        }
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::fuzzylogic::*;
    use crate::fuzzy_set::fuzzylogic::FuzzySet;

    #[test]
    fn test_new_fuzzy_rule_antecedent() {
        //let  fuzzy_output:FuzzyOutput =  FuzzyOutput::new(1);
        assert_eq!(1, 1);
    }
}
