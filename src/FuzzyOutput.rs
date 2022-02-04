use std::ops::DerefMut;
use std::ops::Deref;

use crate::fuzzyComposition::FuzzyComposition;
use crate::fuzzyIO::FuzzyIO;

const EPSILON_VALUE: f32 = 1.0E-3;

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

    // Method to run the calculate of FuzzyComposition and return the result
    pub fn getCrispOutput(&self) -> f32{
       return self.fuzzyComposition.calculate();
    }

    // Method to get the value (pointer) of fuzzyComposition
    pub fn getFuzzyComposition(&self) -> &FuzzyComposition {
        return &self.fuzzyComposition;
    }

    // Method to sort the FuzzySet by the reference of the point A in an ascending order
    pub fn order(&self) -> bool {
        // check if the point from the first is bigger the the second
        self.fuzzySetArray.borrow_mut().sort_by(|a,b| a.getPointA().partial_cmp(&b.getPointA()).unwrap());
        return true;
    }

    // Method to rebuild some point, the new point is calculated finding the intersection between two lines
    pub fn rebuild(&self, x1:f32, y1: f32, x2:f32,  y2:f32,  x3:f32,  y3:f32,  x4:f32, y4:f32, point:&mut f32, pertinence:&mut f32)  -> bool{
        // help variables
        let mut denom:f32 = 0.0;
        let mut numera:f32 = 0.0;
        let mut numerb:f32 = 0.0;
    
        let mut mua:f32 = 0.0;
        let mut mub:f32 = 0.0;
        // calculate the denominator and numerator
        denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
        numera = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
        numerb = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);

        // if negative, convert to positive
        if denom < 0.0 {
            denom *= -1.0;
        }
        // If the denominator is zero or close to it, it means that the lines are parallels, so return false for intersection
        if denom < EPSILON_VALUE {
            // return false for intersection
            return false;
        }
        // if negative, convert to positive
        if numera < 0.0 {
            numera *= -1.0;
        }
        // if negative, convert to positive
        if numerb < 0.0 {
            numerb *= -1.0;
        }
        // verify if has intersection between the segments
        mua = numera / denom;
        mub = numerb / denom;
        if mua < 0.0 || mua > 1.0 || mub < 0.0 || mub > 1.0 {
            // return false for intersection
            return false;
        } else {
            // calculate and setting the new point and pertinence
            *point = x1 + mua * (x2 - x1);
            *pertinence = y1 + mua * (y2 - y1);
            // return true for intersection
            return true;
        }
    }

    pub fn truncate(&mut self) -> bool {
        
        // reset fuzzyComposition object
        self.fuzzyComposition.empty();

        // while not in the end of the array, iterate
        for aux in self.fuzzyIO.fuzzySetArray.borrow_mut().iter() {
            
            println!("aux: {:?}", aux);
        
            // if the FuzzySet was trigged (has some pertinence)
            if aux.getPertinence() > 0.0 {
                
                // Check if it is not a "trapeze" without its left triangle or singleton, before include the point A
                if aux.getPointA() != aux.getPointB(){
                    // include it
                    self.fuzzyComposition.addPoint(aux.getPointA(), 0.0);
                }
                // check if it is a triangle (B == C) and (A <> D)
                if aux.getPointB() == aux.getPointC() && aux.getPointA() != aux.getPointD(){
                    // check if the pertinence is the max
                    if aux.getPertinence() == 1.0 {
                        // include it (it will replace previous point if left triangle)
                        self.fuzzyComposition.addPoint(aux.getPointB(), aux.getPertinence());
                        // include it (it will replace previous point if right triangle)
                        self.fuzzyComposition.addPoint(aux.getPointD(), 0.0);
                    }
                    // if the pertinence is below the max, and it is a triangle, calculate the new point B and C
                    else
                    {
                        // rebuild the new point finding the intersection of two lines, the first is the segment from A to B (pertinence here is the y) and the segment of truncate, from A to D
                        // initiate a new point with current values of B (here it does matters, it always will be changed)
                        let mut newPointB:f32 = aux.getPointB();
                        let mut newPertinenceB:f32 = aux.getPertinence();
                        // only if a regular triangle
                        self.rebuild(aux.getPointA(), 0.0, aux.getPointB(), 1.0, aux.getPointA(), aux.getPertinence(), aux.getPointD(), aux.getPertinence(), &mut newPointB, &mut newPertinenceB);
                        // include it
                        self.fuzzyComposition.addPoint(newPointB, newPertinenceB);
                        // rebuild the new point finding the intersection of two lines, the second is the segment from C to D (pertinence here is the y) and the segment of truncate, from A to D
                        // initiate a new point with current values of C (here it does matters, it always will be changed)
                        let mut newPointC = aux.getPointC();
                        let mut newPertinenceC = aux.getPertinence();
                        // only if a regular triangle
                        self.rebuild(aux.getPointC(), 1.0, aux.getPointD(), 0.0, aux.getPointA(), aux.getPertinence(), aux.getPointD(), aux.getPertinence(), &mut newPointC, &mut newPertinenceC);
                        // include it
                        self.fuzzyComposition.addPoint(newPointC, newPertinenceC);
                
                    }
                }
                // if until now, it was not a triangle
                // check if (B <> C), if true, it is a trapeze (this code is the same of the triangle, except when the pertinence is 1.0, here we include the two points [B and C], because they are not equal)
                else if aux.getPointB() != aux.getPointC() {
                    // check if the pertinence is the max
                    if aux.getPertinence() == 1.0 {
                        // include it
                        self.fuzzyComposition.addPoint(aux.getPointB(), aux.getPertinence());
                        // include it
                        self.fuzzyComposition.addPoint(aux.getPointC(), aux.getPertinence());
                    }
                    // if the pertinence is below the max, and it is a triangle, calculate the new point B and C
                    else
                    {
                        // initiate a new point with current values of B
                        let mut newPointB = aux.getPointB();
                        let mut newPertinenceB = aux.getPertinence();
                        // rebuild the new point finding the intersection of two lines, the first is the segment from A to B (pertinence here is the y) and the segment of truncate, from A to D
                        self.rebuild(aux.getPointA(), 0.0, aux.getPointB(), 1.0, aux.getPointA(), aux.getPertinence(), aux.getPointD(), aux.getPertinence(), &mut newPointB, &mut newPertinenceB);
                        // include it
                        self.fuzzyComposition.addPoint(newPointB, newPertinenceB);
                        // initiate a new point with current values of C
                        let mut  newPointC = aux.getPointC();
                        let mut  newPertinenceC = aux.getPertinence();
                        // rebuild the new point finding the intersection of two lines, the first is the segment from C to D (pertinence here is the y) and the segment of truncate, from A to D
                        self.rebuild(aux.getPointC(), 1.0, aux.getPointD(), 0.0, aux.getPointA(), aux.getPertinence(), aux.getPointD(), aux.getPertinence(), &mut newPointC, &mut newPertinenceC);
                        // include it
                        self.fuzzyComposition.addPoint(newPointC, newPertinenceC);
                    }
                }
                // if it is not a triangle non a trapeze, it is a singleton
                else
                {
                    // include it
                    self.fuzzyComposition.addPoint(aux.getPointB(), 0.0);
                    // include it
                    self.fuzzyComposition.addPoint(aux.getPointB(), aux.getPertinence());
                    // include it
                    self.fuzzyComposition.addPoint(aux.getPointB(), 0.0);
                }
                // Check if it is not a "trapeze" without its right triangle or singleton, before include the point D
                if aux.getPointC() != aux.getPointD()
                {
                    // include it
                    self.fuzzyComposition.addPoint(aux.getPointD(), 0.0);
                }
            }
            
        }

        // call build from FuzzyComposition for its self building
        self.fuzzyComposition.build();

        return true;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::fuzzySet::FuzzySet;

    #[test]
    fn test_FuzzyOutput() {
        let  fuzzyOutput:FuzzyOutput =  FuzzyOutput::new(1);
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

        fuzzySet.setPertinence(0.242);

        assert_eq!(fuzzySet.getPertinence(), 0.242);
        
        assert_eq!(fuzzyOutput.addFuzzySet(fuzzySet), 1);

        fuzzyOutput.resetFuzzySets();

        assert_eq!(fuzzyOutput.fuzzySet(0).getPertinence(), 0.0);

    }

    #[test]
    fn test_truncateAndGetCrispOutputAndGetFuzzy() {
       
        let mut fuzzyOutput:FuzzyOutput =  FuzzyOutput::new(1);

        assert_eq!(fuzzyOutput.getIndex(), 1);

        let mut fuzzySet1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);

        fuzzySet1.setPertinence(1.0);

        assert_eq!(fuzzyOutput.addFuzzySet(fuzzySet1), 1);

        let mut fuzzySet2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);

        fuzzySet2.setPertinence(1.0);
        
        assert_eq!(fuzzyOutput.addFuzzySet(fuzzySet2), 2);

        let mut fuzzySet3:FuzzySet =  FuzzySet::new(20.0, 30.0, 30.0, 40.0);

        fuzzySet3.setPertinence(1.0);
        
        assert_eq!(fuzzyOutput.addFuzzySet(fuzzySet3), 3);

        assert_eq!(fuzzyOutput.truncate(), true);

        let fuzzyComposition = fuzzyOutput.getFuzzyComposition();

        assert_eq!(fuzzyComposition.countPoints(), 8);

    }

}