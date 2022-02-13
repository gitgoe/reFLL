
  use crate::fuzzylogic::fuzzy_input::FuzzyInput;
  use crate::fuzzylogic::fuzzy_output::FuzzyOutput;
  use crate::fuzzylogic::fuzzy_rule::FuzzyRule;
  
  #[allow(non_snake_case)]
  pub struct Fuzzy{
    fuzzyInputArray: Vec<FuzzyInput>,
    fuzzyOutputArray: Vec<FuzzyOutput>,
    fuzzyRuleArray: Vec<FuzzyRule>,
  }

  impl Fuzzy {

    fn new() -> Fuzzy {
      Fuzzy{
            fuzzyInputArray: vec![],
            fuzzyOutputArray: vec![],
            fuzzyRuleArray: vec![]
      }
    }

    // Method to include a new FuzzyInput into Fuzzy
    fn add_fuzzyinput(&mut self, fuzzy_input: FuzzyInput) -> bool {
        self.fuzzyInputArray.push(fuzzy_input);
        true
    }

    // Method to include a new FuzzyOutput into Fuzzy
    fn add_fuzzyoutput(&mut self, fuzzy_output: FuzzyOutput) -> bool {
      self.fuzzyOutputArray.push(fuzzy_output);
      true
    }

    // Method to include a new FuzzyRule into Fuzzy
    fn add_fuzzyrule(&mut self, fuzzy_rule: FuzzyRule) -> bool {
      self.fuzzyRuleArray.push(fuzzy_rule);
      true
    }

    // Method to set a crisp value to one FuzzyInput
    fn set_input(&mut self, fuzzy_inputindex: i32, crisp_value: f32) -> bool {
     let fuzzy_input = self.fuzzyInputArray
      .iter_mut()
      .find(|fs| fs.get_index()== fuzzy_inputindex);
      if let Some(fi) = fuzzy_input {
        fi.set_crisp_input(crisp_value);
        return true;
      }
      // if no FuzzyInput was found, return false
      return false;
    }

    // Method to start the calculate of the result
    fn fuzzify(&mut self) -> bool {
      self.fuzzyInputArray.iter_mut().map(|fs| fs.reset_fuzzysets());
      self.fuzzyOutputArray.iter_mut().map(|fs| fs.reset_fuzzysets());
     // self.fuzzyRuleArray.iter().map(|fs| fs.reset_fuzzysets());
      return false;
    }

   
    // Method that clean all FuzzyInput from memory
    pub fn clean_fuzzyinputs(&mut self) -> usize {
      self.fuzzyInputArray.clear();
      return self.fuzzyInputArray.len();
    }

    // Method that clean all FuzzyOutput from memory
    pub fn clean_fuzzyoutputs(&mut self) -> usize {
      self.fuzzyOutputArray.clear();
      return self.fuzzyOutputArray.len();
    }

    // Method that clean all FuzzyRule from memory
    pub fn clean_fuzzyrules(&mut self) -> usize {
      self.fuzzyRuleArray.clear();
      return self.fuzzyRuleArray.len();
    }

    
}



  #[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
   

    #[test]
    fn test_new() {
        assert_eq!(true, true);
    }
}