use log::debug;
use yew::prelude::*;
use lib::interpreter::interpreter::interpret;
use lib::test::test::greet;
use log::Level;
use log::info;

#[function_component]
fn App() -> Html{

    let counter = use_state(|| 0);

    let code = "start, 0, iit_gate_in_1;
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
    iit_gate_out_1, 0, finish;";

    let inp = "5";

    let out = interpret(code , inp);
    // debug!("{}",out);

    
    let onclick = {
        let counter = counter.clone();
        move |_|{
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html!{
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{*counter}</p>
            <p>{out}</p>
        </div>
    }
}


fn main(){
    // console_log::init_with_level(Level::Debug);
    yew::Renderer::<App>::new().render();
    // println!("{}", App());


    // let code = "start, 0, iit_gate_in_1;
    // iit_gate_in_1, 0 ,oat_stairs_2;
    // oat_stairs_2, 0, mt_3_2;
    // mt_3_2, 0, oat_stairs_c;
    // oat_stairs_c, 1, hall_13_2;
    // hall_13_2, 1, southern_labs_c;
    // southern_labs_c, 0, lecture_hall_gt; 
    // lecture_hall_gt_t, 0, mt_2_3;
    // mt_2_3, 0, hall_3;
    // hall_3, 0, southern_labs_1;
    // southern_labs_1, 0, hall_13_2;
    // hall_13_2, 0, lecture_hall_gt;
    // lecture_hall_gt_f, 0, mt_1_3;
    // mt_1_3, 0, iit_gate_out_1;
    // iit_gate_out_1, 0, finish;";

    // let code = "start, 0, iit_gate_in_1;
    // iit_gate_in_1, 0, iit_gate_out_1;
    // iit_gate_out_1, 0, finish;";

    // let inp = "16";

    // let out = interpret(code , inp);

    // println!("{}", out);

}