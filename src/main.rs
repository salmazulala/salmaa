use std::io;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let mut todo_list: HashMap<String, Task> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();
    let mut queue: VecDeque<String> = VecDeque::new();

    loop {
        // Menampilkan menu
        println!("Menu:");
        println!("1. Tambahkan transaksi baru");
        println!("2. Menampilkan daftar transaksi");
        println!("3. Mengedit transaksi");
        println!("4. Menghapus transaksi");
        println!("5. Menandai transaksi sebagai selesai");
        println!("6. Push ke Stack");
        println!("7. Pop dari Stack");
        println!("8. Enqueue ke Queue");
        println!("9. Dequeue dari Queue");
        println!("0. Keluar");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Memproses input
        match input {
            "1" => {
                // Tambahkan transaksi baru
                println!("Tambahkan transaksi baru:");
                let mut task_description = String::new();
                io::stdin().read_line(&mut task_description).unwrap();
                todo_list.insert(
                    task_description.trim().to_string(),
                    Task {
                        description: task_description.trim().to_string(),
                        completed: false,
                    },
                );
                println!("Transaksi baru telah ditambahkan!");
            },
            "2" => {
                // Menampilkan daftar transaksi
                println!("Daftar transaksi:");
                for (task_name, task) in &todo_list {
                    if task.completed {
                        println!("* {}: {} (Selesai)", task_name, task.description);
                    } else {
                        println!("* {}: {} (Belum selesai)", task_name, task.description);
                    }
                }
            },
            "3" => {
                // Mengedit transaksi
                println!("Edit transaksi:");
                // ... (implementasi edit transaksi)
            },
            "4" => {
                // Menghapus transaksi
                println!("Hapus transaksi:");
                // ... (implementasi hapus transaksi)
            },
            "5" => {
                // Menandai transaksi sebagai selesai
                println!("Tandai selesai transaksi:");
                // ... (implementasi tandai selesai)
            },
            "6" => {
                // Push ke Stack
                println!("Masukkan item untuk dipush ke Stack:");
                let mut item = String::new();
                io::stdin().read_line(&mut item).unwrap();
                stack.push(item.trim().to_string());
                println!("Item telah di-push ke Stack!");
            },
            "7" => {
                // Pop dari Stack
                if let Some(item) = stack.pop() {
                    println!("Item yang di-pop dari Stack: {}", item);
                } else {
                    println!("Stack kosong");
                }
            },
            "8" => {
                // Enqueue ke Queue
                println!("Masukkan item untuk dienqueue ke Queue:");
                let mut item = String::new();
                io::stdin().read_line(&mut item).unwrap();
                queue.push_back(item.trim().to_string());
                println!("Item telah di-enqueue ke Queue!");
            },
            "9" => {
                // Dequeue dari Queue
                if let Some(item) = queue.pop_front() {
                    println!("Item yang di-dequeue dari Queue: {}", item);
                } else {
                    println!("Queue kosong");
                }
            },
            "0" => {
                // Keluar dari loop
                break;
            },
            _ => {
                // Input tidak valid
                println!("Input tidak valid");
            },
        }
    }
}

