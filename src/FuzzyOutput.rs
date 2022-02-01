
use std::ops::DerefMut;
use std::ops::Deref;
use crate::fuzzyComposition::FuzzyComposition;
use crate::fuzzyIO::FuzzyIO;

#[allow(non_snake_case)]
pub struct FuzzyOutput{
    index: i32,
    fuzzyComposition:FuzzyComposition,
    fuzzyIO: FuzzyIO
} 

impl Deref for FuzzyOutput {
    type Target = FuzzyIO;
    fn deref(&self) -> &FuzzyIO { &self.fuzzyIO }
}

impl DerefMut for FuzzyOutput {
    fn deref_mut(&mut self) -> &mut FuzzyIO { &mut self.fuzzyIO }
}

impl FuzzyOutput {
    pub fn new(index: i32) -> FuzzyOutput {
        FuzzyOutput{
            index,
            fuzzyIO: FuzzyIO::new(index),
            fuzzyComposition: FuzzyComposition::new()
        }
    }

    fn getCrispOutput(&self) -> f32{
       return self.fuzzyComposition.calculate();
    }
    
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::fuzzySet::FuzzySet;

    #[test]
    fn test_FuzzyOutput() {
        let mut fuzzyOutput:FuzzyOutput =  FuzzyOutput::new(1);
        assert_eq!(fuzzyOutput.getIndex(), 1);
    }

    #[test]
    fn test_setCrispInputAndGetCrispInput() {
        let mut fuzzyOutput:FuzzyOutput =  FuzzyOutput::new(1);
        fuzzyOutput.setCrispInput(10.190);
        assert_eq!(fuzzyOutput.getCrispInput(), 10.190);
    }

    #[test]
    fn test_addFuzzySetAndResetFuzzySets() {
        let mut fuzzyOutput:FuzzyOutput =  FuzzyOutput::new(1);

        let mut fuzzySet:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        
        assert_eq!(fuzzyOutput.addFuzzySet(fuzzySet), 1);

        fuzzySet.setPertinence(0.242);
        assert_eq!(fuzzySet.getPertinence(), 0.242);

        fuzzyOutput.resetFuzzySets();

        //assert_eq!(fuzzySet.getPertinence(), 0.0);

    }
/*
    TEST(FuzzyOutput, addFuzzySetAndResetFuzzySets)
{
    FuzzyOutput *fuzzyOutput = new FuzzyOutput(1);

    FuzzySet *fuzzySetTest = new FuzzySet(0, 10, 10, 20);

    ASSERT_TRUE(fuzzyOutput->addFuzzySet(fuzzySetTest));

    fuzzySetTest->setPertinence(0.242);
    ASSERT_FLOAT_EQ(0.242, fuzzySetTest->getPertinence());

    fuzzyOutput->resetFuzzySets();

    ASSERT_FLOAT_EQ(0.0, fuzzySetTest->getPertinence());
}*/
}