#![feature(test)]
extern crate test;

fn main() {
    let mut networks = sysinfo::Networks::new_with_refreshed_list();
    loop {
        networks.refresh_list();
        for (name, data) in networks.iter() {
            println!(
                "{:<10} {:>10} RX    {:>10} TX",
                name,
                data.received(),
                data.transmitted()
            );
        }
        std::thread::sleep(std::time::Duration::new(1, 0));
    }
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    #[bench]
    fn bench_network_refresh(b: &mut Bencher) {
        let mut networks = sysinfo::Networks::new_with_refreshed_list();

        b.iter(|| {
            black_box(networks.refresh());
        });
    }

    #[bench]
    fn bench_network_refresh_list(b: &mut Bencher) {
        let mut networks = sysinfo::Networks::new_with_refreshed_list();

        b.iter(|| {
            black_box(networks.refresh_list());
        });
    }
}
