
    #[allow(non_snake_case)]
    #[derive(Debug,Copy, Clone)]
    pub struct FuzzySet{
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        pertinence: f32
    }

    impl PartialEq for FuzzySet {
        fn eq(&self, other: &FuzzySet) -> bool {
            self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d
        }
    }

    impl FuzzySet {

        pub fn new(a: f32, b: f32, c: f32, d: f32) -> FuzzySet {
            FuzzySet{
                a,b,c,d,pertinence:0.0
            }
        }

        // Method to get the value of point A
        pub fn get_pointa(&self) -> f32 {
            self.a
        }

        // Method to get the value of point B
        pub fn get_pointb(&self) -> f32 {
            self.b
        }

        // Method to get the value of point C
        pub fn get_pointc(&self) -> f32 {
            self.c
        }

        // Method to get the value of point D
        pub fn get_pointd(&self) -> f32 {
            self.d
        }

        // Method to set the value of pertinence
        pub fn set_pertinence(& mut self, pertinence: f32) {
            self.pertinence= pertinence;
            // check if the new pertinence is bigger then the current value because it can be called more then once by different FuzzyRuleConsequent
            if self.pertinence < pertinence {
                self.pertinence = pertinence;
            }
        }

        // Method to get the value of pertinence
        pub fn get_pertinence(&self) -> f32 {
            self.pertinence
        }

        // Method to calculate the pertinence of the FuzzySet, according with the crispValue
        pub fn calculate_pertinence(& mut self, crisp_value: f32)-> bool {
            // check the crispValue is small then A
            if crisp_value < self.a {
                // check if this FuzzySet represents "everithing small is true"
                if self.a == self.b && self.b != self.c && self.c != self.d {
                    // if so, the pertinence is 1
                    self.pertinence = 1.0;
                }
                else
                {
                    // else, pertinence is 0
                    self.pertinence = 0.0;
                }
            }
            // check if the crispValue is between A and B
            else if crisp_value >= self.a && crisp_value < self.b {
                // calculate a slope
                let slope = 1.0 / (self.b - self.a);
                // calculate the value of pertinence
                self.pertinence = slope * (crisp_value - self.b) + 1.0;
            }
            // check if the pertinence is between B and C
            else if crisp_value >= self.b && crisp_value <= self.c{
                self.pertinence = 1.0;
            }
            // check if the pertinence is between C and D
            else if crisp_value > self.c && crisp_value <= self.d {
                // calculate a slope
                let slope = 1.0 / (self.c - self.d);
                // calculate the value of pertinence
                self.pertinence = slope * (crisp_value - self.c) + 1.0;
            }
            // check the crispValue is bigger then D
            else if crisp_value > self.d{
                // check if this FuzzySet represents "everithing bigger is true"
                if self.c == self.d && self.c != self.b && self.b != self.a
                {
                    // if so, the pertinence is 1
                    self.pertinence = 1.0;
                }
                else
                {
                    // else, pertinence is 0
                    self.pertinence = 0.0;
                }
            }
            return true;
        }

        // Method to reset the value of pertinence
        pub fn reset(& mut self){
            self.pertinence = 0.0;
        }
    } 

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    pub fn test_new() {
        let fuzzy_set:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);
        assert_eq!(fuzzy_set.get_pointa(), 0.0);
        assert_eq!(fuzzy_set.get_pointb(), 10.0);
        assert_eq!(fuzzy_set.get_pointc(), 20.0);
        assert_eq!(fuzzy_set.get_pointd(), 30.0);
    } 

    #[test]
    pub fn test_patial_eq() {
        let mut fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);
        fuzzy_set1.set_pertinence(5.0);

        let mut fuzzy_set2:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);
        fuzzy_set2.set_pertinence(10.0);
        assert_eq!(fuzzy_set1 == fuzzy_set2, true);
    } 

    #[test]
    pub fn test_set_pertinence() {
        let mut fuzzy_set:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);

        fuzzy_set.set_pertinence(0.8);
        assert_eq!(fuzzy_set.get_pertinence(), 0.8);  
    }

    #[test]
    pub fn test_calculate_and_get_pertinence(){

        let mut fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);

        fuzzy_set1.calculate_pertinence(-5.0);
        assert_eq!(fuzzy_set1.get_pertinence(), 0.0);

        fuzzy_set1.calculate_pertinence(5.0);
        assert_eq!(fuzzy_set1.get_pertinence(), 0.5);

        fuzzy_set1.calculate_pertinence(10.0);
        assert_eq!(fuzzy_set1.get_pertinence(), 1.0);

        fuzzy_set1.calculate_pertinence(15.0);
        assert_eq!(fuzzy_set1.get_pertinence(), 0.5);

        fuzzy_set1.calculate_pertinence(25.0);
        assert_eq!(fuzzy_set1.get_pertinence(), 0.0); 

        let mut fuzzy_set2:FuzzySet =  FuzzySet::new(0.0, 0.0, 20.0, 30.0);

        fuzzy_set2.calculate_pertinence(-5.0);
        assert_eq!(fuzzy_set2.get_pertinence(), 1.0);

        let mut fuzzy_set3:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 20.0);

        fuzzy_set3.calculate_pertinence(25.0);
        assert_eq!(fuzzy_set3.get_pertinence(), 1.0);
    }

    #[test]
    pub fn test_reset() {

        let mut fuzzy_set:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);

        fuzzy_set.set_pertinence(0.8);
        assert_eq!(fuzzy_set.get_pertinence(), 0.8);  

        fuzzy_set.reset();
        assert_eq!(fuzzy_set.get_pertinence(), 0.0);  
        
    }

    
}

