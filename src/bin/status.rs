use git2::{Repository, StatusOptions};

fn main() {
    let repo = Repository::open(".").expect("open repository");
    let mut opt = StatusOptions::new();
    let sts = repo.statuses(Some(&mut opt)).expect("statuses");
    for entry in sts.iter() {
        println!("{} {:?}", entry.path().expect("path"), entry.status());
    }
}
