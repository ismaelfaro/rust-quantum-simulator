
pub static PI: f32 = 3.141592653589793;
pub static R2: f32 = 0.70710678118;

#[derive(Debug)]
struct QuantumGate{
  gate:  String,
  qubit: u32,
  target:u32,
  angle: f32,
}


#[derive(Debug)]
struct QuantumCircuit{
  number_qubits : u32,
  circuit : Vec<QuantumGate>,
}

impl QuantumCircuit {

  pub fn new(number_qubits: u32)->QuantumCircuit{
    QuantumCircuit {number_qubits, circuit: Vec::new() }  
  }

  pub fn addgate(&mut self, gate: String, qubit:u32, target:u32, angle:f32 ) {
    let element = QuantumGate {gate, qubit, target, angle};
    self.circuit.push(element);
  }

  pub fn x(&mut self, qubit: u32 ){
    let gate = String::from("X");
    self.addgate(gate, qubit, 0, 0.0);
  }
  pub fn z(&mut self, qubit: u32 ){
    let gate = String::from("Z");
    self.addgate(gate, qubit, 0, 0.0);
  }
  pub fn y(&mut self, qubit: u32 ){
    let gate = String::from("Y");
    self.addgate(gate, qubit, 0, 0.0);
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

#[derive(Debug)]
struct ComplexNumber{
  r: f32,
  i: f32
}

#[derive(Debug)]
struct QuantumSimulator{
  circuit : Vec<QuantumGate>,
  number_qubits: u32,
  bits: u32,
  state_vector: Vec<ComplexNumber>
}

impl QuantumSimulator{
  pub fn new(circuit:QuantumCircuit)->QuantumSimulator{
    QuantumSimulator {circuit: circuit.circuit, number_qubits: circuit.number_qubits, bits: circuit.number_qubits, state_vector:Vec::new() }  
  }

  fn initialize_state_vector(&mut self){
    for i in 1..= (2u32.pow(self.number_qubits)){
        self.state_vector.push(ComplexNumber {r:0.0,i:0.0})}
    self.state_vector[0] = ComplexNumber {r:1.0,i:0.0}  
  }
  
  fn print_state_vector(&mut self){
    for i in 0..= (2u32.pow(self.number_qubits) as usize)-1 {
        println!("{:04b}: {},i{}",i,self.state_vector[i].r,self.state_vector[i].i) 
    }}

  fn get_counts(&mut self, shots) -> 

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
///For two elements of the state_vector, x and y, return cos(theta/2)*x - i*sin(theta/2)*y and cos(theta/2)*y - i*sin(theta/2)*x
  let an = angle/2.0;
  (ComplexNumber {r: x.r * an.cos() + y.i * an.sin(), i: x.i * an.cos() - y.r * an.sin()}, 
  ComplexNumber {r: y.r * an.cos() + x.i * an.sin() , i: y.i * an.cos() - x.r * an.sin()})
}


fn main() {
 let mut qc =  QuantumCircuit::new(4);
 qc.h(0);
 qc.cx(0,1);

// qc.x(1);
// qc.rx(1,PI);
// qc.print();

 let mut quantum_simulator =  QuantumSimulator::new(qcir);
 quantum_simulator.run();
}
