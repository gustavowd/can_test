use std::{env, time::Duration};
use std::{thread, time};

use isobus_stack::isobus::{IsoBus, PGN_CODES, IsoBusTypes};

//sudo ip link set can0 up type can bitrate 250000

fn receive_callback(sa: u8, packet: IsoBusTypes) {
    println!("Received {:?} from {}", packet, sa);
}

fn main() -> anyhow::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "can0".into());

    let this_ecu = isobus_stack::isobus::start_isobus_stack(iface, 0xAA, receive_callback);

    let mut error_cnt = 0;
    loop {
        match isobus_stack::isobus::pgn_request(&this_ecu, PGN_CODES::SOFTWARE_IDENTIFICATION, 0xA2, Duration::from_millis(500)) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::SOFTWARE_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0xA2);
                    let packet = my_ecu.other_ecus[idx].packet.clone();
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

        match isobus_stack::isobus::pgn_request(&this_ecu, PGN_CODES::SOFTWARE_IDENTIFICATION, 0x90, Duration::from_millis(500)) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::SOFTWARE_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0x90);
                    let packet = my_ecu.other_ecus[idx].packet.clone();
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

        match isobus_stack::isobus::pgn_request(&this_ecu, PGN_CODES::ECU_IDENTIFICATION, 0xA2, Duration::from_millis(500)) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::ECU_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0xA2);
                    let packet = my_ecu.other_ecus[idx].packet.clone();
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

        match isobus_stack::isobus::pgn_request(&this_ecu, PGN_CODES::ECU_IDENTIFICATION, 0x90, Duration::from_millis(500)) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::ECU_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0x90);
                    let packet = my_ecu.other_ecus[idx].packet.clone();
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

        match isobus_stack::isobus::pgn_request(&this_ecu, PGN_CODES::COMPONENT_IDENTIFICATION, 0xA2, Duration::from_millis(500)) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::COMPONENT_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0xA2);
                    let packet = my_ecu.other_ecus[idx].packet.clone();
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

        match isobus_stack::isobus::pgn_request(&this_ecu, PGN_CODES::COMPONENT_IDENTIFICATION, 0x90, Duration::from_millis(500)) {
            Ok(pgn) => {
                error_cnt = 0;
                if pgn == PGN_CODES::COMPONENT_IDENTIFICATION {
                    println!("Successfull received PGN: {:?}", pgn);
                    let mut my_ecu = this_ecu.lock().unwrap();
                    let idx = my_ecu.get_other_ecu_idx(0x90);
                    let packet = my_ecu.other_ecus[idx].packet.clone();
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
