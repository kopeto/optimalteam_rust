
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
        self.team.iter().fold(0, | acc, &ath | acc + ath.points)
    }

    pub fn price(&self) -> u32 
    {
        self.team.iter().fold(0, | acc, &ath | acc + ath.price)
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
        println!("------------------------------");
        println!("{:4}{:26}",&self.points(), &self.price());
    }

    pub fn at_least_1_in(&self, disc : Discipline) -> bool
    {
        self.team.iter().any(|ath| ath.discipline == disc)
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
        for (i,ath) in self.team.iter().enumerate()
        {
            let a = &self.team[i..];
            if a.iter().filter(|athlete| athlete.country == ath.country ).count() as u32 > max_per_country
            {
                return false;
            }
        }
        true
    }
}