#[derive(Debug)]
struct Heap<T> {
    length: usize,
    data: Vec<T>
}

impl<T: std::cmp::PartialOrd + std::marker::Copy > Heap<T>  {
    fn new() -> Self {
        Heap { length: 0, data: vec![] }
    }

    fn insert(&mut self , value : T){
        self.data.push(value);
        self.heapify_up(self.length);
        self.length += 1;
    }

    fn delete(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        let out = self.data[0];
        self.length -= 1;

        if self.length == 1 {
            self.data = vec![];
            return Some(out);
        }

        self.data[0] = self.data[self.length];
        self.data.pop();

        self.heapify_down(0);

        return Some(out);
    }

    fn heapify_up(&mut self , index :usize) {
        if index == 0{
            return;
        }

        let parent_index = self.parent(index);
        let parent_value = self.data.get(parent_index).unwrap();
        let current_value =  self.data.get(index).unwrap();

        if parent_value > current_value {
            // swap
            self.swap(index, parent_index);
            self.heapify_up(parent_index);
        }
    }

    fn heapify_down(&mut self , index :usize) {
        let left_index = self.left(index); 
        let right_index = self.right(index); 
        
        if index >= self.length || left_index >= self.length{
            return;
        }

        let left_value = self.data.get(left_index).unwrap();
        let right_value = self.data.get(right_index).unwrap();
        let value = self.data.get(index).unwrap();

        if left_value > right_value && value > right_value {
            self.swap(right_index, index);
            self.heapify_down(right_index);
        }else if left_value < right_value && value > left_value {
            self.swap(left_index, index);
            self.heapify_down(left_index);
        }

    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }
    
    fn left(&self , index :usize) -> usize {
        2 * index + 1
    }

    fn right(&self ,index :usize) -> usize {
        2 * index + 2
    }

    fn parent(&self ,index :usize) -> usize {
        if index <= 2 {
            return 0;
        }

        (index - 2) / 2
    } 
}


fn main() {
   let mut heap = Heap::new();
   heap.insert(12.2);
   heap.insert(11.3);
   heap.insert(15.0);
   heap.insert(119.1);
   heap.insert(3.2);
   heap.insert(120.0);
   heap.delete();
   dbg!(heap);
}
