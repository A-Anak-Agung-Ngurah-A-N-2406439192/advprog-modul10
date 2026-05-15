## Experiment 1.2: Understanding how it works

![img.png](img.png)

Pada eksperimen ini, saya menambahkan perintah `println!("Abhivadya's Komputer: hey hey");` setelah pemanggilan `spawner.spawn(...)`. Hasilnya, teks `hey hey` muncul lebih dulu dibandingkan `howdy!`. Hal ini terjadi karena `spawner.spawn(...)` hanya memasukkan task ke dalam queue, tetapi task tersebut belum langsung dijalankan. Task async baru mulai dijalankan ketika `executor.run()` dipanggil. Setelah executor berjalan, future di-poll dan mencetak `howdy!`, lalu menunggu `TimerFuture` selama dua detik. Setelah timer selesai, waker memberi tahu executor untuk melanjutkan task sehingga teks `done!` dicetak.

## Experiment 1.3: Multiple Spawn and removing drop

### Multiple Spawn

![img_1.png](img_1.png)

Pada eksperimen ini, saya menambahkan beberapa pemanggilan `spawner.spawn(...)`. Setiap pemanggilan `spawn` akan memasukkan satu task async ke dalam queue executor. Ketika `executor.run()` dipanggil, executor mulai mengambil task dari queue dan melakukan polling terhadap setiap future. Oleh karena itu, beberapa pesan `howdy` dapat muncul terlebih dahulu, lalu setelah timer selesai, masing-masing task dilanjutkan dan mencetak pesan `done`.

### Removing drop(spawner)

![img_2.png](img_2.png)

Pada eksperimen ini, saya mencoba menghapus `drop(spawner);`. Hasilnya, program tetap mencetak output dari task async, tetapi program tidak langsung selesai. Hal ini terjadi karena executor masih menunggu kemungkinan adanya task baru dari channel. `drop(spawner);` diperlukan untuk menutup sender utama, sehingga executor tahu bahwa tidak akan ada task baru lagi setelah semua task selesai dijalankan.

### Explanation

`Spawner` berfungsi untuk memasukkan task async ke dalam queue. `Executor` berfungsi untuk mengambil task dari queue dan menjalankannya sampai selesai dengan cara melakukan polling terhadap future. `drop(spawner);` digunakan untuk memberi tahu executor bahwa tidak ada lagi task baru yang akan dikirim. Hubungan ketiganya adalah spawner mengirim task, executor menjalankan task, dan drop menghentikan kemungkinan pengiriman task baru agar executor bisa berhenti ketika semua task selesai.