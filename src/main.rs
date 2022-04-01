fn main() {
    let world = lay_tracing::World::new();
    let option =
        lay_tracing::RenderOption { campus_width: 256 * (16. / 9.) as u32, campus_height: 256 };
    world.render(option).save("test.png").unwrap();
}
