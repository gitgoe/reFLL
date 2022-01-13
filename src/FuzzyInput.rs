use crate::FuzzyIO::FuzzyIO;

#[allow(non_snake_case)]
pub struct FuzzyInput{
    index: i64,
    fuzzyIO: FuzzyIO
}

impl FuzzyInput {
    fn new(index: i64) -> FuzzyInput {
        FuzzyInput{
            index,
            fuzzyIO: FuzzyIO::new(index)
        }
    }
    // Method to calculate the pertinence of all FuzzySet
    fn calculateFuzzySetPertinences(& self, crispValue: f64)-> bool {

        let borrow = &mut *self.fuzzyIO.fuzzySetArray.borrow_mut();
        for val in borrow {
            println!("Got: {:?}", val);
        }
        
        /*
        // auxiliary variable to handle the operation
        fuzzySetArray *aux = this->fuzzySets;
        // while not in the end of the array, iterate
        while (aux != NULL)
        {
            // call calculatePertinence for each FuzzySet
            aux->fuzzySet->calculatePertinence(this->crispInput);
            aux = aux->next;
        }*/
        return true;
    }
}