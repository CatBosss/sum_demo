//实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option<u32>，溢出时返回None；

fn total(list: &[u32])->Option<u32>{
    let mut  x = list.iter();
    x.try_fold(0u32,|acc,&x| acc.checked_add(x))
}

fn main() {
    let list1 = [1,2,3,4,5,4294967295,4294967295,4294967295];
    let list2 = [1,2,3,4,5,6,7,8,9];
    let t1 = total(&list1);
    let t2 = total(&list2);
    match t1 {
        Some(c)=>println!("{c}"),
        None =>{
            println!("溢出None")
        }
    }
    match t2 {
        Some(c)=>println!("和为{c}"),
        None =>{
            println!("None")
        }
    }
}
