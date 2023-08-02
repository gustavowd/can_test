use socketcan::Frame;
use std::{env, time::Duration};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use isobus_stack::isobus::{Ecu, IsoBus, PGN_CODES, IsoBusTypes};

//sudo ip link set can0 up type can bitrate 250000

fn isobus_user_thread(this_ecu: Arc<Mutex<Ecu>>, response_rx: std::sync::mpsc::Receiver<PGN_CODES>) -> anyhow::Result<()> {
    let mut error_cnt = 0;
    let my_ecu = this_ecu.lock().unwrap();
    let request_tx = my_ecu.request_tx.clone();
    drop(my_ecu);
    loop {
        match isobus_stack::isobus::pgn_request(PGN_CODES::SOFTWARE_IDENTIFICATION, 0xA2, Duration::from_millis(500), &request_tx, &response_rx) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::SOFTWARE_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0xA2);
                    let other_ecu = &my_ecu.other_ecus[idx];
                    let packet = other_ecu.packet.clone();
                    drop(my_ecu);
                    println!("{:?}", packet);
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error: {}", err)
            }
        }

        thread::sleep(time::Duration::from_millis(200));

        match isobus_stack::isobus::pgn_request(PGN_CODES::SOFTWARE_IDENTIFICATION, 0x90, Duration::from_millis(500), &request_tx, &response_rx) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::SOFTWARE_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0x90);
                    let other_ecu = &my_ecu.other_ecus[idx];
                    let packet = other_ecu.packet.clone();
                    drop(my_ecu);
                    println!("{:?}", packet);
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error: {}", err)
            }
        }

        thread::sleep(time::Duration::from_millis(2000));

        match isobus_stack::isobus::pgn_request(PGN_CODES::ECU_IDENTIFICATION, 0xA2, Duration::from_millis(500), &request_tx, &response_rx) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::ECU_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0xA2);
                    let other_ecu = &my_ecu.other_ecus[idx];
                    let packet = other_ecu.packet.clone();
                    drop(my_ecu);
                    println!("{:?}", packet);
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error: {}", err)
            }
        }

        thread::sleep(time::Duration::from_millis(200));

        match isobus_stack::isobus::pgn_request(PGN_CODES::ECU_IDENTIFICATION, 0x90, Duration::from_millis(500), &request_tx, &response_rx) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::ECU_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0x90);
                    let other_ecu = &my_ecu.other_ecus[idx];
                    let packet = other_ecu.packet.clone();
                    drop(my_ecu);
                    println!("{:?}", packet);
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error: {}", err)
            }
        }

        thread::sleep(time::Duration::from_millis(2000));

        match isobus_stack::isobus::pgn_request(PGN_CODES::COMPONENT_IDENTIFICATION, 0xA2, Duration::from_millis(500), &request_tx, &response_rx) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::COMPONENT_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0xA2);
                    let other_ecu = &my_ecu.other_ecus[idx];
                    let packet = other_ecu.packet.clone();
                    drop(my_ecu);
                    println!("{:?}", packet);
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error: {}", err)
            }
        }

        thread::sleep(time::Duration::from_millis(200));

        match isobus_stack::isobus::pgn_request(PGN_CODES::COMPONENT_IDENTIFICATION, 0x90, Duration::from_millis(500), &request_tx, &response_rx) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::COMPONENT_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0x90);
                    let other_ecu = &my_ecu.other_ecus[idx];
                    let packet = other_ecu.packet.clone();
                    drop(my_ecu);
                    println!("{:?}", packet);
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error: {}", err)
            }
        }

        if error_cnt >= 10 {
            break;
        }
        
        thread::sleep(time::Duration::from_millis(5000));
    }
    Ok(())
}

fn receive_callback(sa: u8, packet: IsoBusTypes) {
    println!("Received {:?} from {}", packet, sa);
}

fn main() -> anyhow::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "can0".into());

    let mut handles = vec![];
    let (ecu, response_rx) = isobus_stack::isobus::start_isobus_stack(iface, 0xAA, receive_callback);

    let handle = thread::spawn(move || isobus_user_thread(ecu, response_rx));
    handles.push(handle);

    for handle in handles {
        match handle.join() {
            Ok(result) => {
                result?
            },
            Err(_err) => {
                return Err(anyhow::format_err!("Not possible to spawn thread!"))
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn frame_to_string<F: Frame>(frame: &F) -> String {
    let id = frame.raw_id();
    let data_string = frame
        .data()
        .iter()
        .fold(String::from(""), |a, b| format!("{} {:02x}", a, b));

    format!("{:X}  [{}] {}", id, frame.dlc(), data_string)
}
