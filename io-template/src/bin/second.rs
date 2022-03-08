//================================================================================
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::Write;
#[allow(unused_imports)]
use std::iter::FromIterator;
static mut GLOBAL_STDIN: *mut std::io::Stdin = 0usize as _;
static mut GLOBAL_STDINLOCK: *mut std::io::StdinLock<'static> = 0usize as _;
static mut GLOBAL_STDOUT: *mut std::io::Stdout = 0usize as _;
static mut GLOBAL_STDOUTLOCK: *mut std::io::StdoutLock<'static> = 0usize as _;
const BUF_SIZE: usize = 1024 * 1024;
static mut GLOBAL_INPUT_BUF: *mut u8 = 0usize as _;
static mut GLOBAL_INPUT_PTR: *const u8 = 0usize as _;
static mut GLOBAL_INPUT_END: *const u8 = 0usize as _;
fn init() {
    unsafe {
        use std::boxed::Box;
        GLOBAL_STDIN = Box::into_raw(Box::new(std::io::stdin()));
        GLOBAL_STDINLOCK = Box::into_raw(Box::new(GLOBAL_STDIN.as_ref().unwrap().lock()));
        GLOBAL_STDOUT = Box::into_raw(Box::new(std::io::stdout()));
        GLOBAL_STDOUTLOCK = Box::into_raw(Box::new(GLOBAL_STDOUT.as_mut().unwrap().lock()));
        let buf = Box::<[u8]>::into_raw(vec![0u8; BUF_SIZE].into_boxed_slice());
        GLOBAL_INPUT_BUF = (*buf).as_mut_ptr();
        GLOBAL_INPUT_PTR = GLOBAL_INPUT_BUF;
        GLOBAL_INPUT_END = GLOBAL_INPUT_BUF;
    }
}
fn peek() -> u8 {
    unsafe {
        use std::io::Read;
        if GLOBAL_INPUT_PTR == GLOBAL_INPUT_END {
            let n = GLOBAL_STDINLOCK
                .as_mut()
                .unwrap()
                .read(std::slice::from_raw_parts_mut(GLOBAL_INPUT_BUF, BUF_SIZE))
                .expect("I/O error");
            GLOBAL_INPUT_PTR = GLOBAL_INPUT_BUF;
            GLOBAL_INPUT_END = GLOBAL_INPUT_PTR.offset(n as isize);
        }
        *GLOBAL_INPUT_PTR
    }
}
fn getchar() -> u8 {
    let c = peek();
    unsafe {
        GLOBAL_INPUT_PTR = GLOBAL_INPUT_PTR.offset(1);
    }
    c
}
fn ungetc() {
    unsafe {
        GLOBAL_INPUT_PTR = GLOBAL_INPUT_PTR.offset(-1);
    }
}
fn skip_whitespaces() {
    loop {
        match getchar() as char {
            ' ' | '\t' | '\n' | '\r' => {}
            _ => {
                ungetc();
                break;
            }
        }
    }
}
trait FastRead {
    fn read() -> Self;
}
macro_rules! read_int_impl {
    ( $signed:expr, $($T:ident,)* ) => {
        $(impl FastRead for $T {
            fn read() -> $T {
                skip_whitespaces(); let is_negative = if $signed && peek() == '-' as u8 { getchar(); true } else { false }; let mut val: $T = 0;
                loop {
                    let c = getchar(); let d = c.wrapping_sub('0' as u8);
                    if d >= 10 { match c as char {
                        ' ' | '\t' | '\n' | '\r' => { ungetc(); return if is_negative { 0-val } else { val }; },
                        _ => panic!("invalid input character: `{}' (code: {})", c as char, c),
                    }}
                    val = 10*val + (d as $T);
                }
            }
        })*
    };
}
macro_rules! read_tuple_impl {
    ( ) => ();
    ( $a:ident, $($name:ident,)* ) => {
        impl<$a:FastRead, $($name:FastRead),*> FastRead for ($a, $($name,)*) { fn read() -> Self { ( $a::read(), $($name::read()),* ) } }
        read_tuple_impl!($($name,)*);
    }
}
macro_rules! snd_arg_impl {
    ($a:expr; $e:expr) => {
        $e
    };
}
macro_rules! read_array_impl {
    () => {};
    ($len:expr, $($lens:expr,)*) => {
        impl <T: FastRead> FastRead for [T; $len] { fn read() -> Self { [ $(snd_arg_impl!($lens; read::<T>())),* ] } }
        read_array_impl!($($lens,)*);
    };
}
unsafe fn extend_vec(v: &mut Vec<u8>, first: *const u8, last: *const u8) {
    let len = usize::wrapping_sub(last as _, first as _);
    v.extend_from_slice(&std::slice::from_raw_parts(first, len));
}
macro_rules! read_string_inplace {
    ($func:ident, $($pattern:pat)|+) => {
        #[allow(unused)] fn $func(s: &mut String) -> bool {
            skip_whitespaces();
            unsafe { let mut ptr = GLOBAL_INPUT_PTR; let end = GLOBAL_INPUT_END; let v = s.as_mut_vec(); v.clear();
                     loop {
                         if ptr == end { extend_vec(v, GLOBAL_INPUT_PTR, end); GLOBAL_INPUT_PTR = GLOBAL_INPUT_END; peek(); ptr = GLOBAL_INPUT_PTR; }
                         match *ptr as char {
                             $($pattern)|+ => { extend_vec(v, GLOBAL_INPUT_PTR, ptr); GLOBAL_INPUT_PTR = ptr; return v.is_empty(); }
                             _ => { ptr = ptr.offset(1); }
                         }
                     }
            }
        }
    };
}
read_string_inplace!(getword, ' ' | '\t' | '\n' | '\r');
read_string_inplace!(getline, '\n');
#[allow(unused)]
fn read_line() -> String {
    let mut s = String::new();
    getline(&mut s);
    s
}
impl FastRead for String {
    fn read() -> String {
        let mut s = String::new();
        getword(&mut s);
        s
    }
}
impl FastRead for char {
    fn read() -> char {
        skip_whitespaces();
        getchar() as char
    }
}
read_int_impl!(false, u8, u16, u32, u64, usize,);
read_int_impl!(true, i8, i16, i32, i64, isize,);
read_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,);
read_array_impl!(12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,);
#[allow(dead_code)]
fn read<T: FastRead>() -> T {
    T::read()
}
#[allow(unused_macros)]
macro_rules! print   { ($($arg:tt)*) => { write!  (unsafe { GLOBAL_STDOUTLOCK.as_mut().unwrap() }, $($arg)*).unwrap() } }
#[allow(unused_macros)]
macro_rules! println { ($($arg:tt)*) => { writeln!(unsafe { GLOBAL_STDOUTLOCK.as_mut().unwrap() }, $($arg)*).unwrap() } }
#[allow(dead_code)]
fn read_vec<T: FastRead>(n: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(read());
    }
    v
}
#[allow(dead_code)]
fn print_iter<I: Iterator<Item = T>, T: std::fmt::Display>(iter: I) {
    let mut iter = iter;
    if let Some(x) = iter.next() {
        print!("{}", x);
        for x in iter {
            print!(" {}", x);
        }
    }
    println!("");
}
#[allow(dead_code)]
fn print_vec<T: std::fmt::Display>(v: &Vec<T>) {
    print_iter(v.iter());
}
#[allow(unused_macros)]
macro_rules! debug { ($a:expr, $($b:expr),*) => (if cfg!(debug_assertions) { println!(concat!("DEBUG: ", stringify!($a), " = {:?}", $(", ", stringify!($b), " = {:?}"),*), $a, $($b),*); } ) }
//================================================================================

