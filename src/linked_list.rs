#![allow(unused)]

struct LinkedList<T> {
    head: Option<Node<T>>,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}
