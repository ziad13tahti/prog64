enum OpCode {  //2bits

    Add,
    Sub,
    Mult,
    Div

}

enum Source {  //3bits

    R0,
    R1,
    R2,
    R3,
    I0,
    I1,
    I2,
    I3
}

enum Dest {  //2bits

    R0,
    R1,
    R2,
    R3

}

struct Instruction { //10bits

    pub Dst : Dest,
    pub Op : OpCode,
    pub Src1 : Source,
    pub Src2 : Source

}



//Decode l'operation (2bits)
fn decode_op(x:u64)->OpCode{

     if  x ==0b00 {
        return OpCode::Add;
     }

     else if x==0b01 {
        return OpCode::Sub;
     }

     else if x==0b10 {
        return OpCode::Mult;
     }

     else {
        return OpCode::Div;
     }
}

//Decode la source (3bits)
fn decode_src(x:u64)->Source{

     if  x ==0b000 {
        return Source::R0;
     }

     else if x==0b001 {
        return Source::R1;
     }

     else if x==0b010 {
        return Source::R2;
     }

     else if  x ==0b011 {
        return Source::R3;
     }

     else if x==0b100 {
        return Source::I0;
     }

     else if x==0b101 {
        return Source::I1;
     }

      else if  x ==0b110 {
        return Source::I2;
     }

     else {
        return Source::I3;
     }

}

//Decode l'Instruction (10 bits)
fn decode_inst(x:u64)->Instruction{


}

//Decode l'operation (60 bits) 
fn decode_prog(x:u64)->[Instruction;6]{


}