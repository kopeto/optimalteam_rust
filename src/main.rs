use std::io::prelude::*;

mod team;

fn get_paddlers() -> Vec<team::Athlete>
{
    let mut line = String::new(); 
    std::io::stdin().read_line(&mut line).unwrap();

    let _n : u32 = line.trim().parse().unwrap();


    let mut athletes : Vec<team::Athlete> = Vec::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap();
        let price = iter.next().unwrap();
        let points = iter.next().unwrap();
        let country = iter.next().unwrap();
        let discipline = iter.next().unwrap();

        let ath = team::Athlete{
            name:  String::from(name),
            price: price.parse::<u32>().unwrap(),
            points: points.parse::<u32>().unwrap(),
            country: String::from(country),
            discipline: match discipline
            {
                "C1M" => team::Discipline::C1M,
                "C1W" => team::Discipline::C1W,
                "K1M" => team::Discipline::K1M,
                "K1W" => team::Discipline::K1W,
                _ => team::Discipline::C1M,
            }
        };

        athletes.push(ath);
    }
    athletes.sort_by(|a,b| b.points.cmp(&a.points));

    athletes
}

fn solver<'a> ( 
            mut candidate_team : &mut team::Team<'a>,
            mut current_team : &mut team::Team<'a>,
            athletes : &'a [team::Athlete],
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
    
    let team_max_size = 7;
    let initial_budget = 25000;
    let max_per_country = 2;
    
    let athletes : Vec<team::Athlete> = get_paddlers();

    let mut best_team = team::Team{
        max_size: team_max_size,
        team : Vec::<&team::Athlete>::new(),
    };
    let mut current_team = team::Team{
        max_size: team_max_size,
        team : Vec::<&team::Athlete>::new(),
    };

    let timer = std::time::Instant::now();

    solver(&mut best_team, &mut current_team, &athletes, initial_budget , max_per_country);

    let duration = timer.elapsed();

    best_team.print();
    println!("{:?}",duration);
}
