#[test]
fn can_run_it_works() {

    let it_works = "It Works!".to_owned();

    assert_eq!(crate::example::it_works(), it_works);

}