#[derive(PartialEq, PartialOrd, Debug, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + -rhs.x,
            y: self.y + -rhs.y,
        }
    }
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn count_integer_points(p: &Point, q: &Point) -> u64 {
    if p.x == q.x {
        (p.y - q.y).abs() as u64 + 1
    } else if p.y == q.y {
        (p.x - q.x).abs() as u64 + 1
    } else {
        gcd((p.x - q.x).abs() as _, (p.y - q.y).abs() as _) as u64 + 1
    }
}

fn ccw(a: Point, b: Point) -> i64 {
    a.x * b.y - a.y * b.x
}

fn ccw3(a: &Point, b: &Point, c: &Point) -> i64 {
    let x = ccw(*b - *a, *c - *a);
    if x > 0 {
        1
    } else if x == 0 {
        0
    } else {
        -1
    }
}

fn boxes_intersect(a1: &Point, a2: &Point, b1: &Point, b2: &Point) -> bool {
    return !(a1.x > b2.x || a2.x < b1.x || a1.y > b2.y || a2.y < b1.y);
}

fn segments_cross(a1: &Point, a2: &Point, b1: &Point, b2: &Point) -> bool {
    boxes_intersect(
        &Point {
            x: min(a1.x, a2.x),
            y: min(a1.y, a2.y),
        },
        &Point {
            x: max(a1.x, a2.x),
            y: max(a1.y, a2.y),
        },
        &Point {
            x: min(b1.x, b2.x),
            y: min(b1.y, b2.y),
        },
        &Point {
            x: max(b1.x, b2.x),
            y: max(b1.y, b2.y),
        },
    ) && ccw3(a1, a2, b1) * ccw3(a1, a2, b2) <= 0
        && ccw3(b1, b2, a1) * ccw3(b1, b2, a2) <= 0
}

