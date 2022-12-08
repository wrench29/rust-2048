mod scene;

fn main() {
    let mut scene = scene::Scene::new();
    scene.init(800, 600).unwrap();
}
