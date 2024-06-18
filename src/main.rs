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
