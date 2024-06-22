use std::rc::Rc;

trait ToValue {
    fn as_value(&self) -> Value;
}

#[derive(Clone, Debug)]
enum Value {
    L(Rc<Lambda>),
    I(Index),
    B(Rc<Beta>),
}

#[derive(Copy, Clone, Debug)]
struct Index {
    pub nth: usize,
}

impl ToValue for Index {
    fn as_value(&self) -> Value {
        Value::I(*self)
    }
}

#[derive(Clone, Debug)]
struct Beta {
    pub to_apply: Value,
    pub value: Value,
}
impl ToValue for Beta {
    fn as_value(&self) -> Value {
        Value::B(self.clone().into())
    }
}

#[derive(Clone, Debug)]
struct Lambda {
    pub value: Value,
}

impl ToValue for Lambda {
    fn as_value(&self) -> Value {
        Value::L(self.clone().into())
    }
}

fn to_lambda(v: Value, curried: &Vec<Rc<Lambda>>) -> Rc<Lambda> {
    println!("{curried:?}");
    match v {
        Value::L(a) => a,
        Value::I(i) => curried[curried.len() - 1 - i.nth].clone(),
        Value::B(b) => reduce(b, curried),
    }
}

fn reduce(b: Rc<Beta>, curried: &Vec<Rc<Lambda>>) -> Rc<Lambda> {
    // to perform a beta reduction, we need to make sure the values are lambdas
    let arg = to_lambda(b.value.clone(), curried);
    println!("{arg:?}");
    let mut new = Vec::new();
    new.push(arg);
    to_lambda(b.to_apply.clone(), &new)
}

// TODO: i have realised that our lambda model is actually wrong:
// given the expression identity(false) we can see it has lambdas
// beta (\1, \\2) : true = \\1, false = \\2
// however it would make more sense to consider it as if we were using alpha conversion
// beta(\a: a, \b: \c: b)
// so in order to avoid alpha conversion and be number we can automatically de index it
// in order to do this it makes sense to traverse the lambdas and store a queue of names
// \1 \\2
// see first lambda, add a to queue in index 1,
// \a: 1 \\2
// see 1, we perform the alpha conversion 1 -> a
// \a: a \\2
// see closing of first lambda, pop a
// see outer false lambda, add b to queue index 1
// \a: a \b: \2
// see inner false lambda, add c to queue index 1, now b is index 2
// \a: a \b: \c: 2
// see 2, we perform the alpha conversion 2 -> b
// \a: a \b: \c: b
// see end, pop queue, repeat
// now the lambdas should be trivial to evaluate, just store the names in a map
fn fixed_lambda() {
    todo!();
}

fn main() {
    let identity: Lambda = Lambda {
        value: Index { nth: 0 }.as_value(),
    };
    let truth = Lambda {
        value: Lambda {
            value: Index { nth: 1 }.as_value(),
        }
        .as_value(),
    };
    let p1 = Beta {
        to_apply: identity.as_value(),
        value: truth.as_value(),
    };
    let p2 = Beta {
        to_apply: Beta {
            to_apply: truth.as_value(),
            value: identity.as_value(),
        }
        .as_value(),
        value: truth.as_value(),
    };
    let mut a = Vec::new();
    let v = reduce(p1.into(), &mut a);
    println!("result: {v:?}");
    let v = reduce(p2.into(), &mut a);
    println!("result: {v:?}");
}
