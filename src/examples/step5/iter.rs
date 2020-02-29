//! Provides traits and iterator implementations necessary for conveniently iterating over tuples
//! of iterators.

/// Implement this on the tuple of iterators you want to iterate on.
pub trait IteratorTuple {
    type ItemTuple;

    fn next_all(&mut self) -> Option<Self::ItemTuple>;
}

// Now, this looks quite intimidating at first, but really it is just the very same implementation
// we had before. It just uses variadic macro syntax to take in a varying number of arguments.
//
// First things first, we declare rules for our new macro using `macro_rules! macro_name { .. }`
// this starts declaration of a "declarative macro"
macro_rules! implement_iterator_tuple {
    // Parameter declaration. Oh boy. This is a bit of a mess, but let's break it down. The basic
    // syntax for macro argument declaration is
    //
    //      ( $arg0:designator, $arg1:designator, $arg2:designator ) => { .. }
    //
    // Each argument consist of `$` followed by argument name and then a "designator". Designators
    // indicate what sort of values the macro accepts and where the value can be used in a macro.
    // Some examples of valid designators are:
    //      block   -   accepts a code block
    //      ident   -   "identifier", accepts any valid identifier (`A`, `a`, `Cat`, `kissa`, etc.)
    //                  Can be used for type parameter names, parameter names, variable names, etc.
    //      literal -   literal constant
    //      expr    -   any expression, like `42 + 69`
    //      tt      -   "token tree". I won't go into detail what token trees are, but compilers
    //                  "tokenize" the code in order to figure out what different words in the code
    //                  actually mean. `tt`-designator accepts any valid token tree, which in
    //                  practice means it accepts almost anything. Read "the Dragon Book" if you are
    //                  interested in what these *actually* are.
    //
    // Ok, now we have a vague picture of how macro parameters and their "types" (aka. designators)
    // are defined. Next, we would like to have varying number of arguments to this macro. This is
    // called "variadics" and definition works as follows:
    //
    //      ( $( $x:designator ),* ) => { .. }  // Macro with argument x repeated 0..n times
    //      ( $( $x:designator ),+ ) => { .. }  // Macro with argument x repeated 1..n times
    //
    // We can also use tuples in out repeat pattern by wrapping the arguments in parentheses.
    // For example
    //
    //      ( $( ($x:ident, $y:expr) ),* ) => { .. } // Arguments x and y repeated 0..n times
    //
    // Now, what we actually have is
    //
    //      ($( ($i:tt, $item_name:ident, $type_name:ident) ),+) => { .. }
    //
    // Which breaks down to
    //      ($( ... ),+) => { .. }                      -   Arguments repeat 1..n times
    //      ($i:tt, $item_name:ident, $type_name:ident) -   Each argument is a 3-tuple, where
    //          $i:tt       -   "i" is a token tree. Tuple indexing is a bit of a wild-card at
    //                          language-level so it's hard for compiler to validate this. Just use
    //                          `tt` to tell the compiler to "not to worry about it"
    //          $item_name  -   a bit of an extra. We don't want to use the capital letters for
    //                          variable names (which we totally could), so pass an extra argument
    //                          for parameter names
    //          $type_name  -   This and the `$item_name` are both `ident` so any valid identifier
    //                          could be used.
    //
    // So, in other words:
    //      "macro accepts 3-tuples of i, an item name and a type name. There should be one or more
    //       of these tuples."
    //
    ($( ($i:tt, $item_name:ident, $type_name:ident) ),+) => {
        // Here we get to expansion. Expanding arguments uses the same syntax as defining them. We
        // can use `*` and `+` to indicate zero-or-more-times or one-or-more-times, respectively.
        //
        //      $( ... ),*            - Expand whatever it is inside the braces 0..n times
        //      $( ... ),+            - Expand whatever it is inside the braces 1..n times
        //
        // In order for that to actually compile, we must have some argument inside the braces, like
        // this:
        //
        //      $( $some_arg ),*    -   Expands to `a, b, c, ...` as many times as there are
        //                              $some_arg values available. Here `a`, `b` and `c` are
        //                              parameter values provided to the $some_arg argument
        //
        // The repeated pattern can also contain other things
        //
        //      $( $some_arg-1 ),*      -   Expands to `1+a-1, 1+b-1, 1+c-1, ...`
        //      $( 1+$some_arg-1 ),*    -   Expands to `1+a-1, 1+b-1, 1+c-1, ...`
        //      $( $a, $b ),*           -   Valid only if there are exactly the same number of both
        //                                  arguments available
        //
        //
        // Ok! Now that we know the basic syntax, let's break down what this implementation actually
        // expands to, when generating a 3-tuple by calling
        //
        //      define_iterator_tuple!((0, a, A), (1, b, B), (2, c, C))
        //
        // First, these are simple expansions, just repeat an argument as many times as we have them
        // available.
        //
        //      impl<$( $type_name ),+>     =>  impl<A, B, C>
        //      IteratorTuple
        //      for ($( $type_name ),+)     =>  for (A, B, C)
        impl<$( $type_name ),+> IteratorTuple for ($( $type_name ),+)
            where $($type_name: Iterator),+
            // => where A: Iterator,
            //          B: Iterator,
            //          C: Iterator,
            //
            // Above we have a bit more sophisticated repeat pattern. The `$type_name`-argument is
            // used as a part of the pattern `$type_name: Iterator`, which is then repeated as many
            // times as there are type names available.
        {
            type ItemTuple = ($($type_name::Item),+);
            // => type ItemTuple = (A::Item, B::Item, C::Item)
            //
            // Similarly, here we append the type name with `::Item` to generate the item tuple type

            fn next_all(&mut self) -> Option<Self::ItemTuple> {
                match ($(self.$i.next()),+) {
                // => match (self.0.next(), self.1.next(), self.2.next()) {
                //
                // Again, more complex pattern. Here the `$i` is used in the middle of a pattern to
                // indicate which index of a tuple we would like to call `.next()` on. Tuple
                // indexing is particularly tricky for the compiler, this is the reason why `i` must
                // have the `tt`-designator (which in practice is a bit of a "anything"-designator)

                    ($( Some($item_name) ),+) => Some(($( $item_name ),+)),
                    // => (Some(a), Some(b), Some(c)) => Some((a, b, c)),
                    //
                    // Two expansions on the same line. Perfectly valid. Apart from that, nothing
                    // new or special here.

                    _ => None,
                }
            }
        }
    };
}

// Call the macro with n = 2..12 to generate the implementations. Tuples have have the shape
//      (i, item_name, type_name)
// as required by the macro definition. This generates `IteratorTuple` implementations for any
// n-tuples of size n = 2..12 containing only iterators.
implement_iterator_tuple!((0, a, A), (1, b, B));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J), (10, k, K));
implement_iterator_tuple!((0, a, A), (1, b, B), (2, c, C), (3, d, D), (4, e, E), (5, f, F), (6, g, G), (7, h, H), (8, i, I), (9, j, J), (10, k, K), (11, l, L));


/// The concrete iterator
pub struct IterTuple<T>
    where T: IteratorTuple
{
    iterators: T,
}

impl<T> From<T> for IterTuple<T>
    where T: IteratorTuple
{
    fn from(iter_tuple: T) -> Self {
        IterTuple { iterators: iter_tuple }
    }
}

impl<T> Iterator for IterTuple<T>
    where T: IteratorTuple
{
    type Item = T::ItemTuple;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.next_all()
    }
}
