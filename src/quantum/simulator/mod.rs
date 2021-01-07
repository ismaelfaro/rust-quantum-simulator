// Simulator Moudule

#[warn(unused_variables)]

use super::circuit;

static R2: f32 = 0.70710678118;

#[derive(Debug)]
struct ComplexNumber{
  r: f32,
  i: f32
}

#[derive(Debug)]
pub struct Simulator{
  circuit : Vec<circuit::Gate>,
  number_qubits: u32,
  bits: u32,
  state_vector: Vec<ComplexNumber>
}

impl Simulator{
  pub fn new(circuit:circuit::Circuit)->Simulator{
    Simulator {circuit: circuit.circuit, number_qubits: circuit.number_qubits, bits: circuit.number_qubits, state_vector:Vec::new() }  
  }

  fn initialize_state_vector(&mut self){
    for _ in 1..= (2u32.pow(self.number_qubits)){
        self.state_vector.push(ComplexNumber {r:0.0,i:0.0})}
    self.state_vector[0] = ComplexNumber {r:1.0,i:0.0}  
  }
  
  fn print_state_vector(&mut self){
    for index in 0..2u32.pow(self.number_qubits) as usize {
        println!("{:04b}: {},i{}",index,self.state_vector[index].r,self.state_vector[index].i) 
    }}

  // fn get_counts(&mut self, shots) -> 

  pub fn run(&mut self){
    self.initialize_state_vector();
    for quantum_gate in self.circuit.iter(){
      if quantum_gate.gate == "X" || quantum_gate.gate == "H" || quantum_gate.gate == "RX" {
        for counter_qubit in 0..=(2u32.pow(quantum_gate.qubit) as usize)-1 {
          for counter_state in 0..=(2u32.pow(self.number_qubits-quantum_gate.qubit-1) as usize)-1 {
            let qb0=counter_qubit +(2u32.pow(quantum_gate.qubit)+1) as usize * counter_state;
            let qb1=qb0+(2u32.pow(quantum_gate.qubit) as usize);

            if quantum_gate.gate == "X"{
              self.state_vector.swap(qb0, qb1);
            }
            if quantum_gate.gate == "H"{
              let (a, b) = superposition(ComplexNumber {r: self.state_vector[qb0].r, i: self.state_vector[qb0].i },
                                         ComplexNumber {r: self.state_vector[qb1].r, i: self.state_vector[qb1].i });
              self.state_vector[qb0] = ComplexNumber {r: a.r, i: a.i };
              self.state_vector[qb1] = ComplexNumber {r: b.r, i: b.i };
          }
          if quantum_gate.gate == "RX" {
              let (a, b) = turn(ComplexNumber {r: self.state_vector[qb0].r, i: self.state_vector[qb0].i },
                                ComplexNumber {r: self.state_vector[qb1].r, i: self.state_vector[qb1].i },
                                quantum_gate.angle);
              self.state_vector[qb0] = ComplexNumber {r: a.r, i: a.i };
              self.state_vector[qb1] = ComplexNumber {r: b.r, i: b.i };
          }
        }
      }
    } else {
      let (low, high) = if quantum_gate.qubit < quantum_gate.target {
                        (quantum_gate.qubit, quantum_gate.target)
                      } else {
                        (quantum_gate.target, quantum_gate.qubit)
                      };
      for cx0 in 0..2u32.pow(low){
        let limit_cx2 = 2u32.pow(high-low-1);
        for cx1 in 0..limit_cx2{
          for cx2 in 0..2u32.pow(self.number_qubits-high-1){
            let qb0 = cx0 + 2u32.pow(low+1)* cx1 + 2u32.pow(high+1)*cx2 + 2u32.pow(quantum_gate.qubit);
            let qb1 = qb0 + 2u32.pow(quantum_gate.target);
            self.state_vector.swap(qb0 as usize, qb1 as usize);
          }
        }
      }
    }
    }
    self.print_state_vector();
  }
}

fn superposition(x: ComplexNumber, y: ComplexNumber )-> (ComplexNumber, ComplexNumber){
// For two elements of the state_vector, x and y, return (x+y)/sqrt(2) and (x-y)/sqrt(2)
  (ComplexNumber {r: R2 * (x.r + y.r), i: R2 * (x.i + y.i)}, 
  ComplexNumber {r: R2 * (x.r - y.r), i: R2 * (x.i - y.i)})
}

fn turn(x: ComplexNumber, y: ComplexNumber, angle: f32 )-> (ComplexNumber, ComplexNumber){
// For two elements of the state_vector, x and y, return cos(theta/2)*x - i*sin(theta/2)*y and cos(theta/2)*y - i*sin(theta/2)*x
  let an = angle/2.0;
  (ComplexNumber {r: x.r * an.cos() + y.i * an.sin(), i: x.i * an.cos() - y.r * an.sin()}, 
  ComplexNumber {r: y.r * an.cos() + x.i * an.sin() , i: y.i * an.cos() - x.r * an.sin()})
}
