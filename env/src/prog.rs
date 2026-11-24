enum OpCode {

    Add,
    Sub,
    Mult,
    Div
}

enum Source {

    R0,
    R1,
    R2,
    R3,
    I0,
    I1,
    I2,
    I3
}

struct Instruction {


}



//Decode l'operation (2bits)
fn decode_op(x:u64)->OpCode{


}

//Decode la source (3bits)
fn decode_src(x:u64)->Source{


}

//Decode l'Instruction (10 bits)
fn decode_inst(x:u64)->Instruction{


}

//Decode l'operation (60 bits)
fn decode_prog(x:u64)->[Instruction;6]{


}