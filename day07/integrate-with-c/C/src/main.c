#include <stdio.h> // standart c kütüphanesi eklenir.

/*
    Rust tarafından çağıracağımız fonksiyonun bildirimi.
    extern ile harici bir fonksiyon olduğu ifade edilir.
    Dikkat edileceği üzere metot bloğu yoktur.
*/
extern char *encrypt(char *str);

/*
    Aşağıdaki fonksiyonlarda Rust kodu içinden çağırılan C fonksiyonları.
    Bu fonksiyonlar için Rust tarafında yine extern ile bildirim yapıldığını hatırlayalım.
*/
extern void pre_encrypt()
{
    printf("Encrypt fonksiyonunu çağırıldı.");
}

extern void post_encrypt()
{
    printf("Encrypt fonksiyonunu çıktı vermeye hazır.");
}

int main()
{
    // Rust tarafındaki encrypt fonksiyonunu çağırıyoruz.
    char *result = encrypt("Rustgele dostlar!");
    // Dönen sonucu ekrana basıyoruz.
    printf("Rustgele dostlar! -> %s", result);
    return 0;
}