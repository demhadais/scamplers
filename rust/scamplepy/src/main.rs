fn main() {
    let stub_info = scamplepy::stub_info().unwrap();
    stub_info.generate().unwrap();
}
