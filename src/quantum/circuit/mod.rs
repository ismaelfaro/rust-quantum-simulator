// Circuit Moudule

#[warn(unused_variables)]

use std::f32::consts::PI;

#[derive(Debug)]
pub struct Gate{
  pub gate:  String,
  pub qubit: u32,
  pub target:u32,
  pub angle: f32,
}

#[derive(Debug)]
pub struct Circuit{
  pub number_qubits : u32,
  pub circuit : Vec<Gate>,
}

#[allow(dead_code)]
impl Circuit {

  pub fn new(number_qubits: u32)->Circuit{
    Circuit {number_qubits, circuit: Vec::new() }  
  }
  pub fn addgate(&mut self, gate: String, qubit:u32, target:u32, angle:f32 ) {
    let element = Gate {gate, qubit, target, angle};
    self.circuit.push(element);
  }
  pub fn x(&mut self, qubit: u32 ){
    let gate = String::from("X");
    self.addgate(gate, qubit, 0, 0.0);
  }
  pub fn z(&mut self, qubit: u32 ){
    self.rz(qubit,PI);
  }
  pub fn y(&mut self, qubit: u32 ){
    self.rz(qubit,PI);
    self.x(qubit);
  }
  pub fn h(&mut self, qubit: u32 ){
    let gate = String::from("H");
    self.addgate(gate, qubit, 0, 0.0);
  }
  pub fn cx(&mut self, control: u32, target: u32 ){
    let gate = String::from("CX");
    self.addgate(gate, control, target, 0.0);
  }
  pub fn rx(&mut self, qubit: u32, angle: f32 ){
    let gate = String::from("RX");
    self.addgate(gate, qubit, 0, angle);
  }
  pub fn ry(&mut self, qubit: u32, angle: f32 ){
    self.rx(qubit, PI/2.0);
    self.h(qubit);
    self.rx(qubit, angle);
    self.h(qubit);
    self.rx(qubit, -PI/2.0);
  }
  pub fn rz(&mut self, qubit: u32, angle: f32 ){
    self.h(qubit);
    self.rx(qubit, angle);
    self.h(qubit);
  }
  pub fn print(&mut self){
    println!("Qubits: {:?}",self.number_qubits);
    println!("{:?}",self.circuit);
  }
}
