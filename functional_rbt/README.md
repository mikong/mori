# Functional Red-Black Tree

Based on the Red-Black Tree discussed in section 3.3 of the book _Purely Functional Data Structures_ with the following Standard ML implementation:

```sml
functor RedBlackSet (Element : ORDERED) : SET =
struct
  type Elem = Element.T

  datatype Color = R | B
  datatype Tree = E | T of Color * Tree * Elem * Tree
  type Set = Tree

  val empty = E

  fun member (x, E) = false
    | member (x, T (_, a, y, b)) =
        if Element.lt (x, y) then member (x, a)
        else if Element.lt (y, x) then member (x, b)
        else true

  fun balance (B, T (R, T (R, a, x, b), y, c), z, d) = T (R, T (B, a, x, b), y, T (B, c, z, d))
    | balance (B, T (R, a, x, T (R, b, y, c)), z, d) = T (R, T (B, a, x, b), y, T (B, c, z, d))
    | balance (B, a, x, T (R, T (R, b, y, c), z, d)) = T (R, T (B, a, x, b), y, T (B, c, z, d))
    | balance (B, a, x, T (R, b, y, T (R, c, z, d))) = T (R, T (B, a, x, b), y, T (B, c, z, d))
    | balance body = T body

  fun insert (x, s) =
    let fun ins E = T (R, E, x, E)
          | ins (s as T (color, a, y, b)) =
              if Element.lt (x, y) then balance (color, ins a, y, b)
              else if Element.lt (y, x) then balance (color, a, y, ins b)
              else s
        val T (_, a, y, b) = ins s  (* guaranteed to be non-empty *)
    in T (B, a, y, b)
    end
end
```
