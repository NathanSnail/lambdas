use std::rc::Rc;

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

#[derive(Clone, Debug)]
struct Beta {
    pub to_apply: Value,
    pub value: Value,
}

#[derive(Clone, Debug)]
struct Lambda {
    pub value: Value,
}

fn evaluate(v: Value, curried: &Vec<Rc<Lambda>>) -> Rc<Lambda> {
    println!("{curried:?}");
    match v {
        Value::L(a) => a,
        Value::I(i) => curried[curried.len() - 1 - i.nth].clone(),
        Value::B(b) => reduce(b, curried),
    }
}

fn reduce(b: Rc<Beta>, curried: &Vec<Rc<Lambda>>) -> Rc<Lambda> {
    // to perform a beta reduction, we need to make sure the values are lambdas
    let arg = evaluate(b.value.clone(), curried);
    println!("{arg:?}");
    let mut new = Vec::new();
    new.push(arg);
    evaluate(b.to_apply.clone(), &new)
}

fn main() {
    let identity: Lambda = Lambda {
        value: Value::I(Index { nth: 0 }),
    };
    let truth = Lambda {
        value: Value::L(
            Lambda {
                value: Value::I(Index { nth: 1 }),
            }
            .into(),
        ),
    };
    let p1 = Beta {
        to_apply: Value::L(identity.clone().into()),
        value: Value::L(truth.clone().into()),
    };
    let p2 = Beta {
        to_apply: Value::B(
            Beta {
                to_apply: Value::L(truth.clone().into()),
                value: Value::L(identity.clone().into()),
            }
            .into(),
        ),
        value: Value::L(truth.clone().into()),
    };
    let mut a = Vec::new();
    let v = reduce(p1.into(), &mut a);
    println!("result: {v:?}");
    let v = reduce(p2.into(), &mut a);
    println!("result: {v:?}");
}
