#[allow(non_snake_case)]
#[derive(Debug,Copy, Clone)]
pub struct FuzzySet{
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    pertinence: f32
}

impl FuzzySet {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> FuzzySet {
        FuzzySet{
            a,b,c,d,pertinence:0.0
        }
    }

    // Method to get the value of point A
    pub fn getPointA(&self) -> f32 {
        self.a
    }

    // Method to get the value of point B
    pub fn getPointB(&self) -> f32 {
        self.b
    }

    // Method to get the value of point C
    pub fn getPointC(&self) -> f32 {
        self.c
    }

    // Method to get the value of point D
    pub fn getPointD(&self) -> f32 {
        self.d
    }

    // Method to set the value of pertinence
    pub fn setPertinence(& mut self, pertinence: f32) {
        self.pertinence= pertinence;
        // check if the new pertinence is bigger then the current value because it can be called more then once by different FuzzyRuleConsequent
        if self.pertinence < pertinence {
            self.pertinence = pertinence;
        }
    }

    // Method to get the value of pertinence
    pub fn getPertinence(&self) -> f32 {
        self.pertinence
    }

    // Method to calculate the pertinence of the FuzzySet, according with the crispValue
    pub fn calculatePertinence(& mut self, crispValue: f32)-> bool {
        // check the crispValue is small then A
        if crispValue < self.a {
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
        else if crispValue >= self.a && crispValue < self.b {
            // calculate a slope
            let slope = 1.0 / (self.b - self.a);
            // calculate the value of pertinence
            self.pertinence = slope * (crispValue - self.b) + 1.0;
        }
        // check if the pertinence is between B and C
        else if crispValue >= self.b && crispValue <= self.c{
            self.pertinence = 1.0;
        }
        // check if the pertinence is between C and D
        else if crispValue > self.c && crispValue <= self.d {
            // calculate a slope
            let slope = 1.0 / (self.c - self.d);
            // calculate the value of pertinence
            self.pertinence = slope * (crispValue - self.c) + 1.0;
        }
        // check the crispValue is bigger then D
        else if crispValue > self.d{
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
        let fuzzySet:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 30.0);
        assert_eq!(fuzzySet.getPointA(), 0.0);
        assert_eq!(fuzzySet.getPointB(), 10.0);
        assert_eq!(fuzzySet.getPointC(), 20.0);
        assert_eq!(fuzzySet.getPointD(), 30.0);
    }

    #[test]
    fn test_calculateAndGetPertinence(){

        let mut fuzzySet1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);

        fuzzySet1.calculatePertinence(-5.0);
        assert_eq!(fuzzySet1.getPertinence(), 0.0);

        fuzzySet1.calculatePertinence(5.0);
        assert_eq!(fuzzySet1.getPertinence(), 0.5);

        fuzzySet1.calculatePertinence(10.0);
        assert_eq!(fuzzySet1.getPertinence(), 1.0);

        fuzzySet1.calculatePertinence(15.0);
        assert_eq!(fuzzySet1.getPertinence(), 0.5);

        fuzzySet1.calculatePertinence(25.0);
        assert_eq!(fuzzySet1.getPertinence(), 0.0); 

        let mut fuzzySet2:FuzzySet =  FuzzySet::new(0.0, 0.0, 20.0, 30.0);

        fuzzySet2.calculatePertinence(-5.0);
        assert_eq!(fuzzySet2.getPertinence(), 1.0);

        let mut fuzzySet3:FuzzySet =  FuzzySet::new(0.0, 10.0, 20.0, 20.0);

        fuzzySet3.calculatePertinence(25.0);
        assert_eq!(fuzzySet3.getPertinence(), 1.0);
    }

    
}

