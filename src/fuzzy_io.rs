pub mod fuzzylogic{

    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::fuzzy_set::fuzzylogic::FuzzySet;

    #[allow(non_snake_case)]
    pub struct FuzzyIO{
        index: i32,
        crispInput:f32,
        pub fuzzySetArray: Rc<RefCell<Vec<FuzzySet>>>,
    }

    impl FuzzyIO {
        pub fn new(index: i32) -> FuzzyIO {
            FuzzyIO{
                index,
                crispInput: 0.0,
                fuzzySetArray: Rc::new(RefCell::new(vec![]))
            }
        }

        // Method to get the value of index
        pub fn get_index(&self) -> i32{
            return self.index;
        }

        // Method to set the value of crispInput
        pub fn set_crisp_input(& mut self, crisp_input:f32){
            self.crispInput = crisp_input;
        }

        // Method to get the value of crispInput
        pub fn get_crisp_input(&self) -> f32{
            return self.crispInput;
        }

        // Method to include a new FuzzySet into FuzzyIO
        pub fn add_fuzzyset(& mut self, fuzzy_set: FuzzySet) -> usize{
            self.fuzzySetArray.borrow_mut().push(fuzzy_set);
            return self.fuzzySetArray.borrow_mut().len()
        }

        pub fn fuzzyset(&self, pos: usize) -> FuzzySet {
            let array =  &*self.fuzzySetArray.borrow();
            return array[pos];
        }

        pub fn clean_fuzzysets(&self) -> usize {
            self.fuzzySetArray.borrow_mut().clear();
            return self.fuzzySetArray.borrow().len();
        }

        pub fn reset_fuzzysets(&self) {
            let borrow = &mut *self.fuzzySetArray.borrow_mut();
            for fs in borrow {
                fs.reset();
                println!("resetFuzzySets: {:?}", fs);
            }
        }

        pub fn calculate_fuzzyset_pertinences(& self, crisp_value: f32) {
            let borrow = &mut *self.fuzzySetArray.borrow_mut();
            for fs in borrow {
                fs.calculate_pertinence(crisp_value);
                println!("calculate: {:?}", fs);
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
    fn test_new() {
        let mut fuzzy_io:FuzzyIO =  FuzzyIO::new(5);
        assert_eq!(fuzzy_io.get_index(), 5);

        fuzzy_io.set_crisp_input(10.190);
        assert_eq!(fuzzy_io.get_crisp_input(), 10.190); 
    }

    #[test]
    fn test_add_fuzzy_set() {
        let mut fuzzy_io:FuzzyIO =  FuzzyIO::new(5);
       
        let fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);
        let fuzzy_set2:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);

        assert_eq!(fuzzy_io.add_fuzzyset(fuzzy_set1), 1);
        assert_eq!(fuzzy_io.add_fuzzyset(fuzzy_set2), 2);     
    }

    #[test]
    fn test_clean_fuzzy_sets() {
        let mut fuzzy_io:FuzzyIO =  FuzzyIO::new(5);
       
        let fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);
        let fuzzy_set2:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);

        assert_eq!(fuzzy_io.add_fuzzyset(fuzzy_set1), 1);
        assert_eq!(fuzzy_io.add_fuzzyset(fuzzy_set2), 2);

        assert_eq!(fuzzy_io.clean_fuzzysets(), 0);   
    }

    #[test]
    fn test_reset_fuzzy_sets() {
        let mut fuzzy_io:FuzzyIO =  FuzzyIO::new(5);
       
        let mut fuzzy_set:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);

        fuzzy_set.set_pertinence(0.242);

        assert_eq!(fuzzy_io.add_fuzzyset(fuzzy_set), 1);

        assert_eq!(0.242, fuzzy_io.fuzzyset(0).get_pertinence());
        
        fuzzy_io.reset_fuzzysets();

        assert_eq!(0.0, fuzzy_io.fuzzyset(0).get_pertinence());

    }
}

