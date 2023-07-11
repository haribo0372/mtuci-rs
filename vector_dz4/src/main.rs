#[derive(Debug, PartialEq)]
struct Vector<T>{
  data: Vec<T>
}
impl<T> Vector<T>{
    // СОЗДАНИЕ НОВОГО ВЕКТОРА
    pub fn new() -> Self{
        Self{
          data: Vec::new()
        }
    }
    // СОЗДАНИЕ ВЕКТОРА С ЗАДАННЫМ РАЗМЕРОМ
    pub fn with_capacity(capacity: usize) -> Self{
        Self{ data: Vec::with_capacity(capacity) }
    }
    // ДОБАВЛЕНИЕ ЭЛЕМЕНТА В ВЕКТОР
    pub fn push_in_vector(&mut self, value: T) {
        self.data.push(value)
    }
    // ВОЗВРАТ ПОСЛЕДНЕГО ЗНАЧЕНИЯ И УДАЛЕНИЕ ЕГО С ВЕКТОРА
    pub fn pop_vector(&mut self) -> Option<T> {
        self.data.pop()
    }
    // УДАЛЕНИЕ ЭЛЕМЕНТА ПО ИНДЕКСУ
    pub fn remove_vector(&mut self, index: usize) -> T {
        self.data.remove(index)
    }
    // ВОЗВРАТ ЗНАЧЕНИЯ ПО ИНДЕКСУ
    pub fn get_vector(&self, index: usize) -> Option<&T>{
        self.data.get(index) as Option<&T>
    }
    // ИЗМЕНЕНИЕ РАЗМЕРА ВЕКТОРА
    pub fn resize_vector(&mut self, new_size : usize , value: T) where T: Clone{
        self.data.resize(new_size, value)
    }

}


fn main(){
    let mut vec = Vector::new();
    vec.push_in_vector(67);
    println!("{:?}", vec)

}

// -----__TESTS__-----
#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn test_push() {
        let mut vec = Vector::new();
        vec.push_in_vector(67);
        assert_eq!(vec.data, vec![67])
    }

    #[test]
    fn test_with_capacity() {
        let mut vec: Vector<usize> = Vector::with_capacity(4);
        assert_eq!(vec.data.capacity(), 4)
    }

    #[test]
    fn test_pop(){
        let mut vec = Vector {
            data: vec![3, 4, 5, 655],
        };
        assert_eq!(vec.pop_vector(), Some(655))
    }

    #[test]
    fn test_remove(){
        let mut vec = Vector {
            data: vec![3, 4, 5, 655],
        };
        vec.remove_vector(0);
        assert_eq!(vec.data, vec![4, 5, 655])
    }
    #[test]
    fn test_get(){
        let mut vec = Vector {
            data: vec![3, 4, 5, 655],
        };
        assert_eq!(vec.get_vector(0), Some(&3))
    }
    #[test]
    fn test_resize(){
        let mut vec = Vector {
            data: vec![3],
        };
        vec.resize_vector(3, 0);
        assert_eq!(vec.data , vec![3, 0, 0] )
    }
}