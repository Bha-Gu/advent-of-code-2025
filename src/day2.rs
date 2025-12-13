pub static EX_INPUT1: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

pub static INPUT1: &str = r"3299143-3378031,97290-131156,525485-660941,7606-10180,961703-1031105,6856273537-6856492968,403537-451118,5330-7241,274725-384313,27212572-27307438,926609-954003,3035-3822,161-238,22625-31241,38327962-38415781,778-1155,141513-192427,2-14,47639-60595,4745616404-4745679582,1296-1852,80-102,284-392,4207561-4292448,404-483,708177-776613,65404-87389,5757541911-5757673432,21-38,485-731,1328256-1444696,11453498-11629572,41-66,2147-3014,714670445-714760965,531505304-531554460,4029-5268,3131222053-3131390224";

pub fn p2(input: &str) -> usize {
    let mut count = 0;
    input.split(',').for_each(|range| {
        let (low, hi) = range.split_once('-').unwrap();
        let (low, hi) = (low.parse::<usize>().unwrap(), hi.parse::<usize>().unwrap());
        for id in low..=hi {
            let ids: String = id.to_string();
            for w in 1..=(ids.len() / 2) {
                if ids.len() % w == 0 {
                    let bytes = ids.bytes().collect::<Vec<_>>();
                    let mut slices = bytes.windows(w).step_by(w);
                    let first = slices.next();
                    let is_pattern = slices.all(|x| x.to_vec() == first.unwrap().to_vec());
                    if is_pattern {
                        count += id;
                        break;
                    }
                }
            }
        }
    });
    count
}

pub fn p1(input: &str) -> usize {
    let mut count = 0;
    input.split(',').for_each(|range| {
        let (low, hi) = range.split_once('-').unwrap();
        let (low, hi) = (low.parse::<usize>().unwrap(), hi.parse::<usize>().unwrap());
        for id in low..=hi {
            let ids: String = id.to_string();
            if ids.len() % 2 != 0 {
                continue;
            }
            let tmp = ids.split_at(ids.len() / 2);
            if tmp.0 == tmp.1 {
                count += id;
            }
        }
    });
    count
}
