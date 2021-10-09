# Rust Programming Cookbook Çalışma Alanım

Bir süre önce aldığım [Rust Programming Cookbook](https://amzn.to/3Bw5ysw) kitabını çalışmak için açtığım repo. Kısa kısa notlar almayı ve örnekler üzerinden ilerlemeyi planlıyorum.

## Ön Hazırlıklar

Her şeyden önce çalışmalarımı Windows 10 tabanlı bir sistemde yapmaktayım. Rust'ın kurulumu için kitabın da önerdiği üzere [https://rustup.rs](https://rustup.rs) adresine gidip indirdiğim installer'ı çalıştırdım. Kurulumun başarılı olup olmadığını görmek için komut satırından _rustc --version_ komutunu çalıştırdım.

![./assets/screenshot_01.png](./assets/screenshot_01.png)

Geliştirmeler için Visual Studio Code kullanacağım. Rust eklentisi için command-line interface'i açıp(Ctrl+P) sonrasında _ext install rust-lang.rust_ komutunu çalıştırmak yeterli.

![./assets/screenshot_02.png](./assets/screenshot_02.png)

## day01

### Komut Satırı ile Temel İşlemler

```bash
# Terminalden aşağıdaki komut çalıştılır ve ilk proje oluşturulur
cargo new wonderful-world

# Örneği çalıştırmak içinse (Tabi kodlama yaptıktan sonra :) )
cd wonderful-world
cargo run
```

![./assets/screenshot_03.png](./assets/screenshot_03.png)

### Veri Türleri

```bash
# Bu sefer bir kütüphane oluşturuyoruz
cargo new data-types --lib
cd data-types
# test fonksiyonları üstünden ilerliyoruz
cargo test
```

![./assets/screenshot_04.png](./assets/screenshot_04.png)

### Karar Yapıları ve Döngüler

```bash
cargo new flows --lib
cd flows
cargo test
```

![./assets/screenshot_05.png](./assets/screenshot_05.png)