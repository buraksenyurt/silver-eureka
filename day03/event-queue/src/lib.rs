//! Tamamen öğrenme amacıyla açılmış önemsiz bir rust modülüdür.
//! Sembolik olarak Event isimli bir veri yapısı ile çalışan Queue modelinde bir koleksiyon ve ilgili fonksiyonlarını barındırır.
//!
//! # Example
//!
//! ```
//! use std::time::SystemTime;
//! use event_queue::Event;
//!
//! let t = SystemTime::now();
//! let e = Event::new("Product_Added".to_string(), t);
//! ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_an_event_works_test() {
        let t = SystemTime::now();
        let e = Event::new("Product_Added".to_string(), t);
        assert_eq!(e.to_str(), format!("Product_Added [{:?}]", t));
    }

    #[test]
    fn should_create_an_eventqueue_works_test() {
        let mut event_queue = EventQueue::new();
        assert_eq!(event_queue.count(), 0);

        let event_1 = Event::new("Product_Created".to_string(), SystemTime::now());
        event_queue.enqueue(event_1);
        let event_2 = Event::new("Basket_Refreshed".to_string(), SystemTime::now());
        event_queue.enqueue(event_2);
        let event_3 = Event::new("Product_Added".to_string(), SystemTime::now());
        event_queue.enqueue(event_3);
        assert_eq!(event_queue.count(), 3);
    }

    #[test]
    fn should_dequeue_fn_works_test() {
        let mut event_queue = EventQueue::new();
        let event_1 = Event::new("Product_Created".to_string(), SystemTime::now());
        event_queue.enqueue(event_1);
        let event_2 = Event::new("Basket_Refreshed".to_string(), SystemTime::now());
        event_queue.enqueue(event_2);
        let some_event = event_queue.dequeue();
        assert_eq!(some_event.name, "Product_Created");
        assert_eq!(event_queue.count(), 1);
        let other_event = event_queue.dequeue();
        assert_eq!(other_event.name, "Basket_Refreshed");
        assert_eq!(event_queue.count(), 0);
    }

    #[test]
    fn should_peek_fn_works_test() {
        let mut event_queue = EventQueue::new();
        let event_1 = Event::new("Product_Created".to_string(), SystemTime::now());
        event_queue.enqueue(event_1);
        let event_2 = Event::new("Basket_Refreshed".to_string(), SystemTime::now());
        event_queue.enqueue(event_2);
        let e = event_queue.peek();
        assert_eq!(e.unwrap().name, "Product_Created");
        event_queue.dequeue();
        event_queue.dequeue();
        assert_eq!(event_queue.peek(), None);
    }

    #[test]
    #[should_panic]
    fn should_dequeue_fn_panic_test() {
        let mut event_queue = EventQueue::new();
        let event_1 = Event::new("Product_Created".to_string(), SystemTime::now());
        event_queue.enqueue(event_1);
        event_queue.dequeue();
        event_queue.dequeue();
    }
}

use std::time::SystemTime;

/// Bir olayı sembolize eden veri yapısıdır.
///
/// # Arguments
///
/// * `name` - Olayın adı.
/// * `time` - Olayın zamanı.
#[derive(Debug, PartialEq)]
pub struct Event {
    pub name: String,
    pub time: SystemTime,
}

/// Event veri türü ile ilgili fonksiyonlar.
impl Event {
    /// Yeni bir event değişkeni oluşturur.
    pub fn new(n: String, t: SystemTime) -> Self {
        Event { name: n, time: t }
    }

    // Event içeriğini String formatta dönen fonksiyondur
    pub fn to_str(&self) -> String {
        format!("{} [{:?}]", self.name, self.time)
    }
}

///
/// Bir olaylar dizisi kuyruğunu temsil eden veri yapısıdır.
/// Queue mantığında çalışır.
/// FIFI - First In First Out
pub struct EventQueue {
    pub events: Vec<Event>,
}

impl EventQueue {
    /// Yeni bir EventQueue nesnesi oluşturmak için kullanılır
    pub fn new() -> Self {
        EventQueue { events: Vec::new() }
    }

    /// Kuyruğa Event eklemek için
    pub fn enqueue(&mut self, e: Event) {
        self.events.push(e)
    }

    /// İlk eklenen elemanı kuyruktan çıkarır ve geriye döndürür
    pub fn dequeue(&mut self) -> Event {
        self.events.remove(0)
    }

    /// Kuyruktaki eleman sayısını elde etmek için
    pub fn count(&self) -> usize {
        self.events.len()
    }

    /// İlk eklenen elemanı verir ama kuyruktan çıkarmaz
    pub fn peek(&self) -> Option<&Event> {
        let e = self.events.first();
        match e {
            Some(_) => e,
            _ => None,
        }
    }
}
