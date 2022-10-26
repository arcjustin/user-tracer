use bpf_api::collections::Queue;
use bpf_api::probes::{AttachInfo, AttachType, Probe};
use bpf_api::prog::{Program, ProgramAttr, ProgramType};
use bpf_script::Compiler;
use btf::traits::AddToBtf;
use btf::BtfTypes;
use btf_derive::AddToBtf;

#[repr(C, align(1))]
#[derive(Copy, Clone, Debug, AddToBtf)]
struct ExecEntry {
    pub uid_gid: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub comm: [u8; 16],
}

impl Default for ExecEntry {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

fn main() {
    println!("Initializing...");

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("usage: user-tracer <function> <addr>");
        return;
    }

    /*
     * Load types from the vmlinux BTF file and add the custom Rust type
     * to the database.
     */
    let mut btf = BtfTypes::from_file("/sys/kernel/btf/vmlinux").unwrap();
    ExecEntry::add_to_btf(&mut btf).unwrap();

    /*
     * Create a BPF script compiler.
     */
    let mut compiler = Compiler::create(&btf);

    /*
     * Create a shared queue and "capture" it in the compiler context.
     */
    let queue = Queue::<ExecEntry>::create(10).unwrap();
    compiler.capture("queue", queue.get_identifier().into());

    /*
     * Compile a program.
     */
    compiler
        .compile(
            r#"
            fn(regs: &bpf_user_pt_regs_t)
                entry: ExecEntry = 0
                entry.uid_gid = get_current_uid_gid()
                entry.rdi = regs.di
                entry.rsi = regs.si
                entry.rdx = regs.dx
                entry.rcx = regs.cx
                get_current_comm(&entry.comm, 16)
                map_push_elem(queue, &entry, 0)
        "#,
        )
        .expect("compilation failed");

    /*
     * Insert the program into the kernel with the intended attachment point.
     */
    let attr = ProgramAttr {
        prog_name: None,
        prog_type: ProgramType::KProbe,
        expected_attach_type: Some(AttachType::PerfEvent),
        attach_btf_id: None,
    };

    let bytecode = compiler.get_bytecode();
    let program = Program::create(&attr, &bytecode, None).unwrap();

    /*
     * Create a probe and attach the program to it.
     */
    let file_path = args[1].clone();
    let address = u64::from_str_radix(&args[2].replace("0x", ""), 16).expect("Bad address.");
    let attach_info = AttachInfo::UProbe((file_path, address));
    let mut probe = Probe::create(attach_info);
    probe.attach(&program).unwrap();

    fn from_cstr(buf: &[u8]) -> String {
        String::from_utf8_lossy(match buf.iter().position(|c| *c == 0) {
            Some(p) => &buf[0..p],
            None => buf,
        })
        .to_string()
    }

    println!("Reading from queue...");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        while let Ok(entry) = queue.pop() {
            println!(
                "comm={}, gid/uid={}/{}, arg[0]={:#016x}, arg[1]={:#016x}, arg[2]={:#016x}, arg[3]={:#016x}",
                from_cstr(&entry.comm),
                entry.uid_gid >> 32,
                entry.uid_gid as u32,
                entry.rdi,
                entry.rsi,
                entry.rdx,
                entry.rcx,
            );
        }
    }
}
