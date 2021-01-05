

enum LinkedList<T> {
    Node<T, Box<LinkedList<T>>>,
    Nil,
}