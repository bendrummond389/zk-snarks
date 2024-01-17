use std::collections::HashMap;

use crate::circuits::{Circuit, IndexedMap, Operand, Operation};
#[allow(unused_variables)]
#[derive(Debug)]
struct Constraint {
    a: Vec<i64>,
    b: Vec<i64>,
    c: Vec<i64>,
}

#[derive(Debug)]
pub struct R1CS {
    a_matrix: Vec<Vec<i64>>,
    b_matrix: Vec<Vec<i64>>,
    c_matrix: Vec<Vec<i64>>,
    pub variable_map: IndexedMap<String>,
}

impl R1CS {
    pub fn new(variable_map: IndexedMap<String>) -> Self {
        R1CS {
            a_matrix: Vec::new(),
            b_matrix: Vec::new(),
            c_matrix: Vec::new(),
            variable_map,
        }
    }

    fn add_constraint(&mut self, constraint: Constraint) {
        self.a_matrix.push(constraint.a);
        self.b_matrix.push(constraint.b);
        self.c_matrix.push(constraint.c);
    }

    pub fn get_constraint_matrices(&self) -> (&Vec<Vec<i64>>, &Vec<Vec<i64>>, &Vec<Vec<i64>>) {
        (&self.a_matrix, &self.b_matrix, &self.c_matrix)
    }

    pub fn get_variable_map(&self) -> &IndexedMap<String> {
        &self.variable_map
    }

    pub fn generate_r1cs_constraints(&mut self, circuit: &Circuit, root: bool) {
        let circuit_hash = circuit.get_hash();
        let length = self.variable_map.len();

        let circuit_index = if root {
            match self.variable_map.get_index(&"out".to_string()) {
                Some(index) => index,
                None => panic!("Cannot find index of 'out' in variable_indices"),
            }
        } else {
            match self.variable_map.get_index(&circuit_hash.to_string()) {
                Some(index) => index,
                None => panic!("Cannot find index of current circuit in variable_indices"),
            }
        };

        let mut constraint = Constraint {
            b: vec![0; length],
            c: vec![0; length],
            a: vec![0; length],
        };

        constraint.c[circuit_index] = 1;

        let operand1 = &circuit.operands[0];
        let operand2 = &circuit.operands[1];

        match (operand1, operand2) {
            (Operand::Number(num1), Operand::Number(num2)) => {
                self.handle_number_number_case(*num1, *num2, &circuit.operation, &mut constraint)
            }
            (Operand::Number(num), Operand::Variable(var))
            | (Operand::Variable(var), Operand::Number(num)) => self.handle_number_variable_case(
                var.to_string(),
                *num,
                &circuit.operation,
                &mut constraint,
            ),
            (Operand::Variable(var1), Operand::Variable(var2)) => self
                .handle_variable_variable_case(
                    var1.to_string(),
                    var2.to_string(),
                    &circuit.operation,
                    &mut constraint,
                ),
            (Operand::NestedCircuit(nested_circuit), Operand::Number(num))
            | (Operand::Number(num), Operand::NestedCircuit(nested_circuit)) => self
                .handle_number_nested_circuit_case(
                    *num,
                    nested_circuit,
                    &circuit.operation,
                    &mut constraint,
                ),
            (Operand::NestedCircuit(nested_circuit), Operand::Variable(var))
            | (Operand::Variable(var), Operand::NestedCircuit(nested_circuit)) => self
                .handle_variable_nested_circuit_case(
                    var.to_string(),
                    nested_circuit,
                    &circuit.operation,
                    &mut constraint,
                ),
            (Operand::NestedCircuit(circuit1), Operand::NestedCircuit(circuit2)) => self
                .handle_double_nested_circuit_case(
                    circuit1,
                    circuit2,
                    &circuit.operation,
                    &mut constraint,
                ),
        }

        self.add_constraint(constraint)
    }

    fn handle_number_number_case(
        &mut self,
        num1: i64,
        num2: i64,
        operation: &Operation,
        constraint: &mut Constraint,
    ) {
        match operation {
            Operation::Add => {
                constraint.a[0] = num1 + num2;
                constraint.b[0] = 1;
            }
            Operation::Multiply => {
                constraint.a[0] = num1;
                constraint.b[0] = num2;
            }
        }
    }

    fn handle_number_variable_case(
        &mut self,
        var: String,
        num: i64,
        operation: &Operation,
        constraint: &mut Constraint,
    ) {
        let index = match self.variable_map.get_index(&var) {
            Some(index) => index,
            None => panic!("Cannot find index of variable in variable_indices"),
        };
        match operation {
            Operation::Add => {
                constraint.a[0] = num;
                constraint.a[index] = 1;
                constraint.b[0] = 1;
            }
            Operation::Multiply => {
                constraint.a[0] = num;
                constraint.b[index] = 1;
            }
        }
    }

