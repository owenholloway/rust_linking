use crate::utils::lata_add;

#[no_mangle]
pub extern "C" fn example() -> i32 {

    return unsafe {
        lata_add(1, 5)
    };
}

pub fn it_works() -> String {

    if unsafe { lata_add(1, 1) } == 2 {
        return "It Works!".to_owned();
    }

    return "It Sort of Works!".to_owned();
    
}