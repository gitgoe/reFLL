use std::rc::Rc;
use std::cell::RefCell;

use crate::FuzzySet::FuzzySet;

#[allow(non_snake_case)]
pub struct FuzzyIO{
    index: i64,
    crispInput:f64,
    pub fuzzySetArray: Rc<RefCell<Vec<FuzzySet>>>,
}

impl FuzzyIO {
    pub fn new(index: i64) -> FuzzyIO {
        FuzzyIO{
            index,
            crispInput: 0.0,
            fuzzySetArray: Rc::new(RefCell::new(vec![]))
        }
    }

    // Method to get the value of index
    pub fn getIndex(&self) -> i64{
        return self.index;
    }

    // Method to set the value of crispInput
    pub fn setCrispInput(& mut self, crispInput:f64){
        self.crispInput = crispInput;
    }

    // Method to get the value of crispInput
    pub fn getCrispInput(&self) -> f64{
        return self.crispInput;
    }

    // Method to include a new FuzzySet into FuzzyIO
    pub fn addFuzzySet(& mut self, fuzzySet: FuzzySet) -> usize{
        self.fuzzySetArray.borrow_mut().push(fuzzySet);
        return self.fuzzySetArray.borrow_mut().len()
    }

    pub fn fuzzySet(&self, pos: usize) -> FuzzySet {
        let array =  &*self.fuzzySetArray.borrow();
        return array[pos];
    }

    pub fn cleanFuzzySets(&self) -> usize {
        self.fuzzySetArray.borrow_mut().clear();
        return self.fuzzySetArray.borrow().len();
    }

    pub fn calculateFuzzySetPertinences(& self, crispValue: f64) {
        let borrow = &mut *self.fuzzySetArray.borrow_mut();
        for fs in borrow {
            fs.calculatePertinence(crispValue);
            println!("calculate: {:?}", fs);
        }

    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new() {
        let mut fuzzyIO:FuzzyIO =  FuzzyIO::new(5);
        assert_eq!(fuzzyIO.getIndex(), 5);

        fuzzyIO.setCrispInput(10.190);
        assert_eq!(fuzzyIO.getCrispInput(), 10.190); 
    }

    #[test]
    fn test_addFuzzySet() {
        let mut fuzzyIO:FuzzyIO =  FuzzyIO::new(5);
       
        let fuzzySet1:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);
        let fuzzySet2:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);

        assert_eq!(fuzzyIO.addFuzzySet(fuzzySet1), 1);
        assert_eq!(fuzzyIO.addFuzzySet(fuzzySet2), 2);
        
    }

    #[test]
    fn test_cleanFuzzySets() {
        let mut fuzzyIO:FuzzyIO =  FuzzyIO::new(5);
       
        let fuzzySet1:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);
        let fuzzySet2:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);

        assert_eq!(fuzzyIO.addFuzzySet(fuzzySet1), 1);
        assert_eq!(fuzzyIO.addFuzzySet(fuzzySet2), 2);

        assert_eq!(fuzzyIO.cleanFuzzySets(), 0);
        
    }
}

