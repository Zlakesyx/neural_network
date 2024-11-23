static mut LATEST_NODE: i8 = 0;
static mut LATEST_CONNECTION: i8 = 0;

pub fn current_node() -> i8 {
    unsafe {
        return LATEST_NODE
    }
}

pub fn current_connection() -> i8 {
    unsafe {
        return LATEST_CONNECTION
    }
}

pub fn next_node() -> i8 {
    unsafe {
        LATEST_NODE += 1;
        return LATEST_NODE
    }
}

pub fn next_connection() -> i8 {
    unsafe {
        LATEST_CONNECTION += 1;
        return LATEST_CONNECTION
    }
}
