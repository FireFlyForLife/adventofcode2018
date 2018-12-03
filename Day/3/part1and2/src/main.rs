mod puzzle_input;

use failure::Error;
use failure::err_msg;
use regex::Regex;

use std::collections::HashSet;

#[derive(Debug)]
struct Claim{
    id: u32,

    x: u32, 
    y: u32, 
    w: u32, 
    h: u32,
}

impl Claim{
    fn from_text(txt: &str) -> Claim{
        let re = Regex::new(r"#(?P<id>[0-9]+) @ (?P<x>[0-9]+),(?P<y>[0-9]+): (?P<w>[0-9]+)x(?P<h>[0-9]+)")
            .expect("Cannot compile regex!");
        
        let caps = re.captures(txt).unwrap();

        Claim{
            id: caps["id"].parse().unwrap(),

            x: caps["x"].parse().unwrap(),
            y: caps["y"].parse().unwrap(),
            w: caps["w"].parse().unwrap(),
            h: caps["h"].parse().unwrap(),
        }
    }
}

const FABRIC_SIZE: usize = 1000*1000;
struct PrototypeFabric{
    surface: Vec<u32>,
    overlapping_plots: HashSet<(u32, u32)>,
}

impl PrototypeFabric{
    fn new() -> PrototypeFabric{
        let mut fabric = PrototypeFabric{
            surface: Vec::new(),
            overlapping_plots: HashSet::new(),
        };
        fabric.surface.resize(FABRIC_SIZE, 0);
        return fabric;
    }

    // @returns if it is overlapping
    fn apply_claim(&mut self, claim: &Claim) -> bool{
        let mut has_overlap = false;
        for x in claim.x..claim.x+claim.w {
            for y in claim.y..claim.y+claim.h {
                if self.set_plot(x, y, claim.id) { has_overlap = true; }
            }
        }
        return has_overlap;
    }

    fn is_claim_open(&self, claim: &Claim) -> bool {
        for x in claim.x..claim.x+claim.w {
            for y in claim.y..claim.y+claim.h {
                if self.overlapping_plots.contains(&(x, y)) {
                    return false;
                }
            }
        }

        return true;
    }

    fn get_overlapping(&self) -> &HashSet<(u32, u32)> {
        &self.overlapping_plots
    }

    // @returns if the current plot had overlap
    fn set_plot(&mut self, x: u32, y: u32, id: u32) -> bool{
        if self.get_plot(x, y) != 0 && self.get_plot(x, y) != id {
            self.surface[(y * 1000 + x) as usize] = id;
            if !self.overlapping_plots.contains(&(x, y)) {
                self.overlapping_plots.insert((x, y));
            }
            return true;
        }else{
            self.surface[(y * 1000 + x) as usize] = id;
            return false;
        }
    }

    // @returns the id of the claim at that position
    fn get_plot(&self, x: u32, y: u32) -> u32 {
        return self.surface[(y * 1000 + x) as usize];
    }
}

fn main() -> Result<(), Error> {
    let mut prototype_fabric = PrototypeFabric::new();
    
    for line in puzzle_input::CLAIMS.lines() {
        let claim = Claim::from_text(line);
        let _ = prototype_fabric.apply_claim(&claim);
    }
    println!("Total amount of overlapping: {}ft2", prototype_fabric.get_overlapping().len());
    for line in puzzle_input::CLAIMS.lines() {
        let claim = Claim::from_text(line);
        let open = prototype_fabric.is_claim_open(&claim);
        if open {
            println!("Non overlapping: \"{:?}\"", claim);
            return Ok(());
        }
    }

    Err(err_msg("Could not find a plot which doesn't overlap!"))
}
