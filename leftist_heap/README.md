# Leftist Heap

Based on the Leftist Heap discussed in section 3.1 of the book _Purely Functional Data Structures_ with the following signature (in Standard ML):

```sml
signature HEAP =
sig
  structure Elem : ORDERED

  type Heap

  val empty     : Heap
  val isEmpty   : Heap -> bool

  val insert    : Elem.T * Heap -> Heap
  val merge     : Heap * Heap -> Heap

  val findMin   : Heap -> Elem.T  (* raises EMPTY if heap is empty *)
  val deleteMin : Heap -> Heap    (* raises EMPTY if heap is empty *)
```
