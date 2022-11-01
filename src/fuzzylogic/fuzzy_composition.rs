
    extern crate iterslide;

    extern crate itertools;


    use itertools::Itertools; 

    use std::fmt;

    const EPSILON_VALUE: f32 = 1.0E-3;

    use uuid::Uuid;
   
    #[allow(non_snake_case)]
    #[derive(Copy,Clone)]
    pub struct PointArray{ 
        point: f32,
        pertinence: f32,
        uuid: Option<Uuid>,
    }

    impl PointArray {
        fn is_previous_greater(&self, next: &PointArray) -> bool {
            self.point> next.point
        }
    }

    impl PartialEq for PointArray {
        fn eq(&self, other: &Self) -> bool {
            self.point == other.point && self.pertinence == other.pertinence
        }
    }

    impl fmt::Debug for PointArray {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("point")
            .field("point", &self.point)
            .field("pertinence", &self.pertinence)
            .field("uuid", &self.uuid.unwrap().as_urn())
            .finish()
        }
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Clone)]
    pub struct FuzzyComposition{
        points : Vec<PointArray>
    }
    impl FuzzyComposition{

        pub fn new() -> FuzzyComposition {
            FuzzyComposition{
                points: vec![]
            }
        }

        // Method to include a new pointsArray struct into FuzzyComposition
        pub fn add_point(&mut self, point: f32, pertinence: f32) -> bool{
            let uuid = Some(Uuid::new_v4());
            self.points.push(PointArray{point, pertinence, uuid});
            true
        }

        // Method to check if FuzzyComposition contains an specific point and pertinence
        pub fn check_point(&self, point: f32, pertinence: f32) -> bool{
            let uuid = None;
            self.points.contains(&PointArray{point, pertinence, uuid})
        }

        
        // Method to search intersection between two segments, if found, create a new pointsArray (in found intersection) and remove not necessary ones
        pub fn rebuild(&self,  a_segment_begin: PointArray, a_segment_end: PointArray, b_segment_begin: PointArray, b_segment_end: PointArray) -> Option<PointArray>{
            
            // create a reference for each point
            let x1 = a_segment_begin.point;
            let y1 = a_segment_begin.pertinence;
            let x2 = a_segment_end.point;
            let y2 = a_segment_end.pertinence;
            let x3 = b_segment_begin.point;
            let y3 = b_segment_begin.pertinence;
            let x4 = b_segment_end.point;
            let y4 = b_segment_end.pertinence;

            println!("rebuild x1:{}  y1:{}",x1, y1);
            println!("rebuild x2:{}  y2:{}",x2, y2);
            println!("rebuild x3:{}  y3:{}",x3, y3);
            println!("rebuild x4:{}  y4:{}",x4, y4);
            println!("==========================");

            // calculate the denominator and numerator
            let mut denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
            let mut numera = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
            let mut numerb = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);

            // if negative, convert to positive
            if denom < 0.0 {
                denom *= -1.0;
            }
            // If the denominator is zero or close to it, it means that the lines are parallels, so return false for intersection
            if denom < EPSILON_VALUE {
                // return false for intersection
                println!("return false for denom < EPSILON_VALUE");
                return None;
            }
            // if negative, convert to positive
            if numera < 0.0 {
                numera *= -1.0;
            }
            // if negative, convert to positive
            if numerb < 0.0{
                numerb *= -1.0;
            }
            // verify if has intersection between the segments
            let mua = numera / denom;
            let mub = numerb / denom;

            if mua <= 0.0 || mua >= 1.0 || mub <= 0.0 || mub >= 1.0 {
                // return false for intersection
                println!("return false for non intersection");
                return None;
            } else {
                // we found an intersection
                // calculate the point (y) and its pertinence (y) for the new element (pointsArray)
                let point = x1 + mua * (x2 - x1);
                let pertinence = y1 + mua * (y2 - y1);

                let uuid = Some(Uuid::new_v4());
                let aux = Some(PointArray{point, pertinence, uuid});
                println!("add the new intersection point: {:?}", aux);
                return aux;
            }
            
        }

        pub fn calculate(&self) -> f32{

            let mut numerator = 0.0;
            let mut denominator = 0.0;

            for ( current, next) in self.points.clone().into_iter().tuple_windows() {

                println!("current:{:?} -- next:{:?}", current, next);

                let mut area = 0.0;
                let mut  middle = 0.0;

                // if it is a singleton
                if current.pertinence != next.pertinence && current.point == next.point {
                    
                    // enter in all points of singleton, but calculate only once
                    if current.pertinence > 0.0 {
                        area = current.pertinence;
                        middle = current.point;
                    }
                }
                // if a triangle (Not properly a membership function)
                else if current.pertinence == 0.0 || next.pertinence == 0.0 {
                    
                    let mut pertinence =0.0;

                    if current.pertinence > 0.0
                    {
                        pertinence = current.pertinence;
                    }else {
                        pertinence = next.pertinence;
                    }

                    area = ((next.point - current.point) * pertinence) / 2.0;
                    
                    if current.pertinence < next.pertinence{
                        middle = ((next.point - current.point) / 1.5) + current.point;
                    } else {
                        middle = ((next.point - current.point) / 3.0) + current.point;
                    }
                }
                // else if a square (Not properly a membership function)
                else if (current.pertinence > 0.0 && next.pertinence > 0.0) && current.pertinence == next.pertinence{
                    area = (next.point - current.point) * current.pertinence;
                    middle = ((next.point - current.point) / 2.0) + current.point; 
                }
                // else if a trapeze (Not properly a membership function)
                else if (current.pertinence > 0.0 && next.pertinence > 0.0) && current.pertinence != next.pertinence{
                    area = ((current.pertinence + next.pertinence) / 2.0) * (next.point - current.point);
                    middle = ((next.point - current.point) / 2.0) + current.point;
                }
                numerator += middle * area;
                denominator += area;
            } // end loop

            // avoiding zero division
            if denominator == 0.0 {
                return 0.0;
            } else {
                return (numerator / denominator).ceil();
            }  
        }

        pub fn empty(&mut self) -> bool{
            self.points.clear();
            self.points.is_empty()
        }

        pub fn count_points(&self) -> usize {
            self.points.len()
        }

        pub fn clean_points(&mut self) {
            self.points.clear();
        }

        pub fn rmv_point(&mut self, point: PointArray){
            if let Some(index) = self.points.iter().position(|p| p.uuid == point.uuid) {
                self.points.remove(index);
            }
        }

        pub fn dump_points(&mut self) {
            for (pos, current) in self.points.iter().enumerate() {
                println!("dump point at position {}: {:?}", pos, current);
            }   
        }

        // Method to iterate over the pointsArray, detect possible intersections and sent these points for "correction"
        pub fn build(&mut self) -> bool{
            let mut previous: Option<PointArray> = None;
            let mut pos = 0;
            while pos < self.points.len(){
                let  current: PointArray = self.points[pos];
                match previous {
                    Some(p) => {
                        // check if the previous point is greater then the current
                        let  is_greater = p.is_previous_greater(&current);
                         // if yes, use this point
                        if is_greater {

                            println!("previus {:?} is greater then the current {:?} at index: {}", p, current,pos); 
                            
                            let a_segment_begin = self.points[pos];
                            let a_segment_end = self.points[pos+1];
                            let b_segment_begin = self.points[pos-1];
                            let b_segment_end = self.points[pos-2];

                            // insert the fixed point
                            if let Some(fixed_point) = self.rebuild(a_segment_begin, a_segment_end, b_segment_begin, b_segment_end){
                               
                                // delete current et previus pointsArray
                                println!("1# remove: {:?}", a_segment_begin);
                                println!("1# remove: {:?}", b_segment_begin);
                                self.rmv_point(a_segment_begin);
                                self.rmv_point(b_segment_begin);
                               
                                 // insert new point
                                 println!("add new point: {:?}", fixed_point);
                                 self.points.insert(pos-1,fixed_point);
                                 break;   
                            }
                        
                            let a_segment_begin = self.points[pos];
                            let a_segment_end = self.points[pos+1];
                            let b_segment_begin = self.points[pos-2];
                            let b_segment_end = self.points[pos-3];

                               // insert the fixed point
                            if let Some(fixed_point) = self.rebuild(a_segment_begin, a_segment_end, b_segment_begin, b_segment_end){
                               
                                // delete current et previus pointsArray
                                println!("2# remove: {:?}", b_segment_begin);
                                println!("2# remove: {:?}", a_segment_begin);
                                println!("2# remove previus: {:?}", p);
                                self.rmv_point(a_segment_begin);
                                self.rmv_point(b_segment_begin);
                                self.rmv_point(p);
                        
                                 // insert new point
                                println!("add new point: {:?}", fixed_point);
                                self.points.insert(pos-2,fixed_point);
                                break;
                            }
                        }
                    }
                    None => {}
                }
                previous = Some(current);
                pos+=1;
                println!("index={} {:?}",pos, current);
            }
             // return to zero and build again until the end
            if pos < self.points.len() {
                self.build();
            } 
           
            return true;
        }

    }


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    pub fn test_loop() {
        let mut fuzzy_composition:FuzzyComposition =  FuzzyComposition::new();

        fuzzy_composition.add_point( 0.0  , 0.0);
        fuzzy_composition.add_point( 10.0 , 1.0);
        fuzzy_composition.add_point( 20.0 , 0.0);
        fuzzy_composition.add_point( 20.0 , 0.0);
        fuzzy_composition.add_point( 10.0 , 0.0);
        fuzzy_composition.add_point( 20.0 , 1.0);
        fuzzy_composition.add_point( 30.0 , 0.0);
        fuzzy_composition.add_point( 30.0 , 0.0);
        fuzzy_composition.add_point( 20.0 , 0.0);
        fuzzy_composition.add_point( 30.0 , 1.0);
        fuzzy_composition.add_point( 40.0 , 0.0);
        fuzzy_composition.add_point( 40.0 , 0.0);

        assert_eq!(fuzzy_composition.build(), true);
    }
    #[test]
    pub fn test_build3() {
        let mut fuzzy_composition:FuzzyComposition =  FuzzyComposition::new();

        fuzzy_composition.add_point( 0.0  , 0.0);
        fuzzy_composition.add_point( 10.0 , 1.0);
        fuzzy_composition.add_point( 20.0 , 0.0);
        fuzzy_composition.add_point( 20.0 , 0.0);
        fuzzy_composition.add_point( 10.0 , 0.0);
        fuzzy_composition.add_point( 20.0 , 1.0);
        fuzzy_composition.add_point( 30.0 , 0.0);
        fuzzy_composition.add_point( 30.0 , 0.0);
        fuzzy_composition.add_point( 20.0 , 0.0);
        fuzzy_composition.add_point( 30.0 , 1.0);
        fuzzy_composition.add_point( 40.0 , 0.0);
        fuzzy_composition.add_point( 40.0 , 0.0);

        assert_eq!(fuzzy_composition.count_points(), 12);
        assert_eq!(fuzzy_composition.build(), true);    
        assert_eq!(fuzzy_composition.count_points(), 8);
       
    }

    #[test]
    pub fn test_new() {
        let mut fuzzy_composition:FuzzyComposition =  FuzzyComposition::new();

        assert_eq!(fuzzy_composition.add_point(1.0, 0.1), true);
        assert_eq!(fuzzy_composition.check_point(1.0, 0.1), true);
        
        assert_eq!(fuzzy_composition.add_point(5.0, 0.5), true);
        assert_eq!(fuzzy_composition.check_point(5.0, 0.5), true);

        assert_eq!(fuzzy_composition.add_point(9.0, 0.9), true);
        assert_eq!(fuzzy_composition.check_point(9.0, 0.9), true);
        
        assert_eq!(fuzzy_composition.check_point(5.0, 0.1), false);

    }
    

    #[test]
    pub fn test_build() {
        let mut fuzzy_composition:FuzzyComposition =  FuzzyComposition::new();

        fuzzy_composition.add_point(0.0, 0.0);
        fuzzy_composition.add_point(10.0, 1.0);
        fuzzy_composition.add_point(20.0, 0.0);

        fuzzy_composition.add_point(10.0, 0.0);
        fuzzy_composition.add_point(20.0, 1.0);
        fuzzy_composition.add_point(30.0, 0.0);

        assert_eq!(fuzzy_composition.build(), true);
        
        assert_eq!(fuzzy_composition.check_point(0.0, 0.0), true);
        assert_eq!(fuzzy_composition.check_point(10.0, 1.0), true);
        assert_eq!(fuzzy_composition.check_point(20.0, 0.0), false);
        assert_eq!(fuzzy_composition.check_point(15.0, 0.5), true);
        assert_eq!(fuzzy_composition.check_point(10.0, 0.0), false);
        assert_eq!(fuzzy_composition.check_point(20.0, 1.0), true);
        assert_eq!(fuzzy_composition.check_point(30.0, 0.0), true);
        assert_eq!(fuzzy_composition.count_points(), 5);
    }

    #[test]
    pub fn test_build2() {
        let mut fuzzy_composition:FuzzyComposition =  FuzzyComposition::new();

        fuzzy_composition.add_point(0.0, 0.0);
        fuzzy_composition.add_point(10.0, 1.0);
        fuzzy_composition.add_point(20.0, 0.0);
        fuzzy_composition.add_point(10.0, 0.0);
        fuzzy_composition.add_point(20.0, 1.0);
        fuzzy_composition.add_point(30.0, 0.0);
        fuzzy_composition.add_point(20.0, 0.0);
        fuzzy_composition.add_point(30.0, 1.0);
        fuzzy_composition.add_point(40.0, 0.0);

        assert_eq!(fuzzy_composition.build(), true);
        
        assert_eq!(fuzzy_composition.check_point(0.0, 0.0), true);
        assert_eq!(fuzzy_composition.check_point(10.0, 1.0), true);
        assert_eq!(fuzzy_composition.check_point(20.0, 0.0), false);
        assert_eq!(fuzzy_composition.check_point(15.0, 0.5), true);
        assert_eq!(fuzzy_composition.check_point(10.0, 0.0), false);
        assert_eq!(fuzzy_composition.check_point(20.0, 1.0), true);
        assert_eq!(fuzzy_composition.check_point(25.0, 0.5), true);
        assert_eq!(fuzzy_composition.check_point(30.0, 0.0), false);
        assert_eq!(fuzzy_composition.check_point(20.0, 0.0), false);
        assert_eq!(fuzzy_composition.check_point(30.0, 1.0), true);
        assert_eq!(fuzzy_composition.check_point(40.0, 0.0), true);
        assert_eq!(fuzzy_composition.count_points(), 7);
      
    }

    #[test]
    pub fn test_calculate() {
        
        let mut fuzzy_composition:FuzzyComposition =  FuzzyComposition::new();

        fuzzy_composition.add_point(25.0, 0.0);
        fuzzy_composition.add_point(25.0, 1.0);
        fuzzy_composition.add_point(25.0, 0.0);

        assert_eq!(fuzzy_composition.build(), true);
        assert_eq!(fuzzy_composition.count_points(), 3);
        assert_eq!(fuzzy_composition.calculate(), 25.0);
        assert_eq!(fuzzy_composition.empty(), true);

        fuzzy_composition.add_point(10.0, 0.0);
        fuzzy_composition.add_point(20.0, 1.0);
        fuzzy_composition.add_point(30.0, 0.0);

        assert_eq!(fuzzy_composition.build(), true);
        assert_eq!(fuzzy_composition.count_points(), 3);
        assert_eq!(fuzzy_composition.calculate(), 20.0);
        assert_eq!(fuzzy_composition.empty(), true);

        fuzzy_composition.add_point(20.0, 0.0);
        fuzzy_composition.add_point(30.0, 1.0);
        fuzzy_composition.add_point(50.0, 1.0);
        fuzzy_composition.add_point(60.0, 0.0);

        assert_eq!(fuzzy_composition.build(), true);
        assert_eq!(fuzzy_composition.count_points(), 4);
        assert_eq!(fuzzy_composition.calculate(), 40.0);
        assert_eq!(fuzzy_composition.empty(), true);
        
        fuzzy_composition.add_point(0.0, 0.0);
        fuzzy_composition.add_point(10.0, 1.0);
        fuzzy_composition.add_point(20.0, 0.0);
        fuzzy_composition.add_point(10.0, 0.0);
        fuzzy_composition.add_point(20.0, 1.0);
        fuzzy_composition.add_point(30.0, 0.0);
        fuzzy_composition.add_point(20.0, 0.0);
        fuzzy_composition.add_point(30.0, 1.0);
        fuzzy_composition.add_point(40.0, 0.0);

        assert_eq!(fuzzy_composition.build(), true);

        assert_eq!(fuzzy_composition.check_point(10.0, 1.0), true);
        assert_eq!(fuzzy_composition.check_point(15.0, 0.5), true);
        assert_eq!(fuzzy_composition.check_point(20.0, 1.0), true);
        assert_eq!(fuzzy_composition.check_point(25.0, 0.5), true);
        assert_eq!(fuzzy_composition.check_point(30.0, 1.0), true);
        assert_eq!(fuzzy_composition.check_point(40.0, 0.0), true);

        assert_eq!(fuzzy_composition.count_points(), 7);
        assert_eq!(fuzzy_composition.calculate(), 20.0);
        assert_eq!(fuzzy_composition.empty(), true);
    }
 
}