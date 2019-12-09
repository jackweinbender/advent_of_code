import { Point } from "./point"
import { Instruction } from "./instruction";

export class Vector {
    instruction: Instruction
    origin: Point
    start_length: number
    constructor(instruction: Instruction, origin: Point = new Point(0,0), start_legth: number = 0) {
        this.instruction = instruction;
        this.origin = origin;
        this.start_length = start_legth;
    }

    end(){
        return this.origin.move(this.instruction)
    }

    orientation() {
        if (this.isVertical()){ return 'v' }
        else { return 'h'}
    }
    
    isVertical(){ return this.instruction.direction === 'U' || this.instruction.direction === 'D' }
    
    contains_point(point: Point){ 
        if (this.orientation() === 'v' && this.origin.x === point.x){
            const v_line = [this.origin, this.end()]
                .sort( (a,b) => { return a.y - b.y })
            return point.y > v_line[0].y && point.y < v_line[1].y
        } else if (this.orientation() === 'h' && this.origin.y === point.y) {
            const h_line = [this.origin, this.end()]
                .sort( (a,b) => { return a.x - b.x })
            return point.x > h_line[0].x && point.x < h_line[1].x
        } else { 
            return false 
        }
    }

    static intersection(vert: Vector, horiz: Vector){
        if( vert.orientation() == horiz.orientation()) { 
            return false
        } else if (vert.orientation() === 'h'){ 
            return this.intersection(horiz, vert) 
        } else {
            const v_line = [vert.origin, vert.end()]
                .sort( (a,b) => { return a.y - b.y })

            const h_line = [horiz.origin, horiz.end()]
                .sort( (a,b) => { return a.x - b.x })

            const x_in_range = vert.origin.x > h_line[0].x && vert.origin.x < h_line[1].x
            const y_in_range = horiz.origin.y > v_line[0].y && horiz.origin.y < v_line[1].y

            if (x_in_range && y_in_range){
                return new Point(vert.origin.x, horiz.origin.y)
            } else {
                return false
            }
        }
    }
}
