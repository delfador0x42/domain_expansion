/*
root@wudan:~# ls -l /proc/1074/fd
total 0
lr-x------ 1 root root 64 Apr  5 19:55 0 -> /dev/null
lrwx------ 1 root root 64 Apr  5 20:02 1 -> 'socket:[11514]'
lrwx------ 1 root root 64 Apr  5 20:02 2 -> 'socket:[11514]'
lrwx------ 1 root root 64 Apr  5 20:02 3 -> 'socket:[11554]'
lrwx------ 1 root root 64 Apr  5 20:02 4 -> 'socket:[11524]'
lrwx------ 1 root root 64 Apr  5 20:02 5 -> 'socket:[11529]'
lrwx------ 1 root root 64 Apr  5 20:02 6 -> /dev/ptmx
lrwx------ 1 root root 64 Apr  5 20:02 7 -> 'socket:[11841]'
l-wx------ 1 root root 64 Apr  5 20:02 9 -> /run/systemd/sessions/2.ref

root@wudan:~# cat /proc/net/tcp
  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode
   0: 3600007F:0035 00000000:0000 0A 00000000:00000000 00:00000000 00000000   992        0 9694 1 ffff000085da0000 100 0 0 10 5
   1: 3500007F:0035 00000000:0000 0A 00000000:00000000 00:00000000 00000000   992        0 9692 1 ffff000085da2600 100 0 0 10 5
   2: 0100007F:9D95 00000000:0000 0A 00000000:00000000 00:00000000 00000000  1000        0 12147 1 ffff000085da5f00 100 0 0 10 0
   3: 0100007F:9D95 0100007F:CA08 01 00000000:00000000 00:00000000 00000000  1000        0 16917 1 ffff000085da2f80 20 4 0 10 -1
   4: 879EA8C0:A40A 27BE7DB9:0050 06 00000000:00000000 03:000007A7 00000000     0        0 0 3 ffff0000854acd68
   5: 0100007F:CA08 0100007F:9D95 01 00000000:00000000 00:00000000 00000000  1000        0 16199 1 ffff000085da5580 20 4 20 10 -1

root@wudan:~# cat /proc/net/tcp6
  sl  local_address                         remote_address                        st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode
   0: 00000000000000000000000000000000:0016 00000000000000000000000000000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 7959 1 ffff000084d9da00 100 0 0 10 0
   1: 0000000000000000FFFF0000879EA8C0:0016 0000000000000000FFFF0000019EA8C0:C11B 01 00000000:00000000 02:0008EC4C 00000000     0        0 11524 4 ffff00008a133c00 20 4 1 10 20
   2: 0000000000000000FFFF0000879EA8C0:0016 0000000000000000FFFF0000019EA8C0:C11F 01 00000000:00000000 02:000926BE 00000000     0        0 12486 2 ffff00008a134600 20 4 30 10 99
root@wudan:~#

*/



// (1) Read from /proc get the inode number from all fd pointing to sockets =============================
/*
root@wudan:~# ls -l /proc/1074/fd
total 0
lr-x------ 1 root root 64 Apr  5 19:55 0 -> /dev/null
lrwx------ 1 root root 64 Apr  5 20:02 1 -> 'socket:[11514]'
lrwx------ 1 root root 64 Apr  5 20:02 2 -> 'socket:[11514]'
lrwx------ 1 root root 64 Apr  5 20:02 3 -> 'socket:[11554]'
lrwx------ 1 root root 64 Apr  5 20:02 4 -> 'socket:[11524]'
lrwx------ 1 root root 64 Apr  5 20:02 5 -> 'socket:[11529]'
lrwx------ 1 root root 64 Apr  5 20:02 6 -> /dev/ptmx
lrwx------ 1 root root 64 Apr  5 20:02 7 -> 'socket:[11841]'
l-wx------ 1 root root 64 Apr  5 20:02 9 -> /run/systemd/sessions/2.ref
*/




// (2) Read from /proc/net/tcp convert the addresses to decimal and match the inodes from the file descriptors to the inode in these outputs



// (3) read /proc/net/tcp6 If the address starts with 0000000000000000FFFF0000 then it's ipv4


use std::fs;
use std::path::Path;
use std::io;
use rayon::prelude::*;

fn main() -> io::Result<()> {
    // Get all entries in /proc
    let entries = fs::read_dir("/proc")?;
    
    // Process entries in parallel for maximum performance
    let paths: Vec<_> = entries
        .filter_map(Result::ok)
        .filter(|entry| {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            name.chars().all(|c| c.is_ascii_digit())
        })
        .collect();
    
    paths.par_iter().for_each(|entry| {
        let fd_path = entry.path().join("fd");
        println!("{}", fd_path.display());
    });

    Ok(())
}
