use std::f32::consts::PI;

mod quantum_simulator;

fn main() {

  let qubits_number = 15;

  let mut qc =  quantum_simulator::QuantumCircuit::new(qubits_number);
  
  qc.h(0);
  for qubit in 1..qubits_number{
    qc.h(qubit);
    qc.cx(qubit-1,qubit);
  }
  qc.h(0);
  qc.cx(0,1);
  qc.x(1);
  qc.rx(2,PI);
  qc.z(0);
  qc.x(1);
  qc.rx(1,PI);
  println!("Quantum circuit:");
  qc.print();
  
  println!("Executiing a quantum circuit:");
  let mut quantum_simulator =  quantum_simulator::QuantumSimulator::new(qc);
  quantum_simulator.run();
}
