pub mod calc1;
pub mod calc2;
pub mod calc3;

#[cfg(test)]
mod tests {
    use super::calc1::{add, sub};
    use super::calc2::{multiply, rate};
    use super::calc3::{pot, log};

    #[test]
    fn teste_add() {
        assert_eq!(add(10, 20), 30);
    }

    #[test]
    fn teste_sub() {
        assert_eq!(sub(20, 10), 10);
    }

    #[test]
    fn teste_multiply() {
        assert_eq!(multiply(2, 3), 6);
    }

    #[test]
    fn teste_rate() {
        assert_eq!(rate(100, 5), 20);
    }

    #[test]
    fn teste_pot() {
        assert_eq!(pot(2, 3), 8);
    }

    #[test]
    fn teste_log() {
        assert_eq!(log(3, 6561), Some(8));
    }

}