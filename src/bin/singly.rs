/*
 * A singly linked list implementation
 */
use fmt::Display;
use std::{fmt::{self, Debug}, ops::{Deref, DerefMut}, ptr};

#[allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut)]

#[derive(Default)]
struct Node<T:PartialEq> 
{
    next: Option<Box<Node<T>>>,
    data: Option<T>,
}


/*struct NodeIter<'a,T> 
    where 
    T:PartialEq
{
    current : &'a Option<Node<T>>,
}
    impl<'a,T> Iterator for NodeIter<'a,T> 
        where
        T:PartialEq
    {
       type Item = &'a T;
       fn next(&self) -> Option<Self::Item> {
           if let Some(x) = self.current { self.current = &self.current.as_deref().unwrap().next; return x;}
           else {None}
       }
    }*/

// impl <T: PartialEq> IntoIterator for Node<T>{
//     type Item;
//     type IntoIter;

//     fn into_iter(self) -> Self::IntoIter {
        
//     }
// }

impl<T: PartialEq + Display + Debug + Sized> Node<T> {
    pub fn new() -> Self {
        Self {
            next: None,
            data: None,
        }
    }

    //Checks if the current Node is none. If it is, then replace it with a new node. Else keep moving to the next node and recursively check for the same condition.
    pub fn add(&mut self, val: T) {
        // Check if the list is empty
        if self.is_empty() {
            self.data = Some(val);
            self.next = None;
            // If the list is not empty recursively iterate through each element until the end of the list. Then add the new element.
        } else {
            match &mut self.next {
                None => {
                    self.next = Some(Box::new(Self {
                        next: None,
                        data: Some(val),
                    }));
                }
                Some(next_val) => next_val.add(val),
            }
        }
    }

    //Iterate through the linked list till the desired index is reached, then add the element there.
    /*pub fn add_index(&mut self, val: T, index: usize) {
        if index < 0 || index > (self.len - 1) as usize {
            eprintln!("Out of bounds error at {:?}", index);
            return;
        }
        else if index==0{
            self.add(val);
            return;
        }
        let mut curr = self;
        let mut prev: Option<T> = None;
        let mut next = &mut curr.next;
        let mut i = 0;

        while i<index && next.is_some() {
            i+=1;

        }

    }*/

    pub fn add_all_index(index:usize, vec: Vec<T>){todo!()}

    pub fn len(&self) -> usize {
        let mut curr = self;
        let mut len = 0;
        if curr.data.is_none() {
            return len;
        }
        len += 1;
        while curr.next.is_some() {
            len += 1;
            curr = curr.next.as_ref().unwrap();
        }
        len
    }

    pub fn add_all(&mut self, vec: Vec<T>) {
        for i in vec {
            self.add(i);
        }
    }

    pub fn clear(&mut self) {
        self.next = None;
        self.data = None;
    }

    pub fn contains(&mut self, val: T) -> bool {
        let mut curr = self;
        if curr.data.is_some() {
            if curr.data.as_ref().unwrap().eq(&val) {
                return true;
            }
        } else if curr.data.is_none() {
            return false;
        }
        while let Some(ref mut next_node) = curr.next { //Check values in between first and last value
            if curr.data.as_ref().unwrap().eq(&val) {
                return true;
            }
            curr = next_node;
        }

        if curr.data.as_ref().unwrap().eq(&val) { //Check Last Value
            return true;
        }

        false
    }

    /*
    Set a prev curr and next node. when curr.data = val take out curr then set prev.next to the next node.
     */

    pub fn remove_key(&mut self, val: T) -> bool{
        //Check if the linkedlist is empty. If so return false.
        if self.is_empty(){
             return false;
        }
        //Check if the first element is the element to be removed. If so, remove the first element and return true.
      if self.data.as_ref().unwrap().eq(&val) {
            self.next = self.next.take();
            return true;
        }
        let mut curr = self;
        //let mut next = curr.next.as_deref_mut().map(|m| m);
        let mut prev:  Option<&mut Node<T>> = None;
        /*while let Some(ref mut next_node) = curr.next { //Check values in between first and last value
            if curr.data.as_ref().unwrap().eq(&val) {
                if let Some(prev_node) = prev {
                    prev_node.next = Some(next_node);
                }
                return true;
            }
            prev = Some(curr);
            curr = next_node.deref_mut();
        }*/
       if curr.data.as_ref().unwrap().eq(&val) {
            
           return true;
       }
        false
    }

