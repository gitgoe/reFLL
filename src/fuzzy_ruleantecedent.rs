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
        fuzzySet1:FuzzySet,
        fuzzySet2:FuzzySet,
        fuzzyRuleAntecedent1: Box<FuzzyRuleAntecedent>,
        fuzzyRuleAntecedent2: Box<FuzzyRuleAntecedent>,
    } 


}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::fuzzylogic::*;
    use crate::fuzzy_set::fuzzylogic::FuzzySet;

    #[test]
    fn test_fuzzy_rule_antecedent() {
        //let  fuzzy_output:FuzzyOutput =  FuzzyOutput::new(1);
        assert_eq!(1, 1);
    }
}
