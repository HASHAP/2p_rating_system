#[cfg(feature ="serde")]
use std::f64::consts::PI;
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserealize))]
pub struct Glicko2{
    pub rating:f64, 
    pub deviation:f64,
    pub vol:f64,
}

impl Glicko2{
    #[must_use]
    //initializing the GLICKO rating system with default values
    pub const fn new() -> Self{
        Self{
            rating:1500.0,
            deviation:350.0,
            vol:0.06,
        }
    }
}

impl Default for Glicko2{
    fn default() -> Self{
        Self::new()
    }
}
impl Glicko2{
    pub fn rating(&self) ->f64{
        self.rating
    }
    pub fn std_deviation(&self) -> f64{
        self.deviation
    }
//     pub fn new(rating: Option<f64>, std_deviation: Option<f64>) -> Self{
//         Self { rating: rating.unwrap_or(1500.00), deviation: std_deviation.unwrap_or(350.0), vol: 0.06}       
//     }
}

impl From<(f64, f64, f64)> for Glicko2{
    fn from((r,d,v): (f64,f64,f64)) -> Self{
        Self{rating:r, deviation:d, vol:v}
    }  
} 


#[derive(Debug, Clone, Copy)]
pub struct GameResult{
    opp_Glicko2: Glicko2,
    score: Vec<f64>,
}

impl GameResult{
    pub fn win<T:Into
}