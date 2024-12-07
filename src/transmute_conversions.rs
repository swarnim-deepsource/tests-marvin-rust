mod test {
    use std::{rc::Rc, sync::Arc};

    fn trivial() {
        let val: u32 = 120;
        let ptr = &val as *const u32;
        unsafe {
            let rf1: &u32 = std::mem::transmute(ptr);
            let r2: *const u32 = std::mem::transmute(rf1);
            let r2: *const u32 = std::mem::transmute(rf1 as *const u32 as *const i32);
        }

        unsafe {
            let cpt1: &u64 = std::mem::transmute(val as u64);
            let cpt2: *const u64 = std::mem::transmute(val as u64);
            let tib: bool = std::mem::transmute(0i8);
            let ut1: u32 = std::mem::transmute(val);
            let ut2: &u64 = std::mem::transmute(cpt1);
            let ut3: *const u64 = std::mem::transmute(cpt2);
            let frwvpt = Box::from_raw(1 as *mut std::ffi::c_void);
            let frwvpt = Arc::from_raw(1 as *const std::ffi::c_void);
            let frwvpt = Rc::from_raw(1 as *const std::ffi::c_void);
        }

        unsafe {
            let nz1: std::num::NonZeroU32 = std::mem::transmute(val);
            let nz2: fn() -> () = std::mem::transmute(1 as *const u32);
            let nz2: fn() -> () = std::mem::transmute(std::ptr::null::<u32>());
            let nz2: *const u32 = std::mem::transmute(0u64);
            &10u32 as *const u32 == std::ptr::null();
        }

        unsafe {
            let x = 10.0f32;
            let y: i32 = std::mem::transmute(x);
            let x: f32 = std::mem::transmute(y);
            let rx: &f32 = std::mem::transmute(x as f64);
            let px: *const f64 = std::mem::transmute(x as f64);
            let px: *const i32 = std::mem::transmute(x as f64);
            let c = 'c';
            // TODO(swarnim): [RS-E1029]: "Transmutes from char to reference is invalid, and unsound"
            let rc: &i32 = std::mem::transmute(c as u64);
            // TODO(swarnim): [RS-E1029]: "Transmutes from char to pointer is invalid, and unsound"
            let pc: *const i32 = std::mem::transmute(c as u64);
            let x: u32 = 0x0000_0061;
            let c: char = std::mem::transmute(x);
            println!("{}", c);
            let x: u32 = 0x0000_D800;

            let c: char = std::mem::transmute(x);
            let zeroed_bytes: [u8; 8] = std::mem::transmute(0u64);
            let zeroed_array: [u64; 2] = std::mem::transmute((0u64, 0u64));
        }
    }
}