    fn handle_variable_variable_case(
        &mut self,
        var1: String,
        var2: String,
        operation: &Operation,
        constraint: &mut Constraint,
    ) {
        let index1 = match self.variable_map.get_index(&var1) {
            Some(index) => index,
            None => panic!("Cannot find index of variable in variable_indices"),
        };
        let index2 = match self.variable_map.get_index(&var2) {
            Some(index) => index,
            None => panic!("Cannot find index of variable in variable_indices"),
        };
        match operation {
            Operation::Add => {
                if var1 == var2 {
                    constraint.a[index1] = 2;
                    constraint.b[0] = 1;
                } else {
                    constraint.a[index1] = 1;
                    constraint.a[index2] = 1;
                    constraint.b[0] = 1;
                }
            }
            Operation::Multiply => {
                constraint.a[index1] = 1;
                constraint.b[index2] = 1;
            }
        }
    }

    fn handle_number_nested_circuit_case(
        &mut self,
        num: i64,
        circuit: &Circuit,
        operation: &Operation,
        constraint: &mut Constraint,
    ) {
        let index = match self.variable_map.get_index(&circuit.hash.to_string()) {
            Some(index) => index,
            None => panic!("Cannot find index of variable in variable_indices"),
        };
        match operation {
            Operation::Add => {
                constraint.a[0] = num;
                constraint.a[index] = 1;
                constraint.b[0] = 1;
            }
            Operation::Multiply => {
                constraint.a[0] = num;
                constraint.b[index] = 1;
            }
        }
        self.generate_r1cs_constraints(&circuit, false);
    }

    fn handle_variable_nested_circuit_case(
        &mut self,
        var: String,
        circuit: &Circuit,
        operation: &Operation,
        constraint: &mut Constraint,
    ) {
        let nested_index = match self.variable_map.get_index(&circuit.hash.to_string()) {
            Some(index) => index,
            None => panic!("Cannot find index of variable in variable_indices"),
        };

        let var_index = match self.variable_map.get_index(&var) {
            Some(index) => index,
            None => panic!("Cannot find index of variable in variable_indices"),
        };
        match operation {
            Operation::Add => {
                constraint.a[nested_index] = 1;
                constraint.a[var_index] = 1;
                constraint.b[0] = 1;
            }
            Operation::Multiply => {
                constraint.a[nested_index] = 1;
                constraint.b[var_index] = 1;
            }
        }
        self.generate_r1cs_constraints(&circuit, false);
    }

    fn handle_double_nested_circuit_case(
        &mut self,
        circuit1: &Circuit,
        circuit2: &Circuit,
        operation: &Operation,
        constraint: &mut Constraint,
    ) {
        let index1 = match self.variable_map.get_index(&circuit1.hash.to_string()) {
            Some(index) => index,
            None => panic!("Cannot find index of variable in variable_indices"),
        };
        let index2 = match self.variable_map.get_index(&circuit2.hash.to_string()) {
            Some(index) => index,
            None => panic!("Cannot find index of variable in variable_indices"),
        };

        match operation {
            Operation::Add => {
                constraint.b[0] = 1;
                if index1 == index2 {
                    constraint.a[index1] = 2;
                } else {
                    constraint.a[index1] = 1;
                    constraint.a[index2] = 1;
                }
            }
            Operation::Multiply => {
                constraint.a[index1] = 1;
                constraint.b[index2] = 1;
            }
        }

        self.generate_r1cs_constraints(&circuit1, false);
        self.generate_r1cs_constraints(&circuit2, false);
    }

    pub fn compute_witness(
        &self,
        circuit: &Circuit,
        inputs: HashMap<String, i64>,
    ) -> HashMap<String, i64> {
        let mut witness: HashMap<String, i64> = HashMap::new();

        for (var, value) in inputs {
            witness.insert(var, value);
        }

        let output = self.evaluate_circuit_recursively(circuit, &mut witness, true);
        witness.insert("out".to_string(), output);
        witness
    }

    fn evaluate_circuit_recursively(
        &self,
        circuit: &Circuit,
        witness: &mut HashMap<String, i64>,
        root: bool,
    ) -> i64 {
        let mut values: [i64; 2] = [0; 2];
        for (i, operand) in circuit.operands.iter().enumerate() {
            values[i] = match operand {
                Operand::Number(num) => *num,
                Operand::Variable(var) => *witness.get(var).expect("Missing input variable"),
                Operand::NestedCircuit(nested_circuit) => {
                    self.evaluate_circuit_recursively(nested_circuit, witness, false)
                }
            }
        }

        let output = match circuit.operation {
            Operation::Add => values[0] + values[1],
            Operation::Multiply => values[0] * values[1],
        };

        if !root {
            witness.insert(circuit.hash.to_string(), output);
        }

        output
    }
}
