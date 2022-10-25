#![feature(allocator_api, vec_into_raw_parts)]
extern crate x11;
//extern crate EPD_IT8951;
use std::ffi::CString;

use x11::xlib;
use std::process::Command;
mod terminal;
mod xorg;

mod IT8951;


use IT8951::epaper;
fn main() {
    mountdisplay();
    //unsafe{IT8951::DEV_Module_Init()};
    //unsafe{ DEV_Module_Init()};
    let _rm = Command::new("rm").args(["screen.png","output.raw"]).spawn().unwrap().wait();
    let _scrot = Command::new("scrot").args(["-D",":0","screen.png"]).spawn().unwrap().wait();
    //let ffmpeg= Command::new("ffmpeg").args(["-vcodec","png","-i","./screen.png","-vf","\"transpose=1\"","-c",":a","copy","-vcodec","rawvideo","-f","rawvideo","-pix_fmt","monow","output.raw"]).spawn().unwrap().wait();
    //let _ffmpeg= Command::new("ffmpeg").args(["-vcodec","png","-i","./screen.png","-vf","transpose=1","-c:a","copy","-vcodec","rawvideo","-f","rawvideo","-pix_fmt","monow","output.raw"]).spawn().unwrap().wait();
    let _ffmpeg= Command::new("ffmpeg").args(["-vcodec","png","-i","./screen.png","-vcodec","rawvideo","-f","rawvideo","-pix_fmt","monow","output.raw"]).spawn().unwrap().wait();
    let buffer= std::fs::read(std::path::Path::new("./output.raw")).unwrap();
    unsafe{

    let epaper: epaper = epaper::init(1810);
    println!("a:{}",epaper.info.Memory_Addr_L);
    println!("b:{}",epaper.info.Memory_Addr_H);
    println!("target:{}",epaper.gettargetaddr());
    epaper.clear();
    //let image: Vec<u8> = vec![1;<u32 as TryInto<usize>>::try_into(epaper.getimagesize()).unwrap()*2];
    epaper.writeimage(buffer);
    let _ = Command::new("sleep").arg("10").spawn().unwrap().wait();
    //dropped explicitly as 
    drop(epaper);
    }
    
    println!("Hello, world!");
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