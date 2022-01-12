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
        if (self.pertinence < pertinence){
            self.pertinence = pertinence;
        }
    }

    // Method to get the value of pertinence
    fn getPertinence(&self) -> f64 {
        self.pertinence
    }

    // Method to calculate the pertinence of the FuzzySet, according with the crispValue
    fn calculatePertinence(crispValue: f64)-> bool {
        true
    }

    // Method to reset the value of pertinence
    fn reset(& mut self){
        self.pertinence = 0.0;
    }
} 

