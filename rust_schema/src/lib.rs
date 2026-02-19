use facet::Facet;

#[derive(Facet, Debug, Default)]
struct Config {
    x: (u32, String),
    y: u32,
}




#[test]
fn it_works() {

    dbg!(Config::SHAPE);


    let config = Config::default();

}
