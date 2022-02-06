
pub mod fuzzylogic{

    use std::ops::DerefMut;
    use std::ops::Deref;

    use crate::fuzzy_composition::fuzzylogic::FuzzyComposition;
    use crate::fuzzy_io::fuzzylogic::FuzzyIO;

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
        pub fn get_crisp_output(&self) -> f32{
        return self.fuzzyComposition.calculate();
        }

        // Method to get the value (pointer) of fuzzyComposition
        pub fn get_fuzzy_composition(&self) -> &FuzzyComposition {
            return &self.fuzzyComposition;
        }

        // Method to sort the FuzzySet by the reference of the point A in an ascending order
        pub fn order(&mut self) -> bool {
            // check if the point from the first is bigger the the second
            self.fuzzySetArray.sort_by(|a,b| a.get_pointa().partial_cmp(&b.get_pointa()).unwrap());
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
            for aux in self.fuzzyIO.fuzzySetArray.iter() {
                
                println!("aux: {:?}", aux);
            
                // if the FuzzySet was trigged (has some pertinence)
                if aux.get_pertinence() > 0.0 {
                    
                    // Check if it is not a "trapeze" without its left triangle or singleton, before include the point A
                    if aux.get_pointa() != aux.get_pointb(){
                        // include it
                        self.fuzzyComposition.add_point(aux.get_pointa(), 0.0);
                    }
                    // check if it is a triangle (B == C) and (A <> D)
                    if aux.get_pointb() == aux.get_pointc() && aux.get_pointa() != aux.get_pointd(){
                        // check if the pertinence is the max
                        if aux.get_pertinence() == 1.0 {
                            // include it (it will replace previous point if left triangle)
                            self.fuzzyComposition.add_point(aux.get_pointb(), aux.get_pertinence());
                            // include it (it will replace previous point if right triangle)
                            self.fuzzyComposition.add_point(aux.get_pointd(), 0.0);
                        }
                        // if the pertinence is below the max, and it is a triangle, calculate the new point B and C
                        else
                        {
                            // rebuild the new point finding the intersection of two lines, the first is the segment from A to B (pertinence here is the y) and the segment of truncate, from A to D
                            // initiate a new point with current values of B (here it does matters, it always will be changed)
                            let mut new_point_b:f32 = aux.get_pointb();
                            let mut new_pertinence_b:f32 = aux.get_pertinence();
                            // only if a regular triangle
                            self.rebuild(aux.get_pointa(), 0.0, aux.get_pointb(), 1.0, aux.get_pointa(), aux.get_pertinence(), aux.get_pointd(), aux.get_pertinence(), &mut new_point_b, &mut new_pertinence_b);
                            // include it
                            self.fuzzyComposition.add_point(new_point_b, new_pertinence_b);
                            // rebuild the new point finding the intersection of two lines, the second is the segment from C to D (pertinence here is the y) and the segment of truncate, from A to D
                            // initiate a new point with current values of C (here it does matters, it always will be changed)
                            let mut new_point_c = aux.get_pointc();
                            let mut new_pertinence_c= aux.get_pertinence();
                            // only if a regular triangle
                            self.rebuild(aux.get_pointc(), 1.0, aux.get_pointd(), 0.0, aux.get_pointa(), aux.get_pertinence(), aux.get_pointd(), aux.get_pertinence(), &mut new_point_c, &mut new_pertinence_c);
                            // include it
                            self.fuzzyComposition.add_point(new_point_c, new_pertinence_c);
                    
                        }
                    }
                    // if until now, it was not a triangle
                    // check if (B <> C), if true, it is a trapeze (this code is the same of the triangle, except when the pertinence is 1.0, here we include the two points [B and C], because they are not equal)
                    else if aux.get_pointb() != aux.get_pointc() {
                        // check if the pertinence is the max
                        if aux.get_pertinence() == 1.0 {
                            // include it
                            self.fuzzyComposition.add_point(aux.get_pointb(), aux.get_pertinence());
                            // include it
                            self.fuzzyComposition.add_point(aux.get_pointc(), aux.get_pertinence());
                        }
                        // if the pertinence is below the max, and it is a triangle, calculate the new point B and C
                        else
                        {
                            // initiate a new point with current values of B
                            let mut new_point_b = aux.get_pointb();
                            let mut new_pertinence_b = aux.get_pertinence();
                            // rebuild the new point finding the intersection of two lines, the first is the segment from A to B (pertinence here is the y) and the segment of truncate, from A to D
                            self.rebuild(aux.get_pointa(), 0.0, aux.get_pointb(), 1.0, aux.get_pointa(), aux.get_pertinence(), aux.get_pointd(), aux.get_pertinence(), &mut new_point_b, &mut new_pertinence_b);
                            // include it
                            self.fuzzyComposition.add_point(new_point_b, new_pertinence_b);
                            // initiate a new point with current values of C
                            let mut  new_point_c = aux.get_pointc();
                            let mut  new_pertinence_c = aux.get_pertinence();
                            // rebuild the new point finding the intersection of two lines, the first is the segment from C to D (pertinence here is the y) and the segment of truncate, from A to D
                            self.rebuild(aux.get_pointc(), 1.0, aux.get_pointd(), 0.0, aux.get_pointa(), aux.get_pertinence(), aux.get_pointd(), aux.get_pertinence(), &mut new_point_c, &mut new_pertinence_c);
                            // include it
                            self.fuzzyComposition.add_point(new_point_c, new_pertinence_c);
                        }
                    }
                    // if it is not a triangle non a trapeze, it is a singleton
                    else
                    {
                        // include it
                        self.fuzzyComposition.add_point(aux.get_pointb(), 0.0);
                        // include it
                        self.fuzzyComposition.add_point(aux.get_pointb(), aux.get_pertinence());
                        // include it
                        self.fuzzyComposition.add_point(aux.get_pointb(), 0.0);
                    }
                    // Check if it is not a "trapeze" without its right triangle or singleton, before include the point D
                    if aux.get_pointc() != aux.get_pointd()
                    {
                        // include it
                        self.fuzzyComposition.add_point(aux.get_pointd(), 0.0);
                    }
                }
                
            }

            // call build from FuzzyComposition for its self building
            //self.fuzzyComposition.build();

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
    fn test_fuzzy_output() {
        let  fuzzy_output:FuzzyOutput =  FuzzyOutput::new(1);
        assert_eq!(fuzzy_output.get_index(), 1);
    }

    #[test]
    fn test_set_crisp_input_and_get_crisp_input() {
        let mut fuzzy_output:FuzzyOutput =  FuzzyOutput::new(1);
        fuzzy_output.set_crisp_input(10.190);
        assert_eq!(fuzzy_output.get_crisp_input(), 10.190);
    }

    #[test]
    fn test_add_fuzzy_set_and_reset_fuzzy_sets() {
       
        let mut fuzzy_output:FuzzyOutput =  FuzzyOutput::new(1);

        let mut fuzzy_set:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);

        fuzzy_set.set_pertinence(0.242);

        assert_eq!(fuzzy_set.get_pertinence(), 0.242);
        
        assert_eq!(fuzzy_output.add_fuzzyset(fuzzy_set), 1);

        fuzzy_output.reset_fuzzysets();

        assert_eq!(fuzzy_output.fuzzyset(0).get_pertinence(), 0.0);

    }

    #[test]
    fn test_truncate_and_get_crisp_output_and_get_fuzzy() {
       
        let mut fuzzy_output:FuzzyOutput =  FuzzyOutput::new(1);

        assert_eq!(fuzzy_output.get_index(), 1);

        let mut fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);

        fuzzy_set1.set_pertinence(1.0);

        assert_eq!(fuzzy_output.add_fuzzyset(fuzzy_set1), 1);

        let mut fuzzy_set2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);

        fuzzy_set2.set_pertinence(1.0);
        
        assert_eq!(fuzzy_output.add_fuzzyset(fuzzy_set2), 2);

        let mut fuzzy_set3:FuzzySet =  FuzzySet::new(20.0, 30.0, 30.0, 40.0);

        fuzzy_set3.set_pertinence(1.0);
        
        assert_eq!(fuzzy_output.add_fuzzyset(fuzzy_set3), 3);

        assert_eq!(fuzzy_output.truncate(), true);

        let fuzzy_composition = fuzzy_output.get_fuzzy_composition();

        //assert_eq!(fuzzy_composition.count_points(), 8);

    }

}