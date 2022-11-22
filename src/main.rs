#![feature(allocator_api, vec_into_raw_parts)]
extern crate x11;
//extern crate EPD_IT8951;
use std::ffi::CString;

use termion::{style, color};


use x11::xlib;
use std::process::Command;
mod terminal;
mod xorg;

mod IT8951;

#[macro_use] extern crate rocket;
use std::io::{Write, Read};
#[post("/", data = "<data>")]
async fn writeconf(data: String){
    let mut _file = std::fs::File::create("config.json").unwrap();
    writeln!(&mut _file, "{}",data).unwrap();
    std::fs::copy("config.json",  home::home_dir().unwrap().join("smartclock/config.json")).unwrap();
    match home::home_dir() {
        Some(path) => println!("{}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }
}

async fn readasynync() -> std::io::Result<()>{
    let mut str = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut str)?;
    println!("{}{}{}",color::Fg(color::LightGreen),str,color::Fg(color::Reset));

    Ok(())
} 
use IT8951::epaper;
#[rocket::main]
async fn main(){
        let mut dir: String =home::home_dir().unwrap().join("smartclock/updatefifo").into_os_string().into_string().unwrap();
    println!("{}",dir);
    let result = Command::new("mkfifo")
        .arg(dir)
        .status()
        .unwrap();

    println!("test");
    rocket::build()
        .mount("/config", routes![writeconf]).launch();

    mountdisplay();
    //unsafe{IT8951::DEV_Module_Init()};
    //unsafe{ DEV_Module_Init()};
    unsafe{

    let epaper: epaper = epaper::init(1810);
    println!("a:{}",epaper.info.Memory_Addr_L);
    println!("b:{}",epaper.info.Memory_Addr_H);
    println!("target:{}",epaper.gettargetaddr());
    epaper.clear();
    let _ = Command::new("sleep").arg("5").spawn().unwrap().wait();
    //let image: Vec<u8> = vec![1;<u32 as TryInto<usize>>::try_into(epaper.getimagesize()).unwrap()*2];
    //async_std::io

    //xauth to let root allow to use the display
    let _ = Command::new("bash").args(["-c","xauth add $(xauth -f ~pk/.Xauthority list | tail -1)"]).spawn().unwrap().wait();
    let _ = Command::new("./smartclock/build/linux/arm64/release/bundle/smartclock").spawn().expect("smartclock binary not found.");
    let _ = Command::new("sleep").arg("10").spawn().unwrap().wait();
    let _ = Command::new("wmctrl").args(["-r","smartclock","-b","add,fullscreen"]).spawn().unwrap().wait();
    //let mut stdin = std::io::stdin();
    //let mut buf =std::io::BorrowedBuf();
    //let mut buf=vec![0; 100];
    
    loop {
        //terminal::success(stdin.read(&mut buf).unwrap());
        //println!("{}{}{}",color::Fg(color::LightGreen),stdin.read(&mut buf).unwrap(),color::Fg(color::Reset));
        let _rm = Command::new("rm").args(["screen.png","output.raw"]).spawn().unwrap().wait();
        let _scrot = Command::new("scrot").args(["-D",":0","screen.png"]).spawn().unwrap().wait();
        //let ffmpeg= Command::new("ffmpeg").args(["-vcodec","png","-i","./screen.png","-vf","\"transpose=1\"","-c",":a","copy","-vcodec","rawvideo","-f","rawvideo","-pix_fmt","monow","output.raw"]).spawn().unwrap().wait();
        //let _ffmpeg= Command::new("ffmpeg").args(["-vcodec","png","-i","./screen.png","-vf","transpose=1","-c:a","copy","-vcodec","rawvideo","-f","rawvideo","-pix_fmt","monow","output.raw"]).spawn().unwrap().wait();
        //let _ffmpeg= Command::new("ffmpeg").args(["-vcodec","png","-i","./screen.png","-vcodec","rawvideo","-f","rawvideo","-pix_fmt","monow","output.raw"]).spawn().unwrap().wait();
        let _ffmpeg= Command::new("ffmpeg").args(["-vcodec","png","-i","./screen.png","-vcodec","rawvideo","-f","rawvideo","-pix_fmt","gray","output.raw"]).spawn().unwrap().wait();
        let buffer= std::fs::read(std::path::Path::new("./output.raw")).unwrap();
        epaper.writeimage(buffer);
        let _ = Command::new("sleep").arg("5").spawn().unwrap().wait();
        //readasynync().await;
    }
    //dropped explicitly as it also cleares the screen 
    drop(epaper);
    }
    println!("exit successfully");     
}

fn mountdisplay(){
    unsafe {
        let _handle = Command::new("bash").args(["-c","xrandr -d :0 --newmode \"1200x825R\"   69.00  1200 1248 1280 1360  825 828 838 849 +hsync -vsync"]).spawn().unwrap().wait();
        // Open display connection.
        #[allow(temporary_cstring_as_ptr)] //the deallocation at the end is fine here as XOpenDisplay doesn't use the pointer after the call,
        let display = xlib::XOpenDisplay(CString::new(":0").unwrap().as_ptr());
        //let display = xlib::XOpenDisplay(std::ptr::null());
        
        if display.is_null() {
            panic!("XOpenDisplay failed. This is propably due to ':0' not being the XID.");
        }

        let window= xlib::XDefaultRootWindow(display);
        let resources= x11::xrandr::XRRGetScreenResources(display,window);
        if resources.is_null(){
            panic!("failed to obtain resources from XRR");
        }
        terminal::success("Getting XRR Resources");
        println!(
             "    found {} crtc's
            \r    found {} mode's
            \r    found {} output's",
            (*resources).ncrtc,(*resources).nmode,(*resources).noutput
        );

        /*for i in 0..(*resources).ncrtc{
            let crtc = *((*resources).crtcs.offset((i*1).try_into().unwrap()));

            let crtcinfo = xrandr::XRRGetCrtcInfo(display,resources,crtc);

            println!("width:{},height:{}",(*crtcinfo).width,(*crtcinfo).height);

            xrandr::XRRFreeCrtcInfo(crtcinfo);
        }*/
        xorg::mountmodetofree(display,resources,"1200x825R");
    }
}
