
extern crate iterslide;

extern crate itertools;

use iterslide::SlideIterator;

use itertools::Itertools; // 0.9.0

use std::fmt;

const EPSILON_VALUE: f32 = 1.0E-3;

#[allow(non_snake_case)]
#[derive(Copy,Clone)]
pub struct PointArray{ 
    point: f32,
    pertinence: f32,
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
    pub fn addPoint(&mut self, point: f32, pertinence: f32) -> bool{
        self.points.push(PointArray{point, pertinence});
        true
    }

    // Method to check if FuzzyComposition contains an specific point and pertinence
    pub fn checkPoint(&self, point: f32, pertinence: f32) -> bool{
        self.points.contains(&PointArray{point, pertinence})
    }

    // Method to iterate over the pointsArray, detect possible intersections and sent these points for "correction"
    pub fn build(&mut self) -> bool{
        let mut cleanOnExit : Vec<PointArray> = vec![];
        let mut previous: Option<PointArray> = None;
        let mut is_greater = false;
        println!("====================================================");
        println!("==   BUILD    nbr_of_points: {}", self.points.len());
        println!("====================================================");
        for current in self.points.clone().into_iter() {
            println!("{:?} ", current);
            match previous {
                Some(p) => {
                    // if the previous point is greater then the current
                    is_greater = p.is_previous_greater(&current);
                    // if yes, break an use this point
                    if is_greater {
                       // println!("previus {:?} is greater then {:?}", p, current);
                       // find the index of the previus
                        let index = self.points.iter().position(|&r| r == previous.unwrap()).unwrap();

                        println!("found greater {:?} at index [{:?}] points.len():{:?}", previous, index, self.points.clone().len());

                        // search the rigth windows that contains the previeus at index 1 to get the 4 tuple<_,_,_,_>
                        for window in self.points.clone().into_iter().slide(4) { 
                           // println!(" windows: {:?}", window); 
                            if previous.as_ref() == window.get(1) {
                                //println!("==>> found point on windows: {:?}", window);
                                let aSegmentBegin = *window.get(0).unwrap();
                                let aSegmentEnd = *window.get(1).unwrap();
                                let bSegmentBegin = *window.get(2).unwrap();
                                let bSegmentEnd = *window.get(3).unwrap();
                                // insert the fixed point
                                if let Some(fixedPoint) = self.rebuild(aSegmentBegin, aSegmentEnd, bSegmentBegin, bSegmentEnd){
                                    // insert new point
                                    //println!("add new point: {:?}", fixedPoint);
                                    self.points.insert(index,fixedPoint);
                                    // delete current et previus pointsArray
                                    cleanOnExit.push(aSegmentEnd);
                                    cleanOnExit.push(bSegmentBegin);
                                    break;
                                }

                            }       
                        } // end window
                        
                    } 
                }
                None => {}
            }
            previous = Some(current);
        }
    
        println!("=== Remove invalid points =============");

        for current in cleanOnExit.iter() {
            println!("cleanOnExit: {:?}", current);
            self.rmvPoint(*current);
        }

        println!("==== ==================================");

        for current in self.points.iter() {
            println!("remain: {:?}", current);
        }

        println!("==== ==================================");

        self.points.sort_by(|a,b| a.point.partial_cmp(&b.point).unwrap());

        for current in self.points.iter() {
            println!("sorted: {:?}", current);
        }

        true
    }

    // Method to search intersection between two segments, if found, create a new pointsArray (in found intersection) and remove not necessary ones
    pub fn rebuild(&self,  aSegmentBegin: PointArray, aSegmentEnd: PointArray, bSegmentBegin: PointArray, bSegmentEnd: PointArray) -> Option<PointArray>{
        
        // create a reference for each point
        let x1 = aSegmentBegin.point;
        let y1 = aSegmentBegin.pertinence;
        let x2 = aSegmentEnd.point;
        let y2 = aSegmentEnd.pertinence;
        let x3 = bSegmentBegin.point;
        let y3 = bSegmentBegin.pertinence;
        let x4 = bSegmentEnd.point;
        let y4 = bSegmentEnd.pertinence;

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
            return None;
        } else {
            // we found an intersection
            // calculate the point (y) and its pertinence (y) for the new element (pointsArray)
            let point = x1 + mua * (x2 - x1);
            let pertinence = y1 + mua * (y2 - y1);

            let aux = Some(PointArray{point, pertinence});
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
        if (denominator == 0.0){
            return 0.0;
        } else {
            return (numerator / denominator).ceil();
        }  
    }

    pub fn empty(&mut self) -> bool{
        self.points.clear();
        self.points.is_empty()
    }

    pub fn countPoints(&self) -> usize {
        self.points.len()
    }

    pub fn cleanPoints(&mut self) {
        self.points.clear();
    }

