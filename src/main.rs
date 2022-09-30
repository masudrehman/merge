fn main() {
    let mut a = [2, 8, 15, 18, 100];
    let mut b = [5, 9, 12, 17, 19, 25, 30];

    let m = a.len() as i32;
    let n = b.len() as i32;

    let v = merge(&mut a, &mut b, m, n);

    println!("Merge = {:?}", v);

}

fn merge(a: &mut [i32], b: &mut [i32], m: i32, n: i32) -> Vec<i32> {

    let mut v: Vec<i32> = Vec::new();

    let mut i = 0;
    let mut j = 0;
    // let mut k = 0;

    while i < m && j < n {
        if a[i as usize] < b[j as usize] {
            v.push(a[i as usize]);
            i = i + 1;
        } else {
            v.push(b[j as usize]);
            j = j + 1;
        }
    }

    while i < m {
        v.push(a[i as usize]);
        i = i + 1;
    }

    while j < n {
        v.push(b[j as usize]);
        j = j + 1;
    }

    return v;
}