#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos"))]
extern crate metal;
#[cfg(target_os = "linux")]
extern crate vulkano;
#[cfg(target_os = "windows")]
extern crate winapi;


#[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos"))]
#[ path = "./darwin.rs" ]
mod platform;

#[cfg(target_os = "linux")]
#[ path = "./linux.rs" ]
mod platform;




fn main() {
    println!("System Default:\n\t{:?}\n\n", platform::default() );

    println!("All:");
    for gpu_info in platform::all() {
        println!("\t{:?}", gpu_info);
    }
}
