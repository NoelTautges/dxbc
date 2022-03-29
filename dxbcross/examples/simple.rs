extern crate dxbcross;
extern crate pretty_hex;
extern crate rspirv;

use pretty_hex::PrettyHex;
use rspirv::binary::Disassemble;

fn main() {
    let spirv = include_bytes!("shader.spirv");

    let module = dxbcross::SpirvModule::from_bytes(spirv);
    let dxbc = module.translate_entrypoint("vs", dxbcross::TargetVersion::V5_0);

    let mut loader = rspirv::dr::Loader::new();
    rspirv::binary::parse_bytes(&spirv[..], &mut loader).unwrap();
    let module = loader.module();

    println!("{}", module.disassemble());
    //println!("{:#?}", module);
    let bytes = unsafe { std::slice::from_raw_parts(dxbc.as_ptr() as _, dxbc.len() * 4) };
    println!("{:?}", bytes.hex_dump());

    use std::fs::File;
    use std::io::Write;
    File::create("..\\dxbcd\\assembled.dxbc")
        .unwrap()
        .write_all(bytes)
        .unwrap();
}
