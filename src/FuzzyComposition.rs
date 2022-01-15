
extern crate array_tool;

use array_tool::vec::*;

const EPSILON_VALUE: f32 = 1.0E-3;

#[allow(non_snake_case)]
#[derive(Debug)]
struct PointArray{ 
    point: f32,
    pertinence: f32,
}
impl PartialEq for PointArray{
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.pertinence == other.pertinence
    }
}

#[allow(non_snake_case)]
struct FuzzyComposition{
    points : Vec<PointArray>,
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
    pub fn build(&self) -> bool{
        true
    }

    // Method to search intersection between two segments, if found, create a new pointsArray (in found intersection) and remove not necessary ones
    pub fn rebuild(&self,  aSegmentBegin: PointArray, aSegmentEnd: PointArray, bSegmentBegin: PointArray, bSegmentEnd: PointArray) -> bool{
        
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
            return false;
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
            return false;
        }
        
        true
    }

    pub fn calculate(&self) -> f32{
        0.0
    }

    pub fn empty(&mut self) -> bool{
        self.points.clear();
        true
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