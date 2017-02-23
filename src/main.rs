#[macro_use]
extern crate nom;
#[macro_use]
extern crate enum_primitive;

extern crate subprocess;
use subprocess::*;

mod types;
mod parser;
mod encoder;

use std::io::{Read, Write};
use std::str::from_utf8;
use nom::newline;
use std::env;

fn not_newline(c: u8) -> bool {
    c != b'\n'
}

enum Info {
    String(String),
}

enum Response {
    UsiOk,
    ReadyOk,
    BestMove,
    Info(Info),
}

named!(info<&[u8], Response>,
       chain!(
       tag!(b"info string ") ~
       ret: take_while!(not_newline) ~
       newline,
       || Response::Info(Info::String(from_utf8(ret).unwrap().to_string()))
       ));

fn main() {
    //let mut buffer = String::new();
    //io::stdin().read_to_string(&mut buffer).unwrap();
    let buffer = "+7776FU,L599	-3334FU,L599	+2726FU,L598	-8384FU,L596	+2625FU,L588	-4132KI,L593	\
                  +6978KI,L587	-2288UM,L589	+7988GI,L585	-3122GI,L588	+3938GI,L583	-2233GI,L586	\
                  +8877GI,L581	-7172GI,L585	+1716FU,L577	-1314FU,L583	+9796FU,L565	-9394FU,L580	\
                  +4746FU,L562	-5142OU,L578	+3847GI,L561	-7283GI,L576	+3736FU,L557	-6152KI,L573	\
                  +2937KE,L556	-8485FU,L569	+5968OU,L554	-8384GI,L567	+6879OU,L552	-9495FU,L537	\
                  +9695FU,L550	-8495GI,L535	+0094FU,L540	-8586FU,L521	+8786FU,L538	-9586GI,L519	\
                  +7786GI,L536	-8286HI,L517	+0087FU,L535	-8683HI,L505	+0072KA,L520	-8385HI,L477	\
                  +9493TO,L511	-9193KY,L437	+9993NY,L508	-8193KE,L434	+7294UM,L505	-8555HI,L401	\
                  +5756FU,L499	-5554HI,L395	+9493UM,L497	-0098FU,L377	+0055KY,L491	-5464HI,L371	\
                  +6766FU,L485	-9899TO,L367	+3745KE,L483	-9989TO,L362	+7989OU,L482	-3344GI,L340	\
                  +2524FU,L479	-2324FU,L336	+0065KE,L471	-4445GI,L269	+4645FU,L466	-0095KY,L250	\
                  +0061GI,L460	-5262KI,L232	+9371UM,L456	-0085KE,L230	+7162UM,L453	-0057KA,L229	\
                  +6253UM,L445	-4233OU,L227	+5364UM,L429	-0098GI,L226	+8988OU,L426	-9899GI,L210	\
                  +8889OU,L421	-0077KE,L202	+7877KI,L416	-8577NK,L200	+0042GI,L406	-3323OU,L181	\
                  +0033KI,L400	-2133KE,L177	+4233NG,L397	-2333OU,L174	+0025KE,L392	-2425FU,L169	\
                  +6442UM,L371	-3242KI,L166	+0031HI,L368	-0032KE,L160	GOTE_WIN_TORYO"
        .to_string();

    let args: Vec<String> = env::args().collect();

    let g = parser::shougi_wars::parse(buffer.as_bytes()).unwrap();

    let p = Popen::create(&["/home/sunaemon/Gikou/bin/release"],
                          PopenConfig {
                              stdin: Redirection::Pipe,
                              stdout: Redirection::Pipe,
                              stderr: Redirection::Pipe,
                              ..Default::default()
                          })
        .unwrap();

    let mut stdin_ref = p.stdin.as_ref().unwrap();
    let mut stdout_ref = p.stdout.as_ref().unwrap();
    //let mut stderr_ref = p.stderr.as_ref().unwrap();

    stdin_ref.write_all(b"isready\n").unwrap();
    while {
        let mut buf = [0u8; 4096];
        let n = stdout_ref.read(&mut buf).unwrap();
        let s = from_utf8(&buf[0..n]).unwrap();
        println!("{:?}", s);
        !s.contains("readyok\n")
    } {}

    let n = args[1].parse::<usize>().unwrap();
    println!("{}", n);
    let pos_string = ::encoder::usi::position(&g.position, &g.moves[0..n].to_vec());

    let pos = pos_string.as_bytes();

    stdin_ref.write(pos).unwrap();
    stdin_ref.write_all(b"\ngo\n").unwrap();

    while {
        let mut buf = [0u8; 4096];
        let n = p.stdout.as_ref().unwrap().read(&mut buf).unwrap();
        let s = from_utf8(&buf[0..n]).unwrap();
        println!("{:?}", s);
        true
    } {}
}
