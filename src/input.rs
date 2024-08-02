use user32;
use winapi::ctypes::c_int;

fn is_key_pressed(key_code:i32)->bool{
    unsafe { user32::GetAsyncKeyState(key_code as c_int) } == -32767
}