use crate::config::{APP_ADDRESS_STEP, APP_BASE_ADDRESS, MAX_JOB_NUM};
use crate::kernel_debug;
use core::slice::from_raw_parts;
use core::slice::from_raw_parts_mut;

pub fn load_apps() -> ([usize; MAX_JOB_NUM], usize) {
    extern "C" {
        fn _num_app();
    }
    kernel_debug!("load app to memory");
    let app_meta_table_ptr = _num_app as usize as *const u64;
    let app_num = unsafe { app_meta_table_ptr.read_volatile() };
    let app_link_addr_table =
        unsafe { core::slice::from_raw_parts(app_meta_table_ptr.add(1), (app_num + 1) as usize) };
    kernel_debug!("app_link_addr_table = {:?}", app_link_addr_table);
    let mut app_runtime_addresses = [0; MAX_JOB_NUM];
    for (i, address_range) in app_link_addr_table.windows(2).enumerate() {
        let (app_start, app_end) = (address_range[0], address_range[1]);
        let dst_addr = APP_BASE_ADDRESS + i * APP_ADDRESS_STEP;
        kernel_debug!(
            "app_{}, copy from [{:#x}, {:#x}] to {:#x}",
            i,
            app_start,
            app_end,
            dst_addr
        );
        app_runtime_addresses[i] = dst_addr;
        (dst_addr..dst_addr + APP_ADDRESS_STEP).for_each(|addr| unsafe {
            (addr as *mut usize).write_volatile(0);
        });
        let src =
            unsafe { from_raw_parts(app_start as *const usize, (app_end - app_start) as usize) };
        let dst = unsafe { from_raw_parts_mut(dst_addr as *mut usize, src.len()) };
        dst.copy_from_slice(src);
    }
    (app_runtime_addresses, app_num as usize)
}
