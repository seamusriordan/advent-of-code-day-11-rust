use day_11_rust::Grid;

fn main() {
    let input = "7232374314
8531113786
3411787828
5482241344
5856827742
7614532764
5311321758
1255116187
5821277714
2623834788
".lines();

    let mut grid = Grid::new(input);

    let mut sync_step = -1;
    for i in 0..500 {
        grid.iterate();

        // println!("{}", grid.flashes);
        if grid.flashes == 100 {
            sync_step = i+1;
            break;
        }
    }
    grid.print();

    println!("step {}", sync_step);
}
