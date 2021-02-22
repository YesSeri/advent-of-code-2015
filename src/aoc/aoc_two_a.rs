use std::fs;
#[derive(Debug)]
struct Gift {
    h: u32,
    w: u32,
    l: u32,
}
impl Gift {
    fn package_paper(&self) -> u32 {
        let a1 = 2 * self.h * self.w;
        let a2 = 2 * self.h * self.l;
        let a3 = 2 * self.l * self.w;
        a1 + a2 + a3
    }
    fn slack_paper(&self) -> u32 {
        let s1;
        let s2;
        let other;
        if &self.h < &self.w {
            s1 = &self.h;
            other = &self.w;
        } else {
            s1 = &self.w;
            other = &self.h;
        }
        if &self.l < other {
            s2 = &self.l;
        } else {
            s2 = other;
        }
        s1 * s2
    }
    pub fn total_paper(&self) -> u32 {
        self.package_paper() + self.slack_paper()
    }
}
impl From<&str> for Gift {
    fn from(s: &str) -> Self {
        let dist: Vec<&str> = s.split("x").collect();
        let values: Vec<u32> = dist.iter().map(|s| s.parse().unwrap()).collect();

        Gift { w: values[0], h: values[1], l: values[2] }
    }
}
pub fn solution_two_a() {

    let contents = fs::read_to_string("./two.txt").unwrap();
    let mut gift_vec: Vec<Gift> = Vec::new();
    for line in contents.lines() {
        let gift: Gift = Gift::from(line);
        gift_vec.push(gift);
    }
    let mut sum = 0;

    for g in gift_vec{
        sum += g.total_paper();
    }
    dbg!(sum);

}

