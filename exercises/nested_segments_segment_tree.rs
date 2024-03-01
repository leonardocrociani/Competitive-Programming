use segment_tree::SegmentPoint;
use segment_tree::ops::Add;

pub fn nested_segments_segment_tree (arr: &mut Vec<(u32, u32)>) {
    let max: u32= arr.iter().map(|e| e.1).max().unwrap(); 
    let mut endings = vec![0; (max+1) as usize];
    
    for i in 0..arr.len() {
        let (_, end) = arr[i];
        endings[end as usize] += 1;
    }
    
    let mut tree = SegmentPoint::build(endings, Add);

    for item in arr {
        let (start, end) = item;

        let res = tree.query(*start as usize, *end as usize);
        tree.modify(*end as usize, -1);
        println!("{res}");
    }
}