    pub fn rmvPoint(&mut self, point: PointArray){
        if let Some(index) = self.points.iter().position(|p| *p == point) {
            self.points.swap_remove(index);
        }

    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    pub fn test_new() {
        let mut fuzzyComposition:FuzzyComposition =  FuzzyComposition::new();

        assert_eq!(fuzzyComposition.addPoint(1.0, 0.1), true);
        assert_eq!(fuzzyComposition.checkPoint(1.0, 0.1), true);
        
        assert_eq!(fuzzyComposition.addPoint(5.0, 0.5), true);
        assert_eq!(fuzzyComposition.checkPoint(5.0, 0.5), true);

        assert_eq!(fuzzyComposition.addPoint(9.0, 0.9), true);
        assert_eq!(fuzzyComposition.checkPoint(9.0, 0.9), true);
        
        assert_eq!(fuzzyComposition.checkPoint(5.0, 0.1), false);

    }

    #[test]
    pub fn test_build() {
        let mut fuzzyComposition:FuzzyComposition =  FuzzyComposition::new();

        fuzzyComposition.addPoint(0.0, 0.0);
        fuzzyComposition.addPoint(10.0, 1.0);
        fuzzyComposition.addPoint(20.0, 0.0);

        fuzzyComposition.addPoint(10.0, 0.0);
        fuzzyComposition.addPoint(20.0, 1.0);
        fuzzyComposition.addPoint(30.0, 0.0);

        assert_eq!(fuzzyComposition.build(), true);

        assert_eq!(fuzzyComposition.checkPoint(0.0, 0.0), true);
        assert_eq!(fuzzyComposition.checkPoint(10.0, 1.0), true);
        assert_eq!(fuzzyComposition.checkPoint(20.0, 0.0), false);
        assert_eq!(fuzzyComposition.checkPoint(15.0, 0.5), true);
        assert_eq!(fuzzyComposition.checkPoint(10.0, 0.0), false);
        assert_eq!(fuzzyComposition.checkPoint(20.0, 1.0), true);
        assert_eq!(fuzzyComposition.checkPoint(30.0, 0.0), true);
        assert_eq!(fuzzyComposition.countPoints(), 5);
    }

    #[test]
    pub fn test_build2() {
        let mut fuzzyComposition:FuzzyComposition =  FuzzyComposition::new();

        fuzzyComposition.addPoint(0.0, 0.0);
        fuzzyComposition.addPoint(10.0, 1.0);
        fuzzyComposition.addPoint(20.0, 0.0);
        fuzzyComposition.addPoint(10.0, 0.0);
        fuzzyComposition.addPoint(20.0, 1.0);
        fuzzyComposition.addPoint(30.0, 0.0);
        fuzzyComposition.addPoint(20.0, 0.0);
        fuzzyComposition.addPoint(30.0, 1.0);
        fuzzyComposition.addPoint(40.0, 0.0);

        assert_eq!(fuzzyComposition.build(), true);

        assert_eq!(fuzzyComposition.checkPoint(0.0, 0.0), true);
        assert_eq!(fuzzyComposition.checkPoint(10.0, 1.0), true);
        assert_eq!(fuzzyComposition.checkPoint(20.0, 0.0), false);
        assert_eq!(fuzzyComposition.checkPoint(15.0, 0.5), true);
        assert_eq!(fuzzyComposition.checkPoint(10.0, 0.0), false);
        assert_eq!(fuzzyComposition.checkPoint(20.0, 1.0), true);
        assert_eq!(fuzzyComposition.checkPoint(25.0, 0.5), true);
        assert_eq!(fuzzyComposition.checkPoint(30.0, 0.0), false);
        assert_eq!(fuzzyComposition.checkPoint(20.0, 0.0), false);
        assert_eq!(fuzzyComposition.checkPoint(30.0, 1.0), true);
        assert_eq!(fuzzyComposition.checkPoint(40.0, 0.0), true);
        assert_eq!(fuzzyComposition.countPoints(), 7);
      
    }

    #[test]
    pub fn test_calculate() {
        
        let mut fuzzyComposition:FuzzyComposition =  FuzzyComposition::new();

        fuzzyComposition.addPoint(25.0, 0.0);
        fuzzyComposition.addPoint(25.0, 1.0);
        fuzzyComposition.addPoint(25.0, 0.0);

        assert_eq!(fuzzyComposition.build(), true);
        assert_eq!(fuzzyComposition.countPoints(), 3);
        assert_eq!(fuzzyComposition.calculate(), 25.0);
        assert_eq!(fuzzyComposition.empty(), true);

        fuzzyComposition.addPoint(10.0, 0.0);
        fuzzyComposition.addPoint(20.0, 1.0);
        fuzzyComposition.addPoint(30.0, 0.0);

        assert_eq!(fuzzyComposition.build(), true);
        assert_eq!(fuzzyComposition.countPoints(), 3);
        assert_eq!(fuzzyComposition.calculate(), 20.0);
        assert_eq!(fuzzyComposition.empty(), true);

        fuzzyComposition.addPoint(20.0, 0.0);
        fuzzyComposition.addPoint(30.0, 1.0);
        fuzzyComposition.addPoint(50.0, 1.0);
        fuzzyComposition.addPoint(60.0, 0.0);

        assert_eq!(fuzzyComposition.build(), true);
        assert_eq!(fuzzyComposition.countPoints(), 4);
        assert_eq!(fuzzyComposition.calculate(), 40.0);
        assert_eq!(fuzzyComposition.empty(), true);
        
        fuzzyComposition.addPoint(0.0, 0.0);
        fuzzyComposition.addPoint(10.0, 1.0);
        fuzzyComposition.addPoint(20.0, 0.0);
        fuzzyComposition.addPoint(10.0, 0.0);
        fuzzyComposition.addPoint(20.0, 1.0);
        fuzzyComposition.addPoint(30.0, 0.0);
        fuzzyComposition.addPoint(20.0, 0.0);
        fuzzyComposition.addPoint(30.0, 1.0);
        fuzzyComposition.addPoint(40.0, 0.0);

        assert_eq!(fuzzyComposition.build(), true);
        assert_eq!(fuzzyComposition.countPoints(), 7);
        assert_eq!(fuzzyComposition.calculate(), 20.0);
        assert_eq!(fuzzyComposition.empty(), true);
        


    }
}