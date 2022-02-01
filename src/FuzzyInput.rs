
use std::ops::DerefMut;
use std::ops::Deref;
use crate::fuzzyIO::FuzzyIO;


#[allow(non_snake_case)]
pub struct FuzzyInput{
    index: i32,
    pub fuzzyIO: FuzzyIO
}

impl FuzzyInput {
    fn new(index: i32) -> FuzzyInput {
        FuzzyInput{
            index,
            fuzzyIO: FuzzyIO::new(index)
        }
    }
    // Method to calculate the pertinence of all FuzzySet
    fn calculateFuzzySetPertinences(& self)-> bool {
        // call calculatePertinence for each FuzzySet
        let crispInput = self.fuzzyIO.getCrispInput();
        self.fuzzyIO.calculateFuzzySetPertinences(crispInput);
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
    use crate::fuzzySet::FuzzySet;

    #[test]
    fn test_addFuzzySet() {
        let mut fuzzyInput:FuzzyInput =  FuzzyInput::new(1);
        let fuzzySet:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        assert_eq!(fuzzyInput.addFuzzySet(fuzzySet), 1);
    }

    #[test]
    fn test_setCrispInputAndGetCrispInput() {
        let mut fuzzyInput:FuzzyInput =  FuzzyInput::new(1);
        fuzzyInput.setCrispInput(10.190);
        assert_eq!(fuzzyInput.getCrispInput(), 10.190);
    }

    #[test]
    fn test_calculateFuzzySetPertinences() {
        
        let mut fuzzyInput:FuzzyInput =  FuzzyInput::new(1);

        let fuzzySet1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        fuzzyInput.addFuzzySet(fuzzySet1);

        let fuzzySet2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);
        fuzzyInput.addFuzzySet(fuzzySet2);

        fuzzyInput.setCrispInput(5.0);
        
        fuzzyInput.calculateFuzzySetPertinences();
        assert_eq!(fuzzyInput.fuzzySet(0).getPertinence(), 0.5);
        assert_eq!(fuzzyInput.fuzzySet(1).getPertinence(), 0.0);
    }
}
