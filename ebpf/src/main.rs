#![no_std]
#![no_main]

use aya_ebpf::macros::xdp;

use aya_ebpf::programs::XdpContext;
//
use aya_ebpf::bindings::xdp_action;

#[xdp]
pub fn fluxdrop_main(ctx: XdpContext) -> u32 {
    match try_fluxdrop_main(ctx) {
        Ok(verdict) => verdict,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[inline(always)]
fn try_fluxdrop_main(_ctx: XdpContext) -> Result<u32, u32> {
    Ok(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
