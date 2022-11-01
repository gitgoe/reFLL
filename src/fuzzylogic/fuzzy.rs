
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
    pub fn add_fuzzyinput(&mut self, fuzzy_input: FuzzyInput) -> usize {
        self.fuzzyInputArray.push(fuzzy_input);
        return self.fuzzyInputArray.len();
    }

    // Method to include a new FuzzyOutput into Fuzzy
    pub fn add_fuzzyoutput(&mut self, fuzzy_output: FuzzyOutput) -> usize {
      self.fuzzyOutputArray.push(fuzzy_output);
      return self.fuzzyOutputArray.len();
    }

    // Method to include a new FuzzyRule into Fuzzy
    pub fn add_fuzzyrule(&mut self, fuzzy_rule: FuzzyRule) -> usize {
      self.fuzzyRuleArray.push(fuzzy_rule);
      return self.fuzzyRuleArray.len();
    }

    // Method to set a crisp value to one FuzzyInput
    pub fn set_input(&mut self, fuzzy_input_index: i32, crisp_value: f32) -> bool {
     let fuzzy_input = self.fuzzyInputArray
      .iter_mut()
      .find(|fs| fs.get_index()== fuzzy_input_index);
      if let Some(fi) = fuzzy_input {
        fi.set_crisp_input(crisp_value);
        return true;
      }
      // if no FuzzyInput was found, return false
      return false;
    }

    // Method to start the calculate of the result
    pub fn fuzzify(&mut self) -> bool {

      // for each FuzzyInput, reset its data
      self.fuzzyInputArray.iter_mut().for_each(|fi| fi.reset_fuzzysets());

      // for each FuzzyOutput, reset its data
      self.fuzzyOutputArray.iter_mut().for_each(|fo| fo.reset_fuzzysets());

      // calculate the pertinence of all FuzzyInput objects
      // for each FuzzyInput, calculate its pertinence
      self.fuzzyInputArray.iter_mut().map(|fi| fi.calculate_fuzzyset_pertinences()).collect::<Vec<_>>();

      self.update_fuzzyRule();

      // evaluate which rules were triggered
      // for each FuzzyRule, evaluate its expressions
      self.fuzzyRuleArray.iter_mut().map(|fr| fr.evaluate_expression()).collect::<Vec<_>>();

      self.update_fuzzyOutput();
      
      // to truncate the output sets
      // for each FuzzyOutput, truncate the result
      self.fuzzyOutputArray.iter_mut().map(|fo| fo.truncate()).collect::<Vec<_>>();

      return true;
    }

    pub fn update_fuzzyRule(&mut self){
      for fia in  self.fuzzyInputArray.iter() {
        for fs in fia.fuzzyIO.fuzzySetArray.iter(){
          println!("updated_fuzzyset : {:?}", fs);
          for fr in self.fuzzyRuleArray.iter_mut(){
            fr.fuzzyRuleAntecedent.as_mut().unwrap().update_fuzzysets(Some(*fs));
          }  
        }
      }
    }

    pub fn update_fuzzyOutput(&mut self){
      for fia in  self.fuzzyInputArray.iter() {
        for fs in fia.fuzzyIO.fuzzySetArray.iter(){
          println!("updated_fuzzyset : {:?}", fs);
          for fo in self.fuzzyOutputArray.iter_mut(){
            fo.update_fuzzyset(*fs);
          }  
        }
      }
    }

    // Method to verify if one specific FuzzyRule was triggered
    pub fn is_firedrule(&mut self, fuzzy_rule_index: i32 ) -> bool {
      
      // if the FuzzyRule index match with the desired
      if let Some(fired_ruled) =  self.fuzzyRuleArray.iter().find(|fr| fr.get_index() == fuzzy_rule_index ) {
        return fired_ruled.is_fired();
      }
      // if no FuzzyRule was found, return false
      return false;
    }

    // Method to retrieve the result of the process for one specific FuzzyOutput
    pub fn defuzzify(&mut self, fuzzy_output_index: i32) -> f32 {

      // if the FuzzyOutput index match with the desired
      if let Some(fuzzy_output) = self.fuzzyOutputArray.iter().find(|fo| fo.get_index() == fuzzy_output_index){
        // return the calculated result
        return fuzzy_output.get_crisp_output();
      }
      // if not found return 0.0
      return 0.0;
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
    use crate::fuzzylogic::fuzzy_set::FuzzySet;
    use crate::fuzzylogic::fuzzy_ruleconsequent::FuzzyRuleConsequent;
    use crate::fuzzylogic::fuzzy_ruleantecedent::FuzzyRuleAntecedent;
   

    #[test]
    fn test_add_fuzzyinput() {

      let mut fuzzy = Fuzzy::new();

      let mut fuzzy_input:FuzzyInput =  FuzzyInput::new(1);

      let fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
      fuzzy_input.add_fuzzyset(fuzzy_set1);

      let fuzzy_set2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);
      fuzzy_input.add_fuzzyset(fuzzy_set2);

      let fuzzy_set3:FuzzySet =  FuzzySet::new(20.0, 30.0, 30.0, 40.0);
      fuzzy_input.add_fuzzyset(fuzzy_set3);

      assert_eq!(fuzzy.add_fuzzyinput(fuzzy_input), 1); 
    }

    #[test]
    fn test_add_fuzzyoutput() {

      let mut fuzzy = Fuzzy::new();

      let mut fuzzy_output:FuzzyOutput =  FuzzyOutput::new(1);

      let fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
      fuzzy_output.add_fuzzyset(fuzzy_set1);

      let fuzzy_set2:FuzzySet =  FuzzySet::new(10.0, 20.0, 20.0, 30.0);
      fuzzy_output.add_fuzzyset(fuzzy_set2);

      let fuzzy_set3:FuzzySet =  FuzzySet::new(20.0, 30.0, 30.0, 40.0);
      fuzzy_output.add_fuzzyset(fuzzy_set3);

      assert_eq!(fuzzy.add_fuzzyoutput(fuzzy_output), 1); 
    }

    #[test]
    fn test_add_fuzzy_rule() {

      let mut fuzzy = Fuzzy::new();

      let mut fuzzy_ruleantecedent1 = FuzzyRuleAntecedent::new();
      let mut fuzzy_set1:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
      fuzzy_set1.set_pertinence(0.25);
      assert_eq!(fuzzy_ruleantecedent1.join_single(Some(fuzzy_set1)), true);

      let mut fuzzy_ruleantecedent2 = FuzzyRuleAntecedent::new();
      let mut fuzzy_set2:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
      fuzzy_set2.set_pertinence(0.75);
      assert_eq!(fuzzy_ruleantecedent2.join_single(Some(fuzzy_set2)), true);

      let mut fuzzy_ruleantecedent3 = FuzzyRuleAntecedent::new();
      assert_eq!(fuzzy_ruleantecedent3.join_with_and_with_frafra(Some(Box::new(fuzzy_ruleantecedent1)), Some(Box::new(fuzzy_ruleantecedent2))), true);
      
      let mut fuzzy_ruleconsequent:FuzzyRuleConsequent =  FuzzyRuleConsequent::new();
      let fuzzy_set3:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
      assert_eq!(fuzzy_ruleconsequent.add_output(fuzzy_set3), 1);
      
      let mut fuzzy_rule = FuzzyRule::new(1,Some(fuzzy_ruleantecedent3), Some(fuzzy_ruleconsequent));
      
      assert_eq!(fuzzy.add_fuzzyrule(fuzzy_rule), 1); 
    }

    #[test]
    fn test_setinput_and_fuzzify_and_isfiredrule_and_defuzzify() {

      let mut fuzzy = Fuzzy::new();

      // FuzzyInput
      let mut temperature:FuzzyInput =  FuzzyInput::new(1);

      let low:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
      temperature.add_fuzzyset(low);

      let mean:FuzzySet =  FuzzySet::new(10.0, 20.0, 30.0, 40.0);
      temperature.add_fuzzyset(mean);

      let high:FuzzySet =  FuzzySet::new(30.0, 40.0, 10.0, 50.0);
      temperature.add_fuzzyset(high);
   
      fuzzy.add_fuzzyinput(temperature);

      // FuzzyOutput
      let mut climate:FuzzyOutput =  FuzzyOutput::new(1);

      let cold:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
      climate.add_fuzzyset(cold);

      let good:FuzzySet =  FuzzySet::new(10.0, 20.0, 30.0, 40.0);
      climate.add_fuzzyset(good);

      let hot:FuzzySet =  FuzzySet::new(30.0, 40.0, 10.0, 50.0);
      climate.add_fuzzyset(hot);

      fuzzy.add_fuzzyoutput(climate);

      // Building FuzzyRule
      let mut if_temperature_low = FuzzyRuleAntecedent::new();
      if_temperature_low.join_single(Some(low));

      let mut then_climate_cold:FuzzyRuleConsequent =  FuzzyRuleConsequent::new();
      then_climate_cold.add_output(cold);

      let  fuzzyrule1 = FuzzyRule::new(1,Some(if_temperature_low), Some(then_climate_cold));
          
      fuzzy.add_fuzzyrule(fuzzyrule1);

      // Building FuzzyRule
      let mut if_temperature_mean = FuzzyRuleAntecedent::new();
      if_temperature_mean.join_single(Some(mean));

      let mut then_climate_good:FuzzyRuleConsequent =  FuzzyRuleConsequent::new();
      then_climate_good.add_output(good);

      let fuzzyrule2 = FuzzyRule::new(2,Some(if_temperature_mean), Some(then_climate_good));
          
      fuzzy.add_fuzzyrule(fuzzyrule2);

      // Building FuzzyRule
      let mut if_temperature_high = FuzzyRuleAntecedent::new();
      if_temperature_high.join_single(Some(high));

      let mut then_climate_hot:FuzzyRuleConsequent =  FuzzyRuleConsequent::new();
      then_climate_hot.add_output(cold);

      let fuzzyrule3 = FuzzyRule::new(3,Some(if_temperature_high), Some(then_climate_hot));
          
      assert_eq!(fuzzy.add_fuzzyrule(fuzzyrule3), 3);

      assert_eq!(fuzzy.set_input(1, 15.0), true);

      assert_eq!(fuzzy.fuzzify(), true);

      assert_eq!(fuzzy.is_firedrule(1), true);
      assert_eq!(fuzzy.is_firedrule(2), true);
      assert_eq!(fuzzy.is_firedrule(3), false);
  
      assert_eq!(fuzzy.defuzzify(1), 20.0); 
    }
}