CC=gcc # Geniue Complier Collectiın kullandık ama zorunlu değil. Eklemezsek CC ile ilerlenir.

# Rust kütüphanesinin yerini bildirmemiz lazım
# app klasörüne geçip release'e inmemiz gerekiyor
LIBS := -lkrip -L../app/target/release

ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip $(LIBS)
else
    LDFLAGS := -Wl,--gc-sections $(LIBS)
endif

release: target/main

# çıktı için target klasörü oluşturuluyor
target:
	@mkdir $@

# C Compiler ile derleme işlemleri icra ediliyor
target/main: target/main.o 
	$(CC) -o $@ $^ $(LDFLAGS)

target/main.o: src/main.c | target
	$(CC) -o $@ -c $<

clean:
	@rm -rf target