fn intersect(r1: &Point, r2: &Point, s1: &Point, s2: &Point) -> Option<Point> {
    if !segments_cross(r1, r2, s1, s2) {
        return None;
    }

    let a1 = r2.y - r1.y;
    let b1 = r1.x - r2.x;
    let c1 = a1 * r1.x + b1 * r1.y;

    let a2 = s2.y - s1.y;
    let b2 = s1.x - s2.x;
    let c2 = a2 * s1.x + b2 * s1.y;

    let det = a1 * b2 - a2 * b1;
    if det == 0 {
        return None;
    }
    let x = b2 * c1 - b1 * c2;
    let y = a1 * c2 - a2 * c1;
    if x % det == 0 && y % det == 0 {
        Some(Point {
            x: x / det,
            y: y / det,
        })
    } else {
        None
    }
}

fn main() {
    init();
    let n: usize = read();
    let mut lines = Vec::new();
    for _ in 0..n {
        let (a, b, c, d) = read();
        lines.push((Point { x: a, y: b }, Point { x: c, y: d }));
    }
    let mut ans = 0;
    let mut dupes: HashMap<Point, HashSet<usize>> = HashMap::new();
    for i in 0..n {
        let (ref r1, ref r2) = lines[i];
        ans += count_integer_points(&r1, &r2);
        //println!("pts on {:?}--{:?}: {}", r1, r2, count_integer_points(&r1, &r2));
        for j in 0..i {
            let (ref s1, ref s2) = lines[j];
            if let Some(p) = intersect(&r1, &r2, &s1, &s2) {
                //println!("intersection between {:?} {:?} and {:?} {:?}: {:?}", r1, r2, s1, s2, &p);
                let hs = dupes.entry(p).or_insert_with(HashSet::new);
                hs.insert(i);
                hs.insert(j);
            }
        }
    }
    for (p, idx) in &dupes {
        debug!(p, idx);
    }
    for (_, s) in &dupes {
        //println!("ans ({}) -= {}", ans, s.len());
        ans -= (s.len() - 1) as u64;
    }
    println!("{}", ans);
}
