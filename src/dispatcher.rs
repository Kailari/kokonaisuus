use crate::component_storage::ComponentStorage;
use crate::components::{AccelerationComponent, FrictionComponent, PositionComponent, VelocityComponent};
use crate::systems::{ApplyAccelerationSystem, ApplyFrictionSystem, ApplyVelocitySystem, PrintPositionsSystem, PrintStateSystem, System};

pub struct Dispatcher {
    print_state: PrintStateSystem,
    print_positions: PrintPositionsSystem,
    apply_acceleration: ApplyAccelerationSystem,
    apply_friction: ApplyFrictionSystem,
    apply_velocity: ApplyVelocitySystem,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            print_state: PrintStateSystem,
            print_positions: PrintPositionsSystem,
            apply_acceleration: ApplyAccelerationSystem,
            apply_friction: ApplyFrictionSystem,
            apply_velocity: ApplyVelocitySystem,
        }
    }

    // Fun fact:
    //  The component storage does not need to be mutable even though we technically mutate its
    //  contents. This is again due to `RefCell` moving the ownership of the data elsewhere, so
    //  mutating the data inside the cell isn't actually mutating the storage (as it does not own
    //  the data, it just holds a pointer to it). We are declaring the storage as mutable anyway,
    //  because it would be quite confusing that the contents of the storage mutate while
    //  the storage is being borrowed immutably.
    pub fn dispatch(&self, storage: &mut ComponentStorage) {
        // This is the actual reason why `RefCells` are needed (If you haven't take a look at
        // `component_storage.rs` now).
        //
        // So, instead of referring directly to the fields on the storage, we are using `fetch_ref`
        // and `fetch_mut` to get references to the component vectors. As returned values are of
        // types `Ref<T>` and `RefMut<T>`, we use `as_ref` and `as_mut` to convert them into raw
        // references (systems still want them as `&Vec<C>` not `Ref<Vec<C>>` and `as_ref`
        // performs the conversion)
        //
        // However, there can only be one simultaneous mutable borrow of an object at once (multiple
        // simultaneous immutable borrows are legal and called "aliasing"). Additionally, we cannot
        // borrow as immutable if we have already borrowed as mutable. This means that only the
        // `print_state` and `print_positions` tick calls would be legal if storage was allowed to
        // be borrowed mutably (those two systems make only immutable borrows, they are read-only).
        println!("State before tick:");
        self.print_state.tick((
            storage.fetch_ref::<PositionComponent>().as_ref(),
            storage.fetch_ref::<VelocityComponent>().as_ref(),
            storage.fetch_ref::<AccelerationComponent>().as_ref(),
            storage.fetch_ref::<FrictionComponent>().as_ref(),
        ));

        // This is example of a situation where we have more information about execution of the
        // program than the compiler. Why does the compiler prevent this if `storage` was borrowed
        // mutably?
        //  1.  In this situation, `storage.fetch_mut()` would have `&mut self` parameter, meaning
        //      that we borrowed it mutably when evaluating the first parameter
        //  2.  Borrow does not end when the method returns, as the returned value refers to
        //      some storage vector, obtained from the borrowed storage (cannot release the borrow
        //      before all references to borrowed value are released)
        //  3.  Now, the second parameter `storage.fetch_ref()` has `&self`, which means that we are
        //      trying to perform an immutable borrow, but the first parameter still holds mutable
        //      borrow! We cannot borrow immutably while there a mutable one still exists!
        //
        // But this is wrong! We clearly see that the first parameter ends up getting storage vector
        // for `VelocityComponent` and second parameter gets `AccelerationComponent`. There is no
        // risk of simultaneous mutable access, but the compiler does not know that; all it sees is
        // that there is simultaneous mutable and immutable access into `storage` itself, and that's
        // illegal.
        //
        // So, what have we done to circumvent this? The `RefCell` sort-of allows us to *bypass the
        // compile-time borrow checks* and perform the borrow-checking on runtime instead. Yes, this
        // again is a powerful ability, best used sparingly as there is a whole lot that can go
        // wrong.
        //
        // In this case, this allows us to always borrow the storage immutably. Why? We are not
        // mutating the storage anymore. All we are doing with the storage, is getting the stored
        // *pointer* inside the `RefCell` and then borrowing the memory that cell points to. This
        // borrow happens purely during the runtime and does not require storage to be mutable, thus
        // we are allowed to borrow individual storage vectors as we wish.
        //
        // However, if one system holds any storage vector as mutable, and another system tries to
        // borrow that before the first system has a chance to release the borrow, the program will
        // crash with a panic. That is, the borrow rules are still in place, they just are not
        // checked at compile time.
        self.apply_acceleration.tick((
            storage.fetch_mut::<VelocityComponent>().as_mut(),
            storage.fetch_ref::<AccelerationComponent>().as_ref(),
        ));

        self.apply_friction.tick((
            storage.fetch_mut::<VelocityComponent>().as_mut(),
            storage.fetch_ref::<FrictionComponent>().as_ref(),
        ));

        self.apply_velocity.tick((
            storage.fetch_mut::<PositionComponent>().as_mut(),
            storage.fetch_ref::<VelocityComponent>().as_ref(),
        ));

        println!("\nPositions after tick:");
        self.print_positions.tick(
            storage.fetch_ref::<PositionComponent>().as_ref()
        );
    }
}
