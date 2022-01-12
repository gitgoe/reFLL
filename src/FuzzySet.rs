#[allow(non_snake_case)]
pub struct FuzzySet{
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    pertinence: f64
}

impl FuzzySet {
    fn new(&mut self, a: f64, b: f64, c: f64, d: f64) -> FuzzySet {
        FuzzySet{
            a,b,c,d,pertinence:0.0
        }
    }

    // Method to get the value of point A
    fn getPointA(&self) -> f64 {
        self.a
    }

    // Method to get the value of point B
    fn getPointB(&self) -> f64 {
        self.b
    }

    // Method to get the value of point C
    fn getPointC(&self) -> f64 {
        self.c
    }

    // Method to get the value of point D
    fn getPointD(&self) -> f64 {
        self.d
    }

    // Method to set the value of pertinence
    fn setPertinence(& mut self, pertinence: f64) {
        self.pertinence= pertinence;
        // check if the new pertinence is bigger then the current value because it can be called more then once by different FuzzyRuleConsequent
        if self.pertinence < pertinence {
            self.pertinence = pertinence;
        }
    }

    // Method to get the value of pertinence
    fn getPertinence(&self) -> f64 {
        self.pertinence
    }

    // Method to calculate the pertinence of the FuzzySet, according with the crispValue
    fn calculatePertinence(& mut self, crispValue: f64)-> bool {
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
    fn reset(& mut self){
        self.pertinence = 0.0;
    }
} 

