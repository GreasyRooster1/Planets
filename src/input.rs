use user32;
use winapi::ctypes::c_int;

pub fn is_key_pressed(key_code:i32)->bool{
    (unsafe { winapi::um::winuser::GetKeyState(key_code as c_int) } & (1 << 15) != 0)
}