    pub fn remove_index(&mut self, index: usize) {
        todo!()
    }

    /*pub fn remove_first(&mut self) -> Option<T> {
        //Check if the linkedlist is empty. If so return None.
        let mut curr = self;
        if curr.is_empty(){
            return None;
        }
        let next_node = *curr.next.as_ref().unwrap().deref();
        self = &mut next_node;
        self.data
    }*/

    pub fn remove_last(&mut self) {
        if self.is_empty(){
            return;
        }
        else if self.next.is_none(){
            self.clear();
            return;
        }
        else{
        match &mut self.next.as_mut().unwrap().next {
            None => {
                 self.next = None;
            },
            
            Some(next_val) => next_val.remove_last(),
        }
    }
    }
    

    pub fn index_of(&mut self,val: T) -> isize {
        let mut curr = self;
        if curr.is_empty(){
            return -1;
        }
        else{
            let mut index = 0;
            while let Some(ref mut next_node) = curr.next{
                if curr.data.as_ref().unwrap().eq(&val){
                    return index;
                }
                index+=1;
                curr = next_node;
            }
            return -1
        }
    }
    

    pub fn add_first(&mut self, val: T) {
        if self.is_empty(){
            self.add(val); 
            return;
        }
        let mut new_node1 = Self{
            next:self.next.take(),
            data:Some(val),
        };
        std::mem::swap(new_node1.data.as_mut().unwrap(),self.data.as_mut().unwrap());
        self.next = Some(Box::new(new_node1));
    }    

    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    // O(n) time complexity
    // pub fn to_array(&self)->[T]{
    //     let len = self.len();
    //     let arr:[T;len];
    // }

    pub fn reverse(&mut self){
        if self.is_empty() || self.len()==1{
            return;
        }
        else {
            let mut curr = self;
            let mut reversed_vec:Vec<Option<Box<Node<T>>>> = vec![];
            //reversed_vec.push(curr);
            while curr.next.as_ref().unwrap().next.is_some(){
                let node = curr.next.take();
                reversed_vec.push(node);
                curr = curr.next;
            } 
        }
    }
}

/*
   Implementation of Node<T> to print it.
*/
impl<T: fmt::Debug + Display + std::cmp::PartialEq> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = self;
        //If the List is empty just print "[]"
         if self.data.is_none() {
             return Ok(write!(f, "[None]")?);
         }
        

        if let Some(n_curr) = current.next.as_ref(){
            write!(f, "[{:?},{:p}]", current.data.as_ref().unwrap(), n_curr)?;
        }
        else{
            write!(f, "[{:?},{:p}]",self.data.as_ref().unwrap(),self)?;
        }
        /*
        Continue writing each element after. 'current.next' is an Option hence 'next' is also Option. ref is used in pattern matching 
        */
        while let Some(ref next_node) = current.next {
            let next_ptr = next_node.as_ref() as *const Node<T>;
            let value = next_node.data.as_ref();
            if next_node.next.is_some(){
                write!(f,"-->[{:?},{:p}]",value.unwrap(),next_ptr)?;
            }
            else {
                write!(f,"-->[{:?},{:}]",value.unwrap(),"0x0")?;
                break;
            }
            current = next_node;
        }
        write!(f, "-->[None]") // End of the list
    }
}

fn main() {
    let mut ll: Node<i32> = Node::new();
    //ll.add(1);
    //ll.add(2);
    //ll.add(3);

    //let vec = vec![15, 43, 340,235,435,2341,3655];
    
    //ll.add_first(762);

    println!("{:?}",ll);

    ll.remove_last();

    println!("{:?}",ll);

}