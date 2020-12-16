
#[derive(PartialEq, Eq)]
pub enum Discipline
{
    C1M,
    C1W,
    K1M,
    K1W,
}

pub struct Athlete
{
    pub name: String,
    pub price: u32,
    pub points: u32,
    pub country: String,
    pub discipline: Discipline,
}

pub struct Team<'a> 
{
    pub max_size : u32,

    pub team : Vec<&'a Athlete>,
}

impl<'a> Team<'a>
{
    pub fn points(&self) -> u32 
    {
        let mut points : u32 = 0;
        for ath in &self.team 
        {
            points += ath.points;
        }
        points
    }

    pub fn price(&self) -> u32 
    {
        let mut price : u32 = 0;
        for ath in &self.team 
        {
            price += ath.price;
        }
        price
    }

    pub fn print(&self)
    {
        for ath in &self.team
        {
            println!("{:4} {:9} {:4} {:4} {:5}",ath.points, ath.name, match ath.discipline {
                Discipline::C1M => "C1M",
                Discipline::K1M => "K1M",
                Discipline::C1W => "C1W",
                Discipline::K1W => "K1W",
            }, ath.country, ath.price);
        }
        println!("{:4}{:26}",&self.points(), &self.price());
    }

    pub fn at_least_1_in(&self, disc : Discipline) -> bool
    {
        for ath in &self.team
        {
            if disc == ath.discipline
            {
                return true;
            }
        }
        false
    }

    pub fn at_least_1_per_discipline(&self) -> bool
    {
        self.at_least_1_in(Discipline::C1M) 
        && self.at_least_1_in(Discipline::C1W) 
        && self.at_least_1_in(Discipline::K1M) 
        && self.at_least_1_in(Discipline::K1W) 
    }

    pub fn per_country_max(&self,max_per_country : u32) -> bool
    {
        let mut map = std::collections::HashMap::<String,u32>::new();
        for ath in &self.team
        {
            let country = map.get(&ath.country);
            match country {
                None => {map.insert(String::from(&ath.country), 1); ()},
                Some(n) =>
                {
                    if n < &max_per_country
                    {
                        let a = n+1;
                        map.insert(String::from(&ath.country), a);
                    }
                    else
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}