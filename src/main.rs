use std::fs::File;
use std::io::Read;

fn main()
{
    let mut file=File::open("load.gif").unwrap();

    let mut buf=[0u8;1];

    let mut gif = [0u8;3];
    file.read(&mut gif).unwrap();
    println!("{}{}{}", gif[0] as char, gif[1] as char, gif[2] as char);


    let mut version = [0u8;3];
    file.read(&mut version).unwrap();
    println!("Version: {}{}{}", version[0] as char, version[1] as char, version[2] as char);


    let mut dim = [0u8;4];
    file.read(&mut dim).unwrap();
    let width = { ((dim[1] as u16) << 8) | dim[0] as u16 };
    let height = { ((dim[3] as u16) << 8) | dim[2] as u16 };
    println!("Width: {}px; Height: {}px", width, height);

    file.read(&mut buf).unwrap();
    let b = buf[0];
    println!("Global color table: {:?}", b & 0b10000000 >> 7);
    println!("Color resolution: {}", b & 0b01110000 >> 4);
    println!("Sort flag: {}", b & 0b00001000 >> 3);
    let binary_color_table_size = b & 0b00000111;
    println!("Size of global color table: {}", binary_color_table_size);

    let mut infos = [0u8;2];
    file.read(&mut infos).unwrap();
    println!("Background color index: {}", infos[0]);
    println!("Pixel aspect ratio: {}", infos[1]);

    let nb_color = (2 as usize).pow(binary_color_table_size as u32 + 1);
    println!("Colors: #{}", nb_color);
    for n in 0..nb_color {
        let mut color = [0u8; 3];
        file.read(&mut color).unwrap();
        println!("#{}: {}, {}, {}", n, color[0], color[1], color[2]);
    }
}
