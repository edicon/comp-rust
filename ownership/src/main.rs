// Ownerchip(Part1): https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b
mod print_macro;
mod ownership_part1;
mod ownership_part2;
// cow: Copy on Write
mod cow_demo;

fn main() {
    print_macro::run();
    ownership_part1::run();
    ownership_part2::run();
    cow_demo::run();
}
