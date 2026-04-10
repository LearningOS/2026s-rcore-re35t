##### 实现功能：

---

实现了sys_trace调用，通过在TaskControlBlock里加入一个task_syscall_count的定长数组存放每个syscall id的次数，

再通过对TaskManagerInner获取当前任务写读取这个定长数组的接口，在syscall里面进行调用recor_current_task_syscall进行计数，在sys_trace调用get_current_task_syscall_count实现读取。

##### 问答题：

---

###### 1、

```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

```
[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
```

ch2b_bad_address.rs

```
unsafe {
        #[allow(clippy::zero_ptr)]
        (0x0 as *mut u8).write_volatile(0);
    }
```

访问非U态空间地址0x0,并尝试写入，在store指令时cpu抛出异常被trap/mod.rs中Exception::StoreFault捕捉最终输出。

ch2b_bad_instructions.rs

```rust
unsafe {
        core::arch::asm!("sret");
    }
```

U态程序尝试使用S态返回指令sret,特权级不够，抛出IllegalInstruction异常被捕捉

ch2b_bad_register.rs

```
unsafe {
        core::arch::asm!("csrr {}, sstatus", out(reg) sstatus);
    }
```

尝试访问S态控制寄存器sstatus,也抛出IllegalInstruction

------

2、

1. sp代表内核栈，__restore用于trap结束时恢复用户程序上下文，或者第一次启动用户程序或切换另一用户程序时加载上下文。

2. sstatus:决定返回后的状态；sepc:决定返回用户态后从那条指令执行;sscratch:临时保存用户栈指针，最后切回用户栈时使用

3. x2是sp, 此时sp是kernal stack 而非需要保存的用户栈，x4是tp,默认用户程序不使用

4. sp回到user stack，sscratch 是kernel stack

5. sret，sret前__restore把栈恢复了用户态程序上下文，sret后，sstatus是用户态，因此回到用户态

6.  sp是kernel stack sscratch 是 userstack

7. ecall 

   

------

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
