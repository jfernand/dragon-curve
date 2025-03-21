use std::collections::HashMap;
use std::f32::consts::PI;
use glam::Vec2;
use crate::DrawingCommand::{LineTo, MoveTo};
use crate::Sym::{Minus, Plus, F, X, Y};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Sym {
    X, Y, F, Plus, Minus
}

pub fn iterate(v: Vec<Sym>, map: &HashMap<Sym, Vec<Sym>>) -> Vec<Sym> {
    let mut new_symset: Vec<Sym> = vec![];
    for sym in v {
        match sym {
            X => new_symset.append(&mut map[&X].clone()),
            Y => new_symset.append(&mut map[&Y].clone()),
            F => new_symset.push(F),
            Plus => new_symset.push(Plus),
            Minus => new_symset.push(Minus),
        }
    }
    new_symset
}

#[derive(Debug)]
pub enum TurtleCommand {
    Forward,
    Left(f32),
    Right(f32),
}
pub fn to_turtle_commands(v: Vec<Sym>) -> Vec<TurtleCommand> {
    let mut commands: Vec<TurtleCommand> = vec![];
    for sym in v {
        match sym {
            X => {}
            Y => {}
            F => commands.push(TurtleCommand::Forward),
            Plus => commands.push(TurtleCommand::Left(PI/2.)),
            Minus => commands.push(TurtleCommand::Right(PI/2.)),
        }
    }
    commands
}

#[derive(Debug)]
pub enum DrawingCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
}
pub fn to_drawing_commands(start:Vec2, dir:Vec2, length: f32, v: Vec<TurtleCommand>) -> Vec<DrawingCommand> {
    let mut pos = start;
    let mut current_dir = dir;
    let mut commands: Vec<DrawingCommand> = vec![MoveTo(start.x, start.y)];
    for command in v {
        match command {
            TurtleCommand::Forward => {
                pos += current_dir * length;
                commands.push(LineTo(pos.x, pos.y))
            }
            TurtleCommand::Left(a) => {
                current_dir = Vec2::from_angle(a).rotate(current_dir);
            }
            TurtleCommand::Right(a) => {
                current_dir = Vec2::from_angle(-a).rotate(current_dir);
            }
        }
    }
    commands
}