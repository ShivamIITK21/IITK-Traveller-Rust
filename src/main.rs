use lib::interpreter::interpreter;

fn main(){
    let out = interpreter::interpret("start, 0, iit_gate_in_1;
    iit_gate_in_1, 0 ,oat_stairs_2;
    oat_stairs_2, 0, mt_3_2;
    mt_3_2, 0, oat_stairs_c;
    oat_stairs_c, 1, hall_13_2;
    hall_13_2, 1, southern_labs_c;
    southern_labs_c, 0, lecture_hall_gt; 
    lecture_hall_gt_t, 0, mt_2_3;
    mt_2_3, 0, hall_3;
    hall_3, 0, southern_labs_1;
    southern_labs_1, 0, hall_13_2;
    hall_13_2, 0, lecture_hall_gt;
    lecture_hall_gt_f, 0, mt_1_3;
    mt_1_3, 0, iit_gate_out_1;
    iit_gate_out_1, 0, finish;", "4");

    println!("{}", out);
}