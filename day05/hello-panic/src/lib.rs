#[cfg(test)]
mod tests {
    
    /*
        Aşağıdaki test fonksiyonunda [should_panic] bildirimi ile test metodundan bilinçli olarak panic üretileceğini belirttik.
        panic!() makrosu çağırıldığında thread anında sonlandırılı ve kodun akan kısmı işletilmez.
    */
    #[test]
    #[should_panic]
    fn what_the_test() {
        panic!();
        // any code following this expression is unreachable uyarısı verilir.
        let x = 10;
        let y = 20;
        let z = x + y;
        assert_eq!(z, 30);
    }
}
