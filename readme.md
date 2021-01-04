# Rust Quantum Simulator 

This is a very basic implementation of a Quantum Simulator in Rust to learn the basic components.
Allow to create your Quantum circuits with the basic Quantum Gates, and you can execute it using plain rust

# Components
- Quantum Circuit Class:
    - Quantum Gates: x, rx, ry, rz, z, y, h, cx, m
- Quanrtum Simulator Class: 
    - imput: Qcircuit
    - outputs: 
        - statevector
        - counts
        - memory

# Example:

    println!("crating a quantum circuit");
    let mut qc =  QuantumCircuit::new(4);
    qc.h(0);
    qc.cx(0,1);
    qc.x(1);
    qc.rx(2,PI);
    qc.z(0);
    
    println!("executiing a quantum circuit");
    let mut quantum_simulator =  QuantumSimulator::new(qc);
    quantum_simulator.run();


# TODO:
- Import Open Quasm 2.0
- create a basic visual interface to create circuits and understand "how work" a quantum executions.
- Integrate with https://github.com/JavaFXpert/grok-bloch bloch sphere visualization
- Integrate with https://github.ibm.com/Ismael-Faro1/box-phase-visualization

# references
Inspired in MicroQiskit python implementation https://github.com/qiskit-community/MicroQiskit by James Wootton