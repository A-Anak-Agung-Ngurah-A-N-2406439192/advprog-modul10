## Experiment 1.2: Understanding how it works

![img.png](img.png)

Pada eksperimen ini, saya menambahkan perintah `println!("Abhivadya's Komputer: hey hey");` setelah pemanggilan `spawner.spawn(...)`. Hasilnya, teks `hey hey` muncul lebih dulu dibandingkan `howdy!`. Hal ini terjadi karena `spawner.spawn(...)` hanya memasukkan task ke dalam queue, tetapi task tersebut belum langsung dijalankan. Task async baru mulai dijalankan ketika `executor.run()` dipanggil. Setelah executor berjalan, future di-poll dan mencetak `howdy!`, lalu menunggu `TimerFuture` selama dua detik. Setelah timer selesai, waker memberi tahu executor untuk melanjutkan task sehingga teks `done!` dicetak.