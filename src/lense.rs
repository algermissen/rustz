use std::marker::PhantomData;


pub trait Lense {
    type Object;
    type Value;

    fn get(&self, obj: &Self::Object) -> Self::Value;
    fn set(&self, obj: &Self::Object, val: &Self::Value) -> Self::Object;
}

pub struct L<O, V, G, S> {
    g: G,
    s: S,
    o: PhantomData<O>,
    v: PhantomData<V>,
}

impl<O, V, G, S> L<O, V, G, S> {
    pub fn new(x: G, y: S) -> L<O, V, G, S> {
        L {
            g: x,
            s: y,
            o: PhantomData,
            v: PhantomData,
        }
    }
}
pub fn lense<O, V, G, S>(x: G, y: S) -> L<O, V, G, S> {
    L::new(x, y)
}


impl<O, V, G, S> Lense for L<O, V, G, S>
where
    G: Fn(&O) -> V,
    S: Fn(&O, &V) -> O,
{
    type Object = O;
    type Value = V;

    fn get(&self, obj: &(Self::Object)) -> Self::Value {
        (self.g)(obj)
    }
    fn set(&self, obj: &Self::Object, val: &Self::Value) -> Self::Object {
        (self.s)(obj, val)
    }
}

pub struct Compose<'composite, OUTER, INNER, VALUE, L1, L2>
where
    L1: 'composite + Lense<Object = OUTER, Value = INNER>,
    L2: 'composite + Lense<Object = INNER, Value = VALUE>,
{
    outer: &'composite L1,
    inner: &'composite L2,
}

impl<'composite, OUTER, INNER, VALUE, L1, L2> Lense
    for Compose<'composite, OUTER, INNER, VALUE, L1, L2>
where
    L1: 'composite + Lense<Object = OUTER, Value = INNER>,
    L2: 'composite + Lense<Object = INNER, Value = VALUE>,
{
    type Object = OUTER;
    type Value = VALUE;

    fn get(&self, obj: &Self::Object) -> Self::Value {
        self.inner.get(&(self.outer.get(obj)))
    }
    fn set(&self, obj: &Self::Object, val: &Self::Value) -> Self::Object {
        self.outer.set(
            obj,
            &(self.inner.set(&(self.outer.get(obj)), val)),
        )
    }
}

// TBD: need an idea how to do this...
//pub fn compose<'composite,OUTER,INNER,VALUE,L1,L2>(outer:&L1,inner:&L2) -> Compose<'composite,OUTER,INNER,VALUE,L1,L2>
//where L1:'composite+Lense<Object=OUTER,Value=INNER>,
//      L2:'composite+Lense<Object=INNER,Value=VALUE> {
//    Compose{outer:outer,inner:inner}
//      }


mod tests {
    use super::Lense;
    use super::L;
    use super::Compose;
    use super::*;

    #[test]
    fn it_works() {

        #[derive(Debug, Clone, Copy)]
        struct Point {
            x: i32,
            y: i32,
        }
        #[derive(Debug, Clone, Copy)]
        struct Turtle {
            id: i32,
            position: Point,
        }


        let turtle_position = lense(|turtle: &Turtle| turtle.position, |turtle: &Turtle,
         pos: &Point| {
            Turtle {
                id: turtle.id,
                position: *pos,
            }
        });

        let point_x = lense(|point: &Point| point.x, |point: &Point, x: &i32| {
            Point { x: *x, y: point.y }
        });

        let turtle_point_x = Compose {
            outer: &turtle_position,
            inner: &point_x,
        };

        let t1 = Turtle {
            id: 1,
            position: Point { x: 0, y: 0 },
        };
        let t11 = Turtle {
            id: 1,
            position: Point { x: 0, y: 0 },
        };
        let p = turtle_position.get(&t1);
        let t2 = turtle_position.set(&t11, &Point { x: 9, y: 9 });

        let turtle_3 = Turtle {
            id: 3,
            position: Point { x: 0, y: 0 },
        };

        let turtle_3_moved = turtle_point_x.set(&turtle_3, &100);

        println!("##### Result: {:?} ", p);
        println!("##### Result: {:?} ", t2);
        println!("##### Result: {:?} ", turtle_3_moved);
        //println!("##### Result: {:?} ", v2);
        assert!(true);
    }
}
