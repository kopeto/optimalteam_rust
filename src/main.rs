use std::io::prelude::*;

#[derive(PartialEq, Eq)]
enum Discipline
{
    C1M,
    C1W,
    K1M,
    K1W,
}

struct Athlete
{
    name: String,
    price: u32,
    points: u32,
    country: String,
    discipline: Discipline,
}

struct Team<'a> 
{
    max_size : u32,

    team : Vec<&'a Athlete>,
}

impl<'a> Team<'a>
{
    fn points(&self) -> u32 
    {
        let mut points : u32 = 0;
        for ath in &self.team 
        {
            points += ath.points;
        }
        points
    }

    fn price(&self) -> u32 
    {
        let mut price : u32 = 0;
        for ath in &self.team 
        {
            price += ath.price;
        }
        price
    }

    fn print(&self)
    {
        for ath in &self.team
        {
            println!("{}|{} {} {} {}",match ath.discipline {
                Discipline::C1M => "C1M",
                Discipline::K1M => "K1M",
                Discipline::C1W => "C1W",
                Discipline::K1W => "K1W",
            }, ath.name, ath.points, ath.country, ath.price);
        }
        println!("Total price: {}", &self.price());
        println!("Total points: {}", &self.points());
    }

    fn at_least_1_in(&self, disc : Discipline) -> bool
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

    fn at_least_1_per_discipline(&self) -> bool
    {
        self.at_least_1_in(Discipline::C1M) 
        && self.at_least_1_in(Discipline::C1W) 
        && self.at_least_1_in(Discipline::K1M) 
        && self.at_least_1_in(Discipline::K1W) 
    }

    fn per_country_max(&self,max_per_country : u32) -> bool
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

fn get_paddlers() -> Vec<Athlete>
{
    let mut line = String::new(); 
    std::io::stdin().read_line(&mut line).unwrap();

    let _n : u32 = line.trim().parse().unwrap();


    let mut athletes : Vec<Athlete> = Vec::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap();
        let price = iter.next().unwrap();
        let points = iter.next().unwrap();
        let country = iter.next().unwrap();
        let discipline = iter.next().unwrap();

        let ath = Athlete{
            name:  String::from(name),
            price: price.parse::<u32>().unwrap(),
            points: points.parse::<u32>().unwrap(),
            country: String::from(country),
            discipline: match discipline
            {
                "C1M" => Discipline::C1M,
                "C1W" => Discipline::C1W,
                "K1M" => Discipline::K1M,
                "K1W" => Discipline::K1W,
                _ => Discipline::C1M,
            }
        };

        athletes.push(ath);
    }
    athletes.sort_by(|a,b| b.points.cmp(&a.points));

    athletes
}

fn solver<'a> ( 
            mut candidate_team : &mut Team<'a>,
            mut current_team : &mut Team<'a>,
            athletes : &'a[Athlete],
            remaining_budget: u32,
            max_per_country: u32,
        ) 
        {
            // TRIVIAL CASE

            if current_team.team.len() as u32 == current_team.max_size 
            {
                if candidate_team.points() < current_team.points() 
                    && current_team.at_least_1_per_discipline()
                    && current_team.per_country_max(max_per_country)
                {
                    candidate_team.team.clear();
                    for ath in &current_team.team 
                    {
                        candidate_team.team.push(ath);
                    }
                }
                
            }

            // RECURSIVE CASE
            else
            {
                for (i,candidate) in athletes.iter().enumerate()
                {
                    if (current_team.max_size - current_team.team.len() as u32) * candidate.points + current_team.points() < candidate_team.points()
                    {
                        return;
                    }
                    else if remaining_budget > candidate.price
                    {
                        let a = &athletes[(i+1)..];
                        current_team.team.push(candidate);
                        solver(&mut candidate_team, &mut current_team, &a, remaining_budget - candidate.price , 2);
                        current_team.team.pop();
                    }
                }
            }
            return
        }

fn main() {
    
    let team_max_size = 8;
    let initial_budget = 25000;
    let max_per_country = 2;
    
    let athletes : Vec<Athlete> = get_paddlers();

    let mut best_team : Team = Team{
        max_size: team_max_size,
        team : Vec::<&Athlete>::new(),
    };
    let mut current_team : Team = Team{
        max_size: team_max_size,
        team : Vec::<&Athlete>::new(),
    };

    solver(&mut best_team, &mut current_team, &athletes, initial_budget , max_per_country);

    best_team.print();
}
