#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn example() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
".lines();

        let mut grid = Grid::new(input);

        let mut sync_step = -1;
        for i in 0..300 {
            grid.iterate();

            // println!("{}", grid.flashes);
            if grid.flashes == 100 {
                sync_step = i+1;
                break;
            }
        }
        grid.print();

        assert_eq!(195, sync_step)
    }

    #[test]
    fn small_example() {
        let input = "11111
19991
19191
19991
11111
".lines();

        let mut grid = Grid::new(input);

        grid.print();
        grid.iterate();
        grid.print();
        grid.iterate();
        grid.print();


        print!("{}", 0);
    }
}