use std::{collections::HashMap, str::Chars};

pub fn ans1(input: &str) -> u64 {
    ai(parse(input))
}

#[derive(Debug)]
struct Inst {
    name: String,
    conds: Vec<Cond>,
}

#[derive(Debug)]
struct Cond {
    expr: Option<Expr>,
    // a name or "A" or "R"
    dest: String,
}

#[derive(Debug)]
struct Expr {
    lhs: XMAS,
    cmp: Cmp,
    rhs: u64,
}

#[derive(Debug)]
enum Cmp {
    Gt,
    Lt,
}

#[derive(Debug)]
struct Param {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug)]
enum XMAS {
    X,
    M,
    A,
    S,
}

fn ai(spec: (Vec<Inst>, Vec<Param>)) -> u64 {
    let mut map = HashMap::new();
    for i in spec.0 {
        map.insert(i.name.to_owned(), i);
    }

    let mut total = 0;
    for p in &spec.1 {
        if approval(&map, p) {
            let single = p.x + p.m + p.a + p.s;
            // dbg!(single);
            total += single;
        }
    }

    total
}

fn approval(insts: &HashMap<String, Inst>, param: &Param) -> bool {
    // start with in
    let mut inst = insts.get("in").unwrap();
    loop {
        let mut dst = String::new();
        // front to back
        for cond in &inst.conds {
            match &cond.expr {
                Some(exp) => {
                    let Expr { lhs, cmp, rhs } = exp;
                    let var = match lhs {
                        XMAS::X => param.x,
                        XMAS::M => param.m,
                        XMAS::A => param.a,
                        XMAS::S => param.s,
                    };

                    let pass = match cmp {
                        Cmp::Gt => &var > rhs,
                        Cmp::Lt => &var < rhs,
                    };

                    if pass {
                        dst = cond.dest.to_owned();
                        break;
                    }
                }
                None => dst = cond.dest.to_owned(),
            }
        }

        match dst.as_str() {
            "A" => return true,
            "R" => return false,
            _ => {
                // go on with next inst
                inst = insts.get(&dst).unwrap();
            }
        }
    }
}

fn parse(input: &str) -> (Vec<Inst>, Vec<Param>) {
    let mut parsing_inst = true;
    let mut insts: Vec<Inst> = vec![];
    let mut params: Vec<Param> = vec![];
    for l in input.lines() {
        //dbg!(l);
        if parsing_inst {
            if l.len() == 0 {
                parsing_inst = false;
                continue;
            } else {
                let mut chars = l.chars().peekable();
                let mut name = String::new();
                loop {
                    let ch = chars.next().unwrap();
                    if ch == '{' {
                        break;
                    } else {
                        name.push(ch);
                    }
                }
                //dbg!(&name);

                let mut conds: Vec<Cond> = vec![];
                loop {
                    let mut ch = chars.next().unwrap();
                    let maybe_cmp = chars.peek().unwrap();
                    let xmas = if maybe_cmp == &'<' || maybe_cmp == &'>' {
                        match ch {
                            'x' => XMAS::X,
                            'm' => XMAS::M,
                            'a' => XMAS::A,
                            's' => XMAS::S,
                            _ => unreachable!(),
                        }
                    } else {
                        let mut end_clause = String::new();
                        end_clause.push(ch);
                        loop {
                            ch = chars.next().unwrap();
                            if ch != '}' {
                                end_clause.push(ch);
                            } else {
                                break;
                            }
                        }
                        //dbg!(&end_clause);
                        debug_assert_eq!(ch, '}');
                        conds.push(Cond {
                            expr: None,
                            dest: end_clause,
                        });
                        break;
                    };

                    let cmp = match chars.next().unwrap() {
                        '<' => Cmp::Lt,
                        '>' => Cmp::Gt,
                        c => unreachable!("{}", c),
                    };

                    let mut number_buf = String::new();
                    // number
                    loop {
                        ch = chars.next().unwrap();
                        if ch.is_numeric() {
                            number_buf.push(ch);
                        } else {
                            break;
                        }
                    }

                    debug_assert_eq!(ch, ':');

                    let mut dest = String::new();
                    // dest
                    loop {
                        ch = chars.next().unwrap();
                        if ch.is_alphabetic() {
                            dest.push(ch);
                        } else {
                            break;
                        }
                    }

                    let c = Cond {
                        expr: Some(Expr {
                            lhs: xmas,
                            cmp,
                            rhs: number_buf.parse().unwrap(),
                        }),
                        dest,
                    };
                    //dbg!(&c);
                    conds.push(c);

                    debug_assert_eq!(ch, ',');
                }

                //dbg!(&conds);
                insts.push(Inst { name, conds });
            }
        } else {
            //println!("ipt: {}", l);
            let mut chs = l.chars();
            // first
            debug_assert_eq!(chs.next().unwrap(), '{');
            debug_assert_eq!(chs.next().unwrap(), 'x');
            let x = chs_to_num(&mut chs);
            debug_assert_eq!(chs.next().unwrap(), 'm');
            let m = chs_to_num(&mut chs);
            debug_assert_eq!(chs.next().unwrap(), 'a');
            let a = chs_to_num(&mut chs);
            debug_assert_eq!(chs.next().unwrap(), 's');
            let s = chs_to_num(&mut chs);
            // last
            //debug_assert_eq!(chs.next().unwrap(), '}');

            params.push(Param { x, m, a, s });
        }
    }

    //dbg!(insts);
    //dbg!(params);
    (insts, params)
}

fn chs_to_num(chs: &mut Chars) -> u64 {
    debug_assert_eq!(chs.next().unwrap(), '=');
    let mut buf = String::new();
    while let Some(c) = chs.next() {
        if c == ',' || c == '}' {
            break;
        }

        buf.push(c);
    }
    //dbg!(&buf);

    buf.parse().unwrap()
}

#[test]
fn test1() {
    assert_eq!(
        ans1(
            r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#
        ),
        19114
    );
}

#[test]
fn run1() {
    let q = include_str!("../inputs/input19.txt");
    println!("{}", ans1(&q));
}

pub fn ans2(input: &str) -> () {
    todo!();
}

#[test]
fn test2() {
    assert_eq!(ans2(r#""#), todo!());
}

#[test]
fn run2() {
    // let q = include_str!("../inputs/input.txt");
    // println!("{}", ans2(&q));
}
