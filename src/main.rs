use anyhow::{Context, Error};
use embedded_can::blocking::Can;
//use embedded_can::blocking::Can;
//use socketcan::{CanFrame, CanSocket, Frame, Socket};
use socketcan::{CanSocket, Frame, Socket};
//use socketcan::{CanSocket, Socket};
use std::{env, time::Duration};
//use std::env;
use std::sync::{Arc, Mutex, mpsc};
use std::{thread, time};

use isobus_stack::isobus::{Ecu, IsoBus, PGN_CODES, MessageTypes};

//sudo ip link set can0 up type can bitrate 250000

/*
fn isobus_user_thread2(tx: std::sync::mpsc::SyncSender<(PGN_CODES, u8)>) -> anyhow::Result<()> {
    let mut error_cnt = 0;
    loop {
        match tx.send((PGN_CODES::SOFTWARE_IDENTIFICATION, 0xA2)) {
            Ok(()) => {
                error_cnt = 0;
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error when sending request: {}", err)
            },
        }

        if error_cnt >= 10 {
            break;
        }
        
        thread::sleep(time::Duration::from_millis(1000));
    }
    Ok(())
}
*/

fn isobus_user_thread(request_tx: std::sync::mpsc::SyncSender<(PGN_CODES, u8, MessageTypes)>, response_rx: std::sync::mpsc::Receiver<PGN_CODES>) -> anyhow::Result<()> {
    let mut error_cnt = 0;
    loop {
        match request_tx.send((PGN_CODES::SOFTWARE_IDENTIFICATION, 0xA2, MessageTypes::RequestMessage)) {
            Ok(()) => {
                error_cnt = 0;
                match response_rx.recv_timeout(Duration::from_millis(500)) {
                    Ok(pgn) => {
                        println!("Successfull received PGN: {:?}", pgn);
                    },
                    Err(err) => {
                        error_cnt += 1;
                        println!("Error when sending request: {}", err)
                    }
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error when sending request: {}", err)
            },
        }

        thread::sleep(time::Duration::from_millis(200));

        match request_tx.send((PGN_CODES::SOFTWARE_IDENTIFICATION, 0x90, MessageTypes::RequestMessage)) {
            Ok(()) => {
                error_cnt = 0;
                match response_rx.recv_timeout(Duration::from_millis(500)) {
                    Ok(pgn) => {
                        println!("Successfull received PGN: {:?}", pgn);
                    },
                    Err(err) => {
                        error_cnt += 1;
                        println!("Error when sending request: {}", err)
                    }
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error when sending request: {}", err)
            },
        }

        thread::sleep(time::Duration::from_millis(2000));

        match request_tx.send((PGN_CODES::ECU_IDENTIFICATION, 0xA2, MessageTypes::RequestMessage)) {
            Ok(()) => {
                error_cnt = 0;
                match response_rx.recv_timeout(Duration::from_millis(500)) {
                    Ok(pgn) => {
                        println!("Successfull received PGN: {:?}", pgn);
                    },
                    Err(err) => {
                        error_cnt += 1;
                        println!("Error when sending request: {}", err)
                    }
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error when sending request: {}", err)
            },
        }

        thread::sleep(time::Duration::from_millis(200));

        match request_tx.send((PGN_CODES::ECU_IDENTIFICATION, 0x90, MessageTypes::RequestMessage)) {
            Ok(()) => {
                error_cnt = 0;
                match response_rx.recv_timeout(Duration::from_millis(500)) {
                    Ok(pgn) => {
                        println!("Successfull received PGN: {:?}", pgn);
                    },
                    Err(err) => {
                        error_cnt += 1;
                        println!("Error when sending request: {}", err)
                    }
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error when sending request: {}", err)
            },
        }

        thread::sleep(time::Duration::from_millis(2000));

        match request_tx.send((PGN_CODES::COMPONENT_IDENTIFICATION, 0xA2, MessageTypes::RequestMessage)) {
            Ok(()) => {
                error_cnt = 0;
                match response_rx.recv_timeout(Duration::from_millis(500)) {
                    Ok(pgn) => {
                        println!("Successfull received PGN: {:?} from 0xA2", pgn);
                    },
                    Err(err) => {
                        error_cnt += 1;
                        println!("Error when sending request: {}", err)
                    }
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error when sending request: {}", err)
            },
        }

        thread::sleep(time::Duration::from_millis(200));

        match request_tx.send((PGN_CODES::COMPONENT_IDENTIFICATION, 0x90, MessageTypes::RequestMessage)) {
            Ok(()) => {
                error_cnt = 0;
                match response_rx.recv_timeout(Duration::from_millis(500)) {
                    Ok(pgn) => {
                        println!("Successfull received PGN: {:?} from 0x90", pgn);
                    },
                    Err(err) => {
                        error_cnt += 1;
                        println!("Error when sending request: {}", err)
                    }
                }
            },
            Err(err) => {
                error_cnt += 1;
                println!("Error when sending request: {}", err)
            },
        }

        if error_cnt >= 10 {
            break;
        }
        
        thread::sleep(time::Duration::from_millis(5000));
    }
    Ok(())
}

fn isobus_transmit_thread(iface: String, this_ecu: Arc<Mutex<Ecu>>, rx: std::sync::mpsc::Receiver<(PGN_CODES, u8, MessageTypes)>) -> anyhow::Result<()> {
    let mut sock = CanSocket::open(&iface)
        .with_context(|| format!("Failed to open socket on interface {}", iface))?;

    let mut my_ecu = this_ecu.lock().unwrap();

    my_ecu.ecu_information.identifications.software_identification.identifications.push("SAE J1939 3!".to_string());

    let frames = my_ecu.start_ecu();
    drop(my_ecu);
    match frames {
        Ok(frames) => {
            for frame in frames.iter() {
                if let Err(err) = sock.transmit(frame).context("Transmitting frame") {
                    println!("Transmit result: {:?}", err);
                }
            }
        },
        Err(err) => return Err(Error::msg(err)),
    }

    loop {
        let request = rx.recv();
        match request {
            Ok((pgn, da, message_type)) => {
                match message_type{
                    MessageTypes::RequestMessage => {
                        let my_ecu = this_ecu.lock().unwrap();
                        let frame = my_ecu.prepare_send_request(pgn, 6, da);
                        drop(my_ecu);
                
                        match frame {
                            Ok(frame) => {
                                if let Err(err) = sock.transmit(&frame).context("Transmitting frame") {
                                    println!("Transmit result: {:?}", err);
                                }
                                //println!("Transmit result: {:?}", frame);
                            },
                            Err(err) => println!("Error: {:?}", err),
                        }
                    },
                    MessageTypes::FastPacket => {

                    },
                    MessageTypes::TPMessage => {
                        loop {
                        let mut my_ecu = this_ecu.lock().unwrap();
                        let frame = my_ecu.send_transport_protocol_data_transfer(da);
                        drop(my_ecu);
                            if let Some(frame) = frame {
                                if let Err(err) = sock.transmit(&frame).context("Transmitting frame") {
                                    let mut my_ecu = this_ecu.lock().unwrap();
                                    let idx = my_ecu.get_other_ecu_idx(da);
                                    let other_ecu = &mut my_ecu.other_ecus[idx];
                                    other_ecu.tp_send.transmitting = false;
                                    other_ecu.tp_send.message_size = 0;
                                    other_ecu.tp_send.number_of_packets = 0;
                                    drop(my_ecu);
                                    println!("Transmit result: {:?}", err);
                                }
                                //println!("Transmit result: {:?}", frame);
                            }else {
                                let mut my_ecu = this_ecu.lock().unwrap();
                                let idx = my_ecu.get_other_ecu_idx(da);
                                let other_ecu = &mut my_ecu.other_ecus[idx];
                                other_ecu.tp_send.transmitting = false;
                                other_ecu.tp_send.message_size = 0;
                                drop(my_ecu);
                                break;
                            }
                        }
                    }
                    _ => {
                        // Do nothing
                    }
                }
            },
            Err(err) => println!("Error: {:?}", err),
        }
    }
}

fn isobus_receive_thread(iface: String, this_ecu: Arc<Mutex<Ecu>>, response_tx: std::sync::mpsc::Sender<PGN_CODES>, request_tx: std::sync::mpsc::SyncSender<(PGN_CODES, u8, MessageTypes)>) -> anyhow::Result<()> {
    let mut sock = CanSocket::open(&iface)
        .with_context(|| format!("Failed to open socket on interface {}", iface))?;

    //sock.set_read_timeout(Duration::from_secs(10))?;
    loop {
        let frame = sock.receive().context("Receiving frame");
        match frame {
            Ok(frame) => {
                //println!("{}  {}", iface, frame_to_string(&frame));
                let mut my_ecu = this_ecu.lock().unwrap();
                let result = my_ecu.frame_decode(&frame, &mut sock);
                drop(my_ecu);
                match result {
                    Ok((pgn, sa, message_type)) => {
                        if message_type == MessageTypes::ReceivedMessage {
                            //println!("PGN {:?} received from node {}. ", pgn, sa);
                            let mut my_ecu = this_ecu.lock().unwrap();
                            let pgn_code = pgn as u32;
                            if pgn_code < 0xFEFEFEFE {
                                let idx = my_ecu.get_other_ecu_idx(sa);
                                let packet = my_ecu.other_ecus[idx].packet.clone();
                                let message_type = my_ecu.other_ecus[idx].message_event.message_type;
                                let message_pgn = my_ecu.other_ecus[idx].message_event.message_pgn;
                                drop(my_ecu);
                                match message_type {
                                    Some(MessageTypes::TPMessage) => {
                                        if let Some(message_pgn) = message_pgn {
                                            match response_tx.send(message_pgn) {
                                                Ok(()) => {},
                                                Err(_err) => {}
                                            }
                                        }
                                    },
                                    Some(MessageTypes::SingleMessage) => {
                                        if let Some(message_pgn) = message_pgn {
                                            match response_tx.send(message_pgn) {
                                                Ok(()) => {},
                                                Err(_err) => {}
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                                println!("{:?}", packet);
                            }
                        }
                        if message_type == MessageTypes::TPMessage {
                            match request_tx.send((pgn, sa, message_type)) {
                                Ok(()) => {},
                                Err(_err) => {}
                            }
                        }
                    },
                    Err(_err) => {},//println!("Decoding error: {}", err)
                }
            },
            Err(err) => println!("Erro: {:?}", err),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "can0".into());

    //let text = String::from("Teste de split de string****Teste2");
    //let parts = text.split('*');
    //let collection: Vec<&str> = parts.collect();
    //println!("Coleção: {:?}", collection);

    let my_ecu = Arc::new(Mutex::new(Ecu::new(0xAA)));
    #[allow(clippy::type_complexity)]
    let (request_tx, request_rx) = mpsc::sync_channel(32);
    let (response_tx, response_rx) = mpsc::channel();

    let mut handles = vec![];
    let ecu1: Arc<Mutex<Ecu>> = Arc::clone(&my_ecu);
    let t1_iface = iface.clone();
    let tx2 = request_tx.clone();
    let handle = thread::spawn(move || isobus_receive_thread(t1_iface, ecu1, response_tx, tx2));
    handles.push(handle);

    let t2_iface = iface.clone();
    let ecu2: Arc<Mutex<Ecu>> = Arc::clone(&my_ecu);
    let handle = thread::spawn(move || isobus_transmit_thread(t2_iface, ecu2, request_rx));
    handles.push(handle);

    //let tx2 = request_tx.clone();
    //let handle = thread::spawn(move || isobus_user_thread2(tx2));
    //handles.push(handle);

    let handle = thread::spawn(move || isobus_user_thread(request_tx, response_rx));
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
