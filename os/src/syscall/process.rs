//! Process management syscalls
use crate::mm::{
    translated_byte_buffer, MapPermission, PageTable, PTEFlags, VirtAddr,
};
use crate::task::{
    change_program_brk, current_task_mmap, current_task_munmap, current_user_token,
    exit_current_and_run_next, get_current_syscall_count, suspend_current_and_run_next,
};
use crate::timer::get_time_us;
use core::mem::size_of;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let timeval = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let token = current_user_token();
    let mut buffers = translated_byte_buffer(token, ts as *const u8, size_of::<TimeVal>());
    let src = unsafe {
        core::slice::from_raw_parts(
            &timeval as *const TimeVal as *const u8,
            size_of::<TimeVal>(),
        )
    };

    let mut copied = 0;
    for buffer in buffers.iter_mut() {
        let len = buffer.len();
        buffer.copy_from_slice(&src[copied..copied + len]);
        copied += len;
    }

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            let page_table = PageTable::from_token(current_user_token());
            let va = VirtAddr::from(id);
            let vpn = va.floor();
            let offset = va.page_offset();

            match page_table.translate(vpn) {
                Some(pte)
                    if pte.is_valid()
                        && pte.flags().contains(PTEFlags::U)
                        && pte.flags().contains(PTEFlags::R) =>
                {
                    pte.ppn().get_bytes_array()[offset] as isize
                }
                _ => -1,
            }
        }
        1 => {
            let page_table = PageTable::from_token(current_user_token());
            let va = VirtAddr::from(id);
            let vpn = va.floor();
            let offset = va.page_offset();

            match page_table.translate(vpn) {
                Some(pte)
                    if pte.is_valid()
                        && pte.flags().contains(PTEFlags::U)
                        && pte.flags().contains(PTEFlags::W) =>
                {
                    pte.ppn().get_bytes_array()[offset] = data as u8;
                    0
                }
                _ => -1,
            }
        }
        2 => get_current_syscall_count(id),
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap");
    if start % crate::config::PAGE_SIZE != 0 {
        return -1;
    }
    if prot & !0x7 != 0 || prot & 0x7 == 0 {
        return -1;
    }
    let Some(_end) = start.checked_add(len) else {
        return -1;
    };
    if len == 0 {
        return 0;
    }

    let mut permission = MapPermission::U;
    if prot & 0x1 != 0 {
        permission |= MapPermission::R;
    }
    if prot & 0x2 != 0 {
        permission |= MapPermission::W;
    }
    if prot & 0x4 != 0 {
        permission |= MapPermission::X;
    }

    if current_task_mmap(start, len, permission) {
        0
    } else {
        -1
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    if start % crate::config::PAGE_SIZE != 0 {
        return -1;
    }
    let Some(_end) = start.checked_add(len) else {
        return -1;
    };
    if len == 0 {
        return 0;
    }
    if current_task_munmap(start, len) {
        0
    } else {
        -1
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
