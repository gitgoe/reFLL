

    use std::ops::DerefMut;
    use std::ops::Deref;
    use crate::fuzzylogic::fuzzy_io::FuzzyIO;

    #[allow(non_snake_case)]
    pub struct FuzzyInput{
        index: i32,
        pub fuzzyIO: FuzzyIO
    }

    impl FuzzyInput {
        pub fn new(index: i32) -> FuzzyInput {
            FuzzyInput{
                index,
                fuzzyIO: FuzzyIO::new(index)
            }
        }
        // Method to calculate the pertinence of all FuzzySet
        pub fn calculate_fuzzyset_pertinences(&mut self)-> bool {
            // call calculatePertinence for each FuzzySet
            let crisp_input = self.fuzzyIO.get_crisp_input();
            self.fuzzyIO.calculate_fuzzyset_pertinences(crisp_input);
            return true;
        }
    }
    impl Deref for FuzzyInput {
        type Target = FuzzyIO;
        fn deref(&self) -> &FuzzyIO { &self.fuzzyIO }
    }

    impl DerefMut for FuzzyInput {
        fn deref_mut(&mut self) -> &mut FuzzyIO { &mut self.fuzzyIO }
    }


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::fuzzylogic::fuzzy_set::FuzzySet;

   
    #[test]
    fn test_add_fuzzyset() {
        let mut fuzzy_input:FuzzyInput =  FuzzyInput::new(1);
        let fuzzy_set:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        assert_eq!(fuzzy_input.add_fuzzyset(fuzzy_set), 1);
    }

    #[test]
    fn test_set_crisp_input_and_get_crisp_input() {
        let mut fuzzy_input:FuzzyInput =  FuzzyInput::new(1);
        fuzzy_input.set_crisp_input(10.190);
        assert_eq!(fuzzy_input.get_crisp_input(), 10.190);
    }

    #[test]
    fn test_calculate_fuzzy_set_pertinences() {
        
        let mut fuzzy_input:FuzzyInput =  FuzzyInput::new(1);

        let fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        fuzzy_input.add_fuzzyset(fuzzy_set1);

        let fuzzy_set2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);
        fuzzy_input.add_fuzzyset(fuzzy_set2);

        fuzzy_input.set_crisp_input(5.0);
        
        fuzzy_input.calculate_fuzzyset_pertinences();
        assert_eq!(fuzzy_input.fuzzyset(0).get_pertinence(), 0.5);
        assert_eq!(fuzzy_input.fuzzyset(1).get_pertinence(), 0.0);
    }
}
