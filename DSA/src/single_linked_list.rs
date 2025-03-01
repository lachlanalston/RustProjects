use std::collections::LinkedList;

//  Single Linked List
//
//                   Node 1              Node 2              Node 3
//  Head Pointer -> [Data | Pointer] -> [Data | Pointer] -> [Data | Pointer] -> Null
//
pub fn test() {

    //Create empty linked list
    let empty_list: LinkedList<u32> = LinkedList::new();

    //Create linked list with data
    let data_list = LinkedList::from([1,2,3]);
}
