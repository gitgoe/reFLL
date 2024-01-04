mod fuzzylogic;

use simple_event_bus::{Event, EventBus, Subscriber};
use console::Term;
use crate::fuzzylogic::fuzzy::Fuzzy;
use crate::fuzzylogic::fuzzy_input::FuzzyInput;
use crate::fuzzylogic::fuzzy_output::FuzzyOutput;
use crate::fuzzylogic::fuzzy_rule::FuzzyRule;
use crate::fuzzylogic::fuzzy_ruleantecedent::FuzzyRuleAntecedent;
use crate::fuzzylogic::fuzzy_ruleconsequent::FuzzyRuleConsequent;
use crate::fuzzylogic::fuzzy_set::FuzzySet;

#[warn(non_snake_case)]
struct FuzzySubscriber {
    pub name: String,
    pub min: i32,
    pub max: i32,
    pub input_value:i32,
    pub fuzzylogic: Fuzzy,
}

impl FuzzySubscriber {
    pub fn new(name: String, min: i32, max:i32) -> FuzzySubscriber {

        let mut fuzzylogic = Fuzzy::new();

        // FuzzyInput
        let mut temperature:FuzzyInput =  FuzzyInput::new(1);

        let low:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        temperature.add_fuzzyset(low);

        let mean:FuzzySet =  FuzzySet::new(10.0, 20.0, 30.0, 40.0);
        temperature.add_fuzzyset(mean);

        let high:FuzzySet =  FuzzySet::new(30.0, 40.0, 10.0, 50.0);
        temperature.add_fuzzyset(high);

        fuzzylogic.add_fuzzyinput(temperature);

        // FuzzyOutput
        let mut climate:FuzzyOutput =  FuzzyOutput::new(1);

        let cold:FuzzySet =  FuzzySet::new(0.0, 10.0, 10.0, 20.0);
        climate.add_fuzzyset(cold);

        let good:FuzzySet =  FuzzySet::new(10.0, 20.0, 30.0, 40.0);
        climate.add_fuzzyset(good);

        let hot:FuzzySet =  FuzzySet::new(30.0, 40.0, 10.0, 50.0);
        climate.add_fuzzyset(hot);

        fuzzylogic.add_fuzzyoutput(climate);

        // Building FuzzyRule
        let mut if_temperature_low = FuzzyRuleAntecedent::new();
        if_temperature_low.join_single(Some(low));

        let mut then_climate_cold:FuzzyRuleConsequent =  FuzzyRuleConsequent::new();
        then_climate_cold.add_output(cold);

        let  fuzzyrule1 = FuzzyRule::new(1,Some(if_temperature_low), Some(then_climate_cold));

        fuzzylogic.add_fuzzyrule(fuzzyrule1);

        // Building FuzzyRule
        let mut if_temperature_mean = FuzzyRuleAntecedent::new();
        if_temperature_mean.join_single(Some(mean));

        let mut then_climate_good:FuzzyRuleConsequent =  FuzzyRuleConsequent::new();
        then_climate_good.add_output(good);

        let fuzzyrule2 = FuzzyRule::new(2,Some(if_temperature_mean), Some(then_climate_good));

        fuzzylogic.add_fuzzyrule(fuzzyrule2);

        // Building FuzzyRule
        let mut if_temperature_high = FuzzyRuleAntecedent::new();
        if_temperature_high.join_single(Some(high));

        let mut then_climate_hot:FuzzyRuleConsequent =  FuzzyRuleConsequent::new();
        then_climate_hot.add_output(cold);

        let fuzzyrule3 = FuzzyRule::new(3,Some(if_temperature_high), Some(then_climate_hot));

        fuzzylogic.add_fuzzyrule(fuzzyrule3);

        FuzzySubscriber { name, min, max, input_value:0, fuzzylogic }
    }
}

impl Subscriber for FuzzySubscriber {
    type Input = i32;

    fn on_event(&mut self, event: &Event<Self::Input>) {
        println!("{} received data: {}", self.name, event.get_data());
        self.input_value = self.input_value + event.get_data();
        self.fuzzylogic.set_input(1, self.input_value as f32);
        self.fuzzylogic.fuzzify();
        println!("input_value: {} defuzzify value: {}", self.input_value,  self.fuzzylogic.defuzzify(1));
    }
}

fn main() {
    println!("start FuzzySubscriber");

    let stdout = Term::buffered_stdout();
    let mut event_bus = EventBus::new();

    // We have to manually create and add each subscriber to the event bus.
    event_bus.subscribe_listener(FuzzySubscriber::new("Fuzzy-1".to_string(),0,40));

    'game_loop: loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                'w' =>  event_bus.publish(Event::new(1)),
                'a' =>  event_bus.publish(Event::new(-1)),
                's' =>  event_bus.publish(Event::new(2)),
                'd' =>  event_bus.publish(Event::new(-2)),
                _ => break 'game_loop,
            }
            event_bus.run();
        }
    }
}
