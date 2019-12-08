import { Point } from "./point"
import { Instruction } from "./instruction";

export class Vector {
    instruction: Instruction
    origin: Point

    constructor(instruction: Instruction, origin: Point = new Point(0,0)) {
        this.instruction = this.instruction;
        this.origin = origin;
    }

    end(){
        return this.origin.move(this.instruction)
    }

    isVertical(){ return this.instruction.direction === 'U' || this.instruction.direction === 'D' }
    isHorizontal(){ return this.instruction.direction === 'L' || this.instruction.direction === 'R' }

    static intersection(left: Vector, right: Vector){
        if( left.isVertical() && right.isVertical() ) { return false }
        if( left.isHorizontal() && right.isHorizontal() ) { return false }

    }
}
